//! Capability layer — known-capability lookup (F2-07).
//!
//! In the spec's fixed layer order this is the **capability** layer:
//!
//! ```text
//! structural -> version -> semantic -> capability -> filesystem
//! ```
//!
//! Every capability identifier a document declares is resolved against a
//! checked-in known-capability registry (`schemas/capabilities.json`) embedded
//! read-only at build time. An identifier absent from the registry is an
//! explicit [`Code::capability_unknown`] error naming the identifier.
//!
//! A **missing or unreadable registry is a loud failure** ([`CapabilityError`]):
//! the loader never falls back to treating every identifier as known. Which
//! capabilities exist, and the Core-versus-extension distinction, remain OPEN
//! under §58 — this layer only checks membership in the checked-in default set.

use std::collections::BTreeSet;
use std::fmt;
use std::sync::OnceLock;

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic};
use crate::document::Kind;

include!(concat!(env!("OUT_DIR"), "/capability_registry.rs"));

/// A known-capability registry that could not be loaded.
///
/// This is always a loud failure: it is never silently downgraded to an empty
/// (or, worse, an "everything is known") set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityError {
    message: String,
}

impl CapabilityError {
    fn new(message: impl Into<String>) -> Self {
        CapabilityError {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CapabilityError {}

/// The parsed known-capability set.
#[derive(Debug, Clone)]
pub struct CapabilityRegistry {
    known: BTreeSet<String>,
}

impl CapabilityRegistry {
    /// Whether `id` is a known capability.
    #[must_use]
    pub fn contains(&self, id: &str) -> bool {
        self.known.contains(id)
    }

    /// Number of known capability identifiers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.known.len()
    }

    /// Whether the registry declares no capability at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.known.is_empty()
    }
}

/// Parse a known-capability registry body.
///
/// `body` is `None` when the registry is missing. A missing body, invalid JSON,
/// a missing `capabilities` array, or a non-string entry is a loud failure: the
/// loader never falls back to "all known".
///
/// # Errors
///
/// [`CapabilityError`] for any of the conditions above.
pub fn parse_registry(body: Option<&str>) -> Result<CapabilityRegistry, CapabilityError> {
    let Some(body) = body else {
        return Err(CapabilityError::new(
            "known-capability registry is missing; refusing to treat all capabilities as known",
        ));
    };
    let value: Value = serde_json::from_str(body).map_err(|e| {
        CapabilityError::new(format!("known-capability registry is not valid JSON: {e}"))
    })?;
    let entries = value
        .get("capabilities")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            CapabilityError::new("known-capability registry has no `capabilities` array")
        })?;
    let mut known = BTreeSet::new();
    for entry in entries {
        let id = entry.as_str().ok_or_else(|| {
            CapabilityError::new("known-capability registry has a non-string entry")
        })?;
        known.insert(id.to_owned());
    }
    Ok(CapabilityRegistry { known })
}

static REGISTRY: OnceLock<Result<CapabilityRegistry, String>> = OnceLock::new();

/// The embedded known-capability registry, parsed once.
///
/// # Errors
///
/// [`CapabilityError`] if the embedded registry is absent or malformed. Because
/// `build.rs` embeds the checked-in file, this can only fire if the file was
/// malformed at build time or the constant was tampered with — and it fires
/// loudly rather than degrading to "all known".
pub fn registry() -> Result<&'static CapabilityRegistry, CapabilityError> {
    match REGISTRY.get_or_init(|| parse_registry(Some(CAPABILITIES_JSON)).map_err(|e| e.message)) {
        Ok(registry) => Ok(registry),
        Err(message) => Err(CapabilityError::new(message.clone())),
    }
}

/// A declared capability identifier and its JSON Pointer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredCapability {
    pub path: String,
    pub id: String,
}

