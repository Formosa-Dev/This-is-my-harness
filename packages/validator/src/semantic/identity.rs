//! L2 identity and naming rules (task 3.2).
//!
//! Enforces, from a single document, the parts of `spec/core/identity.md` and
//! `spec/core/dependencies.md` §1 that JSON Schema cannot:
//!
//! * **canonical-ref grammar** — references are classified as canonical
//!   (`https://<host>/h/<owner>/<slug>[@<version>]`), scoped
//!   (`@owner/slug[@<version>]`), short (`owner/slug`) or unclassifiable. The
//!   *shape* of a malformed reference is rejected by L1 (`schema.pattern`);
//!   this layer classifies so the F1 rules below can be applied.
//! * **short reference forbidden** — a short reference carries no host and
//!   MUST NOT be used inside a published document (`identity.md` §2.3;
//!   `dependencies.md` §1.4). → `semantic.identity_ref_short_forbidden`.
//! * **host consistency** — the schema `$id` host is a *schema-identity
//!   namespace*, explicitly **not** the canonical harness host
//!   (`schemas/README.md` "Namespace, not harness host"; `identity.md` §8), and
//!   the same canonical identity MUST NOT be resolved to two different hosts in
//!   one document. → `semantic.identity_host_mismatch`.
//!
//! Cross-document resolution (digest, network, scoped-host comparison against a
//! resolved manifest host) is deferred to the resolver and reported as
//! `semantic.not_evaluated` by the parent module (F4).

use std::collections::BTreeMap;

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic};
use crate::document::Kind;

use super::child;

/// The host used by the schema `$id` namespace. It is a schema-identity
/// namespace, **not** the canonical harness-resolution host, which is a
/// variable left OPEN by §58 (`schemas/README.md`; `spec/core/identity.md` §8).
pub(crate) const SCHEMA_NAMESPACE_HOST: &str = "thisismyharness.dev";

/// The recognized reference forms (`spec/core/dependencies.md` §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Reference<'a> {
    /// `https://<host>/h/<owner>/<slug>` (a trailing `@<version>` is ignored).
    Canonical {
        host: &'a str,
        owner: &'a str,
        slug: &'a str,
    },
    /// `@<owner>/<slug>` (a trailing `@<version>` is ignored).
    Scoped { owner: &'a str, slug: &'a str },
    /// `<owner>/<slug>` — forbidden inside a published document.
    Short { owner: &'a str, slug: &'a str },
    /// Not one of the recognized forms (the raw shape is L1's concern).
    Other,
}

/// Classify a reference string against the F1 grammar.
#[must_use]
pub(crate) fn classify_reference(text: &str) -> Reference<'_> {
    let text = text.trim();
    if text.is_empty() {
        return Reference::Other;
    }

    if let Some(rest) = text.strip_prefix("https://") {
        // https://<host>/h/<owner>/<slug>[@<version>]
        let Some((host, path)) = rest.split_once("/h/") else {
            return Reference::Other;
        };
        let mut segments = path.split('/');
        let (Some(owner), Some(slug_and_version), None) =
            (segments.next(), segments.next(), segments.next())
        else {
            return Reference::Other;
        };
        let slug = slug_and_version
            .split('@')
            .next()
            .unwrap_or(slug_and_version);
        if host.is_empty() || owner.is_empty() || slug.is_empty() {
            return Reference::Other;
        }
        return Reference::Canonical { host, owner, slug };
    }

    if let Some(rest) = text.strip_prefix('@') {
        // @<owner>/<slug>[@<version>]
        let Some((owner, slug_and_version)) = rest.split_once('/') else {
            return Reference::Other;
        };
        if slug_and_version.contains('/') {
            return Reference::Other;
        }
        let slug = slug_and_version
            .split('@')
            .next()
            .unwrap_or(slug_and_version);
        if owner.is_empty() || slug.is_empty() {
            return Reference::Other;
        }
        return Reference::Scoped { owner, slug };
    }

    if text.contains("://") || text.contains(char::is_whitespace) {
        return Reference::Other;
    }
    // A short reference is exactly `owner/slug`.
    let mut parts = text.split('/');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(owner), Some(slug), None) if !owner.is_empty() && !slug.is_empty() => {
            Reference::Short { owner, slug }
        }
        _ => Reference::Other,
    }
}

/// The reference-bearing positions this layer knows about, as `(pointer, text)`
/// pairs. Positions whose contents are not references (for example a
/// distribution `reference`) are deliberately excluded.
fn references(kind: Kind, value: &Value) -> Vec<(String, String)> {
    let mut references = Vec::new();
    match kind {
        Kind::Manifest => {
            if let Some(extends) = value.pointer("/spec/extends").and_then(Value::as_array) {
                for (index, entry) in extends.iter().enumerate() {
                    if let Some(text) = entry.get("reference").and_then(Value::as_str) {
                        references.push((
                            child(&child("/spec/extends", &index.to_string()), "reference"),
                            text.to_owned(),
                        ));
                    }
                }
            }
            if let Some(text) = value
                .pointer("/spec/conformance/artifact")
                .and_then(Value::as_str)
            {
                references.push(("/spec/conformance/artifact".to_owned(), text.to_owned()));
            }
        }
        Kind::InstallPlan => {
            if let Some(text) = value.pointer("/harness/identifier").and_then(Value::as_str) {
                references.push(("/harness/identifier".to_owned(), text.to_owned()));
            }
        }
        Kind::ModelContract => {}
    }
    references
}

