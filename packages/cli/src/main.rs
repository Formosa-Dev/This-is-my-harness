//! `harness` — the cross-platform CLI.
//!
//! F2 scope: `harness validate <path>`. The binary is a thin adapter over
//! `harness-validator`: argument parsing, output formatting and exit codes live
//! here; every validation decision lives in the library.
//!
//! Work unit 1: skeleton only. The `validate` command is declared (so
//! `--help` is honest about the surface) but its pipeline is wired in later
//! work units. It fails loudly rather than pretending to validate.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
        kind: Option<String>,

        /// Emit the machine-readable report as JSON on stdout.
        #[arg(long)]
        json: bool,
    },
}

fn main() {
    // `clap` handles `--help` / `--version` and usage errors (exit code 2).
    let cli = Cli::parse();
    match cli.command {
        Command::Validate { .. } => {
            // Work unit 1 skeleton: the layered pipeline is implemented in
            // later work units. Fail loudly; never report a false "valid".
            eprintln!("harness validate: not implemented yet (F2 work unit 1 skeleton)");
            std::process::exit(2);
        }
    }
}