/// Collect every capability identifier a document declares at the positions the
/// schema set types as a `capabilityId`.
///
/// The loosely-typed `conformance.capabilities[].capability` position is
/// deliberately **not** checked: the schema does not bind it to `capabilityId`
/// (it is a per-capability *result* label, not a capability contract).
#[must_use]
pub fn declared_capabilities(kind: Kind, value: &Value) -> Vec<DeclaredCapability> {
    let mut declared = Vec::new();
    match kind {
        Kind::Manifest => {
            push_capability(
                &mut declared,
                "/spec/requirements/runtime".to_owned(),
                value.pointer("/spec/requirements/runtime"),
            );
            push_object_field_array(
                &mut declared,
                value,
                "/spec/requirements/modelCapabilities",
                "capability",
            );
            push_object_field_array(
                &mut declared,
                value,
                "/spec/requirements/services",
                "capability",
            );
            push_object_field_array(
                &mut declared,
                value,
                "/spec/requirements/backends",
                "capability",
            );
            push_object_field_array(
                &mut declared,
                value,
                "/spec/compatibility/declared",
                "capability",
            );
        }
        Kind::ModelContract => {
            push_string_array(&mut declared, value, "/capabilities");
        }
        Kind::InstallPlan => {
            push_object_field_array(&mut declared, value, "/adaptations", "capability");
            push_object_field_array(
                &mut declared,
                value,
                "/unsupportedCapabilities",
                "capability",
            );
            push_nested_string_array(&mut declared, value, "/mcp", "capabilities");
        }
    }
    declared
}

/// Check a document against the embedded registry.
///
/// # Errors
///
/// [`CapabilityError`] if the embedded registry is absent or malformed.
pub fn validate(kind: Kind, value: &Value) -> Result<Vec<Diagnostic>, CapabilityError> {
    let registry = registry()?;
    Ok(check_with(registry, kind, value))
}

/// Check a document against a registry *source* (for tests and future callers).
///
/// `body` is `None` when the registry is missing. A missing or malformed source
/// is a loud failure, even when the document declares capabilities.
///
/// # Errors
///
/// [`CapabilityError`] if `body` is absent or malformed.
pub fn validate_with_source(
    body: Option<&str>,
    kind: Kind,
    value: &Value,
) -> Result<Vec<Diagnostic>, CapabilityError> {
    let registry = parse_registry(body)?;
    Ok(check_with(&registry, kind, value))
}

/// Look up every declared capability against an already-parsed registry.
#[must_use]
pub fn check_with(registry: &CapabilityRegistry, kind: Kind, value: &Value) -> Vec<Diagnostic> {
    declared_capabilities(kind, value)
        .into_iter()
        .filter(|declared| !registry.contains(&declared.id))
        .map(|declared| {
            Diagnostic::error(
                declared.path,
                Code::capability_unknown(),
                format!(
                    "capability `{}` is not in the known-capability registry",
                    declared.id
                ),
            )
        })
        .collect()
}

fn push_capability(out: &mut Vec<DeclaredCapability>, path: String, value: Option<&Value>) {
    if let Some(id) = value.and_then(Value::as_str) {
        out.push(DeclaredCapability {
            path,
            id: id.to_owned(),
        });
    }
}

fn push_object_field_array(
    out: &mut Vec<DeclaredCapability>,
    root: &Value,
    array_ptr: &str,
    field: &str,
) {
    let Some(items) = root.pointer(array_ptr).and_then(Value::as_array) else {
        return;
    };
    for (index, item) in items.iter().enumerate() {
        let path = format!("{array_ptr}/{index}/{field}");
        push_capability(out, path, item.get(field));
    }
}

fn push_string_array(out: &mut Vec<DeclaredCapability>, root: &Value, array_ptr: &str) {
    let Some(items) = root.pointer(array_ptr).and_then(Value::as_array) else {
        return;
    };
    for (index, item) in items.iter().enumerate() {
        let path = format!("{array_ptr}/{index}");
        push_capability(out, path, Some(item));
    }
}

