# Manifest Semantics

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [identity](../core/identity.md) · [versioning](../core/versioning.md) · [package layout](../package/layout.md) · [component types](../package/component-types.md)

This document defines the semantics of the canonical manifest — the single machine-readable entrypoint of a harness. It defines fields, required and optional status, and the single-entrypoint rule. It does **not** define the JSON Schema or any concrete instance; those are produced in a later phase. The working filename is `harness.yaml`; the definitive name is an open question (decision record §58).

---

## 1. The single entrypoint

1. A harness package MUST contain exactly one canonical manifest at the package root.
2. The manifest is the only machine-readable entrypoint that resolves to the composition of the harness. If the manifest is absent or cannot be interpreted, the package MUST NOT be treated as a harness.
3. A package MUST NOT contain more than one canonical manifest. Additional configuration files MUST NOT be interpreted as manifests and MUST NOT override the canonical manifest.
4. The manifest describes the **composition** of the system. It MUST NOT be treated as a replacement for a runtime's native files or for the integrated open standards it composes.
5. The manifest MUST NOT encode vendor-specific behavior as a requirement. Runtime- and product-specific behavior belongs in an adapter or an extension (see [`../STYLE.md`](../STYLE.md), §5).

---

## 2. Top-level fields

The manifest has exactly four top-level fields.

| Field | Required | Type | Semantics |
| --- | --- | --- | --- |
| `apiVersion` | REQUIRED | string | The spec version the artifact targets. Exactly one value. See [`../core/versioning.md`](../core/versioning.md), §3. |
| `kind` | REQUIRED | string | The artifact kind. See §2.2. |
| `metadata` | REQUIRED | object | Identity and descriptive metadata. See §3. |
| `spec` | REQUIRED | object | The declarative composition. See §4. |

### 2.1 `apiVersion`

1. `apiVersion` MUST be present and MUST be a single supported spec version.
2. A validator MUST reject a manifest whose `apiVersion` it does not support.
3. `apiVersion` MUST NOT be a range. Only an extension manifest MAY carry a compatibility range, and that range is expressed under `spec`, not in `apiVersion` (see [`../core/versioning.md`](../core/versioning.md), §4).

### 2.2 `kind`

`kind` declares what the artifact is. The recognized artifact kinds are:

| `kind` | Meaning |
| --- | --- |
| `Harness` | A full harness: a complete, installable composition. |
| `Component` | A reusable typed element intended to be composed via `extends`. |
| `Preset` | A curated selection and set of defaults over components; it introduces no new semantics. |
| Extension kind | A kind declared by an extension under its own `apiVersion`. |

1. `kind` MUST be present.
2. The only `kind` value fixed in the Core is `Harness`. The `Component` and `Preset` values are recognized artifact kinds whose full schema is defined in a later phase.
3. An extension MAY declare its own `kind`, which MUST be namespaced so that it cannot collide with a Core kind.
4. A validator MUST reject a `kind` it does not recognize unless an extension defining that kind is present and compatible.

---

## 3. `metadata`

`metadata` carries identity and descriptive fields. It is REQUIRED as an object; its children are REQUIRED only as stated below.

| Field | Required | Semantics |
| --- | --- | --- |
| `name` | REQUIRED | The slug of the harness. MUST conform to the slug grammar in [`../core/identity.md`](../core/identity.md), §2. |
| `version` | REQUIRED | The package version. MUST be valid SemVer. See [`../core/versioning.md`](../core/versioning.md), §2. |
| `owner` | OPTIONAL | The namespace authority. When present it MUST match the owner of the canonical identifier under which the artifact is published. When absent, the owner is established by the publishing namespace. |
| `description` | OPTIONAL | Human-readable summary. It carries no semantics. |
| `license` | OPTIONAL | The license of the package (for example an SPDX identifier). If declared, it MUST appear in exactly one place for the package. |
| `author` | OPTIONAL | The author or maintainer name. **Not** a security guarantee and MUST NOT be presented as verified identity; verified identity is a trust label, not a manifest field. |
| `homepage` | OPTIONAL | A URL for human consumption. |
| `repository` | OPTIONAL | The source repository. Distribution provenance MAY restate this; see [`../core/distribution.md`](../core/distribution.md). |
| `keywords` | OPTIONAL | A list of human-facing tags. Tags MUST NOT be used to infer capabilities. |

