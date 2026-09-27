//! The primary gate: drive the promoted corpus (7 positive / 16 negative).
//!
//! Every positive MUST validate; every negative MUST be rejected with at least
//! one diagnostic matching the corpus' golden `{path, keyword}` — where the
//! corpus `keyword` is mapped to this validator's own `schema.<keyword>` code.
//! The counts are asserted so a silently-truncated corpus is a failure.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use harness_validator::{Format, Status};

#[derive(Debug, Deserialize)]
struct Corpus {
    positive: Vec<PositiveEntry>,
    negative: Vec<NegativeEntry>,
}

#[derive(Debug, Deserialize)]
struct PositiveEntry {
    schema: String,
    file: String,
}

#[derive(Debug, Deserialize)]
struct NegativeEntry {
    schema: String,
    file: String,
    expect: Expect,
}

#[derive(Debug, Deserialize)]
struct Expect {
    path: String,
    keyword: String,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn load_corpus() -> Corpus {
    let path = fixtures_dir().join("corpus.json");
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
}

fn read_instance(file: &str) -> Vec<u8> {
    let path = fixtures_dir().join(file);
    std::fs::read(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

#[test]
fn corpus_positive_and_negative_match_golden_expectations() {
    let corpus = load_corpus();

    assert_eq!(corpus.positive.len(), 7, "positive corpus count changed");
    assert_eq!(corpus.negative.len(), 16, "negative corpus count changed");

    let mut mismatches: Vec<String> = Vec::new();
    let schemas = BTreeSet::new();
    let mut schemas = schemas;

    for entry in &corpus.positive {
        schemas.insert(entry.schema.clone());
        let report = harness_validator::validate_structural(
            &entry.schema,
            &read_instance(&entry.file),
            Format::from_filename(&entry.file),
        );
        if report.status != Status::Valid {
            mismatches.push(format!(
                "positive {}: expected valid, got {:?} errors={:?}",
                entry.file, report.status, report.errors
            ));
        }
    }

    for entry in &corpus.negative {
        schemas.insert(entry.schema.clone());
        let report = harness_validator::validate_structural(
            &entry.schema,
            &read_instance(&entry.file),
            Format::from_filename(&entry.file),
        );
        if report.status != Status::Invalid {
            mismatches.push(format!(
                "negative {}: expected invalid, got {:?} (diagnostics={:?})",
                entry.file, report.status, report.errors
            ));
            continue;
        }
        let expected_code = format!("schema.{}", entry.expect.keyword);
        let matched = report.errors.iter().any(|diagnostic| {
            diagnostic.path == entry.expect.path && diagnostic.code.as_str() == expected_code
        });
        if !matched {
            mismatches.push(format!(
                "negative {}: no diagnostic {{path: {:?}, code: {:?}}}; got {:?}",
                entry.file,
                entry.expect.path,
                expected_code,
                report
                    .errors
                    .iter()
                    .map(|d| (d.path.as_str(), d.code.as_str()))
                    .collect::<Vec<_>>()
            ));
        }
    }

    assert!(
        mismatches.is_empty(),
        "corpus mismatches ({}):\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );

    println!(
        "positive={} negative={} mismatches={} schemas={}",
        corpus.positive.len(),
        corpus.negative.len(),
        mismatches.len(),
        schemas.len()
    );
}

#[test]
fn prove_one_positive_validates_and_one_negative_fails_with_expected_path_and_code() {
    const HARNESS_ID: &str = "https://thisismyharness.dev/schemas/v1alpha1/harness.schema.json";

    let positive = harness_validator::validate_structural(
        HARNESS_ID,
        &read_instance("positive/manifest.minimal.yaml"),
        Format::Yaml,
    );
    println!(
        "POSITIVE positive/manifest.minimal.yaml -> status={:?} errors={}",
        positive.status,
        positive.errors.len()
    );
    assert_eq!(positive.status, Status::Valid);

    let negative = harness_validator::validate_structural(
        HARNESS_ID,
        &read_instance("negative/manifest.bad-slug.json"),
        Format::Json,
    );
    println!(
        "NEGATIVE negative/manifest.bad-slug.json -> status={:?} diagnostics={:?}",
        negative.status,
        negative
            .errors
            .iter()
            .map(|d| (d.path.as_str(), d.code.as_str()))
            .collect::<Vec<_>>()
    );
    assert_eq!(negative.status, Status::Invalid);
    assert!(
        negative
            .errors
            .iter()
            .any(|d| d.path == "/metadata/name" && d.code.as_str() == "schema.pattern"),
        "expected {{path:/metadata/name, code:schema.pattern}}, got {:?}",
        negative.errors
    );
}
