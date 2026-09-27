//! Conformance fixture runner (F3-06).
//!
//! Drives `conformance/fixtures/index.json` through the same entry the CLI
//! calls ([`harness_validator::validate`]) and compares each fixture against its
//! golden `{status, diagnostics}` from the index.
//!
//! Categories:
//!
//! * **conformant** — MUST be `valid` with no error diagnostics;
//! * **invalid** — MUST be `invalid` (or `error` for a cannot-evaluate
//!   condition) and produce every expected `{path, code}`;
//! * **path-safety** — as invalid, plus the synthetic `symlink` runner below;
//! * **plan** — the Install Plan shape MUST validate and its golden
//!   `.expected.json` MUST be coherent with it. The runtime diff itself is NOT
//!   evaluated until F4 ships the plan engine (`pending_f4`).
//!
//! The suite is one command: `cargo test -p harness-validator --test conformance`
//! (or `cargo test --workspace`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

use harness_validator::pathsafe::{check_with_resolver, declared_paths, PathIssue};
use harness_validator::{parse, Format, Kind, Options, Report, Source, Status};

#[derive(Debug, Deserialize)]
struct Index {
    counts: Counts,
    fixtures: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Counts {
    fixtures: usize,
    conformant: usize,
    invalid: usize,
    #[serde(rename = "path-safety")]
    path_safety: usize,
    plan: usize,
}

#[derive(Debug, Deserialize)]
struct Entry {
    id: String,
    category: String,
    root: String,
    #[serde(default = "default_manifest")]
    manifest: String,
    #[serde(default)]
    conformant: bool,
    #[serde(default = "default_runner")]
    runner: String,
    #[serde(default)]
    pending_f4: bool,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Expected {
    status: String,
    #[serde(default)]
    diagnostics: Vec<ExpectedDiagnostic>,
}

#[derive(Debug, Deserialize)]
struct ExpectedDiagnostic {
    path: String,
    code: String,
}

fn default_manifest() -> String {
    "harness.yaml".to_owned()
}

fn default_runner() -> String {
    "manifest".to_owned()
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the repo root must resolve")
}

fn load_index() -> Index {
    let path = repo_root()
        .join("conformance")
        .join("fixtures")
        .join("index.json");
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|error| panic!("cannot parse {}: {error}", path.display()))
}

fn read_value(path: &Path) -> Value {
    let raw = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|error| panic!("cannot parse {}: {error}", path.display()))
}

fn run_validate_with(
    dir: &Path,
    manifest: &str,
    kind: Option<Kind>,
    root: Option<&Path>,
) -> Report {
    let path = dir.join(manifest);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    let format = Format::from_filename(&path.to_string_lossy());
    harness_validator::validate(
        &Source {
            bytes: &bytes,
            format,
        },
        &Options {
            kind,
            root: root.map(Path::to_path_buf),
        },
    )
}

fn codes(report: &Report) -> Vec<String> {
    report
        .errors
        .iter()
        .map(|diagnostic| format!("{}@{}", diagnostic.code.as_str(), diagnostic.path))
        .collect()
}

#[test]
fn conformance_suite_matches_the_index() {
    let index = load_index();
    assert!(
        index.counts.fixtures > 0,
        "the fixture index must not be empty"
    );

    let mut by_category: BTreeMap<String, usize> = BTreeMap::new();
    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut failures: Vec<String> = Vec::new();

    for entry in &index.fixtures {
        if !seen_ids.insert(entry.id.as_str()) {
            failures.push(format!("{}: duplicate fixture id", entry.id));
            continue;
        }

        *by_category.entry(entry.category.clone()).or_default() += 1;

        if (entry.category == "conformant") != entry.conformant {
            failures.push(format!(
                "{}: conformant flag does not match category {}",
                entry.id, entry.category
            ));
        }
        if entry.pending_f4 != (entry.category == "plan") {
            failures.push(format!(
                "{}: pending_f4 does not match category {}",
                entry.id, entry.category
            ));
        }

        if let Err(message) = evaluate(entry) {
            failures.push(message);
        }
    }

    check_counts(&index.counts, &by_category, &mut failures);

    assert!(
        failures.is_empty(),
        "conformance mismatches ({}):\n{}",
        failures.len(),
        failures.join("\n")
    );

    println!(
        "conformance fixtures={} conformant={} invalid={} path_safety={} plan={}",
        index.counts.fixtures,
        count(&by_category, "conformant"),
        count(&by_category, "invalid"),
        count(&by_category, "path-safety"),
        count(&by_category, "plan"),
    );
}