pub(crate) fn check(kind: Kind, value: &Value, out: &mut Vec<Diagnostic>) {
    let references = references(kind, value);

    // A short reference (`owner/slug`) carries no host and MUST NOT appear in a
    // published document (`identity.md` §2.3; `dependencies.md` §1.4).
    for (path, text) in &references {
        if let Reference::Short { .. } = classify_reference(text) {
            out.push(Diagnostic::error(
                path.clone(),
                Code::semantic_identity_ref_short_forbidden(),
                "a short reference (owner/slug) MUST NOT be used inside a published document: it carries no host; use a scoped reference (@owner/slug) or a canonical identifier",
            ));
        }
    }

    // Host consistency. The schema `$id` namespace host is not the harness host,
    // and one canonical identity cannot resolve to two hosts in one document.
    for (path, text) in &references {
        if let Reference::Canonical { host, .. } = classify_reference(text) {
            if host.eq_ignore_ascii_case(SCHEMA_NAMESPACE_HOST) {
                out.push(Diagnostic::error(
                    path.clone(),
                    Code::semantic_identity_host_mismatch(),
                    "a canonical identifier MUST NOT use the schema-identity namespace host as the harness host",
                ));
            }
        }
    }

    // Same canonical `(owner, slug)` resolved against two different hosts.
    let mut canonical: Vec<(String, &str, &str, &str)> = Vec::new();
    for (path, text) in &references {
        if let Reference::Canonical { host, owner, slug } = classify_reference(text) {
            canonical.push((path.clone(), host, owner, slug));
        }
    }
    canonical.sort_by(|left, right| left.0.cmp(&right.0));
    let mut hosts: BTreeMap<(&str, &str), &str> = BTreeMap::new();
    for (path, host, owner, slug) in &canonical {
        match hosts.get(&(*owner, *slug)) {
            Some(previous) if !previous.eq_ignore_ascii_case(host) => out.push(Diagnostic::error(
                path.clone(),
                Code::semantic_identity_host_mismatch(),
                "the same canonical identity (owner/slug) is declared against two different hosts in one document",
            )),
            Some(_) => {}
            None => {
                hosts.insert((owner, slug), host);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_each_reference_form() {
        assert_eq!(
            classify_reference("@example-owner/base"),
            Reference::Scoped {
                owner: "example-owner",
                slug: "base"
            }
        );
        assert_eq!(
            classify_reference("@example-owner/base@1.2.0"),
            Reference::Scoped {
                owner: "example-owner",
                slug: "base"
            }
        );
        assert_eq!(
            classify_reference("https://example.org/h/example-owner/base@1.0.0"),
            Reference::Canonical {
                host: "example.org",
                owner: "example-owner",
                slug: "base"
            }
        );
        assert_eq!(
            classify_reference("example-owner/base"),
            Reference::Short {
                owner: "example-owner",
                slug: "base"
            }
        );
        assert_eq!(classify_reference("just-a-name"), Reference::Other);
        assert_eq!(classify_reference("a/b/c"), Reference::Other);
    }

    #[test]
    fn short_reference_is_reported() {
        let value = serde_json::json!({ "spec": { "extends": [{ "reference": "acme/base" }] } });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out.iter().any(|d| d.path == "/spec/extends/0/reference"
            && d.code.as_str() == "semantic.identity_ref_short_forbidden"));
    }

    #[test]
    fn schema_namespace_host_is_a_mismatch() {
        let value = serde_json::json!({
            "spec": { "extends": [{ "reference": "https://thisismyharness.dev/h/acme/base" }] }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out.iter().any(|d| d.path == "/spec/extends/0/reference"
            && d.code.as_str() == "semantic.identity_host_mismatch"));
    }

    #[test]
    fn same_identity_on_two_hosts_is_a_mismatch() {
        let value = serde_json::json!({
            "spec": { "extends": [
                { "reference": "https://one.example/h/acme/base" },
                { "reference": "https://two.example/h/acme/base" }
            ] }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert_eq!(
            out.iter()
                .filter(|d| d.code.as_str() == "semantic.identity_host_mismatch")
                .count(),
            1
        );
        assert!(out.iter().any(|d| d.path == "/spec/extends/1/reference"));
    }

    #[test]
    fn scoped_references_alone_are_clean() {
        let value = serde_json::json!({
            "spec": { "extends": [{ "reference": "@acme/base" }] }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out.is_empty(), "unexpected: {out:?}");
    }
}
