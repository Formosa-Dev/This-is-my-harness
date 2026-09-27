# schema-strategy Specification

> Capability `schema-strategy` introduced by change `harness-schema-v1alpha1` (F2-01..F2-04), archived 2026-09-26.
> Modified by change `f2-validator-rust` (F2-05..F2-12), archived 2026-09-26 — R3 record flipped `pending → confirmed` (executed `jsonschema 0.58.1` offline spike) and the semantic boundary now requires exactly ONE authoritative layer→task matrix.
> Normative source: `spec/**` (F1). Engram lineage (introduced): explore #4368 · proposal #4369 · spec #4372 · design #4370 · tasks #4373 · apply-progress #4375 · verify-report #4378.
> Engram lineage (modified): explore #4382 · proposal #4383 · spec #4385 · design #4384 · tasks #4386 · apply-progress #4387 · verify-report #4393.
> Archived delta (introduced): `openspec/changes/archive/2026-09-26-harness-schema-v1alpha1/specs/schema-strategy/spec.md`.
> Archived delta (modified): `openspec/changes/archive/2026-09-26-f2-validator-rust/specs/schema-strategy/spec.md`.

## Requirements

### Requirement: Single machine-readable source of truth

`schemas/**` MUST be the single machine-readable source of truth for the manifest, the model contract and the Install Plan shapes. No language, package or UI MAY define a parallel authoritative type for a concept already defined in a schema. Every consumer MUST resolve the same `$id` to the same definition.

#### Scenario: A shared concept is defined once

- GIVEN a concept used by more than one schema (for example a capability identifier or a risk class)
- WHEN a consumer needs that concept
- THEN it MUST resolve one shared definition by `$id`
- AND no second authoritative definition MUST exist in `spec/**`, a language package or a UI

#### Scenario: A parallel hand-written type is a divergence

- GIVEN a language package that re-declares a shape already defined in a schema
- WHEN the schema and the package disagree
- THEN the package type MUST be treated as a divergence, not as authoritative
- AND generated code produced *from* a schema MAY exist, but hand-authored parallel types MUST NOT

### Requirement: Language-neutral schema artifacts

Every schema artifact MUST be language-neutral and MUST NOT embed language-specific constructs, runtime-specific behavior or vendor-specific requirements.

#### Scenario: A schema is consumable from any language

- GIVEN the same schema files
- WHEN a Rust consumer and a JavaScript consumer resolve them
- THEN both MUST reach the same validation result
- AND no language MUST be required to interpret the schema

### Requirement: Schema `$id` namespace

Every schema and shared definition MUST declare a unique `$id` under the namespace `https://thisismyharness.dev/schemas/v1alpha1/`. The namespace is a schema-identity namespace and MUST be documented as explicitly distinct from the harness host that decision record §58 leaves OPEN.

#### Scenario: Namespace is a schema address, not a harness host

- GIVEN the `$id` host `thisismyharness.dev`
- WHEN a reader asks whether it fixes the canonical harness host
- THEN the strategy MUST state it does not, and that the canonical host remains OPEN
- AND treating the `$id` host as the normative harness host MUST be rejected

### Requirement: Shared `$defs` and reference strategy

Each shared concept MUST be defined exactly once and referenced by absolute `$id`. The strategy MUST NOT allow a reference cycle; in particular the model-contract schema MUST NOT reference the component descriptor.

#### Scenario: No duplicate definition

- GIVEN a concept referenced by the manifest and the model contract
- WHEN both are validated
- THEN they MUST resolve the same definition file
- AND the concept MUST NOT be redefined inline in either schema

#### Scenario: A cyclic reference chain is rejected

- GIVEN a set of schemas where A references B and B references A
- WHEN the reference graph is inspected
- THEN the cycle MUST be reported
- AND the schemas MUST NOT be frozen while a cycle exists

### Requirement: Offline-resolvable references

A schema-external reference MUST be resolvable from a checked-in resolver map keyed by `$id`, so validation can run without network access.

#### Scenario: Offline resolution succeeds

- GIVEN no network access
- WHEN a consumer validates an instance that references a shared definition
- THEN the resolver map MUST resolve the `$id` to the intended local file
- AND validation MUST succeed

#### Scenario: A broken `$ref` fails loudly

- GIVEN a `$ref` whose target `$id` is not declared in the resolver map or any schema
- WHEN validation runs
- THEN it MUST fail with an unresolved-reference error
- AND it MUST NOT silently ignore the reference

### Requirement: Closure mechanism for REQUIRED objects

Closed REQUIRED objects MUST be expressed with the composition-aware closure keyword of the chosen dialect (`unevaluatedProperties`), not with a same-level-only keyword that breaks under `allOf`/`$ref` composition. Positions that decision record §58 leaves OPEN MUST remain permissive and MUST be marked OPEN in `description`.

#### Scenario: Closure holds under composition

- GIVEN a REQUIRED object assembled from a `$ref` and an `allOf` branch
- WHEN an instance adds an undeclared property
- THEN validation MUST reject it

#### Scenario: OPEN positions stay permissive

- GIVEN a position marked OPEN by §58
- WHEN an instance adds implementation detail the Core does not define
- THEN validation MUST accept it
- AND the schema MUST NOT impose a fixed value there

