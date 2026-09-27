//! Typed diagnostics contract (design §5, spec "Typed, stable diagnostics").
//!
//! Every issue is `Diagnostic { severity, path, code, message }`:
//!   * `path` is a JSON Pointer identical to the ajv `instancePath` (`""` is the
//!     document root), so the frozen corpus `expect` blocks stay golden;
//!   * `code` is drawn from the validator's **own** namespace and never leaks a
//!     raw library keyword;
//!   * `message` is human-facing and **never** normative.
//!
//! Ordering is total and deterministic: layer, then severity (errors before
//! warnings), then JSON Pointer, then `code`, then `message`.

use std::fmt;

/// Errors before warnings (the derived ordering places `Error` first).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Error,
    Warning,
}

/// The validator's own, stable diagnostic-code namespace.
///
/// A code is an interned `&'static str`; constructors exist for every code the
/// validator can emit, and JSON Schema keywords are mapped via
/// [`Code::schema_keyword`]. Keywords outside the map fall back to
/// `schema.other`, so a library string can never become a code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Code(&'static str);

impl Code {
    #[must_use]
    pub const fn new(code: &'static str) -> Self {
        Code(code)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    // --- parse.* --------------------------------------------------------
    #[must_use]
    pub const fn parse_invalid() -> Self {
        Code("parse.invalid")
    }
    #[must_use]
    pub const fn parse_duplicate_key() -> Self {
        Code("parse.duplicate_key")
    }
    #[must_use]
    pub const fn parse_multiple_documents() -> Self {
        Code("parse.multiple_documents")
    }

    // --- document.* -----------------------------------------------------
    #[must_use]
    pub const fn document_unknown_kind() -> Self {
        Code("document.unknown_kind")
    }
    #[must_use]
    pub const fn document_kind_ambiguous() -> Self {
        Code("document.kind_ambiguous")
    }

    // --- semantic.* -----------------------------------------------------
    #[must_use]
    pub const fn semantic_identity_ref_short_forbidden() -> Self {
        Code("semantic.identity_ref_short_forbidden")
    }
    #[must_use]
    pub const fn semantic_identity_host_mismatch() -> Self {
        Code("semantic.identity_host_mismatch")
    }
    #[must_use]
    pub const fn semantic_kind_composition() -> Self {
        Code("semantic.kind_composition")
    }
    #[must_use]
    pub const fn semantic_dependency_cycle() -> Self {
        Code("semantic.dependency_cycle")
    }
    #[must_use]
    pub const fn semantic_dependency_order() -> Self {
        Code("semantic.dependency_order")
    }
    #[must_use]
    pub const fn semantic_permission_coverage() -> Self {
        Code("semantic.permission_coverage")
    }
    #[must_use]
    pub const fn semantic_effective_risk() -> Self {
        Code("semantic.effective_risk")
    }
    #[must_use]
    pub const fn semantic_autonomy_below_floor() -> Self {
        Code("semantic.autonomy_below_floor")
    }
    #[must_use]
    pub const fn semantic_env_value_like() -> Self {
        Code("semantic.env_value_like")
    }
    #[must_use]
    pub const fn semantic_license_shape() -> Self {
        Code("semantic.license_shape")
    }
    #[must_use]
    pub const fn semantic_license_conflict() -> Self {
        Code("semantic.license_conflict")
    }
    #[must_use]
    pub const fn semantic_not_evaluated() -> Self {
        Code("semantic.not_evaluated")
    }

    // --- capability.* ---------------------------------------------------
    #[must_use]
    pub const fn capability_unknown() -> Self {
        Code("capability.unknown")
    }
    /// The checked-in known-capability registry is absent or malformed. This is
    /// a loud, cannot-evaluate failure (exit-2 semantics), never a silent
    /// "all known" fallback. (Necessary addition to the design §5 catalogue: the
    /// capability requirement names the condition but no code.)
    #[must_use]
    pub const fn capability_registry_unavailable() -> Self {
        Code("capability.registry_unavailable")
    }

    // --- version.* ------------------------------------------------------
    #[must_use]
    pub const fn version_missing() -> Self {
        Code("version.missing")
    }
    #[must_use]
    pub const fn version_unsupported() -> Self {
        Code("version.unsupported")
    }

    // --- schema.* -------------------------------------------------------
    #[must_use]
    pub const fn schema_unresolved_ref() -> Self {
        Code("schema.unresolved_ref")
    }
    #[must_use]
    pub const fn schema_other() -> Self {
        Code("schema.other")
    }

    // --- path.* (filesystem layer, F2-08) -------------------------------
    #[must_use]
    pub const fn path_traversal() -> Self {
        Code("path.traversal")
    }
    #[must_use]
    pub const fn path_absolute() -> Self {
        Code("path.absolute")
    }
    #[must_use]
    pub const fn path_symlink_escape() -> Self {
        Code("path.symlink_escape")
    }

    // --- io.* / usage.* (pre-validation, cannot-evaluate) ---------------
    #[must_use]
    pub const fn io_read_failed() -> Self {
        Code("io.read_failed")
    }
    #[must_use]
    pub const fn usage_ambiguous_input() -> Self {
        Code("usage.ambiguous_input")
    }

    /// Map a JSON Schema keyword to this validator's stable `schema.*` code.
    #[must_use]
    pub fn schema_keyword(keyword: &str) -> Self {
        let code = match keyword {
            "required" => "schema.required",
            "type" => "schema.type",
            "pattern" => "schema.pattern",
            "enum" => "schema.enum",
            "const" => "schema.const",
            "unevaluatedProperties" => "schema.unevaluatedProperties",
            "additionalProperties" => "schema.additionalProperties",
            "unevaluatedItems" => "schema.unevaluatedItems",
            "additionalItems" => "schema.additionalItems",
            "minItems" => "schema.minItems",
            "maxItems" => "schema.maxItems",
            "uniqueItems" => "schema.uniqueItems",
            "contains" => "schema.contains",
            "minLength" => "schema.minLength",
            "maxLength" => "schema.maxLength",
            "minimum" => "schema.minimum",
            "maximum" => "schema.maximum",
            "exclusiveMinimum" => "schema.exclusiveMinimum",
            "exclusiveMaximum" => "schema.exclusiveMaximum",
            "multipleOf" => "schema.multipleOf",
            "minProperties" => "schema.minProperties",
            "maxProperties" => "schema.maxProperties",
            "propertyNames" => "schema.propertyNames",
            "anyOf" => "schema.anyOf",
            "oneOf" => "schema.oneOf",
            "not" => "schema.not",
            "format" => "schema.format",
            "contentEncoding" => "schema.contentEncoding",
            "contentMediaType" => "schema.contentMediaType",
            "falseSchema" => "schema.falseSchema",
            _ => "schema.other",
        };
        Code(code)
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Layer rank used by the deterministic ordering (spec layer order:
/// structural, version, semantic, capability, filesystem). `parse`/`document`
/// run before any layer; `io`/`usage` are pre-validation failures.
fn layer_rank(code: &str) -> u8 {
    if code.starts_with("parse.")
        || code.starts_with("document.")
        || code.starts_with("io.")
        || code.starts_with("usage.")
    {
        0
    } else if code.starts_with("schema.") {
        1
    } else if code.starts_with("version.") {
        2
    } else if code.starts_with("semantic.") {
        3
    } else if code.starts_with("capability.") {
        4
    } else if code.starts_with("path.") {
        5
    } else {
        9
    }
}

/// A single typed diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub path: String,
    pub code: Code,
    pub message: String,
}

impl Diagnostic {
    #[must_use]
    pub fn error(path: impl Into<String>, code: Code, message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Error,
            path: path.into(),
            code,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn warning(path: impl Into<String>, code: Code, message: impl Into<String>) -> Self {
        Diagnostic {
            severity: Severity::Warning,
            path: path.into(),
            code,
            message: message.into(),
        }
    }

    fn sort_key(&self) -> (u8, Severity, &str, &str, &str) {
        (
            layer_rank(self.code.as_str()),
            self.severity,
            self.path.as_str(),
            self.code.as_str(),
            self.message.as_str(),
        )
    }

    /// The document-root pointer rendered for humans (`""` -> `<root>`). The
    /// machine contract keeps the raw JSON Pointer in [`Diagnostic::to_json`].
    #[must_use]
    pub fn path_display(&self) -> &str {
        if self.path.is_empty() {
            "<root>"
        } else {
            &self.path
        }
    }

    /// The documented `--json` diagnostic object: `{path, code, message}`.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "path": self.path,
            "code": self.code.as_str(),
            "message": self.message,
        })
    }
}

impl Ord for Diagnostic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl PartialOrd for Diagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Overall outcome of a validation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// No error diagnostics.
    Valid,
    /// At least one evaluated-and-failing diagnostic.
    Invalid,
    /// Cannot evaluate the document (parse/document/io/usage/unresolved `$ref`).
    Error,
}

