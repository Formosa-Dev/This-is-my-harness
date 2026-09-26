# Permissions

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [risk classes](risk-classes.md) · [requirements](requirements.md) · [dependencies](dependencies.md)

This document defines the permission model: how a harness declares what it needs, how scope governs reach, and how declared permissions relate to approval and autonomy. It answers §49 and the permission/risk parts of §57 and §50. Risk classes A–D are defined in [`risk-classes.md`](risk-classes.md).

---

## 1. Scope

Scope is the reach of a mutation. There are two scopes.

| Scope | Definition | Default | Risk |
| --- | --- | --- | --- |
| **Project scope** | Changes are limited to the target project directory. | Yes — this is the default. | Lower. |
| **User scope** | Changes touch the user's global environment. | No. | Higher; Class D. |

Rules:

1. Project scope MUST be the default. An operation MUST NOT silently escalate to user scope.
2. A user-scope operation MUST require an exact preview of the affected paths plus an explicit warning and explicit approval.
3. The two scopes MUST NOT be treated as interchangeable.
4. Scope MUST be recorded in the Install Plan so that the reach of the operation is inspectable before Apply.

---

## 2. Permission declaration model

1. A harness MUST declare every capability it needs that exceeds Passive. A capability used but not declared is a conformance failure.
2. A permission declaration MUST include, at minimum: the capability it grants, the scope in which it applies, and the risk class of the operations it allows.
3. Permissions MUST be expressed as capability contracts, not as vendor-specific grants.
4. A declaration MAY include a human-readable justification, which is informative and MUST NOT reduce the declared risk class.

### 2.1 Capability grouping

Permission capabilities SHOULD be declared along these axes:

| Axis | Example capability areas |
| --- | --- |
| Filesystem | Read/write within project scope; write to user scope. |
| Network | Outbound calls to declared endpoints; inbound local service binding. |
| Execution | Hooks, scripts, commands. |
| Tools | MCP servers, plugins, integrations. |
| Models | Model capability contracts. |
| Services | Managed service lifecycle. |
| Secrets | Environment-variable names only (see §4). |
| System | Administrative or elevated operations. |

The concrete capability identifiers are defined by the schema and extensions; the axes above are normative in scope.

---

## 3. Declared permission versus granted approval

1. A **declared** permission states what the harness can do. It is part of the artifact.
2. An **approval** is granted by the user or by a policy at resolution time. Declaration does not imply approval.
3. A resolver MUST NOT grant an operation whose permission was not declared. An undeclared operation MUST cause rejection, not approval.
4. An approval MUST be revocable and MUST remain inspectable.

---

## 4. Secrets

1. A harness MUST NOT request, store or transmit credentials as part of the standard contract. Authentication belongs to the provider.
2. A harness MAY declare the **names** of required environment variables, so that the resolver can report what is missing. A harness MUST NOT declare their values.
3. How secrets are handled declaratively without storing them is an open question (decision record §58) and MUST NOT be fixed by the Core at `v1alpha1`.
4. A permission that grants access to a secret is risk class D, regardless of how the secret is supplied.

---

## 5. Network and services

1. A local service MUST bind to localhost by default.
2. Exposure beyond localhost MUST be declared explicitly and approved; it is risk class D.
3. Outbound network access MUST be declared as a permission. An undeclared outbound call is a conformance failure.

---

## 6. Permissions and autonomy

1. The autonomy level applied to an operation MUST be at least the floor implied by its risk class (see [`risk-classes.md`](risk-classes.md), §4).
2. Autonomy levels 0–3 are defined in [`glossary.md`](../glossary.md). The mapping is: Preview (0), Safe Apply for passive project-scope changes (1), Trusted Harness for opted-in low-risk updates (2), Elevated for executable and privileged operations (3).
3. There MUST NOT be an absolute bypass that skips Elevated operations. The operating system remains the final authority for privileged operations.
4. Reduced friction for a trusted publisher or harness MUST be opt-in and MUST NOT be granted silently.

---

## 7. Permission conflicts

1. When a harness or its dependencies declare incompatible permissions — for example a Policy that denies what another component requires — the conflict MUST be detected **before** Apply.
2. A conflict MUST be reported explicitly. It MUST NOT be resolved by silently dropping a permission or a component.
3. Where an explicit override is permitted, it MUST be declared in the extending artifact and MUST be visible in the Install Plan.
4. Conflict detection is part of dependency resolution (see [`dependencies.md`](dependencies.md), §6).

---

## 8. Summary

- Project scope default; user scope explicit, previewed and approved.
- Every capability above Passive MUST be declared; declaration is not approval.
- Secrets are declared by name only; values are never part of the contract.
- Localhost by default; broader exposure is Class D.
- Autonomy is floored by risk class; no absolute bypass.
- Permission conflicts are detected before Apply and never resolved silently.