### Requirement: F1 traceability and no weakened MUST

Every non-OPEN schema constraint MUST cite an F1 anchor from `spec/**`. A schema convenience that would permit, weaken or contradict an F1 `MUST` MUST be treated as a specification violation and MUST be rejected before freeze.

#### Scenario: Every constraint is traceable

- GIVEN a non-OPEN constraint in a schema
- WHEN the traceability matrix is reviewed
- THEN the constraint MUST map to an F1 section anchor
- AND the anchor MUST support the constraint's strength

#### Scenario: A weakening constraint is rejected

- GIVEN a schema that would accept an instance the F1 text forbids
- WHEN the schema is reviewed against `spec/**`
- THEN the schema MUST be rejected as wrong by definition
- AND the inconsistency MUST NOT be resolved by editing `spec/**` inside this change

### Requirement: §58 open items remain OPEN

Every schema position that touches an item left OPEN by decision record §58 MUST carry an explicit OPEN annotation naming §58. The schema MUST NOT resolve, hardcode or freeze such an item.

#### Scenario: OPEN is annotated, not decided

- GIVEN a position touching an OPEN item such as the artifact media type, the manifest filename or the capability contract
- WHEN the schema is reviewed
- THEN the position MUST be marked OPEN and MUST point to §58
- AND the schema MUST NOT constrain it to a fixed value

#### Scenario: Fixing a §58 item is rejected

- GIVEN a proposed constraint that would fix an OPEN item
- WHEN the schema is reviewed
- THEN the constraint MUST be rejected
- AND the item MUST remain OPEN

### Requirement: Schema-versus-semantic boundary

The strategy MUST publish exactly ONE authoritative layer→task matrix mapping each validation layer (structural, version, semantic, capability, filesystem) to the checks it owns, its scope (`IN`, `PARTIAL`, `OUT`), and the F2 task and owning phase. The `Owner` column of `schemas/README.md` MUST be consistent with that matrix; where they disagree the matrix prevails and the README MUST be corrected. Every check JSON Schema cannot express MUST be listed and handed to the named layer, and every deferred check MUST name its owning phase (F4/F7/F15). The schema MUST NOT claim to be a validator and MUST NOT rely on a schema to detect graph-level conditions.
(Previously: required a semantic-only list with an owning phase, but not a single authoritative layer→task matrix, leaving the README Owner column free to over-promise.)

#### Scenario: The boundary is documented

- GIVEN a check such as reference resolution, permission coverage or dependency-cycle detection
- WHEN the strategy is read
- THEN the check MUST appear in the semantic-only list
- AND it MUST name the phase that owns it

#### Scenario: One authoritative matrix wins

- GIVEN the layer→task matrix and the `schemas/README.md` Owner column disagree about a check's ownership
- WHEN the boundary is read
- THEN the matrix MUST prevail
- AND the README MUST be corrected to match it

#### Scenario: Scope and deferral are explicit

- GIVEN a check the change does not deliver (graph resolution, digest/immutability, trust, plan determinism)
- WHEN its matrix row is read
- THEN it MUST be marked `OUT` with its owning phase
- AND it MUST NOT be implied as delivered

### Requirement: Verification evidence and trust signals

Verification of this change MUST include at least one positive instance that validates and at least one negative instance that fails, and MUST state exactly what was checked. Documentation MUST NOT claim that the schemas are "100% safe" or that validation proves conformance.

#### Scenario: A negative example fails

- GIVEN an instance that violates a constraint
- WHEN it is validated against the schema
- THEN validation MUST fail

#### Scenario: Trust statement is precise

- GIVEN a statement about the schema layer
- WHEN it describes validation
- THEN it MUST name only the checks actually performed (parse, metaschema, example passes and failures)
- AND it MUST NOT claim safety or conformance beyond those checks

### Requirement: Artifact risk and default autonomy

The schema artifacts are risk class A (Passive) with default autonomy level 0 (Preview). The strategy MUST state that the schema layer performs no mutation and executes no third-party code.

#### Scenario: Risk and autonomy are stated

- GIVEN the schema-strategy artifact
- WHEN its risk posture is read
- THEN it MUST state risk class A and default autonomy 0
- AND it MUST state that no schema stage mutates target state

### Requirement: R3 dialect-and-closure portability CONFIRMED

The strategy MUST record that Draft 2020-12 with composition-aware closure (`unevaluatedProperties`) is portable to the Rust implementation language. The record MUST cite an executed offline spike with the `jsonschema 0.58.1` engine (with `referencing 0.58.1`) that reproduced the frozen corpus result — 7/7 positives valid, 16/16 negatives rejected, 0 keyword mismatches — identically to the L1/L2 cross-checks. This MUST replace the prior `r3=pending` state. The record MUST NOT be stated as a safety or conformance claim.

#### Scenario: R3 is recorded as confirmed

- GIVEN the strategy's R3 record
- WHEN it is read
- THEN it MUST state `confirmed` together with the engine, version and corpus result
- AND it MUST NOT state `pending`

#### Scenario: Confirmation is not a safety claim

- GIVEN the R3 confirmation
- WHEN it describes the result
- THEN it MUST name only the executed evidence (dialect, closure keyword, corpus counts)
- AND it MUST NOT claim "100% safe" or conformance