fn push_nested_string_array(
    out: &mut Vec<DeclaredCapability>,
    root: &Value,
    array_ptr: &str,
    field: &str,
) {
    let Some(items) = root.pointer(array_ptr).and_then(Value::as_array) else {
        return;
    };
    for (index, item) in items.iter().enumerate() {
        let Some(nested) = item.get(field).and_then(Value::as_array) else {
            continue;
        };
        for (nested_index, entry) in nested.iter().enumerate() {
            let path = format!("{array_ptr}/{index}/{field}/{nested_index}");
            push_capability(out, path, Some(entry));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn embedded_registry_has_the_default_known_set() {
        let registry = registry().expect("the embedded registry must load");
        assert!(!registry.is_empty(), "the default set must not be empty");
        for id in [
            "models.text-generation",
            "models.structured-decision",
            "models.escalation",
            "services.vector-store",
            "backends.local-inference",
            "tools.search",
            "runtime.node",
        ] {
            assert!(registry.contains(id), "expected `{id}` to be known");
        }
    }

    #[test]
    fn missing_registry_is_a_loud_failure() {
        let error = parse_registry(None).expect_err("a missing registry must fail loudly");
        assert!(error.message().contains("missing"), "got {error:?}");
    }

    #[test]
    fn missing_registry_fails_even_when_a_capability_is_declared() {
        let value = json!({
            "spec": { "requirements": { "modelCapabilities": [ { "capability": "models.text-generation" } ] } }
        });
        let error = validate_with_source(None, Kind::Manifest, &value)
            .expect_err("no registry must never mean 'all known'");
        assert!(error.message().contains("missing"), "got {error:?}");
    }

    #[test]
    fn malformed_registry_is_a_loud_failure() {
        assert!(parse_registry(Some("not json")).is_err());
        assert!(parse_registry(Some("{}")).is_err());
        assert!(parse_registry(Some("{\"capabilities\": [1]}")).is_err());
    }

    #[test]
    fn unknown_capability_is_an_explicit_error_naming_the_id() {
        let registry = parse_registry(Some("{\"capabilities\": [\"models.known\"]}")).unwrap();
        let value = json!({
            "spec": { "requirements": { "modelCapabilities": [ { "capability": "models.unknown" } ] } }
        });
        let diagnostics = check_with(&registry, Kind::Manifest, &value);
        assert_eq!(diagnostics.len(), 1, "got {diagnostics:?}");
        assert_eq!(diagnostics[0].code.as_str(), "capability.unknown");
        assert_eq!(
            diagnostics[0].path,
            "/spec/requirements/modelCapabilities/0/capability"
        );
        assert!(
            diagnostics[0].message.contains("models.unknown"),
            "the diagnostic must name the identifier: {}",
            diagnostics[0].message
        );
        assert_eq!(
            crate::Report::from_diagnostics(diagnostics).status,
            crate::Status::Invalid,
            "an unknown capability is evaluated & invalid (exit 1), not exit 2"
        );
    }

    #[test]
    fn known_capabilities_produce_no_diagnostics() {
        let registry = parse_registry(Some(
            "{\"capabilities\": [\"models.known\", \"runtime.node\"]}",
        ))
        .unwrap();
        let value = json!({
            "spec": {
                "requirements": {
                    "runtime": "runtime.node",
                    "modelCapabilities": [ { "capability": "models.known" } ]
                }
            }
        });
        assert!(check_with(&registry, Kind::Manifest, &value).is_empty());
    }

    #[test]
    fn all_typed_positions_are_collected() {
        let manifest = json!({
            "spec": {
                "requirements": {
                    "runtime": "runtime.node",
                    "modelCapabilities": [ { "capability": "a.one" } ],
                    "services": [ { "capability": "b.two" } ],
                    "backends": [ { "capability": "c.three" } ]
                },
                "compatibility": { "declared": [ { "capability": "d.four" } ] }
            }
        });
        let paths: Vec<String> = declared_capabilities(Kind::Manifest, &manifest)
            .into_iter()
            .map(|d| d.path)
            .collect();
        assert_eq!(
            paths,
            vec![
                "/spec/requirements/runtime",
                "/spec/requirements/modelCapabilities/0/capability",
                "/spec/requirements/services/0/capability",
                "/spec/requirements/backends/0/capability",
                "/spec/compatibility/declared/0/capability",
            ]
        );

        let model = json!({ "capabilities": ["a.one", "b.two"] });
        let plan = json!({
            "adaptations": [ { "capability": "a.one" } ],
            "unsupportedCapabilities": [ { "capability": "b.two" } ],
            "mcp": [ { "capabilities": ["c.three"] } ]
        });
        assert_eq!(
            declared_capabilities(Kind::ModelContract, &model)
                .into_iter()
                .map(|d| d.path)
                .collect::<Vec<_>>(),
            vec!["/capabilities/0", "/capabilities/1"]
        );
        assert_eq!(
            declared_capabilities(Kind::InstallPlan, &plan)
                .into_iter()
                .map(|d| d.path)
                .collect::<Vec<_>>(),
            vec![
                "/adaptations/0/capability",
                "/unsupportedCapabilities/0/capability",
                "/mcp/0/capabilities/0",
            ]
        );
    }
}
