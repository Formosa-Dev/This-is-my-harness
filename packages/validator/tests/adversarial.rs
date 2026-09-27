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

// --- WU4 capability + version layers (tasks 4.1-4.5) ------------------------
//
// The capability registry is embedded at build time; the version gate supersedes
// the structural layer on `/apiVersion` (one issue, one code) and classifies an
// unsupported generation as evaluated-and-invalid (Q9: exit-1 semantics).

#[test]
fn unknown_capability_is_an_explicit_error() {
    let bytes = read_fixture("negative/capability.unknown.json");
    let value = harness_validator::parse::parse(&bytes, Format::Json)
        .unwrap_or_else(|diagnostic| panic!("cannot parse fixture: {diagnostic:?}"));
    let diagnostics = harness_validator::capability::validate(Kind::Manifest, &value)
        .expect("the embedded known-capability registry must load");
    let report = Report::from_diagnostics(diagnostics);
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(
        has_error(
            &report,
            "/spec/requirements/modelCapabilities/0/capability",
            "capability.unknown"
        ),
        "got {report:?}"
    );
    assert!(
        report
            .errors
            .iter()
            .any(|d| d.message.contains("models.definitely-not-registered")),
        "the diagnostic must name the identifier: {report:?}"
    );
}

#[test]
fn missing_capabilities_registry_fails_loudly() {
    // No registry available: the loader must fail even though the document
    // declares a capability. It must never treat the identifier as known.
    let value = serde_json::json!({
        "spec": { "requirements": { "modelCapabilities": [ { "capability": "models.text-generation" } ] } }
    });
    let error = harness_validator::capability::validate_with_source(None, Kind::Manifest, &value)
        .expect_err("a missing registry must fail loudly");
    assert!(error.message().contains("missing"), "got {error:?}");

    assert!(
        harness_validator::capability::parse_registry(Some("{}")).is_err(),
        "a registry with no capability set must fail loudly"
    );
}

#[test]
fn unsupported_api_version_is_version_unsupported_and_invalid() {
    let report = harness_validator::validate_structural_and_version(
        HARNESS_ID,
        br#"{"apiVersion":"thisismyharness.dev/v2alpha1","kind":"Harness","metadata":{"name":"x","version":"1.0.0"},"spec":{}}"#,
        Format::Json,
    );
    // Q9: evaluated & invalid (exit-1 semantics), never cannot-evaluate.
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    let version_codes: Vec<&str> = report
        .errors
        .iter()
        .map(|d| d.code.as_str())
        .filter(|code| code.starts_with("version."))
        .collect();
    assert_eq!(version_codes, vec!["version.unsupported"], "got {report:?}");
    assert!(has_error(&report, "/apiVersion", "version.unsupported"));
    assert!(
        !report
            .errors
            .iter()
            .any(|d| d.code.as_str() == "schema.const"),
        "the structural const on /apiVersion must be suppressed: {report:?}"
    );
}

#[test]
fn missing_api_version_is_version_missing_and_invalid() {
    let report = harness_validator::validate_structural_and_version(
        HARNESS_ID,
        br#"{"kind":"Harness","metadata":{"name":"x","version":"1.0.0"},"spec":{}}"#,
        Format::Json,
    );
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(&report, "/apiVersion", "version.missing"));
    assert!(
        !report
            .errors
            .iter()
            .any(|d| d.code.as_str() == "schema.required" && d.path.is_empty()),
        "the superseded required on /apiVersion must be suppressed: {report:?}"
    );
}

#[test]
fn supported_api_version_passes_the_version_gate() {
    let bytes = read_fixture("positive/manifest.minimal.yaml");
    let report =
        harness_validator::validate_structural_and_version(HARNESS_ID, &bytes, Format::Yaml);
    assert_eq!(report.status, Status::Valid, "got {report:?}");
    assert!(!report
        .errors
        .iter()
        .any(|d| d.code.as_str().starts_with("version.")));
}

