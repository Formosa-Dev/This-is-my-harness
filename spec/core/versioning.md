# Versioning Semantics

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [VERSIONING.md](../VERSIONING.md) · [glossary](../glossary.md) · [STYLE](../STYLE.md) · [distribution](distribution.md) · [compatibility](compatibility.md)

This document defines the versioning **semantics** that apply inside the Core. The authoritative policy for how the specification and its artifacts are versioned is [`VERSIONING.md`](../VERSIONING.md); this document MUST NOT restate or contradict it. Where this document adds detail, it does so for the fields the manifest and the Core consume.

---

## 1. Two independent version series

The project versions two different things. They MUST NOT be conflated.

| Series | Field | Meaning | Governed by |
| --- | --- | --- | --- |
| **Spec version** | `apiVersion` | Which generation of the standard an artifact targets. | [`VERSIONING.md`](../VERSIONING.md) |
| **Package version** | `metadata.version` | Which release of one harness or one extension an artifact is. | This document, per SemVer |

A change to the spec version is independent of a change to a package version. A package MAY publish a new `metadata.version` without any spec change, and the spec MAY publish a new `apiVersion` without changing any package.

Concrete values, stability stages and the `v1.0` gate are defined in [`VERSIONING.md`](../VERSIONING.md) and are referenced here, not repeated.

---

## 2. Package versioning (SemVer)

1. `metadata.version` MUST be a valid [Semantic Versioning 2.0.0](https://semver.org/) string, in the form `MAJOR.MINOR.PATCH`, optionally with pre-release and build metadata.
2. The version fields carry these meanings for a harness or component package:
   - **MAJOR** — an incompatible change to the declared interface, components, permissions or requirements.
   - **MINOR** — a backwards-compatible addition, such as a new optional component or capability.
   - **PATCH** — a backwards-compatible fix or documentation change.
3. A change that removes or reinterprets a declared permission, tightens a requirement, or changes a default behavior MUST be a MAJOR change, because a consumer that resolved the previous package can no longer rely on the same behavior.
4. Because the current spec stage is `v1alpha1`, a package author SHOULD treat the supported `apiVersion` as part of the package's compatibility surface. A package that raises or lowers the `apiVersion` it supports is making at least a MINOR-level change, and a change from one incompatible alpha generation to another is a MAJOR-level change.
5. A `metadata.version` MUST NOT be reused after it has been published, even if the previous artifact is withdrawn.

### 2.1 Pre-release and build metadata

1. A pre-release version (for example `1.2.0-alpha.1`) MUST be selected only when it is explicitly requested or when the resolution policy explicitly permits pre-releases. A resolver MUST NOT silently prefer a pre-release over a stable release.
2. Build metadata (`+...`) MUST NOT affect precedence or resolution. Two packages that differ only in build metadata MUST be treated as the same precedence for selection purposes.

---

## 3. Schema version (`apiVersion`)

1. `apiVersion` is a single string of the shape `thisismyharness.dev/v<major><stability><minor>`, as defined in [`VERSIONING.md`](../VERSIONING.md).
2. A manifest MUST declare exactly one `apiVersion`. A manifest MUST NOT declare a list of `apiVersion` values.
3. A validator MUST reject a manifest whose `apiVersion` is not one it supports. A consumer that cannot interpret an `apiVersion` MUST fail loudly with a typed error and MUST NOT guess, degrade or partially apply (see [`VERSIONING.md`](../VERSIONING.md), "Compatibility and migration rules").
4. The `apiVersion` of a **full harness** states the spec generation the harness targets. The `apiVersion` of an **extension** states the extension's own version series (see §5).

---

## 4. Compatibility ranges

1. A compatibility range expresses the set of spec `apiVersion` values an artifact can be consumed under.
2. A **full harness** declares a single target `apiVersion`. It MUST NOT declare a range, because a runnable composition targets one generation of the standard.
3. An **extension** MUST declare the Core `apiVersion` range it is compatible with (§5).
4. The semantics of a range are: an inclusive lower bound, an upper bound, and an ordering over the ordered stability stages defined in [`VERSIONING.md`](../VERSIONING.md). A consumer whose `apiVersion` falls outside the range MUST reject the extension with a typed error.
5. The concrete surface syntax used to express a compatibility range is defined by the schema in a later phase and is deliberately not fixed here. The normative semantics in rule 4 apply regardless of the surface syntax chosen.

---

## 5. Extension versioning

1. An extension is versioned independently of the Core, using the same `apiVersion` shape under the extension's own namespace.
2. An extension MUST declare the Core `apiVersion` range it is compatible with (see §4).
3. An extension MUST carry its own compatibility rules and conformance requirements, as required by [`core-vs-extension.md`](core-vs-extension.md).
4. A change to an extension's meaning or shape MUST follow the same incompatible-versus-SemVer rules as the Core, scoped to that extension.
5. An extension MUST NOT silently shadow a Core concept. Where an extension and the Core define the same name, the Core definition governs.

---

## 6. Immutability of published versions

1. A published `(owner, slug, version)` MUST be immutable. This is a Core invariant, restated from [`VERSIONING.md`](../VERSIONING.md) for the fields the Core consumes.
2. A tag or channel alias MUST resolve to exactly one digest at a time. A digest MUST always resolve to the same bytes.
3. A published version MUST NOT be silently replaced, mutated or re-pointed. A correction is a new version.
4. A resolver MUST pin the resolved digest before Apply. Resolution to a tag alone is insufficient; the digest is the immutable identity (see [`distribution.md`](distribution.md)).
5. A signature or attestation attached to a version MUST be verifiable independently of any social or registry layer.

---

## 7. Migration and deprecation

1. A migration note is mandatory for every incompatible change, as required by [`VERSIONING.md`](../VERSIONING.md).
2. A deprecated field MUST be marked in the schema and documented with a stated removal version. It MUST NOT be removed without at least one announced deprecated version.
3. A migration that performs a managed write MUST be reversible under the same snapshot/rollback contract as an install (see [`glossary.md`](../glossary.md), "Snapshot" and "Apply / Revert").

---

## 8. Summary of Core-consumed rules

- Spec version and package version are independent.
- `metadata.version` is SemVer; MAJOR/MINOR/PATCH carry the meanings in §2.
- `apiVersion` is a single value; unsupported values are rejected, never guessed.
- A full harness targets one `apiVersion`; an extension declares a compatibility range.
- Published versions and digests are immutable; tags move, digests do not.
- The resolver pins a digest before Apply.
