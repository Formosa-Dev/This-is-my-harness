//! Kind detection and `apiVersion` read (design §6, tasks 2.4).
//!
//! Discrimination is structural, on root-schema discriminators:
//!   * **manifest** — `kind` is one of `Harness`, `Component`, `Preset`;
//!   * **model-contract** — any of `executionLocation`, `lifecycle`, `router`;
//!   * **install-plan** — any of `environmentVariableNames`, `risk`, `snapshot`,
//!     `verificationSteps`.
//!
//! Zero matches is `document.unknown_kind` (spec code); more than one match is
//! `document.kind_ambiguous` (a design-only extension, kept distinct so a
//! consumer can tell "unknown" from "contradictory").

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic};

/// The three document kinds recognized at v1alpha1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Manifest,
    ModelContract,
    InstallPlan,
}

impl Kind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Kind::Manifest => "manifest",
            Kind::ModelContract => "model-contract",
            Kind::InstallPlan => "install-plan",
        }
    }

    /// The embedded root schema `$id` for this kind.
    #[must_use]
    pub const fn schema_id(self) -> &'static str {
        match self {
            Kind::Manifest => "https://thisismyharness.dev/schemas/v1alpha1/harness.schema.json",
            Kind::ModelContract => {
                "https://thisismyharness.dev/schemas/v1alpha1/model-contract.schema.json"
            }
            Kind::InstallPlan => {
                "https://thisismyharness.dev/schemas/v1alpha1/install-plan.schema.json"
            }
        }
    }

    /// Parse a `--kind` override name (`manifest | model-contract | install-plan`).
    #[must_use]
    pub fn from_name(name: &str) -> Option<Kind> {
        match name {
            "manifest" => Some(Kind::Manifest),
            "model-contract" => Some(Kind::ModelContract),
            "install-plan" => Some(Kind::InstallPlan),
            _ => None,
        }
    }
}

/// Outcome of kind detection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Detection {
    Known(Kind),
    UnknownKind,
    AmbiguousKind(Vec<Kind>),
}

const MANIFEST_KINDS: &[&str] = &["Harness", "Component", "Preset"];
const MODEL_CONTRACT_DISCRIMINATORS: &[&str] = &["executionLocation", "lifecycle", "router"];
const INSTALL_PLAN_DISCRIMINATORS: &[&str] = &[
    "environmentVariableNames",
    "risk",
    "snapshot",
    "verificationSteps",
];

/// Detect the document kind from root-schema discriminators.
#[must_use]
pub fn detect(value: &Value) -> Detection {
    let Some(object) = value.as_object() else {
        return Detection::UnknownKind;
    };

    let mut candidates = Vec::new();

    let is_manifest = object
        .get("kind")
        .and_then(Value::as_str)
        .is_some_and(|kind| MANIFEST_KINDS.contains(&kind));
    if is_manifest {
        candidates.push(Kind::Manifest);
    }
    if MODEL_CONTRACT_DISCRIMINATORS
        .iter()
        .any(|key| object.contains_key(*key))
    {
        candidates.push(Kind::ModelContract);
    }
    if INSTALL_PLAN_DISCRIMINATORS
        .iter()
        .any(|key| object.contains_key(*key))
    {
        candidates.push(Kind::InstallPlan);
    }

    match candidates.len() {
        0 => Detection::UnknownKind,
        1 => Detection::Known(candidates[0]),
        _ => Detection::AmbiguousKind(candidates),
    }
}

/// Read `apiVersion` when it is a string; `None` otherwise (missing or wrong type).
#[must_use]
pub fn read_api_version(value: &Value) -> Option<String> {
    value
        .as_object()
        .and_then(|object| object.get("apiVersion"))
        .and_then(Value::as_str)
        .map(str::to_owned)
}

/// Detect a kind and read `apiVersion`, or fail with the matching diagnostic.
///
/// # Errors
///
/// `document.unknown_kind` when no kind matches; `document.kind_ambiguous` when
/// more than one does.
pub fn detect_or_diagnose(value: &Value) -> Result<(Kind, Option<String>), Diagnostic> {
    match detect(value) {
        Detection::Known(kind) => Ok((kind, read_api_version(value))),
        Detection::UnknownKind => Err(Diagnostic::error(
            "",
            Code::document_unknown_kind(),
            "document kind could not be determined from its root fields",
        )),
        Detection::AmbiguousKind(kinds) => {
            let names: Vec<&str> = kinds.iter().map(|kind| kind.as_str()).collect();
            Err(Diagnostic::error(
                "",
                Code::document_kind_ambiguous(),
                format!("document matches more than one kind: {}", names.join(", ")),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn manifest_is_detected() {
        let value = json!({
            "apiVersion": "thisismyharness.dev/v1alpha1",
            "kind": "Harness",
            "metadata": { "name": "x", "version": "1.0.0" },
            "spec": {}
        });
        assert_eq!(detect(&value), Detection::Known(Kind::Manifest));
        assert_eq!(
            read_api_version(&value).as_deref(),
            Some("thisismyharness.dev/v1alpha1")
        );
    }

    #[test]
    fn model_contract_is_detected_without_api_version() {
        let value = json!({ "executionLocation": "local", "capabilities": ["x"] });
        assert_eq!(detect(&value), Detection::Known(Kind::ModelContract));
        assert_eq!(read_api_version(&value), None);
    }

    #[test]
    fn install_plan_is_detected() {
        let value = json!({ "environmentVariableNames": [], "risk": {} });
        assert_eq!(detect(&value), Detection::Known(Kind::InstallPlan));
    }

    #[test]
    fn unknown_kind_is_reported() {
        let value = json!({ "name": "name-only-model" });
        assert_eq!(detect(&value), Detection::UnknownKind);
        let diagnostic = detect_or_diagnose(&value).expect_err("must be unknown");
        assert_eq!(diagnostic.code.as_str(), "document.unknown_kind");
    }

    #[test]
    fn ambiguous_kind_is_reported() {
        let value = json!({ "kind": "Harness", "executionLocation": "local" });
        assert_eq!(
            detect(&value),
            Detection::AmbiguousKind(vec![Kind::Manifest, Kind::ModelContract])
        );
        let diagnostic = detect_or_diagnose(&value).expect_err("must be ambiguous");
        assert_eq!(diagnostic.code.as_str(), "document.kind_ambiguous");
    }

    #[test]
    fn non_object_is_unknown_kind() {
        assert_eq!(detect(&json!([1, 2, 3])), Detection::UnknownKind);
        assert_eq!(detect(&json!("scalar")), Detection::UnknownKind);
    }
}
