//! `harness-validator` — offline, read-only validation of
//! `thisismyharness.dev/v1alpha1` documents.
//!
//! # Status
//!
//! Work units 2–4: schema embedding, parsing, diagnostics, kind detection, the
//! **structural** (JSON Schema) layer, the **document-local semantic** layer,
//! the **version** gate (with the one-issue-one-code dedupe) and the
//! **capability** lookup, wired to the promoted corpus. The declared-path safety
//! layer and the full `validate(&Source, &Options)` CLI entry land in later work
//! units (F2-08..F2-12).
//!
//! ```text
//! src/lib.rs        module wiring + the work-unit-2/4 library entries
//! src/diagnostics.rs Diagnostic { path, code, message }, Severity, Code, Report
//! src/registry.rs   embedded schema table + one lazily-built offline Registry
//! src/parse.rs      bytes -> serde_json::Value (JSON / YAML 1.2, fail-closed)
//! src/document.rs   kind detection + apiVersion read
//! src/structural.rs structural layer — JSON Schema
//! src/version.rs    version layer — supported apiVersion set + migration stub
//! src/semantic/*    semantic layer — document-local identity / deps / components / permissions
//! src/capability.rs capability layer — known-capability lookup (embedded registry)
//! src/pathsafe.rs   filesystem layer — declared-path safety (later)
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

pub mod capability;
pub mod diagnostics;
pub mod document;
pub mod parse;
pub mod registry;
pub mod semantic;
pub mod structural;
pub mod version;

pub use capability::{CapabilityError, CapabilityRegistry};
pub use diagnostics::{Code, Diagnostic, Report, Severity, Status};
pub use document::{Detection, Kind};
pub use parse::Format;
pub use version::Migration;

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
        Ok(instance) => Report::from_diagnostics(structural_diagnostics(schema_id, &instance)),
        Err(diagnostic) => Report::from_diagnostics(vec![diagnostic]),
    }
}

/// Work-unit-4 entry: run the structural layer and the version gate, then apply
/// the one-issue-one-code dedupe.
///
/// Read-only and pure, like [`validate_structural`]. When the declared
/// `apiVersion` is missing or unsupported, the version gate emits a single
/// `version.*` diagnostic and the superseded L1 `schema.required`/`schema.const`
/// on `/apiVersion` is suppressed (see [`version::dedupe`]). The result is an
/// **evaluated-and-invalid** report (exit-1 semantics), never a cannot-evaluate
/// one.
#[must_use]
pub fn validate_structural_and_version(schema_id: &str, bytes: &[u8], format: Format) -> Report {
    match parse::parse(bytes, format) {
        Ok(instance) => {
            let mut diagnostics = structural_diagnostics(schema_id, &instance);
            diagnostics.extend(version::validate(&instance));
            Report::from_diagnostics(version::dedupe(diagnostics))
        }
        Err(diagnostic) => Report::from_diagnostics(vec![diagnostic]),
    }
}

fn structural_diagnostics(schema_id: &str, instance: &Value) -> Vec<Diagnostic> {
    match structural::validate(schema_id, instance) {
        Ok(diagnostics) => diagnostics,
        Err(error) => vec![Diagnostic::error(
            "",
            Code::schema_unresolved_ref(),
            error.to_string(),
        )],
    }
}
