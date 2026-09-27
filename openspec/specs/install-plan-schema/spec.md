# install-plan-schema Specification

> Capability `install-plan-schema` introduced by change `harness-schema-v1alpha1` (F2-01..F2-04), archived 2026-09-26.
> Normative source: `spec/**` (F1). Engram lineage: explore #4368 · proposal #4369 · spec #4372 · design #4370 · tasks #4373 · apply-progress #4375 · verify-report #4378.
> Archived delta: `openspec/changes/archive/2026-09-26-harness-schema-v1alpha1/specs/install-plan-schema/spec.md`.

## Requirements

### Requirement: The Install Plan enumerates all sixteen fields

The install-plan schema MUST require every field enumerated by `install-protocol/README.md` §4: runtime, runtimeVersion, project, scope, harness, files, conflicts, adaptations, unsupportedCapabilities, mcp, hooks, services, environmentVariableNames, risk, snapshot and verificationSteps. A plan missing any of the sixteen MUST fail validation.

#### Scenario: A complete plan validates

- GIVEN a plan declaring all sixteen fields
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A plan missing a field fails

- GIVEN a plan that omits one of the sixteen fields
- WHEN it is validated
- THEN it MUST fail

### Requirement: The plan is deterministic and serializable

The plan MUST serialize to a stable machine-readable form. The core plan MUST NOT contain non-deterministic fields such as a generation timestamp or a random identifier. Identical resolved inputs and an identical environment MUST be able to produce an identical plan.

#### Scenario: No timestamp in the core plan

- GIVEN a core plan instance
- WHEN its fields are inspected
- THEN it MUST NOT contain a wall-clock timestamp or another non-deterministic field

#### Scenario: A non-deterministic field is rejected

- GIVEN a plan carrying a generation timestamp
- WHEN it is reviewed against the determinism rule
- THEN it MUST be rejected or flagged as breaking determinism
- AND determinism enforcement itself MUST be listed as a semantic check

### Requirement: Environment-variable names only

`environmentVariableNames` MUST be an array of environment-variable names matching a name-only pattern. Values MUST NOT appear. An entry that carries a value MUST be rejected by the name-only pattern, and a value shaped like a name MUST be flagged by a semantic secret scan, never silently accepted.

#### Scenario: Valid names are accepted

- GIVEN an array of environment-variable names with no values
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A value is rejected

- GIVEN an entry such as `OPENAI_API_KEY=sk-...` that carries a value
- WHEN it is validated
- THEN it MUST fail the name-only pattern

#### Scenario: A name-shaped value is flagged

- GIVEN an entry that is shaped like a name but is actually a secret value
- WHEN the plan is scanned
- THEN it MUST be flagged by the semantic check
- AND it MUST NOT be silently accepted as a name

### Requirement: No local path leakage

The plan contains a local project path and MUST be treated as local-machine data that is never uploaded or transmitted. The schema MUST NOT require or introduce a remote destination for the plan.

#### Scenario: The project path is local

- GIVEN a plan produced for a local project
- WHEN it is serialized
- THEN the project path MUST remain local data
- AND no upload or transmission destination MUST be required

#### Scenario: An upload requirement is rejected

- GIVEN a proposed schema element that would require the plan to be uploaded
- WHEN the schema is reviewed
- THEN it MUST be rejected

### Requirement: Risk and required autonomy are recorded

`risk` MUST record the effective risk class (A–D) and the required autonomy level (0–3). The plan MUST name the autonomy level required by the operation, and no plan MAY declare an absolute bypass of an Elevated operation.

#### Scenario: Risk and autonomy are recorded

- GIVEN a plan declaring an effective risk class and the required autonomy level
- WHEN it is validated
- THEN it MUST pass when otherwise valid

#### Scenario: A sub-floor declaration is flagged

- GIVEN a plan whose declared autonomy is below the floor implied by its effective risk class
- WHEN it is validated
- THEN the floor violation MUST be reported by the semantic layer
- AND the plan MUST NOT proceed to Apply

### Requirement: Conflicts and unsupported capabilities are never resolved silently

`conflicts` MUST carry a reason and an explicit resolution value where a resolution exists; a conflict MUST NOT be resolved silently. `unsupportedCapabilities` MUST mark whether the capability is required and whether it blocks Apply, and a required unsupported capability MUST block Apply.

#### Scenario: A declared conflict validates

- GIVEN a conflict with a reason and an explicit resolution value
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A blocking condition stops Apply

- GIVEN a plan with an unsupported capability marked required and blocking
- WHEN the plan is evaluated
- THEN it MUST NOT proceed to Apply
- AND the blocking condition MUST be reported explicitly

### Requirement: The plan contains no executable third-party code

The plan MUST NOT contain or require the execution of third-party code. Hooks and scripts MUST be declared as component references, not as inline executable content.

#### Scenario: A hook is a reference

- GIVEN a plan that lists a hook as a component reference
- WHEN it is validated
- THEN it MUST pass

#### Scenario: Inline executable content is rejected

- GIVEN a plan carrying an inline command or script body
- WHEN it is validated
- THEN it MUST fail
- AND the executable content MUST be declared separately as a component subject to approval
