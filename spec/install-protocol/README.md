# Install Protocol

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [adapter contract](../adapter-contract/README.md) · [compatibility](../core/compatibility.md) · [permissions](../core/permissions.md) · [risk classes](../core/risk-classes.md)

This document defines the normative **Install Protocol**: the pipeline that moves a harness from a canonical identifier or URL to a verified local installation. It answers §2.5, §16, §18, §20, §21, §24, §25 and §50 of the decision record, and the "Install Protocol" and "Install Plan, snapshot and rollback" sections of the README.

The protocol is the standard. A command such as `harness use <canonical-url-or-owner/slug>` is **one implementation** of the protocol and MUST NOT be confused with it. The semantics defined here govern every surface that installs a harness, including the web, desktop, CLI and agent surfaces.

---

## 1. Scope and terminology

1. The controlled terms used here (harness, runtime, model, adapter, capability, manifest, install protocol, install plan, snapshot, apply, revert, scope, risk class, autonomy level, canonical identifier, conformance, trust label) MUST be used with the meanings defined in [`glossary.md`](../glossary.md).
2. The protocol is language-neutral. Concrete implementations MAY differ in language, process model and user interface. An implementation MUST NOT contradict the semantics defined here.
3. The protocol does not define surface syntax such as field names for a serialized Install Plan; that syntax belongs to the schema artifact and MUST remain consistent with the semantics here.
4. This document is the canonical statement of the protocol stages. Another document that describes the pipeline MUST reference this document. That document MUST NOT restate the stage semantics.

---

## 2. The pipeline

The protocol is the following ordered pipeline:

```text
Canonical ID/URL -> Resolver -> Manifest -> Version -> Artifact -> Hash/Signature
-> Capability Resolution -> Runtime Adapter -> Install Plan -> Snapshot -> Apply
-> Verify -> Launch
```

The pipeline has thirteen stages. The following table states each stage, whether it mutates target state, and whether it belongs to Preview.

| # | Stage | Mutates target state | Part of Preview |
| --- | --- | --- | --- |
| 1 | Canonical identifier / URL | No | Yes |
| 2 | Resolver | No | Yes |
| 3 | Manifest | No | Yes |
| 4 | Version | No | Yes |
| 5 | Artifact | No | Yes |
| 6 | Hash / Signature | No | Yes |
| 7 | Capability Resolution | No | Yes |
| 8 | Runtime Adapter | No | Yes |
| 9 | Install Plan | No | Yes |
| 10 | Snapshot | No (writes only to the snapshot store) | No |
| 11 | Apply | Yes | No |
| 12 | Verify | No | No |
| 13 | Launch | Starts processes | No |

Rules:

1. A stage MUST receive the output of the preceding stage as the primary input to the next stage.
2. A stage MUST fail closed. When a stage cannot establish its output, the pipeline MUST stop. The pipeline MUST NOT continue to a later stage after a stage has failed.
3. The pipeline MUST NOT begin Snapshot before the Install Plan is complete.

---

## 3. Stage-by-stage definition

### 3.1 Canonical identifier / URL

**Purpose.** Accept the reference supplied by a human or an agent and normalize it to a canonical form.

**Inputs.** A canonical identifier or URL, as defined in [`core/identity.md`](../core/identity.md).

**Outputs.** A normalized reference that identifies a harness and, optionally, a requested version.

**Failure modes.** A malformed identifier, an ambiguous reference, or a reference that cannot be normalized.

Rules:

1. The protocol MUST accept the canonical identifier shape defined in [`core/identity.md`](../core/identity.md).
2. A reference that cannot be normalized MUST cause a typed failure.
3. An unresolvable reference MUST NOT be guessed into a different harness.
4. The protocol MUST NOT resolve an arbitrary URL as a command to execute.

### 3.2 Resolver

**Purpose.** Turn the normalized reference into a resolution record that points at a manifest and an artifact.

**Inputs.** The normalized reference.

**Outputs.** A resolution record containing the canonical identity, the manifest location, the candidate versions and the artifact pointer.

**Failure modes.** The harness is not found, the source is unreachable, or the source returns conflicting records.

Rules:

1. The Resolver MUST be deterministic. Identical inputs against identical source state MUST yield the same resolution record.
2. The Resolver MUST NOT mutate target project or user state.
3. The Resolver MUST NOT execute code contained in the harness.
4. The resolution record MUST identify the manifest by canonical identifier and, once a version is pinned, by digest.

