//! L2 permission, risk and autonomy rules (task 3.5).
//!
//! Document-local parts of `spec/core/permissions.md` and
//! `spec/core/risk-classes.md`:
//!
//! * **permission coverage** — every capability above Passive MUST be declared
//!   (`permissions.md` §2.1; `manifest/README.md` §4.3). When a document
//!   declares tooling or executable content and `spec.permissions` carries no
//!   declaration above Passive. → `semantic.permission_coverage`.
//! * **effective risk** — the effective class is the **maximum** over the parts
//!   (`risk-classes.md` §3.2; `install-protocol/README.md` §7.2); a declared
//!   effective class below that maximum is an under-classification.
//!   → `semantic.effective_risk`.
//! * **autonomy floor** — the autonomy level MUST be at least the floor implied
//!   by the risk class (`permissions.md` §6.1; `risk-classes.md` §4).
//!   → `semantic.autonomy_below_floor`.

use serde_json::Value;

use crate::diagnostics::{Code, Diagnostic};
use crate::document::Kind;

use super::child;

/// Minimum autonomy level implied by a risk class. A (Passive) may run at Safe
/// Apply (1); B (Tooling) requires approval reflecting tooling access (2); C
/// (Executable) and D (Privileged) require Elevated (3) with no absolute bypass
/// (`risk-classes.md` §4; `glossary.md` "Autonomy level";
/// `install-protocol/README.md` §7).
#[must_use]
pub(crate) fn autonomy_floor(class: &str) -> u8 {
    match class {
        "A" => 1,
        "B" => 2,
        // "C" and "D"; unknown classes are treated conservatively as Elevated.
        _ => 3,
    }
}

fn class_rank(class: &str) -> u8 {
    match class {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "D" => 3,
        // Unknown classes are treated conservatively as the highest class.
        _ => 3,
    }
}

pub(crate) fn check(kind: Kind, value: &Value, out: &mut Vec<Diagnostic>) {
    match kind {
        Kind::Manifest => manifest(value, out),
        Kind::InstallPlan => install_plan(value, out),
        Kind::ModelContract => {}
    }
}

fn manifest(value: &Value, out: &mut Vec<Diagnostic>) {
    // Permission coverage: tooling/executable content must be declared.
    if requires_non_passive(value) {
        let covered = value
            .pointer("/spec/permissions")
            .and_then(Value::as_array)
            .is_some_and(|permissions| {
                permissions.iter().any(|permission| {
                    matches!(
                        permission.get("riskClass").and_then(Value::as_str),
                        Some("B" | "C" | "D")
                    )
                })
            });
        if !covered {
            out.push(Diagnostic::error(
                "/spec/permissions",
                Code::semantic_permission_coverage(),
                "the document declares tooling or executable content, so every capability above Passive MUST be declared in `spec.permissions`",
            ));
        }
    }

    // Autonomy floor on each declared permission that records a level.
    if let Some(permissions) = value.pointer("/spec/permissions").and_then(Value::as_array) {
        for (index, permission) in permissions.iter().enumerate() {
            let Some(class) = permission.get("riskClass").and_then(Value::as_str) else {
                continue;
            };
            let Some(level) = permission.get("autonomyLevel").and_then(Value::as_u64) else {
                continue;
            };
            let floor = u64::from(autonomy_floor(class));
            if level < floor {
                out.push(Diagnostic::error(
                    child(
                        &child("/spec/permissions", &index.to_string()),
                        "autonomyLevel",
                    ),
                    Code::semantic_autonomy_below_floor(),
                    format!(
                        "autonomyLevel {level} is below the floor {floor} implied by risk class {class}"
                    ),
                ));
            }
        }
    }
}

/// Whether the declared components imply behavior above Passive
/// (`component-types.md` §2 typical risk classes; `risk-classes.md` §2).
fn requires_non_passive(value: &Value) -> bool {
    let Some(components) = value.pointer("/spec/components").and_then(Value::as_array) else {
        return false;
    };
    components.iter().any(
        |component| match component.get("type").and_then(Value::as_str) {
            Some("Service" | "MCP" | "Agent" | "Workflow" | "Router") => true,
            Some("Model") => {
                let payload = component.get("payload");
                payload.is_some_and(|payload| {
                    payload.get("lifecycle").is_some()
                        || payload.get("executionLocation").and_then(Value::as_str)
                            == Some("remote")
                })
            }
            _ => false,
        },
    )
}

