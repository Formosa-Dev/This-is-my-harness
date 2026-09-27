//! L2 component and requirement-coherence rules (task 3.4).
//!
//! Document-local parts of `spec/package/component-types.md`,
//! `spec/core/requirements.md` §2.5 and `spec/core/permissions.md` §4:
//!
//! * **environment-variable names, never values** — an entry that looks like a
//!   `NAME=value` pair is a declared secret value, forbidden by
//!   `permissions.md` §4.2. The value is **never echoed** in the diagnostic.
//!   → `semantic.env_value_like`.
//! * **model license shape** — F1 fixes no SPDX grammar (`component-types.md`
//!   §4.8), so this is a documented heuristic and the position stays OPEN
//!   (§58 / Q7). → `semantic.license_shape`.
//! * **model license coherence** — a model license that contradicts the package
//!   `metadata.license` (`distribution.md` §7). When only one of the two
//!   instances is present the check is deferred, never guessed.
//!   → `semantic.license_conflict` / `semantic.not_evaluated`.
//! * **requirements ↔ components coherence** — a required service or model
//!   capability with no local declaration is deferred to the resolver
//!   (`requirements.md` §2.5.2). → `semantic.not_evaluated`.

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic};
use crate::document::Kind;

use super::{child, not_evaluated};

pub(crate) fn check(kind: Kind, value: &Value, out: &mut Vec<Diagnostic>) {
    match kind {
        Kind::InstallPlan => environment_variable_names(value, out),
        Kind::ModelContract => model_contract_license(value, out),
        Kind::Manifest => manifest_components(value, out),
    }
}

/// `environmentVariableNames` carries NAMES only (`permissions.md` §4.2).
fn environment_variable_names(value: &Value, out: &mut Vec<Diagnostic>) {
    let Some(names) = value
        .get("environmentVariableNames")
        .and_then(Value::as_array)
    else {
        return;
    };
    for (index, entry) in names.iter().enumerate() {
        let Some(name) = entry.as_str() else {
            continue;
        };
        if is_value_like(name) {
            out.push(Diagnostic::error(
                child("/environmentVariableNames", &index.to_string()),
                Code::semantic_env_value_like(),
                "an environment-variable entry looks like a name=value pair; only NAMES may be declared and the value is never echoed",
            ));
        }
    }
}

/// A value-like entry carries an assignment (`=`), embedded whitespace or a URL
/// scheme separator — the shapes of a secret value rather than a name
/// (`permissions.md` §4.2). The content is deliberately never echoed.
fn is_value_like(entry: &str) -> bool {
    entry.contains('=') || entry.contains(char::is_whitespace) || entry.contains("://")
}

/// The model-contract `license` is identifier-shaped under a documented
/// heuristic (`component-types.md` §4.8 fixes no grammar; §58 / Q7 stay OPEN).
fn model_contract_license(value: &Value, out: &mut Vec<Diagnostic>) {
    let Some(license) = value.get("license").and_then(Value::as_str) else {
        return;
    };
    if !is_identifier_shaped(license) {
        out.push(Diagnostic::error(
            "/license",
            Code::semantic_license_shape(),
            "a model license SHOULD be an identifier-shaped string (for example an SPDX identifier); F1 fixes no grammar, so this is a documented heuristic and the position stays OPEN (§58)",
        ));
    }
}

fn manifest_components(value: &Value, out: &mut Vec<Diagnostic>) {
    let components = value.pointer("/spec/components").and_then(Value::as_array);
    let package_license = value.pointer("/metadata/license").and_then(Value::as_str);

    if let Some(components) = components {
        for (index, component) in components.iter().enumerate() {
            component_license(index, component, package_license, out);
        }
    }
    requirements_coherence(value, components, out);
}

/// A declared model license must be identifier-shaped and must not contradict
/// the package license declared once in `metadata.license` (`distribution.md` §7).
fn component_license(
    index: usize,
    component: &Value,
    package_license: Option<&str>,
    out: &mut Vec<Diagnostic>,
) {
    if component.get("type").and_then(Value::as_str) != Some("Model") {
        return;
    }
    let Some(model_license) = component
        .get("payload")
        .and_then(|payload| payload.get("license"))
        .and_then(Value::as_str)
    else {
        // The model license is simply not declared in this document: there is no
        // contradiction to evaluate and nothing to defer.
        return;
    };
    let path = child(
        &child(&child("/spec/components", &index.to_string()), "payload"),
        "license",
    );

    if !is_identifier_shaped(model_license) {
        out.push(Diagnostic::error(
            path.clone(),
            Code::semantic_license_shape(),
            "a model license SHOULD be an identifier-shaped string (for example an SPDX identifier); F1 fixes no grammar, so this is a documented heuristic and the position stays OPEN (§58)",
        ));
    }

    match package_license {
        Some(package) if package == model_license => {}
        Some(_) => out.push(Diagnostic::error(
            path,
            Code::semantic_license_conflict(),
            "the model license contradicts the package `metadata.license`; a package license is declared once and MUST NOT be contradicted (§58 / Q7)",
        )),
        None => not_evaluated(
            path,
            "F2-10",
            "model-license versus package-license conflict (the package license is not declared in this document)",
            out,
        ),
    }
}