#[test]
fn positive_corpus_capabilities_are_all_known() {
    let raw = std::fs::read_to_string(fixtures_dir().join("corpus.json"))
        .expect("the promoted corpus must be readable");
    let corpus: CorpusPositives = serde_json::from_str(&raw).expect("corpus must parse");

    let mut checked = 0;
    for entry in &corpus.positive {
        let bytes = read_fixture(&entry.file);
        let value = harness_validator::parse::parse(&bytes, Format::from_filename(&entry.file))
            .unwrap_or_else(|diagnostic| panic!("cannot parse {}: {diagnostic:?}", entry.file));
        let kind = match harness_validator::document::detect(&value) {
            harness_validator::Detection::Known(kind) => kind,
            other => panic!("{}: expected a known kind, got {other:?}", entry.file),
        };
        checked += harness_validator::capability::declared_capabilities(kind, &value).len();
        let diagnostics = harness_validator::capability::validate(kind, &value)
            .expect("the embedded registry must load");
        assert!(
            diagnostics.is_empty(),
            "{}: unexpected capability errors {diagnostics:?}",
            entry.file
        );
    }
    assert!(
        checked > 0,
        "the positive corpus must exercise at least one declared capability"
    );
}

// --- WU5 declared-path safety + layered `validate` (tasks 5.1-5.6) ----------
//
// The filesystem layer checks paths *declared in the document* against a
// provided project root (PARTIAL, F2-08). The layered `validate()` entry wires
// structural -> version -> semantic -> capability -> filesystem and classifies
// un-evaluable inputs as `Status::Error` (exit-2 semantics).

fn full_report(file: &str) -> Report {
    let bytes = read_fixture(file);
    harness_validator::validate(
        &harness_validator::Source {
            bytes: &bytes,
            format: Format::from_filename(file),
        },
        &harness_validator::Options::default(),
    )
}

#[test]
fn declared_dotdot_escape_is_rejected_as_traversal() {
    let value = serde_json::json!({
        "spec": { "components": [ { "type": "Skill", "path": "../outside/SKILL.md" } ] }
    });
    let diagnostics = harness_validator::pathsafe::validate(Kind::Manifest, &value, None);
    assert_eq!(diagnostics.len(), 1, "got {diagnostics:?}");
    assert_eq!(diagnostics[0].path, "/spec/components/0/path");
    assert_eq!(diagnostics[0].code.as_str(), "path.traversal");
}

#[test]
fn declared_absolute_path_is_rejected() {
    let value = serde_json::json!({
        "spec": { "components": [ { "type": "Skill", "path": "/etc/passwd" } ] }
    });
    let diagnostics = harness_validator::pathsafe::validate(Kind::Manifest, &value, None);
    assert_eq!(diagnostics.len(), 1, "got {diagnostics:?}");
    assert_eq!(diagnostics[0].code.as_str(), "path.absolute");
}

#[test]
fn a_declared_path_that_stays_inside_the_root_is_safe() {
    assert_eq!(
        harness_validator::pathsafe::classify("a/../b"),
        harness_validator::pathsafe::Verdict::Safe
    );
    let value = serde_json::json!({
        "spec": { "components": [ { "type": "Skill", "path": "skills/example-skill" } ] }
    });
    assert!(harness_validator::pathsafe::validate(Kind::Manifest, &value, None).is_empty());
}

#[test]
fn symlink_escaping_the_root_is_rejected() {
    use std::path::{Path, PathBuf};

    // A synthetic resolver stands in for the filesystem so the containment
    // logic is provable without creating a real symlink (which, on Windows,
    // requires privileges). `resolve` maps `/project/link` to an outside target.
    let resolve = |path: &Path| -> Option<PathBuf> {
        if path == Path::new("/project") {
            Some(PathBuf::from("/project"))
        } else if path == Path::new("/project/link") {
            Some(PathBuf::from("/outside/target"))
        } else {
            None
        }
    };
    let issue = harness_validator::pathsafe::check_with_resolver(
        "link",
        Some(Path::new("/project")),
        &resolve,
    );
    assert_eq!(
        issue,
        Some(harness_validator::pathsafe::PathIssue::SymlinkEscape)
    );

    assert!(harness_validator::pathsafe::escapes_root(
        Path::new("/project"),
        Path::new("/outside/target")
    ));
    assert!(!harness_validator::pathsafe::escapes_root(
        Path::new("/project"),
        Path::new("/project/skills/x")
    ));
}