fn install_plan(value: &Value, out: &mut Vec<Diagnostic>) {
    let Some(risk) = value.get("risk") else {
        return;
    };

    // Effective class = the maximum over the plan's parts.
    let mut computed = "A";
    for section in ["hooks", "mcp", "services"] {
        if let Some(parts) = value.get(section).and_then(Value::as_array) {
            for part in parts {
                if let Some(class) = part.get("riskClass").and_then(Value::as_str) {
                    if class_rank(class) > class_rank(computed) {
                        computed = class;
                    }
                }
            }
        }
    }
    // User scope is Class D (`permissions.md` §1; `risk-classes.md` §2.4) and any
    // hook executes code, a Class C floor (`risk-classes.md` §2.3).
    if value.get("scope").and_then(Value::as_str) == Some("user") {
        computed = "D";
    }
    let has_hooks = value
        .get("hooks")
        .and_then(Value::as_array)
        .is_some_and(|hooks| !hooks.is_empty());
    if has_hooks && class_rank(computed) < class_rank("C") {
        computed = "C";
    }

    let declared = risk.get("effectiveClass").and_then(Value::as_str);
    if let Some(declared) = declared {
        if class_rank(declared) < class_rank(computed) {
            out.push(Diagnostic::error(
                "/risk/effectiveClass",
                Code::semantic_effective_risk(),
                format!(
                    "the declared effectiveClass {declared} is below the maximum implied by the plan's parts ({computed})"
                ),
            ));
        }
    }

    if let (Some(declared), Some(level)) =
        (declared, risk.get("autonomyLevel").and_then(Value::as_u64))
    {
        let floor = u64::from(autonomy_floor(declared));
        if level < floor {
            out.push(Diagnostic::error(
                "/risk/autonomyLevel",
                Code::semantic_autonomy_below_floor(),
                format!(
                    "autonomyLevel {level} is below the floor {floor} implied by the declared effective risk class {declared}"
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_component_without_permissions_violates_coverage() {
        let value = serde_json::json!({
            "kind": "Harness",
            "metadata": { "name": "svc", "owner": "acme" },
            "spec": { "components": [{ "type": "Service", "name": "svc" }] }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out
            .iter()
            .any(|d| d.path == "/spec/permissions"
                && d.code.as_str() == "semantic.permission_coverage"));
    }

    #[test]
    fn declared_permission_above_passive_satisfies_coverage() {
        let value = serde_json::json!({
            "kind": "Harness",
            "metadata": { "name": "svc", "owner": "acme" },
            "spec": {
                "components": [{ "type": "Service", "name": "svc" }],
                "permissions": [{ "permission": "services.lifecycle", "scope": "project", "riskClass": "B" }]
            }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out.is_empty(), "unexpected: {out:?}");
    }

    #[test]
    fn permission_autonomy_below_floor_is_reported() {
        let value = serde_json::json!({
            "kind": "Harness",
            "metadata": { "name": "svc", "owner": "acme" },
            "spec": { "permissions": [
                { "permission": "execution.hooks", "scope": "project", "riskClass": "C", "autonomyLevel": 1 }
            ] }
        });
        let mut out = Vec::new();
        check(Kind::Manifest, &value, &mut out);
        assert!(out
            .iter()
            .any(|d| d.path == "/spec/permissions/0/autonomyLevel"
                && d.code.as_str() == "semantic.autonomy_below_floor"));
    }

    #[test]
    fn plan_effective_class_below_parts_is_reported() {
        let value = serde_json::json!({
            "scope": "project",
            "hooks": [{ "path": "scripts/x.sh", "riskClass": "C" }],
            "mcp": [],
            "services": [],
            "risk": { "effectiveClass": "B", "autonomyLevel": 2 }
        });
        let mut out = Vec::new();
        check(Kind::InstallPlan, &value, &mut out);
        assert!(out
            .iter()
            .any(|d| d.path == "/risk/effectiveClass"
                && d.code.as_str() == "semantic.effective_risk"));
        assert!(
            !out.iter()
                .any(|d| d.code.as_str() == "semantic.autonomy_below_floor"),
            "B floor is 2 and the level is 2: no autonomy error expected: {out:?}"
        );
    }

    #[test]
    fn plan_autonomy_below_floor_is_reported() {
        let value = serde_json::json!({
            "scope": "project",
            "hooks": [{ "path": "scripts/x.sh", "riskClass": "C" }],
            "mcp": [],
            "services": [],
            "risk": { "effectiveClass": "C", "autonomyLevel": 1 }
        });
        let mut out = Vec::new();
        check(Kind::InstallPlan, &value, &mut out);
        assert!(out.iter().any(|d| d.path == "/risk/autonomyLevel"
            && d.code.as_str() == "semantic.autonomy_below_floor"));
        assert!(
            !out.iter()
                .any(|d| d.code.as_str() == "semantic.effective_risk"),
            "declared C matches the computed C: no effective-risk error expected: {out:?}"
        );
    }

    #[test]
    fn user_scope_floors_the_effective_class_at_d() {
        let value = serde_json::json!({
            "scope": "user",
            "hooks": [],
            "mcp": [],
            "services": [],
            "risk": { "effectiveClass": "B", "autonomyLevel": 3 }
        });
        let mut out = Vec::new();
        check(Kind::InstallPlan, &value, &mut out);
        assert!(out
            .iter()
            .any(|d| d.path == "/risk/effectiveClass"
                && d.code.as_str() == "semantic.effective_risk"));
    }
}
