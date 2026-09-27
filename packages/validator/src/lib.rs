//! `harness-validator` — offline, read-only validation of
//! `thisismyharness.dev/v1alpha1` documents.
//!
//! # Status
//!
//! Work units 2 and 3: schema embedding, parsing, diagnostics, kind detection,
//! the **structural** (JSON Schema) layer and the **document-local semantic**
//! layer, wired to the promoted corpus. The capability and version layers and
//! the full `validate(&Source, &Options)` CLI entry land in later work units
//! (F2-07..F2-12).
//!
//! ```text
//! src/lib.rs        module wiring + the work-unit-2 structural entry
//! src/diagnostics.rs Diagnostic { path, code, message }, Severity, Code, Report
//! src/registry.rs   embedded schema table + one lazily-built offline Registry
//! src/parse.rs      bytes -> serde_json::Value (JSON / YAML 1.2, fail-closed)
//! src/document.rs   kind detection + apiVersion read
//! src/structural.rs L1 — JSON Schema layer
//! src/semantic/*    L2 — document-local identity / dependencies / components / permissions
//! src/capability.rs L3 — known-capability lookup (later)
//! src/version.rs    L4 — supported apiVersion set + migration stub (later)
//! src/pathsafe.rs   L5 — declared-path safety (later)
//! ```
//!
//! # Contracts
//!
//! * **Read-only** (risk class A, autonomy 0): the validator opens inputs
//!   read-only, writes nothing, touches no network and never executes a
//!   downloaded artifact.
//! * **Offline**: `jsonschema` is built with `default-features = false` and the
//!   per-document validator uses `.offline()`, so an unknown `$ref` fails loudly
//!   instead of resolving over the network.
//! * The schema set in `schemas/` is the single source of truth; this crate
//!   never redefines the standard.

#![forbid(unsafe_code)]

pub mod diagnostics;
pub mod document;
pub mod parse;
pub mod registry;
pub mod semantic;
pub mod structural;

pub use diagnostics::{Code, Diagnostic, Report, Severity, Status};
pub use document::{Detection, Kind};
pub use parse::Format;

use serde_json::Value;

/// Parse `bytes` and validate the document against the embedded schema
/// identified by `schema_id`, returning a [`Report`].
///
/// This is the work-unit-2 structural entry. It is read-only and pure: it
/// returns a `Report` and never exits the process or writes to stdout.
///
/// A `$ref` that cannot be resolved offline becomes a `schema.unresolved_ref`
/// diagnostic and a `Status::Error` report (the "loud failure" of design §3).
#[must_use]
pub fn validate_structural(schema_id: &str, bytes: &[u8], format: Format) -> Report {
    match parse::parse(bytes, format) {
        Ok(instance) => structural_report(schema_id, &instance),
        Err(diagnostic) => Report::from_diagnostics(vec![diagnostic]),
    }
}

fn structural_report(schema_id: &str, instance: &Value) -> Report {
    match structural::validate(schema_id, instance) {
        Ok(diagnostics) => Report::from_diagnostics(diagnostics),
        Err(error) => Report::from_diagnostics(vec![Diagnostic::error(
            "",
            Code::schema_unresolved_ref(),
            error.to_string(),
        )]),
    }
}
