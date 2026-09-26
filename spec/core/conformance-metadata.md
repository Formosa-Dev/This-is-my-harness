# Conformance Metadata

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [compatibility](compatibility.md) · [distribution](distribution.md) · [manifest](../manifest/README.md)

This document defines conformance metadata: what it records, how it differs from a trust label, and what the canonical adapter contract requires. It answers §2.6 and §2.4.

---

## 1. Conformance is verifiable

1. Conformance is the verifiable property that a harness, extension or adapter satisfies the specification: schema validity, package structure, permission declarations, adapter behavior, fixtures and capability-specific requirements.
2. Conformance **MUST** be established by tests. It MUST NOT be established by declaration, by branding or by a badge.
3. A project MUST NOT be able to claim compatibility by adding a badge. Where behavior was not tested, conformance MUST NOT be claimed.
4. A conformance result MUST be reproducible: the same artifact, target and fixtures MUST yield the same result.

---

## 2. What conformance metadata records

For each conformance claim, the metadata MUST record:

| Field | Semantics |
| --- | --- |
| Artifact identity | The canonical identifier of the artifact tested, pinned to a digest. |
| Target | The runtime and runtime version, or the adapter, against which conformance was established. |
| Spec `apiVersion` | The spec generation under which the test was run. |
| Adapter version | The adapter version, where conformance concerns an adapter. |
| Fixtures | The fixtures or conformance suite revision used. |
| Capability results | Per capability: the resulting level from [`compatibility.md`](compatibility.md) (native / adapted / partial / untested / unsupported) and the enumerated gaps. |
| Result | Pass, fail, or partial, with the failing checks named. |
| Tool identity | The tool and version that produced the result. |
| Date | When the result was produced. |

1. A conformance claim without a target and a version is not a conformance claim and MUST NOT be presented as one.
2. A conformance claim that omits the capabilities that failed is non-conformant metadata.

---

## 3. Conformance evidence levels

The following are the recognized checks. A conformance report MUST state which of them were performed and their results.

| Check | Verifies |
| --- | --- |
| Schema validity | The manifest conforms to the supported schema. |
| Package validity | The package layout and discovery rules are satisfied. |
| Permission declaration | Every capability above Passive is declared, with a risk class and scope. |
| Adapter behavior | The adapter implements the canonical contract and reports capability loss explicitly. |
| Fixtures passed | The conformance fixtures for the artifact's profile and capabilities pass. |
| Capability result | Each capability's level is established and its gaps enumerated. |

A claim that a capability is supported MUST rest on the "Capability result" check; it MUST NOT rest on schema validity alone.

---

## 4. Conformance of the Runtime Adapter Contract

1. The canonical Runtime Adapter Contract has **eleven operations**: `detect`, `installPlan`, `install`, `authStatus`, `inspectExisting`, `capabilities`, `planApply`, `apply`, `verify`, `launch`, `revert`.
2. The README lists eight operations. Where the two disagree, the eleven-operation contract governs; the discrepancy is resolved in favor of the eleven-operation contract (see [`compatibility.md`](compatibility.md), §4).
3. An adapter's conformance metadata MUST state, per operation: whether it is implemented, and whether capability loss is reported. An operation that is implemented but silently discards behavior MUST be recorded as a failing check.
4. An adapter that implements only a subset of the contract MUST be recorded as `partial` or `untested` for the missing operations, never as conformant.

---

## 5. Conformance metadata versus trust label

Conformance metadata and trust labels are distinct and MUST NOT be conflated.

| Signal | Answers | Example content |
| --- | --- | --- |
| **Conformance metadata** | "Does this artifact/adapter satisfy the specification, and to what level?" | schema valid, fixtures passed, capability native on target X |
| **Trust label** | "What was verified about this artifact?" | manifest valid, hash verified, signature verified, author identity verified, maintainer reviewed, runtime tested |

1. A trust label MUST report exactly which integrity, provenance and identity checks passed. It MUST NOT imply more assurance than was established, and MUST NOT assert absolute safety.
2. Conformance MUST NOT be inferred from a trust label, and a trust label MUST NOT be inferred from conformance.
3. Both MUST be machine-readable and MUST remain independently verifiable.

---

## 6. Conformance is per runtime

1. What conformance means per runtime is not fixed at `v1alpha1` and is an open question (decision record §58). The rule that holds regardless is: a capability's conformance is established against a **specific** runtime or adapter and version, and MUST be reported at that granularity.
2. Who may issue conformance statements is an open question (decision record §58). Until resolved, a conformance statement MUST be accompanied by the evidence described in §2 so that it can be independently checked.

---

## 7. Summary

- Conformance is established by tests, never by declaration or badge.
- Metadata records artifact, target, versions, fixtures, capability results and tool identity.
- The canonical adapter contract has **11 operations**; the README's 8-op list is superseded.
- Conformance metadata and trust labels are distinct and must not be conflated.
- Conformance is per `(artifact, capability, target, version)`, and untested is not supported.
