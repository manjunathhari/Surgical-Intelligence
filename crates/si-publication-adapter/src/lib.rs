//! Offline adapter for a user's explicitly retained publication library.
//! This crate never downloads papers or attempts to bypass publisher access controls.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use si_core::{EvidenceClaim, PortableStateV2, Provenance, PublicationRecord, SourceArtifact};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use uuid::Uuid;

pub const PUBLICATION_IMPORT_FORMAT: &str = "surgical-intelligence-publication-import";
pub const PUBLICATION_IMPORT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicationImport {
    pub format: String,
    pub version: u32,
    pub exported_at: String,
    pub source_system: String,
    pub retained_by: Option<String>,
    pub papers: Vec<ImportedPaper>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportedPaper {
    pub source_ref: String,
    pub doi: Option<String>,
    pub pmid: Option<String>,
    pub pmcid: Option<String>,
    pub title: String,
    #[serde(default)]
    pub authors: Vec<String>,
    pub journal: Option<String>,
    pub publication_year: Option<i32>,
    pub version: Option<String>,
    pub retention_state: String,
    pub decision_at: Option<String>,
    pub file: Option<RetainedFile>,
    #[serde(default)]
    pub claims: Vec<ImportedClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetainedFile {
    pub storage_ref: String,
    pub mime_type: String,
    pub sha256: String,
    pub byte_length: u64,
    pub export_authorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportedClaim {
    pub source_ref: String,
    pub claim: String,
    pub locator: Option<String>,
    pub population: Option<String>,
    pub intervention: Option<String>,
    pub comparator: Option<String>,
    pub outcome: Option<String>,
    pub effect_estimate: Option<String>,
    pub uncertainty: Option<String>,
    #[serde(default)]
    pub limitations: Vec<String>,
    pub epistemic_status: String,
    pub review_state: String,
}

fn stable_id(key: &str) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_URL, key.as_bytes())
}

fn safe_relative_path(raw: &str) -> bool {
    let path = std::path::Path::new(raw);
    !raw.trim().is_empty()
        && !path.is_absolute()
        && !path.components().any(|c| matches!(c, std::path::Component::ParentDir | std::path::Component::RootDir | std::path::Component::Prefix(_)))
        && !raw.contains("://") && !raw.contains('?') && !raw.contains('#')
}

fn validate(input: &PublicationImport) -> Result<(), String> {
    if input.format != PUBLICATION_IMPORT_FORMAT || input.version != PUBLICATION_IMPORT_VERSION {
        return Err("unsupported publication import format/version".into());
    }
    DateTime::parse_from_rfc3339(&input.exported_at).map_err(|e| format!("invalid exported_at: {e}"))?;
    if input.source_system.trim().is_empty() { return Err("source_system is required".into()); }
    let mut papers = std::collections::HashSet::new();
    for paper in &input.papers {
        if paper.source_ref.trim().is_empty() || !papers.insert(&paper.source_ref) { return Err("paper source_ref must be non-empty and unique".into()); }
        if paper.title.trim().is_empty() { return Err(format!("paper {} has empty title", paper.source_ref)); }
        if !matches!(paper.retention_state.as_str(), "kept"|"hidden"|"unreviewed") { return Err(format!("paper {} has invalid retention_state", paper.source_ref)); }
        if let Some(t) = &paper.decision_at { DateTime::parse_from_rfc3339(t).map_err(|e| format!("paper {} invalid decision_at: {e}", paper.source_ref))?; }
        if let Some(file) = &paper.file {
            if !file.export_authorized { return Err(format!("paper {} file is not authorized for export", paper.source_ref)); }
            if !safe_relative_path(&file.storage_ref) { return Err(format!("paper {} has unsafe storage_ref", paper.source_ref)); }
            if file.sha256.len()!=64 || !file.sha256.bytes().all(|b| b.is_ascii_hexdigit()) { return Err(format!("paper {} has invalid SHA-256", paper.source_ref)); }
        }
        let mut claims = std::collections::HashSet::new();
        for claim in &paper.claims {
            if claim.source_ref.trim().is_empty() || !claims.insert(&claim.source_ref) || claim.claim.trim().is_empty() {
                return Err(format!("paper {} has empty or duplicate claim ref/text", paper.source_ref));
            }
            if !matches!(claim.epistemic_status.as_str(), "observed"|"reported"|"inferred"|"hypothesis") { return Err(format!("paper {} claim {} has invalid epistemic status", paper.source_ref, claim.source_ref)); }
            if claim.review_state.trim().is_empty() { return Err(format!("paper {} claim {} has empty review state", paper.source_ref, claim.source_ref)); }
        }
    }
    Ok(())
}

/// Import metadata and claims only. The paper bytes are referenced in source_artifacts;
/// bundle verification is a separate explicit operation.
pub fn map_publications(input: &PublicationImport, state: &mut PortableStateV2) -> Result<(), String> {
    validate(input)?;
    let exported_at = DateTime::parse_from_rfc3339(&input.exported_at).map_err(|e| e.to_string())?.with_timezone(&Utc);
    for paper in &input.papers {
        let key = paper.doi.as_deref().map(|x| format!("doi:{}", x.to_lowercase()))
            .or_else(|| paper.pmid.as_deref().map(|x| format!("pmid:{x}")))
            .unwrap_or_else(|| format!("{}:{}", input.source_system, paper.source_ref));
        let paper_id = stable_id(&format!("si-publication:{key}"));
        let artifact_id = paper.file.as_ref().map(|_| stable_id(&format!("si-publication-file:{key}")));
        let paper_provenance = Provenance {
            source_type: input.source_system.clone(),
            source_id: Some(paper.source_ref.clone()),
            source_version: paper.version.clone(),
            note: Some("Imported from explicitly retained publication library; bibliographic metadata is not an endorsement".into()),
        };
        if let Some(file) = &paper.file {
            state.source_artifacts.push(SourceArtifact {
                id: artifact_id.expect("file implies artifact id"),
                modality: "document".into(),
                media_type: Some("publication_full_text".into()),
                mime_type: Some(file.mime_type.clone()),
                sha256: Some(file.sha256.to_ascii_lowercase()),
                byte_length: Some(file.byte_length),
                availability: "available".into(),
                storage_ref: Some(file.storage_ref.clone()),
                source_node_id: None,
                parent_artifact_id: None,
                frame_start_ms: None,
                frame_end_ms: None,
                imported_at: exported_at,
                provenance: Provenance {
                    source_type: input.source_system.clone(),
                    source_id: Some(paper.source_ref.clone()),
                    source_version: paper.version.clone(),
                    note: Some("User-retained publication file; rights/access remain governed by source library".into()),
                },
                metadata: BTreeMap::new(),
            });
        }
        state.publications.push(PublicationRecord {
            id: paper_id,
            doi: paper.doi.clone(), pmid: paper.pmid.clone(), pmcid: paper.pmcid.clone(),
            title: paper.title.clone(), authors: paper.authors.clone(), journal: paper.journal.clone(),
            publication_year: paper.publication_year, version: paper.version.clone(),
            retention_state: paper.retention_state.clone(), retained_by: input.retained_by.clone(),
            decision_at: paper.decision_at.as_deref().map(|value| DateTime::parse_from_rfc3339(value)).transpose()
                .map_err(|e| format!("invalid publication decision timestamp: {e}"))?.map(|d| d.with_timezone(&Utc)),
            source_artifact_id: artifact_id, provenance: paper_provenance.clone(),
        });
        for claim in &paper.claims {
            state.evidence_claims.push(EvidenceClaim {
                id: stable_id(&format!("si-evidence-claim:{key}:{}", claim.source_ref)),
                publication_id: paper_id,
                claim: claim.claim.clone(), locator: claim.locator.clone(), population: claim.population.clone(),
                intervention: claim.intervention.clone(), comparator: claim.comparator.clone(),
                outcome: claim.outcome.clone(), effect_estimate: claim.effect_estimate.clone(),
                uncertainty: claim.uncertainty.clone(), limitations: claim.limitations.clone(),
                epistemic_status: claim.epistemic_status.clone(), review_state: claim.review_state.clone(),
                provenance: Provenance {
                    source_type: input.source_system.clone(),
                    source_id: Some(format!("{}#{}", paper.source_ref, claim.source_ref)),
                    source_version: paper.version.clone(),
                    note: Some("Structured claim supplied by the source library; claim review state preserved".into()),
                },
            });
        }
    }
    state.manifest.source_systems.push(input.source_system.clone());
    state.validate()
}

/// Actual bytes are checked against both declared length and SHA-256 before a bundle is trusted.
pub fn verify_file_bytes(file: &RetainedFile, bytes: &[u8]) -> Result<(), String> {
    if bytes.len() as u64 != file.byte_length { return Err(format!("byte length mismatch for {}", file.storage_ref)); }
    let digest = hex::encode(Sha256::digest(bytes));
    if !digest.eq_ignore_ascii_case(&file.sha256) { return Err(format!("SHA-256 mismatch for {}", file.storage_ref)); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn imports_retained_paper_and_structured_claim() {
        let input=PublicationImport {
            format:PUBLICATION_IMPORT_FORMAT.into(), version:1, exported_at:"2026-11-04T00:00:00Z".into(),
            source_system:"local-paper-library".into(), retained_by:Some("synthetic-user".into()),
            papers:vec![ImportedPaper {
                source_ref:"paper-1".into(), doi:Some("10.0000/example".into()), pmid:None, pmcid:None,
                title:"Synthetic surgical trial".into(), authors:vec!["A Author".into()], journal:Some("Synthetic Journal".into()),
                publication_year:Some(2026), version:Some("1".into()), retention_state:"kept".into(),
                decision_at:Some("2026-11-04T00:00:00Z".into()), file:None,
                claims:vec![ImportedClaim {
                    source_ref:"claim-1".into(), claim:"Synthetic outcome claim".into(), locator:Some("Table 2".into()),
                    population:None, intervention:None, comparator:None, outcome:Some("complication".into()),
                    effect_estimate:Some("RR 0.8".into()), uncertainty:Some("95% CI 0.6-1.0".into()),
                    limitations:vec!["Synthetic data".into()], epistemic_status:"reported".into(), review_state:"not_reviewed".into()
                }]
            }]
        };
        let mut state=PortableStateV2::empty();
        map_publications(&input,&mut state).unwrap();
        assert_eq!(state.publications.len(),1);
        assert_eq!(state.evidence_claims.len(),1);
        assert_eq!(state.evidence_claims[0].publication_id,state.publications[0].id);
    }
    #[test]
    fn verifies_media_bytes_by_length_and_checksum() {
        let bytes=b"synthetic pdf";
        let file=RetainedFile { storage_ref:"papers/test.pdf".into(), mime_type:"application/pdf".into(),
            sha256:hex::encode(Sha256::digest(bytes)), byte_length:bytes.len() as u64, export_authorized:true };
        assert!(verify_file_bytes(&file,bytes).is_ok());
        assert!(verify_file_bytes(&file,b"tampered").is_err());
    }
}
