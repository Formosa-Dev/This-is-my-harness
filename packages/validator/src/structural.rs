//! L1 — structural validation (JSON Schema, Draft 2020-12).
//!
//! Runs the embedded-schema validator and maps every `jsonschema` error to a
//! typed [`Diagnostic`]: `error.instance_path()` becomes the JSON Pointer and
//! `error.kind().keyword()` is mapped to this validator's own `schema.*` code.
//!
//! A `$ref` that cannot be resolved from the offline registry is a **loud**
//! failure, never a silent skip: [`validate`] returns
//! [`StructuralError::UnresolvedRef`].

use std::fmt;

use jsonschema::{Draft, Validator};
use serde_json::{json, Value};

use crate::diagnostics::{Code, Diagnostic};
use crate::registry::{self, RegistryError};

/// A structural layer that could not run to completion.
#[derive(Debug)]
pub enum StructuralError {
    /// The embedded registry failed to build.
    Registry(RegistryError),
    /// A `$ref` (here: the per-document wrapper) could not be resolved offline.
    UnresolvedRef { schema_id: String, message: String },
}

impl fmt::Display for StructuralError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StructuralError::Registry(error) => write!(f, "{error}"),
            StructuralError::UnresolvedRef { schema_id, message } => {
                write!(f, "cannot resolve `{schema_id}` offline: {message}")
            }
        }
    }
}

impl std::error::Error for StructuralError {}

/// Validate `instance` against the embedded schema identified by `schema_id`.
///
/// # Errors
///
/// Returns [`StructuralError`] when the registry cannot be built or the wrapper
/// `$ref` cannot be resolved offline. It never short-circuits on the first
/// instance error: all errors are aggregated.
pub fn validate(schema_id: &str, instance: &Value) -> Result<Vec<Diagnostic>, StructuralError> {
    let registry = registry::registry().map_err(StructuralError::Registry)?;
    let wrapper = json!({ "$ref": schema_id });
    let validator: Validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(registry)
        .offline()
        .build(&wrapper)
        .map_err(|error| StructuralError::UnresolvedRef {
            schema_id: schema_id.to_owned(),
            message: error.to_string(),
        })?;
    Ok(collect(&validator, instance))
}

fn collect(validator: &Validator, instance: &Value) -> Vec<Diagnostic> {
    validator
        .iter_errors(instance)
        .map(|error| {
            Diagnostic::error(
                error.instance_path().as_str().to_owned(),
                Code::schema_keyword(error.kind().keyword()),
                error.to_string(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const HARNESS_ID: &str = "https://thisismyharness.dev/schemas/v1alpha1/harness.schema.json";

    #[test]
    fn metadata_name_pattern_violation_reports_path_and_code() {
        let instance = json!({
            "apiVersion": "thisismyharness.dev/v1alpha1",
            "kind": "Harness",
            "metadata": { "name": "Bad/Slug", "version": "1.0.0" },
            "spec": {}
        });
        let diagnostics = validate(HARNESS_ID, &instance).expect("schema must resolve");
        assert!(
            diagnostics
                .iter()
                .any(|d| d.path == "/metadata/name" && d.code.as_str() == "schema.pattern"),
            "expected {{path:/metadata/name, code:schema.pattern}}, got {diagnostics:?}"
        );
    }

    #[test]
    fn root_type_violation_reports_empty_pointer() {
        // An aggregate root (an array) is not a manifest object.
        let instance = json!([{ "kind": "Harness" }]);
        let diagnostics = validate(HARNESS_ID, &instance).expect("schema must resolve");
        assert!(
            diagnostics
                .iter()
                .any(|d| d.path.is_empty() && d.code.as_str() == "schema.type"),
            "expected {{path:'', code:schema.type}}, got {diagnostics:?}"
        );
    }

    #[test]
    fn unknown_schema_id_fails_loudly() {
        let error = validate(
            "https://thisismyharness.dev/schemas/v1alpha1/does-not-exist.schema.json",
            &json!({}),
        )
        .expect_err("an unregistered $id must not resolve");
        assert!(matches!(error, StructuralError::UnresolvedRef { .. }));
    }
}
