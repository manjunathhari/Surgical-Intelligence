use chrono::Utc;
use clap::{Parser, Subcommand, ValueEnum};
use si_registry_adapter::{map_registry_export, map_registry_media_assets, RegistryExport};
use si_core::{
    migrate_to_v2, LearningAmendment, LearningRecord, LearningSlot, PortableStateV2, Assertion, AssertionStatus, EpistemicStatus, MemoryState, PortableEnvelope, Provenance,
    RetentionClass,
};
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(
    name = "si",
    version,
    about = "Portable local memory engine for Surgical Intelligence"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new empty memory store. Refuses to overwrite an existing file.
    Init {
        #[arg(long)]
        store: PathBuf,
    },
    /// Add one already-structured assertion to a local store.
    Add {
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        subject: String,
        #[arg(long)]
        predicate: String,
        #[arg(long)]
        object: String,
        #[arg(long)]
        statement: String,
        #[arg(long, default_value = "manual")]
        source_type: String,
        #[arg(long)]
        source_id: Option<String>,
        #[arg(long, value_enum, default_value = "reported")]
        epistemic_status: EpistemicArg,
        #[arg(long)]
        confidence: Option<f32>,
        #[arg(long, value_enum, default_value = "durable")]
        retention: RetentionArg,
        /// Repeat --tag to attach multiple tags.
        #[arg(long)]
        tag: Vec<String>,
    },
    /// List assertions, including their status and provenance.
    List {
        #[arg(long)]
        store: PathBuf,
    },
    /// Validate a store without changing it.
    Validate {
        #[arg(long)]
        store: PathBuf,
    },
    /// Copy a validated store to an export file.
    Export {
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Create a new empty unified v2 state.
    InitState {
        #[arg(long)]
        output: PathBuf,
    },
    /// Validate a unified v2 state file.
    ValidateState {
        #[arg(long)]
        input: PathBuf,
    },
    /// Print a compact summary of a validated v2 state.
    InspectState {
        #[arg(long)]
        input: PathBuf,
    },
    /// Copy a validated v2 state to a new export file.
    ExportState {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Migrate a v1 memory or registry graph JSON file into the unified v2 state.
    Migrate {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Import a de-identified Surgical Registry export as a unified v2 portable state.
    ImportRegistry {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Import a validated portable assertion-memory file as the target store.
    Import {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        store: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum EpistemicArg {
    Observed,
    Reported,
    Inferred,
    Hypothesis,
}

impl From<EpistemicArg> for EpistemicStatus {
    fn from(value: EpistemicArg) -> Self {
        match value {
            EpistemicArg::Observed => Self::Observed,
            EpistemicArg::Reported => Self::Reported,
            EpistemicArg::Inferred => Self::Inferred,
            EpistemicArg::Hypothesis => Self::Hypothesis,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RetentionArg {
    Durable,
    Episodic,
    Temporary,
}

impl From<RetentionArg> for RetentionClass {
    fn from(value: RetentionArg) -> Self {
        match value {
            RetentionArg::Durable => Self::Durable,
            RetentionArg::Episodic => Self::Episodic,
            RetentionArg::Temporary => Self::Temporary,
        }
    }
}

fn invalid_data(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn load_store(path: &Path) -> Result<PortableEnvelope, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let envelope = PortableEnvelope::from_json(&input).map_err(invalid_data)?;
    Ok(envelope)
}

fn write_store(path: &Path, envelope: &PortableEnvelope) -> Result<(), Box<dyn Error>> {
    let json = envelope.to_json().map_err(invalid_data)?;
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    // Write to a unique sibling first so an interrupted write does not leave a
    // partially serialized memory file at the target path.
    let temp_name = format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("si-memory"),
        Uuid::new_v4()
    );
    let temp_path = path.with_file_name(temp_name);
    fs::write(&temp_path, json)?;

    if path.exists() {
        fs::remove_file(path)?;
    }
    if let Err(error) = fs::rename(&temp_path, path) {
        let _ = fs::remove_file(&temp_path);
        return Err(error.into());
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    match Cli::parse().command {
        Commands::Init { store } => {
            if store.exists() {
                return Err(
                    invalid_data(format!("store already exists: {}", store.display())).into(),
                );
            }
            write_store(&store, &PortableEnvelope::new(MemoryState::new()))?;
            println!("Initialised {}", store.display());
        }
        Commands::Add {
            store,
            subject,
            predicate,
            object,
            statement,
            source_type,
            source_id,
            epistemic_status,
            confidence,
            retention,
            tag,
        } => {
            let mut envelope = load_store(&store)?;
            let assertion = Assertion {
                id: Uuid::new_v4(),
                subject,
                predicate,
                object,
                statement,
                epistemic_status: epistemic_status.into(),
                confidence,
                retention: retention.into(),
                status: AssertionStatus::Active,
                provenance: Provenance {
                    source_type,
                    source_id,
                    source_version: None,
                    note: None,
                },
                observed_at: Utc::now(),
                valid_from: None,
                valid_until: None,
                tags: tag,
            };
            let id = assertion.id;
            envelope
                .state
                .add_assertion(assertion)
                .map_err(invalid_data)?;
            write_store(&store, &envelope)?;
            println!("Added assertion {id}");
        }
        Commands::List { store } => {
            let envelope = load_store(&store)?;
            let values: Vec<_> = envelope.state.assertions.values().collect();
            println!("{}", serde_json::to_string_pretty(&values)?);
        }
        Commands::Validate { store } => {
            let envelope = load_store(&store)?;
            envelope.state.validate().map_err(invalid_data)?;
            println!(
                "Valid: {} assertions, {} events (format v{})",
                envelope.state.assertions.len(),
                envelope.state.events.len(),
                envelope.version
            );
        }
        Commands::Export { store, output } => {
            let envelope = load_store(&store)?;
            write_store(&output, &envelope)?;
            println!(
                "Exported {} assertions to {}",
                envelope.state.assertions.len(),
                output.display()
            );
        }
        Commands::InitState { output } => {
            if output.exists() {
                return Err(invalid_data(format!("state already exists: {}", output.display())).into());
            }
            let state = PortableStateV2::empty();
            let json = state.to_json().map_err(invalid_data)?;
            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() { fs::create_dir_all(parent)?; }
            }
            let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&output)?;
            use std::io::Write;
            file.write_all(json.as_bytes())?;
            println!("Initialised v{} state at {}", state.version, output.display());
        }
        Commands::ValidateState { input } => {
            let raw = fs::read_to_string(&input)?;
            let state = PortableStateV2::from_json(&raw).map_err(invalid_data)?;
            println!("Valid v{} state: {} nodes, {} edges, {} learning records, {} publications, {} media manifests",
                state.version, state.graph.nodes.len(), state.graph.edges.len(),
                state.learning_records.len(), state.publications.len(), state.source_artifacts.len());
        }
        Commands::InspectState { input } => {
            let raw = fs::read_to_string(&input)?;
            let state = PortableStateV2::from_json(&raw).map_err(invalid_data)?;
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                "format": state.format, "version": state.version, "manifest": state.manifest,
                "nodes": state.graph.nodes.len(), "edges": state.graph.edges.len(),
                "learning_records": state.learning_records.len(),
                "publications": state.publications.len(), "evidence_claims": state.evidence_claims.len(),
                "source_artifacts": state.source_artifacts.len(), "events": state.events.len(),
                "embedding_profiles": state.embedding_profiles.len()
            }))?);
        }
        Commands::ExportState { input, output } => {
            if output.exists() {
                return Err(invalid_data(format!("output already exists: {}", output.display())).into());
            }
            let raw = fs::read_to_string(&input)?;
            let state = PortableStateV2::from_json(&raw).map_err(invalid_data)?;
            let json = state.to_json().map_err(invalid_data)?;
            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() { fs::create_dir_all(parent)?; }
            }
            let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&output)?;
            use std::io::Write;
            file.write_all(json.as_bytes())?;
            println!("Exported v{} portable state to {}", state.version, output.display());
        }
        Commands::Migrate { input, output } => {
            if output.exists() {
                return Err(invalid_data(format!("output already exists: {}", output.display())).into());
            }
            let input_text = fs::read_to_string(&input)?;
            let state = migrate_to_v2(&input_text).map_err(invalid_data)?;
            let json = state.to_json().map_err(invalid_data)?;
            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() { fs::create_dir_all(parent)?; }
            }
            let mut file = fs::OpenOptions::new().write(true).create_new(true).open(&output)?;
            use std::io::Write;
            file.write_all(json.as_bytes())?;
            println!("Migrated to v{}: {} nodes, {} edges, {} source artifacts, {} publications",
                state.version, state.graph.nodes.len(), state.graph.edges.len(),
                state.source_artifacts.len(), state.publications.len());
        }
        Commands::ImportRegistry { input, output } => {
            if output.exists() {
                return Err(invalid_data(format!(
                    "output already exists: {}",
                    output.display()
                )).into());
            }
            let input_text = fs::read_to_string(&input)?;
            let registry_export: RegistryExport =
                serde_json::from_str(&input_text).map_err(|error| invalid_data(error.to_string()))?;
            let graph = map_registry_export(&registry_export).map_err(invalid_data)?;
            let mut state = PortableStateV2::from_graph_values(
                graph.graph.nodes, graph.graph.edges, "surgical-registry", &graph.exported_at
            ).map_err(invalid_data)?;
            map_registry_media_assets(&registry_export, &mut state).map_err(invalid_data)?;
            for case in &registry_export.cases {
                let episode_id = Uuid::new_v5(
                    &Uuid::NAMESPACE_URL,
                    format!("surgical-registry:{}:episode", case.source_case_ref).as_bytes()
                );
                let sealed_at = case.learning_sealed_at.as_deref()
                    .map(|value| chrono::DateTime::parse_from_rfc3339(value)
                        .map(|dt| dt.with_timezone(&Utc))
                        .map_err(|e| invalid_data(format!("invalid learning seal timestamp: {e}"))))
                    .transpose()?;
                for learning in &case.learnings {
                    let id = Uuid::new_v5(
                        &Uuid::NAMESPACE_URL,
                        format!("surgical-registry:{}:learning:{}:record", case.source_case_ref, learning.ordinal).as_bytes()
                    );
                    let amendments = learning.amendments.iter().map(|amendment| {
                        let amended_at = chrono::DateTime::parse_from_rfc3339(&amendment.created_at)
                            .map(|dt| dt.with_timezone(&Utc))
                            .map_err(|e| invalid_data(format!("invalid learning amendment timestamp: {e}")))?;
                        Ok(LearningAmendment {
                            slot: amendment.slot.clone(),
                            text: amendment.text.clone(),
                            amended_at,
                        })
                    }).collect::<Result<Vec<_>, io::Error>>()?;
                    state.learning_records.push(LearningRecord {
                        id,
                        episode_node_id: Some(episode_id),
                        source_case_ref: Some(case.source_case_ref.clone()),
                        ordinal: learning.ordinal,
                        shown_by: learning.shown_by.clone(),
                        sign_tags: learning.sign_tags.clone(),
                        how_showed: learning.how_showed.clone(),
                        slots: learning.items.iter().map(|item| LearningSlot {
                            name: item.slot.clone(),
                            text: item.text.clone(),
                            origin: item.origin.clone(),
                        }).collect(),
                        amendments,
                        added_after_seal: learning.added_after_seal,
                        sealed_at,
                        provenance: si_core::Provenance {
                            source_type: "surgical_registry_my_experience".into(),
                            source_id: None,
                            source_version: Some("postgres-case-values-learning-v1".into()),
                            note: Some(format!("Pseudonymous case ref {}; learning ordinal {}", case.source_case_ref, learning.ordinal)),
                        },
                    });
                }
            }
            state.validate().map_err(invalid_data)?;
            let json = state.to_json().map_err(invalid_data)?;
            if let Some(parent) = output.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&output)?;
            use std::io::Write;
            file.write_all(json.as_bytes())?;
            println!(
                "Imported {} registry cases as {} graph nodes and {} edges to {}",
                registry_export.cases.len(),
                state.graph.nodes.len(),
                state.graph.edges.len(),
                output.display()
            );
        }
        Commands::Import { input, store } => {
            if store.exists() {
                return Err(invalid_data(format!(
                    "target store already exists: {}",
                    store.display()
                ))
                .into());
            }
            let envelope = load_store(&input)?;
            write_store(&store, &envelope)?;
            println!(
                "Imported {} assertions to {}",
                envelope.state.assertions.len(),
                store.display()
            );
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