impl Status {
    /// The stable `--json` status token; the CLI exit code is derived from it
    /// (`valid` -> 0, `invalid` -> 1, `error` -> 2).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Status::Valid => "valid",
            Status::Invalid => "invalid",
            Status::Error => "error",
        }
    }
}

/// A validation report: status plus deterministically ordered diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub status: Status,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
}

/// A diagnostic that means "cannot evaluate" rather than "evaluated and failed".
fn is_fatal(code: &str) -> bool {
    code.starts_with("parse.")
        || code.starts_with("document.")
        || code.starts_with("io.")
        || code.starts_with("usage.")
        || code == "schema.unresolved_ref"
        || code == "capability.registry_unavailable"
}

impl Report {
    /// Build a report from diagnostics: sort deterministically, split
    /// errors/warnings, and derive the status.
    #[must_use]
    pub fn from_diagnostics(mut diagnostics: Vec<Diagnostic>) -> Self {
        diagnostics.sort();
        let errors: Vec<Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .cloned()
            .collect();
        let warnings: Vec<Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .cloned()
            .collect();
        let status = if errors.is_empty() {
            Status::Valid
        } else if errors.iter().any(|d| is_fatal(d.code.as_str())) {
            Status::Error
        } else {
            Status::Invalid
        };
        Report {
            status,
            errors,
            warnings,
        }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.status == Status::Valid
    }

