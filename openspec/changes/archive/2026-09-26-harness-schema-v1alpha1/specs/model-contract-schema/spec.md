# Delta for model-contract-schema

## ADDED Requirements

### Requirement: A model contract is more than a name

The model-contract schema MUST require the contract elements of `component-types.md` §4 / decision record §8: input contract, output contract, capabilities, resource contract, execution location, lifecycle, fallback, version and artifact source. The model `license` element is declared as an OPTIONAL field, not a REQUIRED one: whether a model license duplicates the package license declared once in `metadata.license` is left OPEN by decision record §58 / Q7, so the schema retains the field and does not require it (see "Version and license").

#### Scenario: A complete model contract validates

- GIVEN a model contract declaring every required element
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A name-only model is rejected

- GIVEN a model declaration that provides only a name
- WHEN it is validated
- THEN it MUST fail

### Requirement: Input and output contracts reuse JSON Schema

The input and output contracts MUST be expressible as embedded JSON Schema fragments with an optional type label. The schema MUST NOT invent a competing contract DSL. Because the exact model capability contract is OPEN under §58, the internals of input and output MUST remain permissive and MUST be marked OPEN.

#### Scenario: The canonical example validates

- GIVEN `input: state` and `output: { route, confidence }`
- WHEN it is validated
- THEN it MUST pass

#### Scenario: OPEN internals stay permissive

- GIVEN an input or output contract with implementation detail the Core does not define
- WHEN it is validated
- THEN it MUST pass
- AND the schema MUST mark that position OPEN and MUST NOT constrain it

### Requirement: Capabilities are contracts, not vendors

`capabilities` MUST be an array of capability requirements shaped as `{ capability, preferred?, alternatives? }`. The schema MUST NOT contain a closed enum of capability identifiers, and MUST NOT name a vendor in any `const` or `enum`.

#### Scenario: A capability requirement validates

- GIVEN a capability requirement with a capability identifier and an optional preferred implementation
- WHEN it is validated
- THEN it MUST pass

#### Scenario: An unknown capability is a registry concern

- GIVEN a capability identifier whose existence is not known to the Core
- WHEN the contract is validated
- THEN the shape MUST be accepted
- AND capability existence MUST be delegated to the capability registry, not enforced by a closed enum

#### Scenario: A vendor constant is rejected

- GIVEN a proposed schema `const` or `enum` containing a vendor name
- WHEN the schema is reviewed
- THEN it MUST be rejected as a violation of Core vendor neutrality

### Requirement: Resource contract is shared, not duplicated

The model resource contract MUST reference the single shared hardware definition and MUST NOT redefine resource fields.

#### Scenario: Resources resolve to the shared definition

- GIVEN a model contract declaring resource requirements
- WHEN it is validated
- THEN it MUST resolve the shared hardware definition by `$id`
- AND the resource fields MUST NOT be redefined in the model-contract schema

### Requirement: Execution location is local or remote

`executionLocation` MUST be one of `local` or `remote`.

#### Scenario: A recognized location is accepted

- GIVEN `executionLocation: remote`
- WHEN it is validated
- THEN it MUST pass

#### Scenario: An unrecognized location is rejected

- GIVEN an `executionLocation` outside `local` and `remote`
- WHEN it is validated
- THEN it MUST fail

### Requirement: Lifecycle is complete when the model runs as a service

When a model runs as a service, its lifecycle MUST declare install, start, health-check, stop, update and remove, reusing the Service lifecycle shape. A partial lifecycle MUST be rejected.

#### Scenario: A complete service lifecycle validates

- GIVEN a service-backed model declaring all six lifecycle actions
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A partial lifecycle is rejected

- GIVEN a service-backed model missing a lifecycle action
- WHEN it is validated
- THEN it MUST fail

### Requirement: Fallback is declared

`fallback` MUST be expressible, covering an alternative implementation and/or the escalation or failure behavior when the model or its host is unavailable.

#### Scenario: A fallback is accepted

- GIVEN a model declaring an alternative implementation or an escalation behavior
- WHEN it is validated
- THEN it MUST pass

### Requirement: Version and license

A model contract MUST declare `version` (valid SemVer). `license` is OPTIONAL: the schema retains a model `license` field carrying an SPDX identifier where one exists, but whether a model license duplicates the package license declared once in `metadata.license` is OPEN (decision record §58 / Q7) and this schema does not decide that policy. Because F1 (`component-types.md` §4.8) fixes no SPDX grammar, the identifier shape is a semantic check rather than a schema pattern (listed on the semantic boundary). The package license is declared once according to `distribution.md` §7; a model-level license MUST NOT contradict the package license, and the consistency rule MUST be stated rather than left silent.

#### Scenario: Version and license validate

- GIVEN a model declaring a SemVer version and an SPDX-shaped license
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A contradictory license is flagged

- GIVEN a model-level license that contradicts the package `metadata.license`
- WHEN validation runs
- THEN the contradiction MUST be reported by the semantic layer
- AND it MUST NOT be silently accepted

### Requirement: Artifact source is data, not schema constant

`artifactSource` MUST be an object carrying a provider, a digest and a reference. A provider name MAY appear in an instance as data, but the schema MUST NOT fix any provider and MUST NOT constrain the field to a specific value.

#### Scenario: An artifact source validates

- GIVEN an artifact source with a provider string, a digest and a reference
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A fixed provider in the schema is rejected

- GIVEN a schema that fixes the provider to a single value
- WHEN the schema is reviewed
- THEN it MUST be rejected

### Requirement: Router is a Model specialization

The model-contract schema MUST support a Router as a Model specialization with an optional router block declaring typed outputs, thresholds and fallback or escalation. The schema MUST NOT reference the component descriptor, to preserve the no-cycle rule.

#### Scenario: A router block validates

- GIVEN a router contract declaring typed outputs, thresholds and a fallback
- WHEN it is validated
- THEN it MUST pass

#### Scenario: No cycle with the component descriptor

- GIVEN the model-contract schema and the component descriptor
- WHEN the reference graph is inspected
- THEN the model-contract schema MUST NOT reference the component descriptor
