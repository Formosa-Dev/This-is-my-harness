# Core vs Extension Boundary

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [versioning](versioning.md) · [conformance metadata](conformance-metadata.md) · [profiles](profiles.md)

This document defines the boundary between the Core and extensions, the criteria for admitting something to the Core, and the process by which an idea becomes a stable extension. It answers §4, §44 and §57, and enforces the design rule *standardize composition, not every primitive*.

---

## 1. The rule

> **Standardize composition, not every primitive.**

1. The Core MUST be small and stable.
2. Advanced or specialized capabilities MUST enter as **versioned extensions**, not by inflating the Core.
3. Where a reasonable open standard already exists for a capability, the project MUST adopt or compose it rather than create a competing closed format.
4. A capability **MUST be an extension** if an existing standard resolves it, or if it is not universal to every harness.

---

## 2. Admission criteria for the Core

A capability MAY be admitted to the Core only if **all** of the following hold:

1. It is required to define what a conformant harness **is** — identity, naming, namespace; versioning; manifest semantics; package layout; component discovery; the component taxonomy; requirements; permissions and risk; dependencies; distribution metadata; compatibility; conformance metadata.
2. It is needed by effectively every harness, not by one class of use case.
3. It is stable enough that encoding it in the Core does not freeze an unsettled experiment.
4. No existing open standard already resolves it. If one does, the correct action is composition, not Core admission.
5. It can be expressed without hardcoding a vendor, product or model brand.

If any criterion fails, the capability MUST be an extension.

---

## 3. What MUST be an extension

The following MUST be an extension and MUST NOT be placed in the Core:

1. Any capability for which an open standard exists — including tool/resource interop, live-web tool exposure, interactive app UI, agent-to-application transport, declarative agent UI, agent-to-agent interop, portable skill knowledge, durable tasks, deterministic API workflows, distributed tracing, content-addressed distribution and signing.
2. Any runtime-specific capability, expressed as a runtime capability contract.
3. Any model capability contract beyond the Core's definition of what a model is.
4. Any capability that only some profiles need.
5. Any capability whose shape is still experimental.

The candidate capability list in §4 of the decision record is the reference set of extensions; concrete identifiers are defined by the extensions themselves and MUST be expressed as contracts.

---

## 4. Extension requirements

An extension MUST:

1. Carry its own `apiVersion` under its own namespace.
2. Declare the Core `apiVersion` range it is compatible with (see [`versioning.md`](versioning.md), §4).
3. Define its own compatibility rules and conformance requirements.
4. Compose an existing open standard where one exists, rather than inventing a competing format.
5. Be independently versioned and independently conformance-tested.
6. Not silently shadow a Core concept. Where names collide, the Core definition governs.

---

## 5. The Labs → extension → stable process

An idea advances only through this pipeline:

```text
Labs → Reference Harness → real-world use → extension candidate → conformance tests → stable extension
```

1. **Labs** — frontier experimentation without a stability promise. Lab material MAY be removed at any time.
2. **Reference Harness** — the idea is exercised in an intentionally different end-to-end harness that stresses the model. A reference harness is a conformance torture test, not a marketing demo.
3. **Real-world use** — the idea is used in real scenarios; findings feed back as corrections to the specification.
4. **Extension candidate** — a written proposal, recorded through an RFC.
5. **Conformance tests** — the candidate gains tests that demonstrate conformance.
6. **Stable extension** — the candidate is published as a versioned extension.

An idea MUST NOT skip from Labs directly to the Core. A change to Core semantics MUST be recorded through an RFC with an explicit justification, including why the capability cannot be an extension.

---

## 6. Prohibited patterns

The following MUST NOT happen:

1. Inventing another skill format or another tool/resource interop format when an adopted standard exists.
2. Hardcoding a model brand or vendor in the Core.
3. Promoting every ecosystem novelty into the Core immediately.
4. Letting the social/registry layer define technical semantics.
5. Declaring a capability stable before reference-harness evidence exists.
6. Using an extension to bypass a Core security or permission constraint.

---

## 7. Escalation to the Core

1. Moving a capability from an extension to the Core is an incompatible-class change to the specification and MUST follow [`VERSIONING.md`](../VERSIONING.md).
2. The RFC that proposes a Core move MUST include: the admission-criteria analysis from §2, evidence from reference harnesses, conformance results, and the reason the capability cannot remain an extension.
3. If the Core cannot represent a reference harness without runtime-specific hacks, the resolution is to change the **specification** through an RFC, not to add a runtime special case to the Core or to a user interface.

---

## 8. Summary

- Core stays small and stable; advanced and specialized capabilities are extensions.
- Admission requires universality, stability, absence of an existing standard, and vendor neutrality.
- If an existing standard resolves a capability, it MUST be an extension that composes that standard.
- Innovation flows Labs → reference harness → real-world use → RFC → conformance → stable extension.
- Core changes require an RFC and reference-harness evidence; the Core never special-cases a runtime.