#[cfg(unix)]
#[test]
fn a_real_symlink_outside_the_root_is_rejected() {
    use std::os::unix::fs::symlink;

    let base = std::env::temp_dir().join(format!("harness-pathsafe-{}", std::process::id()));
    let root = base.join("project");
    let outside = base.join("outside");
    std::fs::create_dir_all(&root).expect("create root");
    std::fs::create_dir_all(&outside).expect("create outside");
    std::fs::write(outside.join("secret.txt"), b"x").expect("write target");
    symlink(&outside, root.join("link")).expect("create symlink");

    let issue = harness_validator::pathsafe::check("link/secret.txt", Some(&root));
    assert_eq!(
        issue,
        Some(harness_validator::pathsafe::PathIssue::SymlinkEscape),
        "a symlink escaping the root must be rejected"
    );

    std::fs::remove_dir_all(&base).ok();
}

#[test]
fn full_validate_rejects_a_declared_traversal_path() {
    let report = full_report("negative/path.traversal.json");
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/components/0/path",
        "path.traversal"
    ));
}

#[test]
fn full_validate_rejects_a_declared_absolute_path() {
    let report = full_report("negative/path.absolute.json");
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/spec/components/0/path",
        "path.absolute"
    ));
}

#[test]
fn full_validate_unknown_kind_is_error_status() {
    // Zero discriminators: the kind cannot be determined -> cannot evaluate.
    let report = full_report("negative/manifest.unknown-kind.json");
    assert_eq!(report.status, Status::Error, "got {report:?}");
    assert!(has_error(&report, "", "document.unknown_kind"));
}

#[test]
fn full_validate_unparsable_json_is_error_status() {
    let report = harness_validator::validate(
        &harness_validator::Source {
            bytes: b"{ not json",
            format: Format::Json,
        },
        &harness_validator::Options::default(),
    );
    assert_eq!(report.status, Status::Error, "got {report:?}");
    assert_eq!(report.errors[0].code.as_str(), "parse.invalid");
}

#[test]
fn full_validate_inline_hook_plan_is_invalid_at_the_entry_pointer() {
    let report = full_report("negative/install-plan.hooks-inline.json");
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(
        &report,
        "/hooks/0",
        "schema.unevaluatedProperties"
    ));
}

#[test]
fn full_validate_plan_timestamp_is_invalid() {
    let report = full_report("negative/install-plan.timestamp.json");
    assert_eq!(report.status, Status::Invalid, "got {report:?}");
    assert!(has_error(&report, "", "schema.unevaluatedProperties"));
}

#[test]
fn full_validate_kind_override_uses_the_named_schema() {
    // Without the override the kind is undetectable (exit 2); with `--kind
    // manifest` the document is checked against the manifest root schema, so the
    // unknown `kind` becomes a structural `schema.enum` (evaluated & invalid).
    let bytes = read_fixture("negative/manifest.unknown-kind.json");
    let source = harness_validator::Source {
        bytes: &bytes,
        format: Format::Json,
    };
    let detected = harness_validator::validate(&source, &harness_validator::Options::default());
    assert_eq!(detected.status, Status::Error, "got {detected:?}");

    let overridden = harness_validator::validate(
        &source,
        &harness_validator::Options {
            kind: Some(Kind::Manifest),
            root: None,
        },
    );
    assert_eq!(overridden.status, Status::Invalid, "got {overridden:?}");
    assert!(has_error(&overridden, "/kind", "schema.enum"));
}

#[test]
fn positive_corpus_is_valid_under_the_full_pipeline() {
    let raw = std::fs::read_to_string(fixtures_dir().join("corpus.json"))
        .expect("the promoted corpus must be readable");
    let corpus: CorpusPositives = serde_json::from_str(&raw).expect("corpus must parse");
    for entry in &corpus.positive {
        let report = full_report(&entry.file);
        assert_eq!(
            report.status,
            Status::Valid,
            "{}: expected valid under the full pipeline, got {report:?}",
            entry.file
        );
        assert!(
            report.errors.is_empty(),
            "{}: unexpected errors {:?}",
            entry.file,
            report.errors
        );
    }
}
