//! Adversarial structural cases (tasks 2.9).
//!
//! Two behaviours the primary corpus does not exercise:
//!   * a `$ref` absent from the embedded registry must fail **loudly** with
//!     `schema.unresolved_ref` and a `Status::Error` (not a silent skip);
//!   * structural errors must **aggregate** — the layer must not short-circuit
//!     on the first violation.

use harness_validator::{Format, Kind, Report, Status};

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

// --- L2 document-local semantic layer (tasks 3.1-3.6) -----------------------
//
// Every rule gets a positive/negative proof with an explicit `{path, code}`.
// The semantic layer is exercised directly (parse -> detect/select kind ->
// `semantic::validate`), so it is independent of the L1 JSON Schema layer.

fn fixtures_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn read_fixture(file: &str) -> Vec<u8> {
    let path = fixtures_dir().join(file);
    std::fs::read(&path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

fn semantic_report(file: &str, kind: Kind) -> Report {
    let bytes = read_fixture(file);
    let value = harness_validator::parse::parse(&bytes, Format::from_filename(file))
        .unwrap_or_else(|diagnostic| panic!("cannot parse {file}: {diagnostic:?}"));
    Report::from_diagnostics(harness_validator::semantic::validate(kind, &value))
}

fn has_error(report: &Report, path: &str, code: &str) -> bool {
    report
        .errors
        .iter()
        .any(|d| d.path == path && d.code.as_str() == code)
}

fn has_warning(report: &Report, path: &str, code: &str) -> bool {
    report
        .warnings
        .iter()
        .any(|d| d.path == path && d.code.as_str() == code)
}

#[test]
fn semantic_short_reference_is_forbidden() {
    let report = semantic_report("negative/semantic.identity-short-ref.json", Kind::Manifest);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/extends/0/reference",
        "semantic.identity_ref_short_forbidden"
    ));
}

#[test]
fn semantic_host_mismatch_is_reported() {
    let report = semantic_report(
        "negative/semantic.identity-host-mismatch.json",
        Kind::Manifest,
    );
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/extends/1/reference",
        "semantic.identity_host_mismatch"
    ));
}

#[test]
fn semantic_self_reference_is_a_cycle() {
    let report = semantic_report("negative/semantic.dependency-cycle.json", Kind::Manifest);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/extends/0/reference",
        "semantic.dependency_cycle"
    ));
}

#[test]
fn semantic_duplicate_extends_entry_is_an_order_violation() {
    let report = semantic_report("negative/semantic.dependency-order.json", Kind::Manifest);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/extends/1/reference",
        "semantic.dependency_order"
    ));
}

#[test]
fn semantic_component_runtime_is_a_kind_composition_violation() {
    let report = semantic_report("negative/semantic.kind-composition.json", Kind::Manifest);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/requirements/runtime",
        "semantic.kind_composition"
    ));
}

#[test]
fn semantic_service_without_permissions_violates_coverage() {
    let report = semantic_report("negative/semantic.permission-coverage.json", Kind::Manifest);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/permissions",
        "semantic.permission_coverage"
    ));
}

#[test]
fn semantic_plan_autonomy_below_floor_is_rejected() {
    let report = semantic_report(
        "negative/semantic.autonomy-below-floor.json",
        Kind::InstallPlan,
    );
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/risk/autonomyLevel",
        "semantic.autonomy_below_floor"
    ));
}

#[test]
fn semantic_plan_effective_class_below_parts_is_rejected() {
    let report = semantic_report("negative/semantic.effective-risk.json", Kind::InstallPlan);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/risk/effectiveClass",
        "semantic.effective_risk"
    ));
}

#[test]
fn semantic_env_value_like_is_reported_and_the_value_is_never_echoed() {
    let report = semantic_report("negative/semantic.env-value.json", Kind::InstallPlan);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/environmentVariableNames/0",
        "semantic.env_value_like"
    ));
    for diagnostic in report.errors.iter().chain(report.warnings.iter()) {
        assert!(
            !diagnostic.message.contains("SUPERSECRETSENTINEL"),
            "the declared value must never be echoed: {diagnostic:?}"
        );
    }
}

#[test]
fn semantic_license_conflict_is_reported() {
    let report = semantic_report("negative/semantic.license-conflict.json", Kind::Manifest);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/components/0/payload/license",
        "semantic.license_conflict"
    ));
}

#[test]
fn semantic_license_shape_heuristic_is_reported() {
    let report = semantic_report("negative/semantic.license-shape.json", Kind::ModelContract);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(&report, "/license", "semantic.license_shape"));
}

#[test]
fn semantic_license_conflict_defers_when_the_package_license_is_missing() {
    let report = semantic_report(
        "negative/semantic.license-not-evaluated.json",
        Kind::Manifest,
    );
    assert_eq!(report.status, Status::Valid, "got {report:?}");
    assert!(has_warning(
        &report,
        "/spec/components/0/payload/license",
        "semantic.not_evaluated"
    ));
}

#[test]
fn semantic_required_service_without_component_is_deferred() {
    let report = semantic_report(
        "negative/semantic.requirements-coherence.json",
        Kind::Manifest,
    );
    assert_eq!(report.status, Status::Valid, "got {report:?}");
    assert!(has_warning(
        &report,
        "/spec/requirements/services/0",
        "semantic.not_evaluated"
    ));
}

#[test]
fn semantic_deferred_checks_warn_without_changing_status() {
    // `extends` alone cannot be resolved document-locally: the warning names the
    // owning phase (F4) and the status stays Valid.
    let report = semantic_report("negative/semantic.dependency-order.json", Kind::Manifest);
    assert!(
        has_warning(&report, "/spec/extends", "semantic.not_evaluated"),
        "expected a deferred F4 warning, got {report:?}"
    );
    assert!(
        report
            .warnings
            .iter()
            .any(|d| d.code.as_str() == "semantic.not_evaluated" && d.message.contains("F4")),
        "the deferred warning must name the owning phase: {report:?}"
    );
}

#[derive(Debug, serde::Deserialize)]
struct PositiveEntry {
    file: String,
}

#[derive(Debug, serde::Deserialize)]
struct CorpusPositives {
    positive: Vec<PositiveEntry>,
}

#[test]
fn positive_corpus_has_no_semantic_errors() {
    let raw = std::fs::read_to_string(fixtures_dir().join("corpus.json"))
        .expect("the promoted corpus must be readable");
    let corpus: CorpusPositives = serde_json::from_str(&raw).expect("corpus must parse");
    assert_eq!(corpus.positive.len(), 7, "positive corpus count changed");

    for entry in &corpus.positive {
        let bytes = read_fixture(&entry.file);
        let value = harness_validator::parse::parse(&bytes, Format::from_filename(&entry.file))
            .unwrap_or_else(|diagnostic| panic!("cannot parse {}: {diagnostic:?}", entry.file));
        let kind = match harness_validator::document::detect(&value) {
            harness_validator::Detection::Known(kind) => kind,
            other => panic!("{}: expected a known kind, got {other:?}", entry.file),
        };
        let report = Report::from_diagnostics(harness_validator::semantic::validate(kind, &value));
        assert!(
            report.errors.is_empty(),
            "{}: unexpected semantic errors {:?}",
            entry.file,
            report.errors
        );
    }
}