### 3.3 Manifest

**Purpose.** Obtain and validate the single canonical manifest of the harness.

**Inputs.** The resolution record.

**Outputs.** A validated manifest, with the semantics defined in [`manifest/README.md`](../manifest/README.md).

**Failure modes.** The manifest is missing, duplicated, invalid, targets an unsupported `apiVersion`, or declares an invalid `kind`.

Rules:

1. The protocol MUST obtain exactly one canonical manifest.
2. The manifest MUST be validated before it is used.
3. An invalid manifest MUST cause rejection.
4. A manifest that is not fully understood MUST NOT be partially applied.

### 3.4 Version

**Purpose.** Resolve a concrete package version from the manifest and any requested version or range.

**Inputs.** The validated manifest and the requested version or range.

**Outputs.** Exactly one resolved package version, pinned to an artifact digest.

**Failure modes.** No version satisfies the request, the request is ambiguous, or the requested version was withdrawn.

Rules:

1. Version resolution MUST yield exactly one package version.
2. The resolved version MUST be pinned to an immutable digest (see [`core/distribution.md`](../core/distribution.md)).
3. A version range MUST NOT silently fall back to a version outside the range.
4. When several versions satisfy a range, the protocol MUST select one deterministically.
5. The selected version MUST be reported. The selection MUST be explainable.

### 3.5 Artifact

**Purpose.** Fetch the immutable artifact for the pinned version.

**Inputs.** The pinned version and its digest.

**Outputs.** The artifact content addressed by the digest.

**Failure modes.** The fetch fails, the transport fails, or the content does not match the digest.

Rules:

1. The artifact MUST be fetched by digest.
2. The artifact MUST be treated as immutable.
3. A tag that resolves to different bytes than a previously observed digest MUST cause a failure.
4. Artifact retrieval MUST NOT execute artifact content.

### 3.6 Hash / Signature

**Purpose.** Verify the integrity of the artifact and, where available, its authenticity.

**Inputs.** The artifact, the expected digest, and any signature or attestation references.

**Outputs.** An integrity result and the trust-label contributions that follow from it.

**Failure modes.** The content hash does not match, a signature is invalid, or a required signature is absent.

Rules:

1. The protocol MUST verify the content hash before any mutation.
2. A hash mismatch MUST abort the protocol.
3. Where a signature is available, the protocol MUST verify it before Apply.
4. The protocol MUST record the verification outcome in a trust label (see [`glossary.md`](../glossary.md), "Trust label").
5. A trust label MUST report exactly which checks were performed.
6. An absent signature MUST NOT be reported as a verified signature.

### 3.7 Capability Resolution

**Purpose.** Determine the outcome of each required or provided capability against the target environment.

**Inputs.** The manifest requirements, the facts about the target environment, and the adapter's capability declarations.

**Outputs.** A per-capability result with one compatibility level and the enumerated gaps.

**Failure modes.** A required capability is unsupported, or a capability cannot be evaluated.

Rules:

1. Capability resolution MUST occur before any mutation.
2. Every capability MUST be assigned exactly one level from [`core/compatibility.md`](../core/compatibility.md).
3. A required capability that is `unsupported` MUST block Apply.
4. A capability that cannot be evaluated MUST be reported as `untested`.
5. A capability that was not evaluated MUST NOT be reported as supported.
6. Hardware and resource evaluation MUST be explainable. The evaluation MUST report the selected implementation (see [`core/requirements.md`](../core/requirements.md)).

### 3.8 Runtime Adapter

**Purpose.** Select the adapter for the target runtime and obtain its runtime-specific contribution.

**Inputs.** The resolved model and the detected target runtime.

**Outputs.** The selected adapter and its runtime-specific plan contribution.

**Failure modes.** No adapter exists for the target, the adapter is incompatible, or the adapter reports a required unsupported capability.

Rules:

1. The protocol MUST select an adapter through the Runtime Adapter Contract defined in [`adapter-contract/README.md`](../adapter-contract/README.md).
2. The adapter MUST report its capabilities and any compatibility loss before any mutation.
3. Runtime-specific behavior MUST remain inside the adapter.
4. When no adapter is available, the protocol MUST report the harness as unsupported for that target. The protocol MUST NOT apply the harness to that target.

