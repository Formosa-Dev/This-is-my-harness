//! L5 — declared-path safety (F2-08, design §4 row 8).
//!
//! In the spec's fixed layer order this is the **filesystem** layer:
//!
//! ```text
//! structural -> version -> semantic -> capability -> filesystem
//! ```
//!
//! A document declares paths at schema-typed positions (a component `path`, a
//! plan's managed-file / conflict / hook paths, a snapshot's `covers`). This
//! layer rejects a declared path that escapes the **provided project root**:
//!
//! * an absolute path (`path.absolute`);
//! * a `..` segment that walks above the root (`path.traversal`);
//! * a symlink that resolves outside the root (`path.symlink_escape`).
//!
//! **Scope is PARTIAL (design §4 row 8):** only paths *declared in the single
//! document* are checked. Package/tree discovery, and symlink containment over a
//! whole tree, stay in F4. Lexical checks (absolute/traversal) run without a
//! root; the symlink check runs only when a root is provided and can be
//! canonicalized.
//!
//! **Read-only:** the symlink check may canonicalize a path (a read), but never
//! opens, writes or executes the target; a link that escapes the root is
//! *reported*, never followed to do work outside the root.

use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic};
use crate::document::Kind;

/// The lexical verdict for a declared path, before any filesystem access.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Relative, and every `..` segment stays within the root.
    Safe,
    /// An absolute path (`/x`, `\x`, `C:\x`, UNC).
    Absolute,
    /// A `..` segment walks above the root.
    Traversal,
}

/// A declared-path problem, resolved after the filesystem check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathIssue {
    Absolute,
    Traversal,
    SymlinkEscape,
}

impl PathIssue {
    /// The stable diagnostic code for this issue.
    #[must_use]
    pub const fn code(self) -> Code {
        match self {
            PathIssue::Absolute => Code::path_absolute(),
            PathIssue::Traversal => Code::path_traversal(),
            PathIssue::SymlinkEscape => Code::path_symlink_escape(),
        }
    }

    /// A human-facing (non-normative) message naming the declared path.
    #[must_use]
    pub fn message(self, declared: &str) -> String {
        match self {
            PathIssue::Absolute => {
                format!("declared path `{declared}` is absolute; paths must be relative to the project root")
            }
            PathIssue::Traversal => {
                format!("declared path `{declared}` escapes the project root via `..`")
            }
            PathIssue::SymlinkEscape => {
                format!("declared path `{declared}` resolves outside the project root through a symlink")
            }
        }
    }
}

/// Whether a declared path is absolute on any supported platform: POSIX (`/x`),
/// a Windows drive path (`C:\x` / `C:/x`) or a UNC path (`\\server\share`).
#[must_use]
pub fn is_absolute_declared(declared: &str) -> bool {
    if declared.starts_with('/') || declared.starts_with('\\') {
        return true;
    }
    let bytes = declared.as_bytes();
    bytes.len() >= 2
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes.len() == 2 || bytes[2] == b'/' || bytes[2] == b'\\')
}

/// Classify a declared path lexically (no filesystem access).
#[must_use]
pub fn classify(declared: &str) -> Verdict {
    if is_absolute_declared(declared) {
        return Verdict::Absolute;
    }
    let mut depth: i64 = 0;
    for segment in declared.split(['/', '\\']) {
        match segment {
            "" | "." => {}
            ".." => {
                depth -= 1;
                if depth < 0 {
                    return Verdict::Traversal;
                }
            }
            _ => depth += 1,
        }
    }
    Verdict::Safe
}

/// Whether `target` (canonical) lies outside `root` (canonical).
#[must_use]
pub fn escapes_root(root: &Path, target: &Path) -> bool {
    !target.starts_with(root)
}

/// Resolve a declared path to the filesystem, using an injected resolver.
///
/// `resolve` maps a path to its canonical form (the production default is
/// [`std::fs::canonicalize`]); tests inject a synthetic mapping so the
/// symlink-containment logic is provable without creating real links. Returns
/// the issue when the declared path is absolute, traverses, or resolves outside
/// the root; `None` when it is safe or cannot be resolved.
pub fn check_with_resolver(
    declared: &str,
    root: Option<&Path>,
    resolve: &dyn Fn(&Path) -> Option<PathBuf>,
) -> Option<PathIssue> {
    match classify(declared) {
        Verdict::Absolute => return Some(PathIssue::Absolute),
        Verdict::Traversal => return Some(PathIssue::Traversal),
        Verdict::Safe => {}
    }

    let root = root?;
    let canonical_root = resolve(root)?;
    let target = resolve(&root.join(declared))?;
    if escapes_root(&canonical_root, &target) {
        Some(PathIssue::SymlinkEscape)
    } else {
        None
    }
}

/// Check a declared path against a real filesystem root (production entry).
#[must_use]
pub fn check(declared: &str, root: Option<&Path>) -> Option<PathIssue> {
    check_with_resolver(declared, root, &canonicalize)
}

fn canonicalize(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}

/// A declared path together with the JSON Pointer of the position that holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredPath {
    pub path: String,
    pub value: String,
}

/// Collect the schema-typed declared-path positions for `kind`.
///
/// * **manifest** — `spec.components[*].path`;
/// * **install-plan** — `files[*].path`, `conflicts[*].path`, `hooks[*].path`,
///   `snapshot.covers[*]`;
/// * **model-contract** — none.
///
/// A plan's `project` field is deliberately **not** checked: it is a target
/// *root* (a local path), not a package-relative path, and may legitimately be
/// absolute.
#[must_use]
pub fn declared_paths(kind: Kind, value: &Value) -> Vec<DeclaredPath> {
    let mut declared = Vec::new();
    match kind {
        Kind::Manifest => collect_object_field(&mut declared, value, "/spec/components", "path"),
        Kind::InstallPlan => {
            collect_object_field(&mut declared, value, "/files", "path");
            collect_object_field(&mut declared, value, "/conflicts", "path");
            collect_object_field(&mut declared, value, "/hooks", "path");
            collect_string_array(&mut declared, value, "/snapshot/covers");
        }
        Kind::ModelContract => {}
    }
    declared
}

