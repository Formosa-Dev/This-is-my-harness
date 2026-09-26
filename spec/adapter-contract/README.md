# Runtime Adapter Contract

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [compatibility](../core/compatibility.md) · [conformance metadata](../core/conformance-metadata.md) · [install protocol](../install-protocol/README.md) · [risk classes](../core/risk-classes.md)

This document defines the normative **Runtime Adapter Contract**. It is the canonical source for the operation set that every adapter implements. It answers §2.4 of the decision record, together with §13, §25, §36, §42, §49 and §50.

An adapter translates the canonical harness model into native runtime behavior and back. Runtimes are not equal, and the contract exists so that this difference is handled explicitly rather than hidden.

---

## 1. Purpose and principles

1. An adapter MUST translate the canonical model into native runtime behavior.
2. An adapter MUST report what a target runtime can and cannot honor.
3. Compatibility loss MUST always be explicit, never silent.
4. Provider-specific and runtime-specific logic MUST live in the adapter.
5. Provider-specific and runtime-specific logic MUST NOT be added to the Core or to a user interface.
6. An adapter MUST NOT pretend that runtimes are equivalent.
7. The contract is language-neutral; an implementation MAY be written in any language that satisfies the semantics here.

---

## 2. The canonical operation set

The Runtime Adapter Contract comprises exactly eleven operations:

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

Rules:

1. A conformant adapter MUST expose exactly the eleven operations defined in §3.
2. This document MUST be treated as the canonical source for the operation set.
3. Another document that needs the operation set MUST reference this document.
4. Another document MUST NOT restate the operation list.
5. An adapter that implements only a subset of the operations MUST be reported as `partial` or `untested`. That adapter MUST NOT be reported as conformant.
6. The operation names and argument shapes used in prose here are semantic; their concrete surface syntax belongs to the schema artifact.

---

## 3. Operation reference

The following table states each operation and whether it is read-only or mutating.

| Operation | Class | Summary |
| --- | --- | --- |
| `detect()` | Read-only | Report whether the target runtime is present and its version. |
| `installPlan()` | Read-only | Produce the runtime-specific contribution to the Install Plan. |
| `install()` | Mutating | Perform the runtime's own installation before harness Apply. |
| `authStatus()` | Read-only | Report authentication status without handling credentials. |
| `inspectExisting(project)` | Read-only | Report existing managed, conflicting and unmanaged state. |
| `capabilities()` | Read-only | Declare supported capabilities with a compatibility level each. |
| `planApply(bundle, scope)` | Read-only | Compute the runtime-specific projection of applying a bundle. |
| `apply(plan)` | Mutating | Materialize an approved apply plan. |
| `verify(project)` | Read-only | Verify that the applied state matches the plan. |
| `launch(project)` | Mutating | Start the environment in the runtime. |
| `revert(snapshot)` | Mutating | Restore the managed state captured in a snapshot. |

### 3.1 `detect()`

**Purpose.** Determine whether the target runtime is present and which version it is.

**Inputs.** Facts about the target environment and, optionally, a requested runtime version.

**Outputs.** A detection result stating presence, runtime version, and the evidence used.

**Preconditions.** None.

**Class.** Read-only.

Rules:

1. `detect()` MUST NOT mutate any state.
2. `detect()` MUST NOT install, download or launch the runtime.
3. `detect()` MUST report the evidence for its version determination.
4. `detect()` MUST report absence as a typed result.
5. `detect()` MUST NOT guess a version when it cannot determine one.

### 3.2 `installPlan()`

**Purpose.** Produce the runtime-specific contribution to the harness Install Plan, including the runtime's own installation steps.

**Inputs.** The resolved harness model, the target environment, and the requested scope.

**Outputs.** A runtime-specific plan describing runtime installation steps, projected writes, adaptations and unsupported capabilities.

**Preconditions.** A resolved harness model.

**Class.** Read-only.

Rules:

1. `installPlan()` MUST NOT mutate any state.
2. `installPlan()` MUST enumerate every step that would execute code.
3. `installPlan()` MUST enumerate unsupported and adapted capabilities with their gaps.

### 3.3 `install()`

**Purpose.** Perform the runtime's own installation or bootstrapping required before harness Apply.

**Inputs.** An approved runtime plan and a snapshot.

**Outputs.** Installed runtime-side state and operation-journal entries.

**Preconditions.** An approved plan and a snapshot covering the affected state.

**Class.** Mutating.

Rules:

1. `install()` MUST consume an approved plan.
2. `install()` MUST NOT begin without a snapshot of the state it will touch.
3. `install()` MUST record every managed write in the operation journal.
4. `install()` MUST NOT overwrite a conflict silently.
5. `install()` MUST NOT delete an unmanaged file.
6. `install()` MUST be revertible through `revert()`.