fn count(by_category: &BTreeMap<String, usize>, category: &str) -> usize {
    by_category.get(category).copied().unwrap_or(0)
}

fn check_counts(
    counts: &Counts,
    by_category: &BTreeMap<String, usize>,
    failures: &mut Vec<String>,
) {
    let actual_total: usize = by_category.values().sum();
    let checks = [
        ("fixtures", counts.fixtures, actual_total),
        (
            "conformant",
            counts.conformant,
            count(by_category, "conformant"),
        ),
        ("invalid", counts.invalid, count(by_category, "invalid")),
        (
            "path-safety",
            counts.path_safety,
            count(by_category, "path-safety"),
        ),
        ("plan", counts.plan, count(by_category, "plan")),
    ];
    for (name, declared, actual) in checks {
        if declared != actual {
            failures.push(format!(
                "index counts.{name}={declared} does not match the {actual} fixture(s) present"
            ));
        }
    }
}

fn evaluate(entry: &Entry) -> Result<(), String> {
    let dir = repo_root().join(&entry.root);
    match entry.runner.as_str() {
        "manifest" => evaluate_manifest(entry, &dir),
        "symlink" => evaluate_symlink(entry, &dir),
        "plan" => evaluate_plan(entry, &dir),
        other => Err(format!("{}: unknown runner `{other}`", entry.id)),
    }
}

fn evaluate_manifest(entry: &Entry, dir: &Path) -> Result<(), String> {
    let report = run_validate_with(dir, &entry.manifest, None, Some(dir));
    let status = report.status.as_str();
    if status != entry.expected.status {
        return Err(format!(
            "{}: expected status {}, got {status} ({:?})",
            entry.id,
            entry.expected.status,
            codes(&report)
        ));
    }
    check_diagnostics(entry, &report)
}

fn check_diagnostics(entry: &Entry, report: &Report) -> Result<(), String> {
    for expected in &entry.expected.diagnostics {
        let found = report.errors.iter().any(|diagnostic| {
            diagnostic.path == expected.path && diagnostic.code.as_str() == expected.code
        });
        if !found {
            return Err(format!(
                "{}: missing expected diagnostic {{path: {:?}, code: {:?}}}; got {:?}",
                entry.id,
                expected.path,
                expected.code,
                codes(report)
            ));
        }
    }
    Ok(())
}

/// The symlink case cannot be committed as a link (see the fixture's SETUP.md),
/// so the runner first tries to materialize a real link and falls back to the
/// filesystem layer's public injected-resolver seam when the host refuses.
fn evaluate_symlink(entry: &Entry, dir: &Path) -> Result<(), String> {
    let path = dir.join(&entry.manifest);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    let value = parse::parse(&bytes, Format::from_filename(&entry.manifest))
        .unwrap_or_else(|diagnostic| panic!("cannot parse {}: {diagnostic:?}", path.display()));

    let declared = value
        .pointer("/spec/components/0/path")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{}: the fixture declares no component path", entry.id))?;

    let expected = entry
        .expected
        .diagnostics
        .first()
        .ok_or_else(|| format!("{}: the fixture expects no diagnostic", entry.id))?;

    let pointer = declared_paths(Kind::Manifest, &value)
        .into_iter()
        .find(|declared_path| declared_path.value == declared)
        .map(|declared_path| declared_path.path)
        .ok_or_else(|| {
            format!(
                "{}: the declared path is not a typed path position",
                entry.id
            )
        })?;
    if pointer != expected.path {
        return Err(format!(
            "{}: expected diagnostic at {}, but the path is declared at {pointer}",
            entry.id, expected.path
        ));
    }

    let issue = match real_symlink_verdict(dir, declared) {
        Some(issue) => issue,
        None => {
            println!(
                "note: {}: the host cannot create symlinks; using the injected resolver",
                entry.id
            );
            let root = dir;
            let outside = std::env::temp_dir().join("harness-symlink-escape-target");
            let resolve = |candidate: &Path| -> Option<PathBuf> {
                if candidate == root {
                    Some(root.to_path_buf())
                } else if candidate == root.join(declared) {
                    Some(outside.clone())
                } else {
                    None
                }
            };
            check_with_resolver(declared, Some(root), &resolve)
        }
    };

    match issue {
        Some(found) if found.code().as_str() == expected.code => Ok(()),
        other => Err(format!(
            "{}: expected {} at {}, got {:?}",
            entry.id,
            expected.code,
            expected.path,
            other.map(|issue| issue.code().as_str())
        )),
    }
}

