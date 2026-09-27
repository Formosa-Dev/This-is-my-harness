//! `harness validate` end-to-end tests (tasks 4.6, 5.3–5.7).
//!
//! Proves the CLI contract at process level: the exit-code taxonomy (0/1/2),
//! the stdout/stderr split, the `--json` shape, kind override, directory
//! discovery, the Q9 version classification, and the read-only guarantee.

use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;

/// The working manifest filename used by the directory-discovery tests.
const VALID_MANIFEST: &[u8] =
    b"apiVersion: thisismyharness.dev/v1alpha1\nkind: Harness\nmetadata:\n  name: minimal-harness\n  version: 0.1.0\nspec: {}\n";

fn harness() -> Command {
    Command::cargo_bin("harness").expect("the `harness` binary must build")
}

fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("validator")
        .join("tests")
        .join("fixtures")
        .join(rel)
}

fn stdout_json(output: &std::process::Output) -> serde_json::Value {
    serde_json::from_slice(&output.stdout).expect("stdout must be exactly one JSON object")
}

fn unique_dir(tag: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock")
        .as_nanos();
    let dir =
        std::env::temp_dir().join(format!("harness-cli-{tag}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn expected_exit(status: &str) -> i32 {
    match status {
        "valid" => 0,
        "invalid" => 1,
        "error" => 2,
        other => panic!("unexpected status `{other}`"),
    }
}

#[test]
fn valid_document_exits_zero_and_human_output_goes_to_stderr() {
    let output = harness()
        .arg("validate")
        .arg(fixture("positive/manifest.minimal.yaml"))
        .output()
        .expect("run harness");

    assert_eq!(output.status.code(), Some(0));
    assert!(
        output.stdout.is_empty(),
        "human mode must leave stdout empty, got {:?}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(!output.stderr.is_empty(), "human output must go to stderr");
}

#[test]
fn json_output_is_one_object_on_stdout_and_suppresses_human_output() {
    let output = harness()
        .args(["validate", "--json"])
        .arg(fixture("positive/manifest.minimal.yaml"))
        .output()
        .expect("run harness");

    assert_eq!(output.status.code(), Some(0));
    assert!(
        output.stderr.is_empty(),
        "--json must not carry human report text on stderr, got {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let value = stdout_json(&output);
    assert_eq!(value["status"], "valid");
    assert_eq!(value["errors"], serde_json::json!([]));
}

#[test]
fn invalid_document_exits_one_with_the_expected_pointer_and_code() {
    let output = harness()
        .args(["validate", "--json"])
        .arg(fixture("negative/manifest.bad-slug.json"))
        .output()
        .expect("run harness");

    assert_eq!(output.status.code(), Some(1));
    let value = stdout_json(&output);
    assert_eq!(value["status"], "invalid");
    let errors = value["errors"].as_array().expect("errors array");
    assert!(
        errors
            .iter()
            .any(|e| e["path"] == "/metadata/name" && e["code"] == "schema.pattern"),
        "expected {{/metadata/name, schema.pattern}}, got {errors:?}"
    );
}

#[test]
fn unsupported_api_version_is_evaluated_and_invalid_exit_one() {
    // Task 4.6 / Q9: an unsupported `apiVersion` is exit 1, NOT exit 2.
    let output = harness()
        .args(["validate", "--json"])
        .arg(fixture("negative/manifest.unknown-apiversion.json"))
        .output()
        .expect("run harness");

    assert_eq!(
        output.status.code(),
        Some(1),
        "Q9: version.unsupported must be exit 1 (evaluated & invalid)"
    );
    let value = stdout_json(&output);
    assert_eq!(value["status"], "invalid");
    let errors = value["errors"].as_array().expect("errors array");
    assert!(
        errors
            .iter()
            .any(|e| e["path"] == "/apiVersion" && e["code"] == "version.unsupported"),
        "expected {{/apiVersion, version.unsupported}}, got {errors:?}"
    );
}

#[test]
fn missing_api_version_is_evaluated_and_invalid_exit_one() {
    // Task 4.6 / Q9: a missing `apiVersion` is exit 1, NOT exit 2.
    let output = harness()
        .args(["validate", "--json"])
        .arg(fixture("negative/manifest.missing-apiversion.json"))
        .output()
        .expect("run harness");

    assert_eq!(
        output.status.code(),
        Some(1),
        "Q9: version.missing must be exit 1 (evaluated & invalid)"
    );
    let value = stdout_json(&output);
    assert_eq!(value["status"], "invalid");
    let errors = value["errors"].as_array().expect("errors array");
    assert!(
        errors
            .iter()
            .any(|e| e["path"] == "/apiVersion" && e["code"] == "version.missing"),
        "expected {{/apiVersion, version.missing}}, got {errors:?}"
    );
}

#[test]
fn unparsable_bytes_exit_two() {
    let dir = unique_dir("unparsable");
    let path = dir.join("broken.json");
    fs::write(&path, b"{ \"apiVersion\": \"x\", ").expect("write fixture");

    let output = harness()
        .args(["validate", "--json"])
        .arg(&path)
        .output()
        .expect("run harness");

    assert_eq!(output.status.code(), Some(2));
    let value = stdout_json(&output);
    assert_eq!(value["status"], "error");
    assert_eq!(value["errors"][0]["code"], "parse.invalid");

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn unknown_kind_exits_two() {
    let output = harness()
        .args(["validate", "--json"])
        .arg(fixture("negative/manifest.unknown-kind.json"))
        .output()
        .expect("run harness");

    assert_eq!(output.status.code(), Some(2));
    let value = stdout_json(&output);
    assert_eq!(value["status"], "error");
    assert_eq!(value["errors"][0]["code"], "document.unknown_kind");
}

#[test]
fn kind_override_uses_the_named_schema() {
    // No override: the kind is undetectable (exit 2). With `--kind manifest` the
    // document is checked against the manifest schema, so `kind: Widget` becomes
    // a structural `schema.enum` (evaluated & invalid, exit 1).
    let detected = harness()
        .args(["validate", "--json"])
        .arg(fixture("negative/manifest.unknown-kind.json"))
        .output()
        .expect("run harness");
    assert_eq!(detected.status.code(), Some(2));

    let overridden = harness()
        .args(["validate", "--json", "--kind", "manifest"])
        .arg(fixture("negative/manifest.unknown-kind.json"))
        .output()
        .expect("run harness");
    assert_eq!(overridden.status.code(), Some(1));
    let value = stdout_json(&overridden);
    let errors = value["errors"].as_array().expect("errors array");
    assert!(
        errors
            .iter()
            .any(|e| e["path"] == "/kind" && e["code"] == "schema.enum"),
        "expected {{/kind, schema.enum}}, got {errors:?}"
    );
}

#[test]
fn declared_path_traversal_exits_one() {
    let output = harness()
        .args(["validate", "--json"])
        .arg(fixture("negative/path.traversal.json"))
        .output()
        .expect("run harness");

    assert_eq!(output.status.code(), Some(1));
    let value = stdout_json(&output);
    let errors = value["errors"].as_array().expect("errors array");
    assert!(
        errors
            .iter()
            .any(|e| e["path"] == "/spec/components/0/path" && e["code"] == "path.traversal"),
        "expected {{/spec/components/0/path, path.traversal}}, got {errors:?}"
    );
}

#[test]
fn missing_path_is_a_usage_error() {
    harness().arg("validate").assert().code(2);
}

#[test]
fn unknown_flag_is_a_usage_error() {
    harness()
        .args(["validate", "--definitely-not-a-flag"])
        .arg(fixture("positive/manifest.minimal.yaml"))
        .assert()
        .code(2);
}

#[test]
fn directory_input_discovers_the_single_manifest() {
    let dir = unique_dir("discovery");
    fs::write(dir.join("harness.yaml"), VALID_MANIFEST).expect("write manifest");

    harness().arg("validate").arg(&dir).assert().code(0);

    // A second candidate is ambiguous: full discovery is deferred to F4.
    fs::write(dir.join("harness.json"), b"{}").expect("write second candidate");
    let ambiguous = harness()
        .args(["validate", "--json"])
        .arg(&dir)
        .output()
        .expect("run harness");
    assert_eq!(ambiguous.status.code(), Some(2));
    assert_eq!(stdout_json(&ambiguous)["status"], "error");

    fs::remove_dir_all(&dir).ok();
}

#[test]
fn exit_code_agrees_with_json_status_on_every_run() {
    let cases = [
        "positive/manifest.minimal.yaml",
        "negative/manifest.bad-slug.json",
        "negative/manifest.unknown-apiversion.json",
        "negative/manifest.unknown-kind.json",
        "negative/path.traversal.json",
    ];
    for case in cases {
        let output = harness()
            .args(["validate", "--json"])
            .arg(fixture(case))
            .output()
            .expect("run harness");
        let value = stdout_json(&output);
        let status = value["status"].as_str().expect("status token");
        assert_eq!(
            output.status.code(),
            Some(expected_exit(status)),
            "{case}: exit code must agree with status `{status}`"
        );
    }
}

#[test]
fn json_matches_the_golden_snapshots() {
    let cases = [
        ("positive/manifest.minimal.yaml", "valid.manifest.json"),
        (
            "negative/manifest.bad-slug.json",
            "invalid.manifest-bad-slug.json",
        ),
    ];
    for (fixture_rel, golden_rel) in cases {
        let output = harness()
            .args(["validate", "--json"])
            .arg(fixture(fixture_rel))
            .output()
            .expect("run harness");
        let actual = stdout_json(&output);

        let golden_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("snapshots")
            .join(golden_rel);
        let golden: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(&golden_path)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", golden_path.display())),
        )
        .expect("golden snapshot must parse");

        assert_eq!(actual, golden, "{fixture_rel}: --json output drifted");
    }
}

#[test]
fn validate_is_read_only() {
    let dir = unique_dir("readonly");
    fs::write(dir.join("harness.yaml"), VALID_MANIFEST).expect("write manifest");
    fs::write(dir.join("notes.txt"), b"do not touch\n").expect("write bystander");

    let before = snapshot(&dir);

    // A directory run (exit 0) and a file run (exit 0) must both be inert.
    harness().arg("validate").arg(&dir).assert().code(0);
    harness()
        .args(["validate", "--json"])
        .arg(dir.join("harness.yaml"))
        .assert()
        .code(0);

    let after = snapshot(&dir);
    assert_eq!(
        before, after,
        "`harness validate` must not create, modify or delete any input file"
    );

    fs::remove_dir_all(&dir).ok();
}

/// A content+name fingerprint of the regular files directly under `dir`.
fn snapshot(dir: &Path) -> Vec<(String, String)> {
    use std::hash::{Hash, Hasher};

    let mut entries: Vec<(String, String)> = Vec::new();
    for entry in fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("dir entry").path();
        if !path.is_file() {
            continue;
        }
        let name = path
            .file_name()
            .expect("file name")
            .to_string_lossy()
            .into_owned();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        fs::read(&path).expect("read file").hash(&mut hasher);
        entries.push((name, format!("{:x}", hasher.finish())));
    }
    entries.sort();
    entries
}