    /// The documented `--json` report shape:
    /// `{status, errors:[{path, code, message}], warnings:[...]}`.
    ///
    /// `status` is one of `valid | invalid | error`; `errors` holds every error
    /// diagnostic (empty when valid); `warnings` holds the not-evaluated
    /// notices. Exactly one such object is emitted on stdout.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        let errors: Vec<serde_json::Value> = self.errors.iter().map(Diagnostic::to_json).collect();
        let warnings: Vec<serde_json::Value> =
            self.warnings.iter().map(Diagnostic::to_json).collect();
        serde_json::json!({
            "status": self.status.as_str(),
            "errors": errors,
            "warnings": warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_precede_warnings_then_path_then_code() {
        let mut diagnostics = [
            Diagnostic::warning("/z", Code::schema_other(), "w"),
            Diagnostic::error("/b", Code::schema_keyword("type"), "b"),
            Diagnostic::error("/a", Code::schema_other(), "a2"),
            Diagnostic::error("/a", Code::schema_keyword("required"), "a1"),
        ];
        diagnostics.sort();
        let keys: Vec<(Severity, &str, &str)> = diagnostics
            .iter()
            .map(|d| (d.severity, d.path.as_str(), d.code.as_str()))
            .collect();
        assert_eq!(
            keys,
            vec![
                // same path `/a`: `schema.other` sorts before `schema.required`
                (Severity::Error, "/a", "schema.other"),
                (Severity::Error, "/a", "schema.required"),
                (Severity::Error, "/b", "schema.type"),
                (Severity::Warning, "/z", "schema.other"),
            ]
        );
    }

    #[test]
    fn schema_keyword_falls_back_to_schema_other() {
        assert_eq!(Code::schema_keyword("required").as_str(), "schema.required");
        assert_eq!(
            Code::schema_keyword("unevaluatedProperties").as_str(),
            "schema.unevaluatedProperties"
        );
        assert_eq!(
            Code::schema_keyword("madeUpKeyword").as_str(),
            "schema.other"
        );
    }

    #[test]
    fn fatal_codes_produce_error_status() {
        let report =
            Report::from_diagnostics(vec![Diagnostic::error("", Code::parse_invalid(), "bad")]);
        assert_eq!(report.status, Status::Error);

        let report = Report::from_diagnostics(vec![Diagnostic::error(
            "/metadata/name",
            Code::schema_keyword("pattern"),
            "bad",
        )]);
        assert_eq!(report.status, Status::Invalid);

        let report = Report::from_diagnostics(vec![Diagnostic::warning(
            "/x",
            Code::schema_other(),
            "warn",
        )]);
        assert_eq!(report.status, Status::Valid);
        assert_eq!(report.warnings.len(), 1);
    }
}
