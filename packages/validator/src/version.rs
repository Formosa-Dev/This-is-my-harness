//! Version gate — supported `apiVersion` set + static migration stub (F2-11).
//!
//! In the spec's fixed layer order this runs **after** the structural layer:
//!
//! ```text
//! structural -> version -> semantic -> capability -> filesystem
//! ```
//!
//! The gate supports exactly one generation, `thisismyharness.dev/v1alpha1`.
//! A missing `apiVersion` yields `version.missing`; a value outside the
//! supported set yields `version.unsupported` at `/apiVersion` naming the
//! supported set. **Unsupported is an evaluated-and-invalid condition
//! (`status: invalid`, exit 1), NOT a cannot-evaluate condition (exit 2).**
//!
//! The migration map is a static stub with a single entry that returns "no
//! migration needed". It MUST NOT invent a generation, host, media type or
//! migration algorithm: §58 stays OPEN.

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic, Severity};

/// The one generation this validator supports at v1alpha1.
pub const SUPPORTED_API_VERSIONS: &[&str] = &["thisismyharness.dev/v1alpha1"];

/// The JSON Pointer of the `apiVersion` position.
pub const API_VERSION_POINTER: &str = "/apiVersion";

/// The static migration map (F2-11).
///
/// Exactly one entry exists for the one supported generation: it says no
/// migration is needed. Anything the map does not cover is an unsupported
/// generation, reported by the gate — never silently upgraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Migration {
    /// The document is already at the supported generation.
    None,
}

impl Migration {
    /// Whether this entry means "no migration needed".
    #[must_use]
    pub const fn is_noop(self) -> bool {
        matches!(self, Migration::None)
    }
}

/// The static migration for `api_version`, or `None` when the generation is
/// outside the supported set.
#[must_use]
pub fn migration_for(api_version: &str) -> Option<Migration> {
    if is_supported(api_version) {
        Some(Migration::None)
    } else {
        None
    }
}

/// Whether `api_version` is in the supported set.
#[must_use]
pub fn is_supported(api_version: &str) -> bool {
    SUPPORTED_API_VERSIONS.contains(&api_version)
}

/// A human-facing list of the supported set (never normative text).
#[must_use]
pub fn supported_list() -> String {
    SUPPORTED_API_VERSIONS.join(", ")
}

/// Whether the version gate applies to `kind`.
///
/// Only the **manifest** root declares an `apiVersion` at v1alpha1. The
/// model-contract and install-plan roots are closed objects with no such field
/// (`unevaluatedProperties: false`), so the gate's precondition does not hold
/// for them: a *missing* `apiVersion` is only meaningful where one is required.
/// Applying the gate to those roots would contradict the corpus-reuse
/// requirement (a positive model contract has no `apiVersion`).
#[must_use]
pub const fn applies_to(kind: crate::document::Kind) -> bool {
    matches!(kind, crate::document::Kind::Manifest)
}

/// Evaluate the version gate for a parsed document.
///
/// Returns an empty vector when the declared generation is supported; otherwise
/// exactly one `version.*` error at `/apiVersion`. It never returns a
/// "cannot-evaluate" diagnostic: an unsupported generation is evaluated and
/// invalid. Call [`applies_to`] first to confirm the gate's precondition.
#[must_use]
pub fn validate(value: &Value) -> Vec<Diagnostic> {
    match crate::document::read_api_version(value) {
        None => vec![Diagnostic::error(
            API_VERSION_POINTER,
            Code::version_missing(),
            format!("apiVersion is required; supported: {}", supported_list()),
        )],
        Some(api_version) if !is_supported(&api_version) => vec![Diagnostic::error(
            API_VERSION_POINTER,
            Code::version_unsupported(),
            format!(
                "apiVersion `{api_version}` is not supported; supported: {}",
                supported_list()
            ),
        )],
        Some(_) => Vec::new(),
    }
}

/// One issue, one code.
///
/// The version gate supersedes the structural layer on `/apiVersion`: the
/// schema can only say a value is not the one `const` (or that the property is
/// required), while the version gate says *which* generation is unsupported.
/// When a `version.*` error is present, the matching L1 `schema.const` at
/// `/apiVersion` or `schema.required` (root) about `apiVersion` is suppressed so
/// the generation problem is reported exactly once.
#[must_use]
pub fn dedupe(diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    let version_fired = diagnostics
        .iter()
        .any(|d| d.severity == Severity::Error && d.code.as_str().starts_with("version."));
    if !version_fired {
        return diagnostics;
    }
    diagnostics
        .into_iter()
        .filter(|d| !is_superseded_api_version_issue(d))
        .collect()
}

