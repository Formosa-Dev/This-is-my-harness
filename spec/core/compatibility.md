# Compatibility

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [conformance metadata](conformance-metadata.md) · [requirements](requirements.md) · [distribution](distribution.md)

This document defines the compatibility levels and the rule that compatibility loss is always explicit. It answers §2.6 and the adapter-contract part of §2.4, and states which adapter contract is canonical.

---

## 1. The rule

> **Compatibility loss MUST always be explicit, never silent.**

1. When a runtime, an adapter or a translation cannot honor a capability, the loss MUST be reported.
2. A conversion MUST NOT silently discard meaningful behavior.
3. An unreported degradation is a conformance failure, even if the resulting behavior is otherwise usable.
4. Compatibility MUST be **verifiable**, not declarative. A project MUST NOT claim compatibility by adding a badge.

---

## 2. Compatibility levels

| Level | Meaning | Required reporting |
| --- | --- | --- |
| **native** | The capability is honored in full by the target without translation loss. | Report as supported; conformance evidence MUST exist. |
| **adapted** | The capability is honored through translation with semantically equivalent behavior; any difference from the canonical semantics MUST be declared. | Enumerate every declared difference. |
| **partial** | Only part of the capability's behavior is honored. | Enumerate the honored and non-honored parts. |
| **untested** | The capability has not been verified on the target. | Report as untested; MUST NOT be presented as supported. |
| **unsupported** | The capability cannot be honored on the target. | Report as unsupported; MUST NOT be silently dropped. |

Rules:

1. A capability in `untested` MUST NOT be promoted to `native` or `adapted` without conformance evidence (see [`conformance-metadata.md`](conformance-metadata.md)).
2. A capability in `partial` MUST enumerate its gap. An unenumerated gap MUST be treated as `unsupported`.
3. A capability in `unsupported` MUST cause the Install Plan to report it, and MUST NOT block Apply only when the capability is not required for the harness's core function. A **required** capability that is `unsupported` MUST block Apply.
4. Levels are properties of a `(capability, target)` pair for a specific artifact version. They MUST NOT be stated globally for a runtime.

---

## 3. Where compatibility is determined

1. Compatibility is determined by the **adapter** for a given runtime, using the Runtime Adapter Contract (see §4), and confirmed by conformance tests.
2. A manifest's `compatibility` section MAY declare **expected** compatibility. Expectations are informative and MUST NOT be presented as verified. The verified level is established by conformance.
3. The Install Plan MUST enumerate, before Apply: unsupported capabilities, adaptations and their declared differences, and any requirement that could not be evaluated.
4. A consumer that cannot determine a compatibility level MUST report `untested`, not `native`.

---

## 4. The Runtime Adapter Contract — canonical operation set

The canonical Runtime Adapter Contract has **eleven (11) operations**, as stated in §2.4 of the decision record:

```text
detect
installPlan
install
authStatus
inspectExisting
capabilities
planApply
apply
verify
launch
revert
```

1. These eleven operations are **canonical**.
2. The README lists eight operations (`detect`, `inspectExisting`, `capabilities`, `planApply`, `apply`, `verify`, `launch`, `revert`). Where the README and the decision record disagree, the **eleven-operation contract governs**; the discrepancy is resolved in favor of the eleven-operation contract. The README's list is an earlier, incomplete enumeration and is superseded.
3. An adapter MUST implement the full eleven-operation contract to be conformant at the current `apiVersion`. Partial implementations MUST be reported as `partial` or `untested`, not as conformant.
4. Each operation MUST report capability loss explicitly (§1). In particular, `capabilities`, `inspectExisting` and `planApply` MUST surface unsupported and adapted capabilities before any mutation.
5. The adapter is the only place runtime- and vendor-specific behavior may live. Provider- and runtime-specific logic MUST NOT leak into the Core or into a user interface.

---

## 5. Declared versus verified

1. **Declared** compatibility is what a manifest or an adapter claims.
2. **Verified** compatibility is what conformance evidence establishes.
3. A consumer MUST distinguish the two in any report it produces.
4. Compatibility loss that is discovered during verification MUST be reported and MUST update the effective level; it MUST NOT be omitted from the report.

---

## 6. Summary

- Compatibility loss is always explicit; silent degradation is a conformance failure.
- Levels: native · adapted · partial · untested · unsupported.
- Compatibility is determined by the adapter and confirmed by conformance, never by badge.
- The canonical Runtime Adapter Contract has **11 operations**; the README's 8-operation list is superseded.
- The Install Plan enumerates unsupported, adapted and unevaluated capabilities before Apply.
