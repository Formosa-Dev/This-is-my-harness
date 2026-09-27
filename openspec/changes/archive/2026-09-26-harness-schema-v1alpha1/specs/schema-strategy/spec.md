# Delta for schema-strategy

## ADDED Requirements

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

The strategy MUST enumerate the checks that JSON Schema cannot express and MUST hand them to the validation phases. The schema MUST NOT claim to be a validator and MUST NOT rely on a schema to detect graph-level conditions.

#### Scenario: The boundary is documented

- GIVEN a check such as reference resolution, permission coverage or dependency-cycle detection
- WHEN the strategy is read
- THEN the check MUST appear in the semantic-only list
- AND it MUST name the phase that owns it

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