fn is_superseded_api_version_issue(diagnostic: &Diagnostic) -> bool {
    match diagnostic.code.as_str() {
        "schema.const" => diagnostic.path == API_VERSION_POINTER,
        "schema.required" => {
            diagnostic.path.is_empty() && diagnostic.message.contains("apiVersion")
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn supported_api_version_is_accepted() {
        let value = json!({ "apiVersion": "thisismyharness.dev/v1alpha1" });
        assert!(validate(&value).is_empty());
        assert!(is_supported("thisismyharness.dev/v1alpha1"));
        assert_eq!(
            migration_for("thisismyharness.dev/v1alpha1"),
            Some(Migration::None)
        );
    }

    #[test]
    fn missing_api_version_is_version_missing() {
        let value = json!({ "kind": "Harness" });
        let diagnostics = validate(&value);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.as_str(), "version.missing");
        assert_eq!(diagnostics[0].path, "/apiVersion");
        assert_eq!(
            crate::Report::from_diagnostics(diagnostics).status,
            crate::Status::Invalid,
            "a missing apiVersion is evaluated & invalid (exit 1), not exit 2"
        );
    }

    #[test]
    fn unsupported_api_version_is_unsupported_and_invalid() {
        let value = json!({ "apiVersion": "thisismyharness.dev/v2alpha1" });
        let diagnostics = validate(&value);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code.as_str(), "version.unsupported");
        assert_eq!(diagnostics[0].path, "/apiVersion");
        assert!(
            diagnostics[0]
                .message
                .contains("thisismyharness.dev/v1alpha1"),
            "the diagnostic must name the supported set: {}",
            diagnostics[0].message
        );
        assert_eq!(
            crate::Report::from_diagnostics(diagnostics).status,
            crate::Status::Invalid,
            "unsupported is evaluated & invalid (exit 1), not exit 2"
        );
        assert_eq!(migration_for("thisismyharness.dev/v2alpha1"), None);
    }

    #[test]
    fn unsupported_wins_over_structural_const() {
        let diagnostics = vec![
            Diagnostic::error(
                "/apiVersion",
                Code::schema_keyword("const"),
                "\"thisismyharness.dev/v1alpha1\" was expected",
            ),
            Diagnostic::error(
                "/apiVersion",
                Code::version_unsupported(),
                "apiVersion `x` is not supported",
            ),
        ];
        let deduped = dedupe(diagnostics);
        assert_eq!(deduped.len(), 1, "got {deduped:?}");
        assert_eq!(deduped[0].code.as_str(), "version.unsupported");
    }

    #[test]
    fn missing_wins_over_structural_required() {
        let diagnostics = vec![
            Diagnostic::error(
                "",
                Code::schema_keyword("required"),
                "\"apiVersion\" is a required property",
            ),
            Diagnostic::error(
                "/apiVersion",
                Code::version_missing(),
                "apiVersion is required",
            ),
        ];
        let deduped = dedupe(diagnostics);
        assert_eq!(deduped.len(), 1, "got {deduped:?}");
        assert_eq!(deduped[0].code.as_str(), "version.missing");
    }

    #[test]
    fn dedupe_leaves_unrelated_required_errors_alone() {
        // A root `required` about a different property is NOT suppressed.
        let diagnostics = vec![
            Diagnostic::error(
                "",
                Code::schema_keyword("required"),
                "\"metadata\" is a required property",
            ),
            Diagnostic::error(
                "/apiVersion",
                Code::version_missing(),
                "apiVersion is required",
            ),
        ];
        let deduped = dedupe(diagnostics);
        assert_eq!(deduped.len(), 2, "got {deduped:?}");
    }

    #[test]
    fn dedupe_is_a_noop_without_a_version_diagnostic() {
        let diagnostics = vec![Diagnostic::error(
            "/apiVersion",
            Code::schema_keyword("const"),
            "expected",
        )];
        let deduped = dedupe(diagnostics.clone());
        assert_eq!(deduped, diagnostics);
    }
}
