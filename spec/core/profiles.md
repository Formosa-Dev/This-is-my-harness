# Profiles

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [manifest](../manifest/README.md) · [core vs extension](core-vs-extension.md) · [`profiles/`](../../profiles/README.md)

This document defines profiles as specializations of one Core. It answers §43 and the profile-versioning open question in §58.

---

## 1. Definition

1. A profile is a coherent, expected subset of capabilities for a use case that specializes UX, validators and defaults **over the same Core**.
2. A profile MUST NOT be a separate standard, a fork of the specification, or a marketplace category.
3. A profile MUST NOT redefine Core semantics. Where a profile and the Core disagree, the Core governs.
4. A profile is a specialization, not a fragment: the same harness model underlies every profile.

---

## 2. The initial profiles

| Profile | Use case | Expected capability focus |
| --- | --- | --- |
| `developer` | Reproducible agentic development environments. | Runtime adapters, portable skills, repository retrieval, context prioritization, policies, command gating, tests and evals. |
| `web-agent` | Agent-native web applications. | Web tool exposure, MCP, interactive app UI, agent-to-application transport, declarative agent UI, browser fallback, local decision models. |
| `local-hybrid` | Frontier and local models combined. | Remote generative capability, local decision capability, hardware detection, local/remote substitution, local service lifecycle. |
| `multi-agent` | Multiple collaborating agents. | Agent-to-agent interoperability, MCP, durable tasks, delegation, approvals, shared policies. |
| `workflow-automation` | Deterministic plus agentic automation. | API-described workflows, deterministic workflow standard, durable tasks, decision gates, human approval, observability. |

The capability names above are described by use, not by vendor. Concrete capability contracts live in [`../../extensions/`](../../extensions/) and MUST be expressed as contracts.

---

## 3. What a profile specializes

A profile MAY specialize only the following, and MUST NOT specialize anything else:

1. **Validators** — additional checks layered on top of Core validation.
2. **Defaults** — sensible defaults for fields the Core leaves optional.
3. **UX** — how a toolchain presents the profile to a user.
4. **Reference implementations** — example or reference harnesses that exercise the profile.
5. **Discoverability** — how the profile is surfaced for humans and agents.

A profile MUST NOT introduce a new manifest field with Core-level semantics, and MUST NOT change the meaning of a Core field.

---

## 4. Declaring a profile

1. A manifest MAY declare a primary profile using `spec.profile`.
2. A manifest MUST declare at most one primary profile.
3. A declared profile MUST be one of the recognized profiles, or a profile defined by a compatible extension.
4. A declared profile MUST NOT relax a Core requirement. A profile MAY add requirements; it MUST NOT remove them.
5. A harness that declares no profile is valid and is validated against the Core alone.

---

## 5. Conformance and profiles

1. Core conformance is what a conformant artifact MUST satisfy. A profile adds optional checks on top of Core conformance.
2. A harness MUST NOT be presented as conformant to a profile if it has not passed that profile's validators.
3. Profile conformance MUST NOT be used to imply Core conformance, and Core conformance MUST NOT be used to imply profile conformance.

---

## 6. Versioning profiles

1. Profiles version with the spec `apiVersion`; a profile's semantics MUST track the Core generation it targets. The versioning policy is [`VERSIONING.md`](../VERSIONING.md); it is referenced here, not restated.
2. A profile MAY additionally carry its own version series to track profile-local changes. When it does, the profile version MUST NOT be confused with the package version or the spec version.
3. A change to a profile that alters what a conformant artifact must do is an incompatible change for that profile and MUST follow the spec's incompatible-change rules.
4. How profiles are versioned is an open question (decision record §58). Until it is resolved, a profile change MUST be recorded through an RFC, and the Core MUST remain the single source of semantic authority.
5. A profile MUST NOT be forked into a separate standard. A divergence from the Core is resolved by changing the Core through an RFC, not by forking the profile.

---

## 7. Summary

- Profiles are specializations over one Core, never separate standards.
- The initial profiles are `developer`, `web-agent`, `local-hybrid`, `multi-agent`, `workflow-automation`.
- A profile specializes validators, defaults, UX, reference implementations and discoverability only.
- A manifest declares at most one primary profile; profiles add requirements and never remove them.
- Profile conformance is additive to Core conformance and never a substitute for it.
