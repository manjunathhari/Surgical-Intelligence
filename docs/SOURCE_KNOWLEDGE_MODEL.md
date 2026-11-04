# Surgical Intelligence source-of-knowledge model

**Status:** Architectural direction agreed 4 November 2026  
**Principle:** Preserve the existing Rust core, portable graph, registry adapter and Python evaluation harness. Extend them so the canonical knowledge is built from surgical experience and its source artefacts—not from embeddings alone.

## 1. The four primary knowledge sources

### A. Structured surgical learning — first-class source

The registry's My Experience learning records are high-value expert knowledge. Preserve:
- The original learning and its labelled slots: own words, expected, saw, did, first told, options, what it told you, reminded me, confidence, changed mind, fellow and lesson.
- Each slot's origin (own words, prompted answer, or structured box).
- The original sealed snapshot and every later amendment as separate, ordered events.
- The surgeon's sign tags and the media type shown with the learning.
- The distinction between what was observed, what the surgeon believed, what was done, and what was learned retrospectively.

Do not flatten the learning into one generated paragraph. A normalized text view may be derived for search and embedding, but the original slots and amendments remain canonical.

### B. Operative images and video — first-class source

Images and video are not optional attachments or decorative context. They can carry the most important surgical information: anatomy, tissue planes, exposure, instrument position, sequence, technical manoeuvres, bleeding, injury, rescue, and the visual result.

Represent a media asset as a source object with:
- Stable asset ID and opaque storage reference (never a public URL or storage credential).
- Media type, MIME type, duration or dimensions, checksum, capture/import time and source provenance.
- Episode, procedure phase and learning links where known.
- De-identification and human-review state.
- Optional annotations: frame/time range, anatomical structure, event, instrument, technique, observation, decision, action, and outcome.
- Derivative lineage: extracted frame, thumbnail, clip segment, transcript, visual embedding, or annotation must link back to the original asset and the exact frame/time range.

Store media bytes in a controlled object store; store metadata, references, annotations, and provenance in the portable knowledge graph. The portable export should support a manifest plus media files or explicitly resolvable content-addressed references. Never silently export a broken URL and call it portable.

**Video is temporal evidence.** Do not represent an entire operation with one embedding. Allow multiple segments and frame/time anchors, preserving order and neighbouring context. A derived clip or selected frame must retain its parent asset, time range and transformation details.

### C. User-retained papers — first-class evidence source

A paper the surgeon explicitly keeps is a curated knowledge signal. Preserve:
- Stable publication identity (DOI, PMID, PMCID or another verified identifier).
- Bibliographic metadata and version.
- The user's Keep/Hide decision, who made it, and when.
- The graph concepts, procedures, techniques, complications or learning questions to which the paper is linked.
- Extracted claims with exact page/table/figure/section locators, population, intervention/comparator, outcome, effect estimate, uncertainty and limitations when available.
- Evidence type and appraisal/review state.

Keep the distinction between the publication itself, an extracted claim, and the surgeon's interpretation. A saved paper is not automatically an endorsed claim. A paper may support one claim and contradict another. Preserve both.

### D. Case, procedure and outcome data — first-class source

Use authorized, de-identified registry exports as the case-level clinical context:
- Procedure and indication.
- Relevant patient/context factors in appropriately generalized form.
- Intraoperative observations, decisions and actions where recorded.
- Complications, rescue responses, postoperative outcomes and follow-up.
- Surgeon reflection and later amendments.

The registry remains the system of record. Surgical Intelligence receives permissioned exports/events; it must not reach into the live database or treat a semantic index as the canonical source.

## 2. One graph, multiple source modalities

The knowledge graph connects source objects to surgical concepts and claims. Add these node types to the current graph vocabulary:

| Node type | Purpose |
|---|---|
| `media_asset` | Original image, video or audio-derived visual source reference |
| `media_segment` | Time-bounded video interval or selected image/frame |
| `media_annotation` | Human- or machine-proposed annotation anchored to a source/time |
| `publication` | User-retained paper or guideline |
| `evidence_claim` | A specific claim extracted from a publication |
| `learning_record` | Original structured surgeon learning with slot-level fidelity |
| `case_episode` | De-identified clinical episode |
| `procedure`, `technique`, `context_factor` | Reusable concepts |

