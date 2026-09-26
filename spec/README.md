# Specification Index

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative for navigation and for the status of each specification area. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](glossary.md) · [STYLE](STYLE.md) · [VERSIONING](VERSIONING.md)

This directory contains the normative specification of This is my Harness. The specification is language-neutral. Concrete schemas, validators and product code are separate artifacts and MUST conform to the semantics defined here.

---

## 1. How to read this specification

1. Semantics are normative in this directory. A JSON Schema, a validator or an implementation MUST NOT contradict the semantics here; where they disagree, the semantics here govern.
2. Every normative file uses the RFC 2119 keywords as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174), and follows [`STYLE.md`](STYLE.md).
3. Controlled terms are defined in [`glossary.md`](glossary.md) and MUST be used with those meanings.

---

## 2. Core specification

| Area | Document | Task |
| --- | --- | --- |
| Controlled vocabulary | [`glossary.md`](glossary.md) | Foundations |
| Versioning policy | [`VERSIONING.md`](VERSIONING.md) | Foundations |
| Normative style guide | [`STYLE.md`](STYLE.md) | F1-13 |
| Identity, naming and namespace | [`core/identity.md`](core/identity.md) | F1-01 |
| Versioning semantics | [`core/versioning.md`](core/versioning.md) | F1-02 |
| Requirements | [`core/requirements.md`](core/requirements.md) | F1-06 |
| Permissions | [`core/permissions.md`](core/permissions.md) | F1-07 |
| Risk classes | [`core/risk-classes.md`](core/risk-classes.md) | F1-07 |
| Dependencies and composition | [`core/dependencies.md`](core/dependencies.md) | F1-08 |
| Distribution metadata | [`core/distribution.md`](core/distribution.md) | F1-09 |
| Compatibility | [`core/compatibility.md`](core/compatibility.md) | F1-10 |
| Conformance metadata | [`core/conformance-metadata.md`](core/conformance-metadata.md) | F1-10 |
| Profiles | [`core/profiles.md`](core/profiles.md) | F1-11 |
| Core vs extension boundary | [`core/core-vs-extension.md`](core/core-vs-extension.md) | F1-12 |

---

## 3. Manifest and package

| Area | Document | Task |
| --- | --- | --- |
| Manifest semantics | [`manifest/README.md`](manifest/README.md) | F1-03 |
| Package layout and discovery | [`package/layout.md`](package/layout.md) | F1-04 |
| Component and artifact types | [`package/component-types.md`](package/component-types.md) | F1-05 |

---

## 4. Profile and extension surfaces

| Area | Document |
| --- | --- |
| Profile refinements | [`../profiles/README.md`](../profiles/README.md) |
| Capability extensions | [`../extensions/`](../extensions/) |

Profiles specialize the Core and MUST NOT fork it; extensions add versioned capabilities and MUST NOT bloat the Core. The boundary is defined in [`core/core-vs-extension.md`](core/core-vs-extension.md).

---

## 5. Areas defined in the repository but not yet specified here

The following directories are part of the planned structure but do not yet contain a normative specification at this stage. They MUST NOT be treated as if a specification existed:

| Area | Directory | Status |
| --- | --- | --- |
| Install protocol | [`install-protocol/`](install-protocol/) | Planned. The pipeline is described in the glossary and in the decision record; a dedicated normative document is pending. |
| Runtime adapter contract | [`adapter-contract/`](adapter-contract/) | Planned. The canonical operation set is the **eleven-operation** contract stated in [`core/compatibility.md`](core/compatibility.md), §4 and [`core/conformance-metadata.md`](core/conformance-metadata.md), §4; a dedicated normative document is pending. |
| JSON Schema | [`../schemas/`](../schemas/) | Planned. Concrete schemas are a separate artifact and MUST conform to the semantics here. |

An implementation MUST NOT claim conformance for an area whose specification is still marked planned.

---

## 6. Status

The specification is pre-alpha. Nothing in this directory is stable until the project's v1.0 gate is met. Declaring `v1` is prohibited before the gate. Versioning rules are in [`VERSIONING.md`](VERSIONING.md).