### 3.4 `authStatus()`

**Purpose.** Report whether the runtime is authenticated, and the method category, without handling credentials.

**Inputs.** The target runtime.

**Outputs.** A status value and a method category, with no secret material.

**Preconditions.** None.

**Class.** Read-only.

Rules:

1. `authStatus()` MUST report authentication status only.
2. `authStatus()` MUST NOT store provider credentials or OAuth tokens.
3. `authStatus()` MUST NOT request provider credentials or OAuth tokens.
4. `authStatus()` MUST NOT transmit or exfiltrate provider credentials or OAuth tokens.
5. The output of `authStatus()` MUST NOT contain secret material.
6. Where authentication is required, the adapter MUST delegate authentication to the provider.

### 3.5 `inspectExisting(project)`

**Purpose.** Report what the runtime and adapter already have in the project, so that the plan can distinguish managed, conflicting and unmanaged state.

**Inputs.** A target project.

**Outputs.** An inventory of existing managed artifacts, conflicts, unmanaged files and existing runtime configuration.

**Preconditions.** The project path is resolvable.

**Class.** Read-only.

Rules:

1. `inspectExisting()` MUST NOT mutate any state.
2. `inspectExisting()` MUST distinguish managed state from unmanaged state.
3. `inspectExisting()` MUST report conflicts explicitly.
4. `inspectExisting()` MUST NOT resolve a conflict.
5. `inspectExisting()` MUST NOT read secret values.

### 3.6 `capabilities()`

**Purpose.** Declare the capabilities that the target runtime supports, with a compatibility level for each.

**Inputs.** The target runtime and its version.

**Outputs.** For each capability: one compatibility level, the enumerated gaps, and a conformance-evidence reference where evidence exists.

**Preconditions.** The runtime is detectable.

**Class.** Read-only.

Rules:

1. `capabilities()` MUST report each capability with exactly one level from [`core/compatibility.md`](../core/compatibility.md).
2. `capabilities()` MUST report a capability that has not been verified as `untested`.
3. `capabilities()` MUST NOT report an unverified capability as supported.
4. `capabilities()` MUST NOT infer capability support from the identity of the runtime alone.
5. `capabilities()` MUST enumerate the gap of every `partial` or `adapted` capability.

### 3.7 `planApply(bundle, scope)`

**Purpose.** Compute the runtime-specific projection of applying a resolved bundle at a given scope, as a plan.

**Inputs.** A resolved bundle (harness model and artifacts) and a scope.

**Outputs.** An apply plan describing projected writes, adaptations, conflicts, unsupported capabilities, executable components and verification steps.

**Preconditions.** A resolved bundle.

**Class.** Read-only.

Rules:

1. `planApply()` MUST NOT mutate any state.
2. `planApply()` MUST compute the projection for the requested scope.
3. `planApply()` MUST report adaptations and unsupported capabilities with their gaps.
4. `planApply()` MUST report conflicts.
5. `planApply()` MUST NOT resolve a conflict.
6. `planApply()` MUST be deterministic for identical inputs.

### 3.8 `apply(plan)`

**Purpose.** Materialize an approved apply plan.

**Inputs.** An approved apply plan and a snapshot.

**Outputs.** Applied managed state and operation-journal entries.

**Preconditions.** A plan, an approval that satisfies the plan's risk class, and a snapshot.

**Class.** Mutating.

Rules:

1. `apply()` MUST consume an approved plan.
2. `apply()` MUST NOT begin without a snapshot.
3. `apply()` MUST write atomically where the platform supports atomic writes.
4. `apply()` MUST NOT overwrite a conflict silently.
5. `apply()` MUST NOT delete an unmanaged file.
6. `apply()` MUST record every managed write in the operation journal.
7. `apply()` MUST be crash-recoverable.
8. `apply()` MUST block path traversal and symlink escape.

### 3.9 `verify(project)`

**Purpose.** Verify that the applied state matches the plan and satisfies the declared verification steps.

**Inputs.** The target project and the plan that was applied.

**Outputs.** A verification report that names each check and its result.

**Preconditions.** `apply()` has completed.

**Class.** Read-only.

Rules:

1. `verify()` MUST NOT mutate managed target state.
2. `verify()` MUST report each check with its result.
3. `verify()` MUST report a failed check as a failure.
4. `verify()` MUST NOT report a failed check as passed.
5. `verify()` MUST produce evidence usable as conformance metadata (see [`core/conformance-metadata.md`](../core/conformance-metadata.md)).

### 3.10 `launch(project)`

**Purpose.** Start the environment in the runtime after successful verification.

**Inputs.** A verified target project.

**Outputs.** A launch result, including a handle to the running environment or a typed failure.

**Preconditions.** `verify()` has succeeded.