/// A required requirement with no local declaration cannot be satisfied
/// document-locally; whether an already-available equivalent exists is a
/// resolver concern (`requirements.md` §2.5.2), reported as deferred.
fn requirements_coherence(
    value: &Value,
    components: Option<&Vec<Value>>,
    out: &mut Vec<Diagnostic>,
) {
    let has_type = |wanted: &[&str]| {
        components.is_some_and(|components| {
            components.iter().any(|component| {
                component
                    .get("type")
                    .and_then(Value::as_str)
                    .is_some_and(|kind| wanted.contains(&kind))
            })
        })
    };

    if !has_type(&["Service"]) {
        if let Some(services) = value
            .pointer("/spec/requirements/services")
            .and_then(Value::as_array)
        {
            for (index, service) in services.iter().enumerate() {
                if service.get("required").and_then(Value::as_bool) == Some(true) {
                    not_evaluated(
                        child("/spec/requirements/services", &index.to_string()),
                        "F4",
                        "resolvability of a required service to a Service component or an already-available equivalent",
                        out,
                    );
                }
            }
        }
    }

    if !has_type(&["Model", "Router"]) {
        if let Some(capabilities) = value
            .pointer("/spec/requirements/modelCapabilities")
            .and_then(Value::as_array)
        {
            for (index, _capability) in capabilities.iter().enumerate() {
                not_evaluated(
                    child("/spec/requirements/modelCapabilities", &index.to_string()),
                    "F4",
                    "whether any declared model component satisfies a required model capability",
                    out,
                );
            }
        }
    }
}

/// Identifier-shaped under a documented heuristic: ASCII alphanumerics plus
/// `.`, `+` and `-`, starting with an alphanumeric. This admits SPDX ids such as
/// `Apache-2.0` and `MIT` without inventing a normative SPDX grammar.
fn is_identifier_shaped(license: &str) -> bool {
    let mut chars = license.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphanumeric() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '+' | '-'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_value_like_entry_is_reported_without_echoing_the_value() {
        let value = serde_json::json!({
            "environmentVariableNames": ["EXAMPLE_API_KEY=SUPER_SECRET_SENTINEL"]
        });
        let mut out = Vec::new();
        check(Kind::InstallPlan, &value, &mut out);
        assert!(out.iter().any(|d| d.path == "/environmentVariableNames/0"
            && d.code.as_str() == "semantic.env_value_like"));
        assert!(
            out.iter()
                .all(|d| !d.message.contains("SUPER_SECRET_SENTINEL")),
            "the value must never be echoed: {out:?}"
        );
    }

    #[test]
    fn pure_uppercase_names_are_clean() {
        let value = serde_json::json!({ "environmentVariableNames": ["EXAMPLE_API_KEY"] });
        let mut out = Vec::new();
        check(Kind::InstallPlan, &value, &mut out);
        assert!(out.is_empty(), "unexpected: {out:?}");
    }

    #[test]
    fn model_contract_license_shape_is_checked() {
        let value =
            serde_json::json!({ "license": "licensed under the Apache License Version 2.0" });
        let mut out = Vec::new();
        check(Kind::ModelContract, &value, &mut out);
        assert!(out
            .iter()
            .any(|d| d.path == "/license" && d.code.as_str() == "semantic.license_shape"));
    }

    #[test]
    fn identifier_shaped_model_license_is_clean() {
        let value = serde_json::json!({ "license": "Apache-2.0" });
        let mut out = Vec::new();
        check(Kind::ModelContract, &value, &mut out);
        assert!(out.is_empty(), "unexpected: {out:?}");
    }

    #[test]
    fn model_license_conflicting_with_package_license_is_reported() {
        let value = serde_json::json!({
            "metadata": { "license": "Apache-2.0" },
            "spec": { "components": [
                { "type": "Model", "payload": { "license": "MIT" } }
            ] }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out
            .iter()
            .any(|d| d.path == "/spec/components/0/payload/license"
                && d.code.as_str() == "semantic.license_conflict"));
    }

    #[test]
    fn required_service_without_component_is_deferred() {
        let value = serde_json::json!({
            "spec": { "requirements": { "services": [
                { "capability": "services.vector-store", "required": true }
            ] } }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out.iter().any(|d| d.path == "/spec/requirements/services/0"
            && d.code.as_str() == "semantic.not_evaluated"));
    }
}
