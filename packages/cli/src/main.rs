//! `harness` — the cross-platform CLI.
//!
//! F2 ships `harness validate <path>`. The binary is a thin adapter over
//! `harness-validator`: argument parsing, input discovery, output formatting and
//! exit codes live here; every validation decision lives in the library.
//!
//! # Contract
//!
//! * `harness validate <path>` accepts one file or one directory.
//! * Human output goes to **stderr**; `--json` writes exactly one JSON object to
//!   **stdout** (and suppresses human output).
//! * Exit codes: `0` valid, `1` evaluated-and-invalid (including an unsupported
//!   or missing `apiVersion`, Q9), `2` could-not-evaluate (unreadable path,
//!   unparsable bytes, undetectable/ambiguous kind, usage error).
//! * The command is **read-only**: it opens inputs read-only, writes no file,
//!   creates no temp file, makes no network request and executes no third-party
//!   code.

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

use harness_validator::{Code, Diagnostic, Format, Kind, Options, Report, Source, Status};

#[derive(Debug, Parser)]
#[command(
    name = "harness",
    version,
    about = "This is my Harness — portable agentic systems toolkit",
    long_about = "Validate harness manifests, model contracts and install plans \
                  against the thisismyharness.dev/v1alpha1 JSON Schema set. \
                  Offline and read-only."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate a harness document (a file, or a project directory).
    Validate {
        /// Path to a document file or a project directory.
        path: PathBuf,

        /// Override kind detection: manifest | model-contract | install-plan.
        #[arg(long, value_name = "KIND")]
        kind: Option<KindArg>,

        /// Emit the machine-readable report as JSON on stdout.
        #[arg(long)]
        json: bool,
    },
}

/// The `--kind` override values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum KindArg {
    Manifest,
    ModelContract,
    InstallPlan,
}

impl From<KindArg> for Kind {
    fn from(value: KindArg) -> Self {
        match value {
            KindArg::Manifest => Kind::Manifest,
            KindArg::ModelContract => Kind::ModelContract,
            KindArg::InstallPlan => Kind::InstallPlan,
        }
    }
}

/// Non-normative working manifest filenames for directory input (design §6).
/// The definitive filename is OPEN (§58); full multi-artifact discovery is F4.
const CANDIDATE_FILENAMES: &[&str] = &["harness.yaml", "harness.yml", "harness.json"];

fn main() {
    // `clap` writes `--help` / `--version` to stdout (exit 0) and usage errors to
    // stderr (exit 2); it never produces a false "valid".
    let cli = Cli::parse();
    let code = match cli.command {
        Command::Validate { path, kind, json } => validate(&path, kind.map(Kind::from), json),
    };
    std::process::exit(code);
}

fn validate(path: &Path, kind: Option<Kind>, json: bool) -> i32 {
    let report = evaluate(path, kind);

    if json {
        // Machine contract: exactly one JSON object on stdout, nothing else.
        let rendered = serde_json::to_string_pretty(&report.to_json())
            .unwrap_or_else(|_| "{\"status\":\"error\",\"errors\":[],\"warnings\":[]}".to_owned());
        println!("{rendered}");
    } else {
        // Human output goes to stderr; stdout stays empty.
        let mut stderr = std::io::stderr();
        let _ = write_human(&mut stderr, &report);
    }

    exit_code(report.status)
}

fn exit_code(status: Status) -> i32 {
    match status {
        Status::Valid => 0,
        Status::Invalid => 1,
        Status::Error => 2,
    }
}

/// Evaluate the input path into a [`Report`], performing the only input I/O.
fn evaluate(path: &Path, kind: Option<Kind>) -> Report {
    if path.is_dir() {
        return evaluate_directory(path, kind);
    }

    let bytes = match std::fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return io_report(path, format!("cannot read {}: {error}", path.display()));
        }
    };
    let format = Format::from_filename(&path.to_string_lossy());
    let options = Options {
        kind,
        root: path.parent().map(Path::to_path_buf),
    };
    harness_validator::validate(
        &Source {
            bytes: &bytes,
            format,
        },
        &options,
    )
}

/// Discover the single candidate manifest in `dir` and validate it.
///
/// Full multi-artifact discovery is deferred to F4. Zero candidates, or more
/// than one, is a cannot-evaluate condition (exit 2).
fn evaluate_directory(dir: &Path, kind: Option<Kind>) -> Report {
    let discovered = match discover_manifest(dir) {
        Ok(discovered) => discovered,
        Err(report) => return report,
    };
    let bytes = match std::fs::read(&discovered) {
        Ok(bytes) => bytes,
        Err(error) => {
            return io_report(
                &discovered,
                format!("cannot read {}: {error}", discovered.display()),
            );
        }
    };
    let format = Format::from_filename(&discovered.to_string_lossy());
    let options = Options {
        kind,
        root: Some(dir.to_path_buf()),
    };
    harness_validator::validate(
        &Source {
            bytes: &bytes,
            format,
        },
        &options,
    )
}

fn discover_manifest(dir: &Path) -> Result<PathBuf, Report> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => {
            return Err(io_report(
                dir,
                format!("cannot read directory {}: {error}", dir.display()),
            ));
        }
    };

    let mut candidates: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
            if CANDIDATE_FILENAMES
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(name))
            {
                candidates.push(path);
            }
        }
    }
    candidates.sort();

    match candidates.len() {
        0 => Err(io_report(
            dir,
            format!(
                "no manifest document ({}) found in {}",
                CANDIDATE_FILENAMES.join(", "),
                dir.display()
            ),
        )),
        1 => Ok(candidates.pop().expect("length is one")),
        _ => Err(Report::from_diagnostics(vec![Diagnostic::error(
            dir.to_string_lossy(),
            Code::usage_ambiguous_input(),
            format!(
                "more than one candidate manifest document in {}; \
                 full multi-artifact discovery is deferred to F4",
                dir.display()
            ),
        )])),
    }
}

fn io_report(path: &Path, message: String) -> Report {
    Report::from_diagnostics(vec![Diagnostic::error(
        path.to_string_lossy(),
        Code::io_read_failed(),
        message,
    )])
}

/// Write the human, path-first report to `out` (stderr).
fn write_human(out: &mut impl std::io::Write, report: &Report) -> std::io::Result<()> {
    writeln!(
        out,
        "{}: {} error(s), {} warning(s)",
        report.status.as_str(),
        report.errors.len(),
        report.warnings.len()
    )?;
    for diagnostic in report.errors.iter().chain(report.warnings.iter()) {
        writeln!(
            out,
            "{}: {}: {}",
            diagnostic.path_display(),
            diagnostic.code,
            diagnostic.message
        )?;
    }
    Ok(())
}
