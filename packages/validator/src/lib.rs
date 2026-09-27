//! `harness-validator` — offline, read-only validation of
//! `thisismyharness.dev/v1alpha1` documents.
//!
//! # Status
//!
//! Work unit 1: workspace skeleton only. No validation logic is implemented
//! yet. The module map below is the agreed layout; each module lands in its
//! own work unit (F2-05..F2-12) and none of them lives here yet.
//!
//! ```text
//! src/lib.rs        public `validate(&Source, &Options) -> Report` (the only entry)
//! src/diagnostics.rs Diagnostic { path, code, message }, Severity, Code, Report
//! src/registry.rs   embedded schema table + one lazily-built offline Registry
//! src/parse.rs      bytes -> serde_json::Value (JSON / YAML 1.2, fail-closed)
//! src/document.rs   kind detection + apiVersion read
//! src/structural.rs L1 — JSON Schema layer
//! src/semantic/*    L2 — identity / dependencies / components / permissions
//! src/capability.rs L3 — known-capability lookup
//! src/version.rs    L4 — supported apiVersion set + migration stub
//! src/pathsafe.rs   L5 — declared-path safety
//! ```
//!
//! # Contracts
//!
//! * **Read-only** (risk class A, autonomy 0): the validator opens inputs
//!   read-only, writes nothing, touches no network and never executes a
//!   downloaded artifact.
//! * **Offline**: `jsonschema` is built with `default-features = false`, so an
//!   unknown `$ref` fails loudly instead of resolving over the network.
//! * The schema set in `schemas/` is the single source of truth; this crate
//!   never redefines the standard.

#![forbid(unsafe_code)]
