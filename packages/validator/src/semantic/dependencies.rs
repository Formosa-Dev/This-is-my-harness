//! L2 dependency and composition rules (task 3.3).
//!
//! Document-local parts of `spec/core/dependencies.md` and
//! `spec/package/component-types.md` §1:
//!
//! * **ordered `extends`** — the array order is significant for precedence
//!   (`dependencies.md` §2.2, §4.2); the same reference declared twice makes the
//!   declared precedence contradictory. → `semantic.dependency_order`.
//! * **kind composition** — a `Component` MUST NOT declare a runtime
//!   requirement (`requirements.md` §2.1.2); this is the document-local proxy
//!   for the kind-composition rules (`component-types.md` §1.2, §11.4,
//!   `dependencies.md` §2.4), whose verdict on the *referenced* artifact needs
//!   the resolver. → `semantic.kind_composition`.
//! * **self / known-set cycle** — a dependency path that returns to the
//!   declaring artifact (`dependencies.md` §5, "Cycle"). → `semantic.dependency_cycle`.
//!
//! Cross-document cycle detection and conflict resolution need the resolver and
//! are reported as `semantic.not_evaluated` (F4) by the parent module.

use std::collections::BTreeSet;

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic};
use crate::document::Kind;

use super::child;
use super::identity::{classify_reference, Reference};

pub(crate) fn check(kind: Kind, value: &Value, out: &mut Vec<Diagnostic>) {
    if kind != Kind::Manifest {
        return;
    }

    kind_composition(value, out);

    let Some(extends) = value.pointer("/spec/extends").and_then(Value::as_array) else {
        return;
    };

    // Ordered `extends`: a repeated reference makes precedence contradictory.
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for (index, entry) in extends.iter().enumerate() {
        let Some(text) = entry.get("reference").and_then(Value::as_str) else {
            continue;
        };
        if !seen.insert(text) {
            out.push(Diagnostic::error(
                child(&child("/spec/extends", &index.to_string()), "reference"),
                Code::semantic_dependency_order(),
                "the same reference appears more than once in the ordered `extends` list, so the declared precedence is contradictory",
            ));
        }
    }

    // Self / known-set cycle: a dependency path returning to the declaring
    // artifact. Requires a declared `metadata.owner` to establish self-identity.
    if let Some((owner, slug)) = self_identity(value) {
        for (index, entry) in extends.iter().enumerate() {
            let Some(text) = entry.get("reference").and_then(Value::as_str) else {
                continue;
            };
            if is_self_reference(text, owner, slug) {
                out.push(Diagnostic::error(
                    child(&child("/spec/extends", &index.to_string()), "reference"),
                    Code::semantic_dependency_cycle(),
                    "a dependency path returns to the declaring artifact",
                ));
            }
        }
    }
}

/// A `Component` MUST NOT declare a runtime requirement: a runtime belongs to a
/// Full Harness (`spec/core/requirements.md` §2.1.2).
fn kind_composition(value: &Value, out: &mut Vec<Diagnostic>) {
    if value.pointer("/kind").and_then(Value::as_str) != Some("Component") {
        return;
    }
    if value.pointer("/spec/requirements/runtime").is_some() {
        out.push(Diagnostic::error(
            "/spec/requirements/runtime",
            Code::semantic_kind_composition(),
            "a Component MUST NOT declare a runtime requirement; a runtime is owned by a Full Harness",
        ));
    }
}

/// The declaring artifact's `(owner, slug)`, when both are declared. A missing
/// `owner` leaves self-identity ambiguous, so the check is skipped (the resolver
/// owns the full graph).
fn self_identity(value: &Value) -> Option<(&str, &str)> {
    let slug = value.pointer("/metadata/name").and_then(Value::as_str)?;
    let owner = value.pointer("/metadata/owner").and_then(Value::as_str)?;
    Some((owner, slug))
}

fn is_self_reference(text: &str, owner: &str, slug: &str) -> bool {
    let (reference_owner, reference_slug) = match classify_reference(text) {
        Reference::Canonical { owner, slug, .. }
        | Reference::Scoped { owner, slug }
        | Reference::Short { owner, slug } => (owner, slug),
        Reference::Other => return false,
    };
    reference_owner.eq_ignore_ascii_case(owner) && reference_slug == slug
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(spec: serde_json::Value) -> Value {
        serde_json::json!({
            "kind": "Harness",
            "metadata": { "name": "self-harness", "owner": "acme" },
            "spec": spec
        })
    }

    #[test]
    fn self_reference_is_a_cycle() {
        let value = manifest(serde_json::json!({
            "extends": [{ "reference": "@acme/self-harness" }]
        }));
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out
            .iter()
            .any(|d| d.code.as_str() == "semantic.dependency_cycle"));
    }

    #[test]
    fn duplicate_extends_entry_is_an_order_violation() {
        let value = manifest(serde_json::json!({
            "extends": [
                { "reference": "@acme/base" },
                { "reference": "@acme/base" }
            ]
        }));
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out.iter().any(|d| d.path == "/spec/extends/1/reference"
            && d.code.as_str() == "semantic.dependency_order"));
    }

    #[test]
    fn component_runtime_requirement_is_a_kind_composition_violation() {
        let value = serde_json::json!({
            "kind": "Component",
            "metadata": { "name": "base", "owner": "acme" },
            "spec": { "requirements": { "runtime": "runtime.node" } }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out.iter().any(|d| d.path == "/spec/requirements/runtime"
            && d.code.as_str() == "semantic.kind_composition"));
    }

    #[test]
    fn distinct_ordered_dependencies_are_clean() {
        let value = manifest(serde_json::json!({
            "extends": [
                { "reference": "@acme/first" },
                { "reference": "@acme/second" }
            ]
        }));
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out.is_empty(), "unexpected: {out:?}");
    }
}
