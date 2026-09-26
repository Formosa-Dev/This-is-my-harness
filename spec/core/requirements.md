# Requirements

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [manifest](../manifest/README.md) · [compatibility](compatibility.md) · [component types](../package/component-types.md)

This document defines the semantics of the `requirements` section — what a harness requires of the target environment before it can be installed and run. It answers §2.1 and §12.

---

## 1. Purpose and resolution model

1. Every requirement is a **predicate** evaluated against the target environment. A predicate MUST have a definite true or false result; it MUST NOT be "probable" or "assumed".
2. A harness MUST declare every requirement that is necessary for it to run. A requirement that is not declared MUST NOT cause the harness to fail after Apply; if undeclared requirements are discovered, the harness is non-conformant.
3. The resolver MUST evaluate requirements **before** any mutation and MUST produce an explicit, explainable result.
4. When a requirement can be satisfied by more than one implementation, the resolver MUST select one and MUST report which implementation it selected (see §4).

---

## 2. Requirement kinds

The `requirements` section MAY declare the following kinds. Each is defined by its testable semantics.

### 2.1 Runtime

1. `runtime` declares the runtime capability the harness targets, expressed as a **runtime capability identifier** (for example a value of the form `runtime.<runtime-id>`).
2. A Full Harness MUST declare exactly one primary runtime requirement. A Component MUST NOT declare a runtime requirement.
3. Runtime-specific behavior MUST NOT be encoded in the Core; the runtime requirement selects an adapter, and the adapter translates the canonical model (see [`compatibility.md`](compatibility.md)).

### 2.2 Runtime version

1. `runtimeVersion` declares the range of runtime versions the harness supports.
2. The range semantics follow [`versioning.md`](versioning.md), §4. A runtime version outside the declared range MUST cause rejection.
3. If a runtime version is not declared, the harness MUST be treated as compatible only when conformance evidence for the detected version exists (see [`conformance-metadata.md`](conformance-metadata.md)).

### 2.3 Model capabilities

1. `modelCapabilities` declares the capability contracts the harness needs (for example a generative capability, a structured decision capability, an embedding capability).
2. Each required capability MAY name a preferred implementation and one or more alternative implementations, each of which MUST satisfy the same capability contract.
3. Capabilities MUST be expressed as capability contracts. A concrete implementation MUST NOT be required for conformance.
4. When no available implementation satisfies a required capability, the resolver MUST report the capability as unsupported and MUST NOT Apply (see [`compatibility.md`](compatibility.md)).

### 2.4 Hardware

The `hardware` object declares the physical and platform resources the harness needs. Each field is a predicate.

| Field | Semantics |
| --- | --- |
| `os` | The set of operating-system families on which the harness can run. An environment whose OS is not in the set fails the predicate. |
| `arch` | The set of CPU architectures on which the harness can run. |
| `cpu` | Minimum logical core count and, where relevant, required CPU feature predicates. |
| `ram` | Minimum available memory. |
| `gpu` | Whether a GPU is required, and any capability predicates the GPU must satisfy. |
| `vram` | Minimum available GPU memory. |
| `disk` | Minimum free disk space for the download and the installed footprint. |

Rules:

1. Hardware requirements MUST be expressed as thresholds and capability predicates, not as vendor names. A GPU requirement MUST NOT name a vendor or product; it MUST state a capability (for example a minimum memory amount or a required compute feature).
2. A hardware field that is declared but not understood by the resolver MUST cause rejection; it MUST NOT be ignored.
3. Threshold comparisons MUST be inclusive of the stated minimum unless a document states otherwise.

### 2.5 Services

1. `services` declares the services the harness requires, expressed as service capability contracts.
2. Each required service MUST be resolvable to a declarable Service component (see [`../package/component-types.md`](../package/component-types.md), §10) or to an already-available equivalent.
3. A required service that cannot be provided MUST cause the plan to report the harness as unsupported.

### 2.6 Backends

1. `backends` declares the execution backends the harness requires, expressed as capability contracts (for example a local inference engine).
2. A backend requirement MUST be expressible independently of a concrete vendor implementation.
3. When several backends satisfy the same contract, the resolver MUST select deterministically and explainably.

---

## 3. Required versus preferred

1. A requirement is either **required** (hard) or **preferred** (soft). The distinction MUST be explicit in the declaration.
2. An unmet required requirement MUST cause the plan to report the harness as unsupported for that target, and the harness MUST NOT be applied unless an alternative implementation satisfying the same contract is resolved.
3. An unmet preferred requirement MUST be reported but MUST NOT block Apply. The report MUST be explicit and MUST NOT be silent.
4. Soft requirements MUST NOT be used to hide a hard dependency. A requirement necessary for correct operation MUST be declared as required.

---

## 4. Explainability

1. For every requirement, the resolver MUST be able to state: what was required, what was detected, which implementation (if any) was selected, and whether the predicate passed.
2. The detection result and the selection MUST be reproducible for the same environment and the same inputs.
3. Detection MUST NOT execute untrusted third-party code (see decision record §50). Detection of hardware and backends MUST use safe, read-only system information.

---

## 5. Interaction with compatibility and conformance

1. An unmet requirement is not by itself a compatibility level; it is an input to the compatibility decision. When a target cannot satisfy a requirement, the compatibility outcome MUST be expressed using the levels in [`compatibility.md`](compatibility.md) and MUST be explicit.
2. A harness MUST NOT claim to run on a target whose declared requirements it cannot evaluate. The absence of evidence MUST be reported as untested, not as support.

---

## 6. Summary

- Requirements are definite predicates evaluated before mutation.
- Kinds: runtime, runtime version, model capabilities, hardware, services, backends.
- Hardware is expressed as thresholds and capability predicates, never as vendor names.
- Required requirements block Apply when unmet; preferred requirements are reported but do not block.
- Every resolution MUST be explicit, explainable and reproducible.
