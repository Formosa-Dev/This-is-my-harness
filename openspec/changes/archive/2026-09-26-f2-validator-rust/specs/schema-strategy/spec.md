# Delta for schema-strategy

> MODIFIED capability. Introduced by `harness-schema-v1alpha1` (F2-01..F2-04); modified by `f2-validator-rust` (F2-05..F2-12).
> Normative source: `spec/**` (F1). Main spec: `openspec/specs/schema-strategy/spec.md`.

## ADDED Requirements

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

## MODIFIED Requirements

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
