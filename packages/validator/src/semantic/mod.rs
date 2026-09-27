//! L2 — document-local semantic validation (design §4, tasks 3.1-3.5).
//!
//! JSON Schema is necessary but not sufficient: the checks here cannot be
//! expressed by a schema, yet they are still decidable from a **single
//! document** and the F1 (`spec/**`) rules. Everything that needs the graph,
//! a registry, the filesystem or a live target is explicitly **not** evaluated
//! here; those checks emit a [`Code::semantic_not_evaluated`] **warning**
//! naming the owning downstream phase (F4 / F7 / F15) and never change the
//! report status.
//!
//! Rules run in a fixed order and are **aggregated** — the layer never
//! short-circuits on the first violation:
//!
//! ```text
//! identity -> dependencies -> components -> permissions -> deferred
//! ```
//!
//! This module is read-only and pure: it takes a parsed [`Value`] and returns
//! diagnostics. It performs no I/O and reaches no network.

pub mod components;
pub mod dependencies;
pub mod identity;
pub mod permissions;

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic};
use crate::document::Kind;

/// Run every document-local semantic rule for `kind` over `value`.
///
/// Rules run in a fixed order (identity, dependencies, components, permissions,
/// then the deferred warnings) and are aggregated. The returned vector is
/// **unsorted**; pass it through [`Report::from_diagnostics`](crate::Report::from_diagnostics)
/// for the deterministic total order.
#[must_use]
pub fn validate(kind: Kind, value: &Value) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    identity::check(kind, value, &mut diagnostics);
    dependencies::check(kind, value, &mut diagnostics);
    components::check(kind, value, &mut diagnostics);
    permissions::check(kind, value, &mut diagnostics);
    deferred(kind, value, &mut diagnostics);
    diagnostics
}

/// Emit a `semantic.not_evaluated` warning naming the owning downstream phase.
///
/// Used for checks that are document-local in shape but need evidence this
/// layer does not have (a resolved graph, a registry, a live target). A warning
/// never changes the report status.
pub(crate) fn not_evaluated(
    path: impl Into<String>,
    phase: &str,
    what: &str,
    out: &mut Vec<Diagnostic>,
) {
    out.push(Diagnostic::warning(
        path,
        Code::semantic_not_evaluated(),
        format!("{what} is not evaluated by this layer; owned by {phase}"),
    ));
}

/// Append a JSON Pointer child segment, escaping `~` and `/` (RFC 6901 §3).
pub(crate) fn child(base: &str, segment: &str) -> String {
    let escaped = segment.replace('~', "~0").replace('/', "~1");
    format!("{base}/{escaped}")
}

/// The deferred checks: document-local in shape, but their verdict needs a
/// resolved graph (F4), registry behaviour (F7) or conformance evidence
/// (F7/F15). Each is reported as a warning instead of being silently skipped.
fn deferred(kind: Kind, value: &Value, out: &mut Vec<Diagnostic>) {
    match kind {
        Kind::Manifest => {
            let has_extends = value
                .pointer("/spec/extends")
                .and_then(Value::as_array)
                .is_some_and(|extends| !extends.is_empty());
            if has_extends {
                not_evaluated(
                    "/spec/extends",
                    "F4",
                    "full dependency-graph resolution (cross-document cycles, version and permission conflicts, digest pinning)",
                    out,
                );
            }
            if value.pointer("/spec/distribution/digest").is_some() {
                not_evaluated(
                    "/spec/distribution/digest",
                    "F7",
                    "digest resolution and immutability semantics",
                    out,
                );
            }
            if value.pointer("/spec/conformance").is_some() {
                not_evaluated(
                    "/spec/conformance",
                    "F7/F15",
                    "declared-versus-verified trust precision",
                    out,
                );
            }
        }
        Kind::InstallPlan => {
            if let Some(files) = value.get("files").and_then(Value::as_array) {
                for (index, file) in files.iter().enumerate() {
                    if file.get("digest").is_some() {
                        not_evaluated(
                            child(&child("/files", &index.to_string()), "digest"),
                            "F7",
                            "digest resolution and immutability semantics",
                            out,
                        );
                    }
                }
            }
        }
        Kind::ModelContract => {}
    }
}
