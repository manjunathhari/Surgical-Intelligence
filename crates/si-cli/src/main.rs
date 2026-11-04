use chrono::Utc;
use clap::{Parser, Subcommand, ValueEnum};
use si_core::{
    Assertion, AssertionStatus, EpistemicStatus, MemoryState, PortableEnvelope, Provenance,
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
    /// Import a validated portable file as the target store.
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