Rules:

1. A field not listed above MUST NOT be assumed to have semantics. Unknown fields are governed by the schema in a later phase (see §5).
2. `metadata.name` is the slug only. It MUST NOT contain a `/` or an owner segment. The qualified identity is formed per [`../core/identity.md`](../core/identity.md).
3. Descriptive fields (`description`, `author`, `homepage`, `keywords`) MUST NOT be used by a resolver to make a compatibility or permission decision.

---

## 4. `spec`

`spec` is the declarative composition of the artifact. It is REQUIRED as an object; every section inside it is OPTIONAL unless a later document states otherwise.

| Section | Required | Semantics | Defined in |
| --- | --- | --- | --- |
| `profile` | OPTIONAL | The primary profile this harness specializes. At most one. | [`../core/profiles.md`](../core/profiles.md) |
| `components` | OPTIONAL | Explicit component declarations. | [`../package/layout.md`](../package/layout.md), [`../package/component-types.md`](../package/component-types.md) |
| `requirements` | OPTIONAL | Runtime, model, hardware, service and backend requirements. | [`../core/requirements.md`](../core/requirements.md) |
| `permissions` | OPTIONAL | Declared permissions and scopes. | [`../core/permissions.md`](../core/permissions.md) |
| `extends` | OPTIONAL | Dependencies on other artifacts. | [`../core/dependencies.md`](../core/dependencies.md) |
| `distribution` | OPTIONAL | Distribution and provenance metadata. | [`../core/distribution.md`](../core/distribution.md) |
| `compatibility` | OPTIONAL | Declared expected compatibility. | [`../core/compatibility.md`](../core/compatibility.md) |
| `conformance` | OPTIONAL | Conformance metadata. | [`../core/conformance-metadata.md`](../core/conformance-metadata.md) |

1. A minimal valid harness MAY have an empty `spec`. Such a harness is passive by construction and MUST NOT declare executable, tooling or privileged behavior.
2. A section that is present MUST be well-formed. A malformed optional section MUST cause rejection; it MUST NOT be ignored.
3. A harness MUST declare, in `permissions`, every capability it needs that has a risk class higher than Passive. A capability used but not declared is a conformance failure.

---

## 5. Validation semantics

1. A validator MUST reject a manifest that omits a REQUIRED field, violates a MUST, or declares an unsupported `apiVersion`, `kind` or profile.
2. A consumer that cannot fully understand a manifest MUST fail loudly with a typed error and MUST NOT guess or partially apply (see [`VERSIONING.md`](../VERSIONING.md), "Compatibility and migration rules").
3. Unknown fields: a validator MUST reject a manifest containing an unknown field in a REQUIRED object, unless a later schema version explicitly marks that position as forward-compatible and ignorable. Adding an optional field that older consumers can safely ignore MAY be done within the same alpha generation.
4. Validation MUST be deterministic: the same manifest bytes MUST produce the same validation result.

---

## 6. Relationship to native and integrated files

1. The manifest composes integrated open standards and native runtime files; it does not replace them.
2. Where an integrated standard defines the content of a component (for example a portable skill), the manifest MUST reference or position that content and MUST NOT redefine it in a competing format.
3. The manifest MUST NOT contain executable code. Executable behavior is declared as a component and is subject to the preview/policy/apply contract.

---

## 7. Summary

- Exactly one canonical manifest at the package root.
- Four top-level fields: `apiVersion`, `kind`, `metadata`, `spec` — all REQUIRED.
- `metadata.name` and `metadata.version` are REQUIRED; the remaining metadata fields are OPTIONAL.
- Every `spec` section is OPTIONAL; a malformed section is rejected, not ignored.
- Unknown fields and unsupported versions fail loudly.
- The manifest describes composition and replaces nothing native.