Use typed edges such as:
- `illustrates`: media asset/segment illustrates a technique, anatomy or event.
- `annotates`: annotation is anchored to a media asset/segment.
- `derived_from`: extracted frame, segment, transcript or claim has a parent source.
- `records_learning`: learning record belongs to an episode.
- `supports` / `contradicts`: paper claim or observed experience bears on another claim.
- `about_procedure`, `about_technique`, `about_complication`: publication or media relates to a surgical concept.
- `kept_by`: publication is deliberately retained by a user/surgeon scope.
- `observed_in`, `performed_in`, `resulted_in`: case facts linked to their episode.

Every node and edge retains provenance, epistemic status, temporal information and lifecycle. Machine-generated annotations are hypotheses until reviewed; a frame similarity score is not a clinical fact.

## 3. The vector index is a derived retrieval layer

Keep the current registry's gte-small/384-dimensional My Experience index as a source-system implementation. It is useful, but it is not the knowledge store and its vectors are not portable meaning.

Surgical Intelligence should support independent, replaceable embedding profiles:
- Text embeddings for learning slots, publication passages and structured claims.
- Image embeddings for images and selected video frames.
- Video segment embeddings for temporally bounded clips, if a suitable model is selected.
- Optional joint text-image embeddings for cross-modal retrieval.

Every embedding record must declare modality, model/provider, model revision, dimensions, normalization, preprocessing version, source node/asset ID and frame/time range. It must be rebuildable from the canonical source. Never compare vectors from different model profiles as if they were in the same space. Changing models means creating a new index profile and re-embedding; it must not rewrite the graph.

Use hybrid retrieval: graph traversal + metadata filters + lexical search + modality-specific vector similarity. Return the source and exact evidence locator with every match.

## 4. Knowledge lifecycle and safety

- Raw source is immutable or versioned; annotations and extracted claims are separately versioned.
- Keep provenance back to the original learning slot, paper page, image, or video time range.
- Preserve conflicting experiences and publications. Do not average away disagreement.
- One case can create a candidate learning, never an automatic general principle.
- Promotion to a reusable principle requires a human review gate, explicit scope, supporting and contradicting cases/evidence, and limitations.
- A retained paper is curated, not necessarily endorsed.
- A machine-generated caption, segmentation, transcript or claim remains unverified until reviewed.
- Patient media is sensitive even after faces/names are removed. Require explicit purpose authorization, de-identification, access controls, audit logs, retention policy and a separate media-export permission. Pseudonymization is not anonymization.
- Do not place raw patient media, public storage URLs, direct identifiers or re-identification keys in default graph exports.

## 5. Portability contract

The canonical export consists of:
1. Versioned graph JSON (nodes, edges, provenance, events and source manifests).
2. Optional media bundle or content-addressed media package with checksums and derivative lineage.
3. Original structured learning records and amendment history.
4. Publication metadata, retained/hidden decisions and claim locators.
5. Optional embedding-index bundle, explicitly marked as rebuildable cache with model profile metadata.

A consumer can import the graph and source manifest without the original model, vector database, Supabase, Vercel or registry. If media bytes are withheld for governance reasons, the manifest must state that the source is intentionally unavailable and preserve its checksum and access policy.

## 6. Implementation sequence

1. Keep existing Rust memory, graph schema, CLI and Python harness intact.
2. Extend graph schema and Rust core with source records, media assets/segments/annotations, publications/claims and typed links.
3. Extend registry adapter to preserve learning slots, amendments and media associations; do not export live media until separate authorization and de-identification controls exist.
4. Add paper import/retention adapter using stable publication identifiers and user Keep/Hide decisions.
5. Add media manifest importer and frame/time annotation support; begin with synthetic fixtures.
6. Add modality-aware embedding interfaces as replaceable derived indexes.
7. Build evaluation around source fidelity, media/time anchoring, claim provenance, graph integrity, retrieval relevance, contradiction preservation and export/import portability.

The next engineering change should be schema and adapter fidelity—not training a neural model.