**Class.** Mutating (starts a process).

Rules:

1. `launch()` MUST NOT execute third-party code outside the declared and approved executable components.
2. `launch()` MUST remain subject to the operating system as the final authority for privileged operations.
3. A failed `launch()` MUST leave the applied managed state intact.
4. `launch()` MUST be optional. A caller MAY stop at `verify()`.

### 3.11 `revert(snapshot)`

**Purpose.** Restore the managed state captured in a snapshot.

**Inputs.** A snapshot.

**Outputs.** Restored managed state and operation-journal entries.

**Preconditions.** A valid snapshot.

**Class.** Mutating.

Rules:

1. `revert()` MUST restore exactly the managed state captured in the snapshot.
2. `revert()` MUST NOT delete or modify an unmanaged file.
3. `revert()` MUST record the restoration in the operation journal.
4. `revert()` MUST be available after any `apply()`.

---

## 4. Plan/apply separation

Planning MUST NOT mutate. A mutation MUST consume a plan.

Rules:

1. A read-only operation MUST NOT mutate any state.
2. The read-only operations are `detect()`, `installPlan()`, `authStatus()`, `inspectExisting()`, `capabilities()`, `planApply()` and `verify()`.
3. The mutating operations are `install()`, `apply()`, `launch()` and `revert()`.
4. `apply()` and `install()` MUST consume a plan that was produced before the mutation.
5. A plan MUST be inspectable before it is applied.
6. An operation that would both plan and mutate MUST be split into a planning operation and a mutating operation.

---

## 5. `authStatus()` and credentials

Authentication belongs to the provider, and the contract MUST NOT become a credential channel.

Rules:

1. An adapter MUST NOT store provider credentials or OAuth tokens.
2. An adapter MUST NOT request provider credentials or OAuth tokens.
3. An adapter MUST NOT transmit or exfiltrate provider credentials or OAuth tokens.
4. `authStatus()` MUST be limited to reporting authentication status and method category.
5. The contract MUST NOT define a way to pass provider secret values through an operation.
6. An adapter that requires authentication MUST delegate the authentication flow to the provider.

---

## 6. `capabilities()` and compatibility

`capabilities()` is where an adapter declares what the target runtime can honor. The canonical definitions of the compatibility levels are in [`core/compatibility.md`](../core/compatibility.md); this document does not redefine them.

The levels are `native`, `adapted`, `partial`, `untested` and `unsupported`.

Rules:

1. Compatibility loss MUST always be explicit, never silent.
2. An adapter MUST declare compatibility per `(capability, target, version)`, not globally for a runtime.
3. An adapter MUST report an `unsupported` capability rather than dropping it silently.
4. An adapter MUST enumerate the gap of a `partial` capability.
5. An adapter MUST declare the difference of an `adapted` capability.
6. An adapter MUST NOT promote an `untested` capability to `native` or `adapted` without conformance evidence.
7. An adapter MUST surface unsupported and adapted capabilities in the plan before any mutation.
8. A required capability that is `unsupported` MUST block Apply.

---

## 7. Adapter translation rules

1. An adapter MUST translate the canonical model into native runtime behavior without pretending that the target runtime is equivalent to another.
2. An adapter MUST NOT silently discard meaningful behavior.
3. An adapter MUST report every point where its native behavior differs from the canonical semantics.
4. Provider-specific and runtime-specific logic MUST live in the adapter.
5. Provider-specific and runtime-specific logic MUST NOT be added to the Core or to a user interface.
6. Where a capability cannot be represented natively, the adapter MUST report the loss rather than approximating it silently.

---

## 8. Relationship to verification and conformance

1. An adapter's conformance MUST be established by tests (see [`core/conformance-metadata.md`](../core/conformance-metadata.md)).
2. An adapter's conformance metadata MUST state, per operation, whether the operation is implemented and whether capability loss is reported.
3. An adapter MUST NOT claim conformance for an operation that silently discards behavior.
4. A compatibility level declared by `capabilities()` MUST be confirmable by conformance evidence.
5. An adapter that is `partial` or `untested` for an operation MUST be reported as such, never as conformant.
6. The Install Protocol MUST use this contract for its Capability Resolution and Runtime Adapter stages (see [`install-protocol/README.md`](../install-protocol/README.md)).

---

## 9. Summary

- The Runtime Adapter Contract has exactly eleven operations; this document is their canonical source.
- Seven operations are read-only and four are mutating; planning never mutates, and mutation consumes a plan.
- `authStatus()` reports status only and never stores, requests or exfiltrates credentials.
- `capabilities()` declares per-capability levels and compatibility loss is always explicit.
- Provider- and runtime-specific logic lives in adapters, never in the Core or a UI.
- Adapter conformance is verifiable by tests, not by declaration.
