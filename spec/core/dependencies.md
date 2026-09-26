# Dependencies and Composition

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [identity](identity.md) · [versioning](versioning.md) · [component types](../package/component-types.md) · [permissions](permissions.md) · [distribution](distribution.md)

This document defines how harnesses compose through `extends`, how references resolve, and which conflicts MUST be detected before Apply. It answers §15 and the dependency-resolution part of §57.

---

## 1. Reference forms

A dependency is expressed as a reference to another artifact. The recognized forms are:

| Form | Shape | Resolution |
| --- | --- | --- |
| Scoped reference | `@owner/slug` | Resolved against the same host as the declaring manifest. |
| Versioned scoped reference | `@owner/slug@<version>` | Resolved against the host, pinned to the stated version. |
| Canonical identifier | `https://<host>/h/<owner>/<slug>` | Resolved against the named host. |
| Versioned canonical identifier | `https://<host>/h/<owner>/<slug>@<version>` | Resolved against the named host, pinned to the stated version. |

Rules:

1. References MUST use the grammar in [`identity.md`](identity.md), §2.
2. A reference without a version is **unpinned** and MUST be resolved to an immutable digest before Apply. The resolved digest MUST be recorded in the Install Plan.
3. A version in a reference MUST follow the SemVer rules in [`versioning.md`](versioning.md).
4. A short reference (`owner/slug`) MUST NOT be used inside a published manifest, because it carries no host (see [`identity.md`](identity.md), §2.3).

---

## 2. `extends`

1. `extends` declares that an artifact composes one or more other artifacts.
2. `extends` MUST be an ordered list; the order is significant for precedence and MUST be preserved by the resolver.
3. An artifact MUST extend only artifact kinds permitted by [`../package/component-types.md`](../package/component-types.md), §1:
   - a Full Harness MAY extend Components, Presets and base Full Harnesses;
   - a Component MAY extend other Components;
   - a Preset MAY extend Components and Presets.
4. A Component MUST NOT extend a Full Harness.
5. A Preset MUST NOT introduce semantics; it MAY only select, default and parameterize.

---

## 3. Resolution rules

1. Resolution MUST be **deterministic**: the same inputs MUST produce the same resolved graph. The resolver MUST NOT depend on network order, filesystem order or time.
2. Resolution MUST be **complete before Apply**. No dependency may be fetched or resolved as a side effect of Apply.
3. Resolution MUST produce a **lock**: the fully resolved graph with each artifact pinned to an immutable digest and, where a version was used, the exact version.
4. Resolution MUST be **reproducible** from the lock alone, without re-resolving unpinned references.
5. Every resolved artifact MUST be verified by hash and, where a signature is present, by signature before Apply (see [`distribution.md`](distribution.md)).
6. The resolver MUST record, for every dependency, which artifact satisfied it and at which version and digest, so that the result is inspectable.

---

## 4. Composition semantics

1. The composed artifact is the union of the declarations contributed by the extending artifact and its dependencies, subject to the precedence rules below.
2. Precedence, from strongest to weakest:
   1. an explicit declaration in the extending artifact;
   2. a declaration contributed by a dependency named earlier in `extends`;
   3. a declaration contributed by a dependency named later in `extends`.
3. The precedence order MUST be documented and stable. It MUST NOT be left to implementation discretion.
4. A dependency MUST NOT silently override a declaration in the extending artifact.
5. A Preset contributes only defaults and selections; it MUST NOT override semantics contributed by a Component.

---

## 5. Conflict detection (before Apply)

The resolver MUST detect each of the following and MUST report it explicitly. A detected conflict MUST NOT be resolved silently.

| Conflict | Definition | Required outcome |
| --- | --- | --- |
| **Cycle** | A dependency path that returns to an already-visited artifact. | Reject the graph with a typed error naming the cycle. |
| **Version conflict** | Two dependencies require the same artifact at incompatible versions that cannot be reconciled by a declared rule. | Reject, or resolve by an explicit declared override; never silently pick one. |
| **Permission conflict** | A declared permission of one part contradicts a Policy that denies it, or two parts declare incompatible scopes for the same capability. | Reject, or require an explicit declared override visible in the Install Plan. |
| **Capability conflict** | Two parts require mutually exclusive capabilities, or a required capability cannot be satisfied by any available implementation. | Report the capability as unsupported; do not Apply. |
| **Kind conflict** | Two references resolve to the same logical identity but different artifact kinds, or a forbidden extension relation is declared. | Reject. |
| **Identity ambiguity** | Explicit and conventional discovery produce the same component identity with different content. | Reject as ambiguous. |
| **Duplicate identity with differing content** | The same `(owner, slug, version)` resolves to different bytes. | Reject; a published version is immutable. |

1. Conflict detection MUST run before any mutation.
2. Every conflict report MUST name the conflicting references, the nature of the conflict, and the options (if any).
3. A reported conflict MUST appear in the Install Plan.

---

## 6. Overrides

1. Where an override is permitted, it MUST be declared explicitly in the extending artifact.
2. An override MUST state which dependency declaration it replaces and why.
3. An override MUST be visible in the Install Plan and MUST NOT reduce a risk class or broaden a scope without the corresponding approval.
4. An override MUST NOT be used to bypass a security constraint; security constraints are not overridable by a manifest.

---

## 7. Effects on package properties

1. The effective permissions of a composed artifact are the union of the permissions of its parts, subject to §5.
2. The effective risk class is the maximum over the parts (see [`risk-classes.md`](risk-classes.md), §3).
3. The effective requirements are the union of the requirements of the parts; conflicting requirements follow §5.

---

## 8. Open items

How dependencies and composition are expressed — including the precise surface syntax of version ranges and override declarations — is an open question (decision record §58). This document fixes the **semantics**; the concrete surface syntax is defined by the schema in a later phase and MUST satisfy the semantics above.
