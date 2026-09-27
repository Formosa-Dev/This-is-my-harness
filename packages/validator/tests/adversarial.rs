//! Adversarial structural cases (tasks 2.9).
//!
//! Two behaviours the primary corpus does not exercise:
//!   * a `$ref` absent from the embedded registry must fail **loudly** with
//!     `schema.unresolved_ref` and a `Status::Error` (not a silent skip);
//!   * structural errors must **aggregate** — the layer must not short-circuit
//!     on the first violation.

use harness_validator::{Format, Status};

const HARNESS_ID: &str = "https://thisismyharness.dev/schemas/v1alpha1/harness.schema.json";
const UNKNOWN_ID: &str = "https://thisismyharness.dev/schemas/v1alpha1/not-registered.schema.json";

#[test]
fn unregistered_schema_id_fails_loudly() {
    let report = harness_validator::validate_structural(UNKNOWN_ID, b"{}", Format::Json);
    assert_eq!(report.status, Status::Error);
    assert_eq!(report.errors.len(), 1, "got {:?}", report.errors);
    assert_eq!(report.errors[0].code.as_str(), "schema.unresolved_ref");
}

#[test]
fn structural_errors_aggregate_without_short_circuit() {
    // `kind` is valid, but the document has: a missing `apiVersion` (required at
    // the root), an empty `metadata` (name + version required) and an unknown
    // top-level field (unevaluatedProperties).
    let instance = br#"{
        "kind": "Harness",
        "metadata": {},
        "spec": {},
        "unknownTopLevel": true
    }"#;
    let report = harness_validator::validate_structural(HARNESS_ID, instance, Format::Json);
    assert_eq!(report.status, Status::Invalid, "got {:?}", report);
    assert!(
        report.errors.len() >= 2,
        "expected aggregated errors, got {:?}",
        report.errors
    );

    let codes: Vec<&str> = report.errors.iter().map(|d| d.code.as_str()).collect();
    assert!(
        codes.contains(&"schema.required"),
        "expected a schema.required, got {codes:?}"
    );
    assert!(
        codes.contains(&"schema.unevaluatedProperties"),
        "expected a schema.unevaluatedProperties, got {codes:?}"
    );
}

#[test]
fn duplicate_key_input_is_a_parse_error_not_a_structural_one() {
    let report = harness_validator::validate_structural(
        HARNESS_ID,
        br#"{"apiVersion": "x", "apiVersion": "y"}"#,
        Format::Json,
    );
    assert_eq!(report.status, Status::Error);
    assert_eq!(report.errors[0].code.as_str(), "parse.duplicate_key");
}

#[test]
fn multiple_yaml_documents_are_rejected_at_the_top_level() {
    let report = harness_validator::validate_structural(
        HARNESS_ID,
        b"---\nkind: Harness\n---\nkind: Preset\n",
        Format::Yaml,
    );
    assert_eq!(report.status, Status::Error);
    assert_eq!(report.errors[0].code.as_str(), "parse.multiple_documents");
}