/// Try to prove the escape with a real directory symlink. Returns `None` when
/// the host cannot create one; otherwise the filesystem layer's verdict.
fn real_symlink_verdict(dir: &Path, declared: &str) -> Option<Option<PathIssue>> {
    let outside = std::env::temp_dir().join(format!("harness-symlink-{}", std::process::id()));
    std::fs::create_dir_all(&outside).ok()?;
    let link = dir.join(declared);
    if link.symlink_metadata().is_ok() {
        let _ = std::fs::remove_dir_all(&outside);
        return None;
    }
    if create_dir_symlink(&outside, &link).is_err() {
        let _ = std::fs::remove_dir_all(&outside);
        return None;
    }
    let report = run_validate_with(dir, "harness.yaml", None, Some(dir));
    let escaped = report
        .errors
        .iter()
        .any(|diagnostic| diagnostic.code.as_str() == "path.symlink_escape");
    let _ = std::fs::remove_file(&link);
    let _ = std::fs::remove_dir_all(&outside);
    Some(escaped.then_some(PathIssue::SymlinkEscape))
}

#[cfg(unix)]
fn create_dir_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_dir_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}

fn evaluate_plan(entry: &Entry, dir: &Path) -> Result<(), String> {
    let report = run_validate_with(dir, &entry.manifest, Some(Kind::InstallPlan), None);
    if report.status != Status::Valid {
        return Err(format!(
            "{}: the plan fixture must be a valid Install Plan, got {:?} ({:?})",
            entry.id,
            report.status,
            codes(&report)
        ));
    }
    if !entry.pending_f4 {
        return Err(format!("{}: a plan fixture must set pending_f4", entry.id));
    }

    let plan = read_value(&dir.join(&entry.manifest));
    let expected = read_value(&dir.join("expected.json"));
    if expected.get("pending_f4").and_then(Value::as_bool) != Some(true) {
        return Err(format!(
            "{}: expected.json must mark the runtime diff pending_f4",
            entry.id
        ));
    }

    check_projection(entry, &plan, &expected, "created", "create")?;
    check_projection(entry, &plan, &expected, "modified", "modify")?;
    check_conflicts(entry, &plan, &expected)
}

/// Every `created`/`modified` entry in the golden diff MUST correspond to a
/// file in the plan with the matching action. This proves the projection is
/// coherent while the diff engine itself is pending F4.
fn check_projection(
    entry: &Entry,
    plan: &Value,
    expected: &Value,
    key: &str,
    default_action: &str,
) -> Result<(), String> {
    let Some(items) = expected.get(key).and_then(Value::as_array) else {
        return Ok(());
    };
    for item in items {
        let path = item
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{}: `{key}` entry without a path", entry.id))?;
        let action = item
            .get("action")
            .and_then(Value::as_str)
            .unwrap_or(default_action);
        let found = plan
            .get("files")
            .and_then(Value::as_array)
            .is_some_and(|files| {
                files.iter().any(|file| {
                    file.get("path").and_then(Value::as_str) == Some(path)
                        && file.get("action").and_then(Value::as_str) == Some(action)
                })
            });
        if !found {
            return Err(format!(
                "{}: golden `{key}` entry {path:?} ({action}) is not in the plan files",
                entry.id
            ));
        }
    }
    Ok(())
}

fn check_conflicts(entry: &Entry, plan: &Value, expected: &Value) -> Result<(), String> {
    let Some(items) = expected.get("conflict").and_then(Value::as_array) else {
        return Ok(());
    };
    for item in items {
        let path = item
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{}: `conflict` entry without a path", entry.id))?;
        let found = plan
            .get("conflicts")
            .and_then(Value::as_array)
            .is_some_and(|conflicts| {
                conflicts
                    .iter()
                    .any(|conflict| conflict.get("path").and_then(Value::as_str) == Some(path))
            });
        if !found {
            return Err(format!(
                "{}: golden `conflict` entry {path:?} is not in the plan conflicts",
                entry.id
            ));
        }
    }
    Ok(())
}