### 3.9 Install Plan

**Purpose.** Produce the deterministic plan that describes the entire pending operation before any mutation.

**Inputs.** The outputs of all preceding stages.

**Outputs.** A deterministic, serializable Install Plan (see §4).

**Failure modes.** The plan cannot be produced, or the plan contains an unresolved blocking condition.

Rules:

1. The Install Plan MUST be generated before any mutation.
2. The Install Plan MUST be complete. An incomplete plan MUST NOT proceed to Apply.

### 3.10 Snapshot

**Purpose.** Capture the restorable managed state that Apply will touch.

**Inputs.** The Install Plan.

**Outputs.** A snapshot and an operation-journal entry.

**Failure modes.** The snapshot cannot be created, or it does not cover the state the plan will touch.

Rules:

1. A snapshot MUST be created before the first mutating operation of Apply.
2. The snapshot MUST cover exactly the managed state that the plan will touch.
3. The snapshot MUST be restorable without damaging unmanaged files.
4. When a snapshot cannot be created, Apply MUST NOT begin.

### 3.11 Apply

**Purpose.** Materialize the Install Plan against a prior snapshot.

**Inputs.** An approved Install Plan and its snapshot.

**Outputs.** The materialized managed state and the corresponding operation-journal entries.

**Failure modes.** A write fails, a conflict is detected, permission is denied, or the process crashes.

Rules:

1. Apply MUST consume an Install Plan produced by this protocol.
2. Apply MUST write atomically where the platform supports atomic writes.
3. Apply MUST NOT overwrite a conflict silently.
4. Apply MUST NOT delete an unmanaged file.
5. Apply MUST block path traversal and symlink escape.
6. Apply MUST hold a project lock so that concurrent applies do not interleave.
7. Apply MUST append to an operation journal sufficient to recover or revert the operation.
8. Apply MUST be crash-recoverable.
9. Apply MUST honor the autonomy floor implied by the risk class of the plan (see §7).

### 3.12 Verify

**Purpose.** Confirm that the applied state matches the plan and satisfies the declared verification steps.

**Inputs.** The applied state and the verification steps from the Install Plan.

**Outputs.** A verification report that names each check and its result.

**Failure modes.** One or more verification steps fail.

Rules:

1. Verify MUST run the verification steps enumerated by the Install Plan.
2. Verify MUST report exactly which checks passed and which failed.
3. A verification failure MUST NOT be reported as a successful installation.
4. Verify MUST NOT mutate managed target state.
5. A check that requires a mutation MUST be performed as a separate Apply.

### 3.13 Launch

**Purpose.** Optionally start the environment in the target runtime after successful verification.

**Inputs.** The verified state and the target runtime.

**Outputs.** A running environment, or a typed launch failure.

**Failure modes.** The runtime cannot start, or the launch is refused by policy.

Rules:

1. Launch MUST be optional. The protocol MUST be able to conclude successfully at Verify.
2. Launch MUST NOT execute third-party code outside the declared and approved executable components.
3. A failed Launch MUST leave the applied managed state intact.
4. Launch MUST remain subject to the operating system as the final authority for privileged operations.

---

## 4. The Install Plan

The Install Plan is the deterministic, serializable intermediate contract produced before any mutation. It is the shared contract between the web, desktop, CLI and agent surfaces.

Rules:

1. The Install Plan MUST be deterministic.
2. Identical resolved inputs and an identical environment MUST produce an identical plan.
3. The Install Plan MUST be serializable to a stable, machine-readable form.
4. The Install Plan MUST be inspectable before Apply.
5. The Install Plan MUST NOT contain or require the execution of third-party code.
6. The Install Plan MUST be bound to the inputs that produced it.
7. A material change to an input MUST produce a new plan.
8. The plan MUST name the autonomy level required by the operation.
9. A plan that declares an unresolvable blocking condition MUST NOT proceed to Apply.
10. The plan MUST enumerate each field in the following table.

