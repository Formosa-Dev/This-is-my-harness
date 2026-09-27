# harness-validator Specification

> Capability `harness-validator` introduced by change `f2-validator-rust` (F2-05..F2-12), archived 2026-09-26.
> Normative sources: `spec/**` (F1), `schemas/**`, `docs/adr/0001`, `docs/adr/0002`. Risk class A (Passive) · default autonomy 0.
> Engram lineage: explore #4382 · proposal #4383 · spec #4385 · design #4384 · tasks #4386 · apply-progress #4387 · verify-report #4393.
> Archived delta: `openspec/changes/archive/2026-09-26-f2-validator-rust/specs/harness-validator/spec.md`.
> All behaviors below are provable at library level, WITHOUT the CLI.

## Purpose

The `harness-validator` library evaluates one artifact document (manifest, model contract, or Install Plan) against the frozen schema set and the document-local semantic rules and returns a typed `Report`. It embeds the schemas, never touches the network, and never mutates the filesystem.

## Requirements

### Requirement: Offline schema resolution

The validator MUST embed `schemas/**` and the `$id`→path registry at build time and resolve every reference offline. A `$ref` whose `$id` is absent from the registry MUST fail loudly and MUST NOT be silently ignored. No validation MAY require network access.

#### Scenario: Offline resolution succeeds

- GIVEN no network access
- WHEN the validator resolves an instance that references a shared definition
- THEN the reference MUST resolve to the embedded definition
- AND the validator MUST NOT attempt a network fetch

#### Scenario: Unknown `$ref` fails loudly

- GIVEN a document referencing an `$id` absent from the registry
- WHEN the validator evaluates it
- THEN it MUST produce a diagnostic with code `schema.unresolved_ref` and the failing JSON Pointer
- AND it MUST NOT skip the reference

### Requirement: Fixed validation layers, no silent pass

The validator MUST apply five layers in a fixed order — structural, version, semantic, capability, filesystem — and MUST run every layer whose preconditions hold, aggregating all diagnostics instead of short-circuiting on the first error. A check the validator does not perform MUST be reported as not-evaluated (a warning naming the owning phase) and MUST NOT be reported as passing. The validator MUST NOT silently accept a check it does not perform.

#### Scenario: All layers run

- GIVEN a document with a structural error and a semantic error
- WHEN the validator evaluates it
- THEN the Report MUST contain diagnostics from both layers

#### Scenario: A deferred check is never a silent pass

- GIVEN a check deferred to a later phase (graph resolution, digest, trust)
- WHEN the validator evaluates a document that would need it
- THEN it MUST emit a not-evaluated warning naming the owning phase
- AND it MUST NOT report that check as satisfied

### Requirement: Typed, stable diagnostics

Every diagnostic MUST be `{path, code, message}` where `path` is a JSON Pointer and `code` is drawn from the validator's own stable namespace (for example `schema.required`, `schema.type`, `schema.const`, `parse.duplicate_key`, `document.unknown_kind`, `version.unsupported`, `capability.unknown`, `semantic.dependency_cycle`, `semantic.permission_coverage`, `path.traversal`). A `code` MUST NOT be a raw library string. `message` MUST NOT be treated as normative.

#### Scenario: A violation yields our code

- GIVEN a document missing a required field
- WHEN the validator evaluates it
- THEN it MUST emit a diagnostic with code `schema.required` and the empty JSON Pointer
- AND the code MUST NOT change when the underlying engine version changes

#### Scenario: Pointer compatibility with the frozen corpus

- GIVEN a pattern violation at `metadata.name`
- WHEN the validator reports it
- THEN `path` MUST be `/metadata/name`

### Requirement: Determinism

For the same input bytes and options the validator MUST return the same `Report`: identical diagnostics, identical `status`, and identical ordering. Diagnostic ordering MUST be total and defined: layer order, then JSON Pointer, then `code`.

#### Scenario: Same input, same output

- GIVEN one input document
- WHEN it is validated twice
- THEN the two Reports MUST be equal

#### Scenario: Stable ordering

- GIVEN several violations across layers
- WHEN they are reported
- THEN they MUST be ordered by layer, then path, then code

### Requirement: Read-only guarantee

Validation MUST NOT mutate the filesystem, MUST NOT execute target or third-party code, and MUST NOT open a network connection. The validator MUST NOT create, modify, move or delete any file as a side effect of validation.

#### Scenario: No filesystem mutation

- GIVEN a document under a project path
- WHEN it is validated
- THEN the observable filesystem state MUST be unchanged
- AND no file MUST be created, modified or deleted

#### Scenario: No execution and no network

- GIVEN a document containing hooks or commands
- WHEN it is validated
- THEN no such command MUST be executed and no connection MUST be opened

### Requirement: No F1 `MUST` is weakened

The validator MUST NOT accept an instance that an F1 `MUST` forbids and MUST NOT introduce a rule that permits, weakens or bypasses an F1 requirement. A proposed rule that would weaken a `MUST` MUST be rejected as a specification violation.

#### Scenario: A weakening rule is rejected

- GIVEN a validation rule that would accept an instance `spec/**` forbids
- WHEN the validator contract is reviewed
- THEN the rule MUST be rejected

