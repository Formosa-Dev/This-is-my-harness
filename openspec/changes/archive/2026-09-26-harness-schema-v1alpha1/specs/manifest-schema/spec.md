# Delta for manifest-schema

## ADDED Requirements

### Requirement: Four top-level fields, all REQUIRED

The manifest schema MUST require exactly the four top-level fields `apiVersion`, `kind`, `metadata` and `spec`, and MUST treat the root object as closed. A manifest that omits a REQUIRED field or adds an unknown top-level field MUST fail validation.

#### Scenario: A minimal valid manifest validates

- GIVEN a manifest with `apiVersion`, `kind`, `metadata.name`, `metadata.version` and an empty `spec`
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A missing REQUIRED field fails

- GIVEN a manifest without `spec`
- WHEN it is validated
- THEN it MUST fail

#### Scenario: An unknown top-level field is rejected

- GIVEN a manifest that adds a top-level field not defined by the schema
- WHEN it is validated
- THEN it MUST fail
- AND the validator MUST NOT ignore the field

#### Scenario: A wrong type is rejected

- GIVEN a manifest whose `metadata` is a string instead of an object
- WHEN it is validated
- THEN it MUST fail with a type error

### Requirement: `apiVersion` is a single supported spec version

`apiVersion` MUST be present, MUST be a string and MUST be a single supported spec version; it MUST NOT be a range.

#### Scenario: The supported version is accepted

- GIVEN `apiVersion: thisismyharness.dev/v1alpha1`
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A range or unknown version fails

- GIVEN an `apiVersion` that is a range or an unsupported value
- WHEN it is validated
- THEN it MUST fail
- AND a validator MUST reject a version it does not support

### Requirement: `kind` is a recognized artifact kind

`kind` MUST be present and MUST be one of the recognized artifact kinds (`Harness`, `Component`, `Preset`). An unrecognized `kind` MUST be rejected unless a compatible extension declaring it is present; that extension check is semantic.

#### Scenario: A recognized kind is accepted

- GIVEN `kind: Harness`
- WHEN it is validated
- THEN it MUST pass

#### Scenario: An unknown kind is rejected

- GIVEN a `kind` not recognized by the Core and no compatible extension
- WHEN it is validated
- THEN it MUST fail

### Requirement: `metadata` is a closed REQUIRED object

`metadata` MUST be an object and MUST require `name` and `version`. `name` MUST match the slug grammar and MUST NOT contain `/` or an owner segment. `version` MUST be valid SemVer. The remaining documented metadata fields are OPTIONAL; an unknown metadata field MUST be rejected.

#### Scenario: name and version are required

- GIVEN `metadata` without `version`
- WHEN it is validated
- THEN it MUST fail

#### Scenario: A malformed slug is rejected

- GIVEN a `metadata.name` containing `/`, an uppercase letter or a trailing hyphen
- WHEN it is validated
- THEN it MUST fail

#### Scenario: An invalid SemVer is rejected

- GIVEN a `metadata.version` that is not valid SemVer
- WHEN it is validated
- THEN it MUST fail

#### Scenario: An unknown metadata field is rejected

- GIVEN a `metadata` object with a property the schema does not define
- WHEN it is validated
- THEN it MUST fail

### Requirement: `spec` is a closed object with eight OPTIONAL sections

`spec` MUST be an object and MUST accept an empty object. Its eight sections (`profile`, `components`, `requirements`, `permissions`, `extends`, `distribution`, `compatibility`, `conformance`) MUST each be OPTIONAL. A present but malformed section MUST cause rejection and MUST NOT be ignored.

#### Scenario: An empty spec is valid

- GIVEN `spec: {}`
- WHEN it is validated
- THEN it MUST pass
- AND the resulting harness MUST be passive by construction

#### Scenario: A malformed optional section is rejected

- GIVEN a manifest where an optional section is present but ill-typed
- WHEN it is validated
- THEN it MUST fail
- AND the section MUST NOT be silently ignored

### Requirement: Single-entrypoint document shape

The schema MUST describe exactly one manifest document. It MUST NOT accept an aggregate or array of manifests, so that the single-entrypoint rule cannot be satisfied by validating a multi-manifest document. Enforcement of one canonical manifest per package is a validator responsibility.

#### Scenario: One document validates

- GIVEN a single manifest document
- WHEN it is validated
- THEN it MUST pass when otherwise valid

#### Scenario: An aggregate of manifests is rejected

- GIVEN a YAML document whose root is a list of manifests
- WHEN it is validated
- THEN it MUST fail
- AND the validator, not the schema, MUST own the package-level single-entrypoint check

### Requirement: `extends` shape and dependency-cycle boundary

`spec.extends` MUST be an ordered array of dependency references with an optional `override`. The schema MUST validate the reference shape. Cycle detection and conflict resolution MUST remain semantic checks owned by the validation phases; the schema MUST NOT be relied upon to detect cycles.

#### Scenario: An ordered extends array validates

- GIVEN `extends` as an ordered array of reference objects
- WHEN it is validated
- THEN it MUST pass

#### Scenario: A cyclic dependency is a semantic rejection

- GIVEN a manifest whose `extends` graph contains a cycle
- WHEN validation runs
- THEN the cycle MUST be rejected with a typed error by the semantic layer
- AND the schema MUST NOT be documented as the mechanism that detects the cycle

### Requirement: No F1 MUST is weakened

Every manifest-schema constraint MUST trace to an F1 anchor, and the schema MUST NOT accept an instance that `spec/**` forbids. Cross-section rules the schema cannot express (for example that every capability above Passive MUST be declared) MUST be listed as semantic checks, not silently dropped.

#### Scenario: A semantic-only rule is listed

- GIVEN a rule such as permission coverage for a capability above Passive
- WHEN the schema boundary is reviewed
- THEN the rule MUST appear as a semantic check
- AND the schema MUST NOT be described as enforcing it