| Field | Semantics |
| --- | --- |
| Runtime | The target runtime. |
| Runtime version | The resolved runtime version. |
| Project | The target project. |
| Scope | Project scope or user scope; project scope is the default. |
| Harness / version | The canonical identity and the resolved package version. |
| Files created / modified | Every managed file the operation will create or modify. |
| Conflicts | Every conflict detected, with the reason; conflicts MUST NOT be resolved silently. |
| Adaptations | Every translation the adapter will perform, with the declared difference. |
| Unsupported capabilities | Every required or provided capability that cannot be honored, and whether it blocks Apply. |
| MCP | Every MCP server or tool component involved. |
| Hooks / scripts | Every hook or script component that would execute. |
| Services | Every service component and its lifecycle actions. |
| Required environment-variable names | The **names** only of required environment variables; values MUST NOT appear. |
| Risk | The effective risk class of the operation. |
| Snapshot | The snapshot that will be taken, and the state it covers. |
| Verification steps | The checks that Verify will run after Apply. |

---

## 5. Snapshot, journal, crash recovery and rollback

Snapshot and rollback are part of the normal product contract, not an emergency feature.

Rules:

1. Every managed write MUST be preceded by a snapshot of the state it touches.
2. Every Apply MUST append a recoverable record to an operation journal.
3. After a crash, the operation journal MUST allow the operation to be either completed or reverted.
4. A revert MUST restore exactly the managed state captured in the snapshot.
5. A revert MUST NOT delete or modify unmanaged files.
6. A revert MUST record the restoration in the operation journal.
7. Rollback MUST be available after any Apply, at every autonomy level.

---

## 6. Preview safety rules

Preview is the portion of the pipeline from Canonical identifier / URL through Install Plan. Preview exists so that a human or an agent can inspect what would happen before anything happens.

Rules:

1. A Preview stage MUST NOT mutate target state.
2. A Preview stage MUST NOT execute third-party code.
3. The protocol MUST NOT resolve an arbitrary URL as a command to execute.
4. The protocol MUST NOT overwrite a conflict silently.
5. The protocol MUST NOT delete an unmanaged file.
6. Metadata resolution and Install Plan generation MAY proceed automatically.
7. Class C and Class D behavior MUST NOT run during Preview.

---

## 7. Autonomy levels and risk classes at Apply

The risk class of the operation sets the minimum autonomy floor that Apply MUST honor. Risk classes are defined in [`core/risk-classes.md`](../core/risk-classes.md); autonomy levels are defined in [`glossary.md`](../glossary.md).

| Autonomy level | Name | Applies to |
| --- | --- | --- |
| 0 | Preview | Resolution, manifest download, plan computation and diff analysis; no mutation. |
| 1 | Safe Apply | Passive, Class A changes within project scope, under a simple confirmation or policy. |
| 2 | Trusted Harness | Reduced friction, opt-in, for a previously authorized publisher or harness on low-risk updates. |
| 3 | Elevated | Class C and Class D operations: scripts, hooks, user scope, runtime installation, administrative access or sensitive access. |

Rules:

1. The Install Plan MUST record the effective risk class and the required autonomy level.
2. The effective risk class MUST be the maximum over the components and permissions of the operation.
3. Apply MUST NOT run below the autonomy floor implied by the effective risk class.
4. A Class C or Class D operation MUST require explicit confirmation.
5. There MUST NOT be an absolute bypass that skips an Elevated operation.
6. Reduced friction at level 2 MUST be opt-in.
7. The operating system MUST remain the final authority for privileged operations.

---

## 8. Relationship to the adapter contract and conformance

1. The Capability Resolution and Runtime Adapter stages MUST use the Runtime Adapter Contract defined in [`adapter-contract/README.md`](../adapter-contract/README.md).
2. A claim that a harness is compatible with a target MUST rest on conformance evidence (see [`core/conformance-metadata.md`](../core/conformance-metadata.md)).
3. A compatibility level MUST be verifiable. A compatibility level MUST NOT rest on a badge (see [`core/compatibility.md`](../core/compatibility.md)).
4. An implementation of this protocol MUST NOT claim conformance for a stage whose specification is still marked planned.

---

## 9. Summary

- The protocol is the thirteen-stage pipeline from canonical identifier or URL to a verified local installation.
- The command is an implementation of the protocol, not the standard.
- The Install Plan is deterministic, serializable and complete before any mutation.
- A snapshot precedes every managed write; the operation journal makes Apply crash-recoverable; rollback is a normal operation.
- Preview is side-effect-free and MUST NOT execute third-party code.
- Apply honors the autonomy floor implied by the risk class, and there is no absolute bypass.
- Compatibility loss is always explicit, never silent.