### Requirement: Environment variable names only

`environmentVariableNames` MUST be validated as names. An entry containing a value separator (for example `TOKEN=secret`) MUST be rejected with a precise diagnostic at that entry's pointer. The validator MUST NOT echo, store or transmit any environment value.

#### Scenario: A value-shaped entry is rejected

- GIVEN an Install Plan whose `environmentVariableNames` entry contains `=`
- WHEN it is validated
- THEN it MUST be rejected with a diagnostic at that entry's pointer
- AND no environment value MUST appear in the Report

### Requirement: Capability known-set (F2-07)

The validator MUST resolve capability identifiers against a checked-in known-capability registry shipped with the schemas. An identifier absent from the registry MUST produce an explicit `capability.unknown` error. If the registry is missing or unreadable the validator MUST fail loudly and MUST NOT treat all identifiers as known. Which capabilities exist, and the Core-versus-extension distinction, remain OPEN under §58.

#### Scenario: Unknown capability is an explicit error

- GIVEN a capability identifier not present in the registry
- WHEN the validator evaluates the document
- THEN it MUST emit `capability.unknown` naming the identifier

#### Scenario: Missing registry fails loudly

- GIVEN no known-capability registry available
- WHEN the validator evaluates a document that declares a capability
- THEN it MUST fail loudly
- AND it MUST NOT accept the capability as known

### Requirement: Supported version gate (F2-11)

The validator MUST support exactly the set `{thisismyharness.dev/v1alpha1}` and MUST apply a static migration map whose only entry returns "no migration needed". A missing `apiVersion` MUST yield `version.missing`; a value outside the supported set MUST yield `version.unsupported` at `/apiVersion` naming the supported set. The map MUST NOT invent a generation, host, media type or migration algorithm; §58 stays OPEN.

#### Scenario: Supported version

- GIVEN `apiVersion: thisismyharness.dev/v1alpha1`
- WHEN it is evaluated
- THEN the version layer MUST accept it

#### Scenario: Unsupported version is invalid

- GIVEN an `apiVersion` outside the supported set
- WHEN it is evaluated
- THEN it MUST produce `version.unsupported` at `/apiVersion`
- AND `status` MUST be `invalid` (not a cannot-evaluate condition)

### Requirement: Declared-path safety

Paths declared in a document MUST be validated for traversal. An absolute path, a `..` segment escaping the provided project root, or a symlink escaping it MUST be rejected. The check MUST operate only on the provided root and MUST NOT follow links outside it.

#### Scenario: Traversal is rejected

- GIVEN a declared path containing `..` that escapes the project root
- WHEN it is validated
- THEN it MUST be rejected with `path.traversal`

#### Scenario: Absolute path or symlink escape is rejected

- GIVEN a declared absolute path or a symlink pointing outside the root
- WHEN it is validated
- THEN it MUST be rejected

### Requirement: Corpus reuse and evidence

The change MUST reuse the frozen corpus (7 positive, 16 negative) as its primary gate, relocated from the archive to a stable, non-archived home. Each positive MUST validate; each negative MUST be rejected with a diagnostic matching the corpus `{path, code}` expectation. Verification MUST state exactly what was checked and MUST NOT claim safety or conformance.

#### Scenario: Positive validates

- GIVEN a corpus positive entry
- WHEN the validator evaluates it
- THEN `status` MUST be `valid`

#### Scenario: Negative matches expectation

- GIVEN a corpus negative entry
- WHEN the validator evaluates it
- THEN `status` MUST be `invalid`
- AND at least one diagnostic MUST match the expected pointer and code

#### Scenario: Trust statement is precise

- GIVEN a statement about the validator
- WHEN it describes what "valid" means
- THEN it MUST name the layers actually run and the checks NOT performed
- AND it MUST NOT claim "100% safe" or conformance

### Requirement: Artifact risk and default autonomy

The validator is risk class A (Passive) with default autonomy level 0 (Preview). It MUST state that it performs no mutation and executes no third-party code.

#### Scenario: Risk and autonomy are stated

- GIVEN the validator artifact
- WHEN its risk posture is read
- THEN it MUST state risk class A and default autonomy 0
- AND it MUST state that no validation stage mutates target state

## Decisions (non-normative)

- **Q6 — capability-registry source (F2-07): RATIFIED.** The known-set source is a checked-in data file `schemas/capabilities.json` (default path; exact name may change in design), shipped and embedded with the schemas. It is consumed read-only; its contents are OPEN under §58. A missing registry is a loud failure, never an implicit "all known".
- **Q7 — YAML parser/version and duplicate keys: RATIFIED.** YAML **1.2** semantics; duplicate keys are **fail-closed** (`parse.duplicate_key`); no silent scalar coercion; a multi-document stream is rejected (`parse.multiple_documents`); cyclic aliases are rejected and alias expansion is bounded. Default parser crate: `serde-saphyr` (YAML 1.2); design MAY swap it for another maintained YAML 1.2 crate if it proves a blocker, which MUST be recorded.
- **Q9 — exit-code taxonomy: RATIFIED (see `harness-cli`).** A version outside the supported set is `invalid` (exit 1), not a cannot-evaluate condition (exit 2).