fn collect_object_field(out: &mut Vec<DeclaredPath>, root: &Value, array_ptr: &str, field: &str) {
    let Some(items) = root.pointer(array_ptr).and_then(Value::as_array) else {
        return;
    };
    for (index, item) in items.iter().enumerate() {
        if let Some(value) = item.get(field).and_then(Value::as_str) {
            out.push(DeclaredPath {
                path: format!("{array_ptr}/{index}/{field}"),
                value: value.to_owned(),
            });
        }
    }
}

fn collect_string_array(out: &mut Vec<DeclaredPath>, root: &Value, array_ptr: &str) {
    let Some(items) = root.pointer(array_ptr).and_then(Value::as_array) else {
        return;
    };
    for (index, item) in items.iter().enumerate() {
        if let Some(value) = item.as_str() {
            out.push(DeclaredPath {
                path: format!("{array_ptr}/{index}"),
                value: value.to_owned(),
            });
        }
    }
}

/// Run the filesystem layer for a document: every declared path is checked and
/// each violation becomes a `path.*` error at the declared path's pointer.
#[must_use]
pub fn validate(kind: Kind, value: &Value, root: Option<&Path>) -> Vec<Diagnostic> {
    declared_paths(kind, value)
        .into_iter()
        .filter_map(|declared| {
            check(&declared.value, root).map(|issue| {
                Diagnostic::error(declared.path, issue.code(), issue.message(&declared.value))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn absolute_paths_are_detected_cross_platform() {
        assert_eq!(classify("/etc/passwd"), Verdict::Absolute);
        assert_eq!(classify("\\windows\\system32"), Verdict::Absolute);
        assert_eq!(classify("C:\\Windows\\System32"), Verdict::Absolute);
        assert_eq!(classify("c:/windows"), Verdict::Absolute);
        assert_eq!(classify("skills/example-skill"), Verdict::Safe);
    }

    #[test]
    fn traversal_is_detected_and_internal_dotdot_is_safe() {
        assert_eq!(classify("../outside"), Verdict::Traversal);
        assert_eq!(classify("a/../../b"), Verdict::Traversal);
        assert_eq!(classify("a/../b"), Verdict::Safe);
        assert_eq!(classify("./a/b"), Verdict::Safe);
    }

    #[test]
    fn escapes_root_predicate() {
        assert!(escapes_root(
            Path::new("/project"),
            Path::new("/outside/target")
        ));
        assert!(!escapes_root(
            Path::new("/project"),
            Path::new("/project/skills/x")
        ));
    }

    #[test]
    fn symlink_escape_is_rejected_with_an_injected_resolver() {
        let root = Path::new("/project");
        let resolve = |path: &Path| -> Option<PathBuf> {
            if path == Path::new("/project") {
                Some(PathBuf::from("/project"))
            } else if path == Path::new("/project/link") {
                Some(PathBuf::from("/outside/target"))
            } else {
                None
            }
        };
        assert_eq!(
            check_with_resolver("link", Some(root), &resolve),
            Some(PathIssue::SymlinkEscape)
        );
    }

    #[test]
    fn a_path_that_stays_inside_the_root_is_safe() {
        let root = Path::new("/project");
        let resolve = |path: &Path| -> Option<PathBuf> { Some(path.to_path_buf()) };
        assert_eq!(check_with_resolver("skills/x", Some(root), &resolve), None);
    }

    #[test]
    fn lexical_checks_run_without_a_root() {
        assert_eq!(check("../x", None), Some(PathIssue::Traversal));
        assert_eq!(check("/etc/passwd", None), Some(PathIssue::Absolute));
        assert_eq!(check("skills/x", None), None);
    }

    #[test]
    fn declared_paths_are_collected_from_schema_typed_positions() {
        let manifest = json!({
            "spec": { "components": [
                { "type": "Skill", "path": "skills/a" },
                { "type": "Model", "path": "models/b.json" }
            ] }
        });
        let paths: Vec<String> = declared_paths(Kind::Manifest, &manifest)
            .into_iter()
            .map(|d| d.path)
            .collect();
        assert_eq!(
            paths,
            vec!["/spec/components/0/path", "/spec/components/1/path"]
        );

        let plan = json!({
            "files": [{ "path": "a", "action": "create" }],
            "conflicts": [{ "path": "b", "reason": "r" }],
            "hooks": [{ "path": "c" }],
            "snapshot": { "id": "s", "covers": ["d", "e"] }
        });
        let plan_paths: Vec<String> = declared_paths(Kind::InstallPlan, &plan)
            .into_iter()
            .map(|d| d.path)
            .collect();
        assert_eq!(
            plan_paths,
            vec![
                "/files/0/path",
                "/conflicts/0/path",
                "/hooks/0/path",
                "/snapshot/covers/0",
                "/snapshot/covers/1",
            ]
        );
    }

    #[test]
    fn validate_emits_the_expected_code_at_the_declared_pointer() {
        let manifest = json!({
            "spec": { "components": [ { "type": "Skill", "path": "../escape" } ] }
        });
        let diagnostics = validate(Kind::Manifest, &manifest, None);
        assert_eq!(diagnostics.len(), 1, "got {diagnostics:?}");
        assert_eq!(diagnostics[0].path, "/spec/components/0/path");
        assert_eq!(diagnostics[0].code.as_str(), "path.traversal");
    }
}
