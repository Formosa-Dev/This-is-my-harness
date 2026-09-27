# harness-cli Specification

> Capability `harness-cli` introduced by change `f2-validator-rust` (F2-05..F2-12), archived 2026-09-26. The `harness validate` command. Risk class A (Passive) · default autonomy 0 · READ-ONLY.
> The CLI is a thin surface over `harness-validator`; all validation semantics live in that library.
> Engram lineage: explore #4382 · proposal #4383 · spec #4385 · design #4384 · tasks #4386 · apply-progress #4387 · verify-report #4393.
> Archived delta: `openspec/changes/archive/2026-09-26-f2-validator-rust/specs/harness-cli/spec.md`.

## Purpose

`harness validate <path>` exposes the validator to humans, scripts and agents with a stable exit-code taxonomy and a machine-readable `--json` contract. It performs no mutation and executes no third-party code.

## Requirements

### Requirement: Command surface

`harness validate <path>` MUST accept exactly one positional path naming a file or a directory. `--json` MUST select machine output. `--kind <kind>` MAY disambiguate the document kind. An unknown flag or a missing path MUST be a usage error. The command MUST be read-only.

#### Scenario: File input

- GIVEN a path to a document file
- WHEN `harness validate <path>` runs
- THEN it MUST validate that document

#### Scenario: Directory input

- GIVEN a project directory
- WHEN `harness validate <dir>` runs
- THEN it MUST validate the single manifest it discovers
- AND full multi-artifact discovery MUST remain deferred to F4

#### Scenario: Usage error

- GIVEN a missing path or an unknown flag
- WHEN the command runs
- THEN it MUST report a usage error and exit 2

### Requirement: Document kind detection

The command MUST detect the document kind (manifest, model contract, or Install Plan) before validation. When the kind cannot be determined and no `--kind` override is given, it MUST fail with `document.unknown_kind` and MUST NOT guess.

#### Scenario: Undetectable kind

- GIVEN a file whose document kind cannot be determined
- WHEN `harness validate <file>` runs without `--kind`
- THEN it MUST report `document.unknown_kind`
- AND it MUST exit 2

#### Scenario: Kind override

- GIVEN a file whose name is ambiguous
- WHEN `harness validate --kind manifest <file>` runs
- THEN it MUST validate against the manifest schema

### Requirement: Exit-code taxonomy (Q9)

The command MUST return exactly one of: `0` when the document is valid; `1` when the document was evaluated and is invalid (at least one error diagnostic), including an unsupported `apiVersion` or an unknown capability; `2` when the document could not be evaluated — an unreadable path or IO error, unparsable or malformed bytes, duplicate keys, multiple documents in one stream, an undetectable kind, a broken schema registry, or a usage error.

#### Scenario: Valid

- GIVEN a valid document
- WHEN it is validated
- THEN the exit code MUST be 0

#### Scenario: Invalid

- GIVEN a document with at least one error diagnostic
- WHEN it is validated
- THEN the exit code MUST be 1

#### Scenario: Unsupported `apiVersion` is invalid, not un-evaluable

- GIVEN an `apiVersion` outside the supported set
- WHEN it is validated
- THEN the exit code MUST be 1
- AND the diagnostic code MUST be `version.unsupported`

#### Scenario: Not evaluable

- GIVEN an unreadable path or unparsable bytes
- WHEN it is validated
- THEN the exit code MUST be 2

### Requirement: Human output goes to stderr

Human-readable output MUST be written to stderr, grouped and path-first, and MUST NOT be written to stdout. When `--json` is set, human output MUST be suppressed.

#### Scenario: Stream split

- GIVEN `harness validate <path>` without `--json`
- WHEN it runs
- THEN human output MUST go to stderr and stdout MUST be empty

#### Scenario: `--json` suppresses human output

- GIVEN `harness validate --json <path>`
- WHEN it runs
- THEN stderr MUST NOT carry human report text

### Requirement: `--json` contract on stdout

With `--json`, the command MUST write exactly one JSON object to stdout of the shape `{status, errors:[{path, code, message}], warnings?}`. `status` MUST be one of `valid | invalid | error`. `errors` MUST hold every error diagnostic; `warnings` MAY hold not-evaluated notices. No diagnostic text MAY be emitted to stdout outside this object.

#### Scenario: Invalid shape

- GIVEN a document with one error
- WHEN `harness validate --json <path>` runs
- THEN stdout MUST parse as an object with `status` and a non-empty `errors` array of `{path, code, message}`

#### Scenario: Valid shape

- GIVEN a valid document
- WHEN `harness validate --json <path>` runs
- THEN `status` MUST be `valid` and `errors` MUST be an empty array

#### Scenario: Warnings do not break the shape

- GIVEN a document needing a deferred check
- WHEN `harness validate --json <path>` runs
- THEN `warnings` MAY be present as an array
- AND the `status`/`errors` shape MUST remain valid

### Requirement: Exit code and `status` agree

The process exit code MUST agree with the `--json` `status`: `valid`→0, `invalid`→1, `error`→2.

#### Scenario: Agreement

- GIVEN any `--json` run
- WHEN it completes
- THEN the exit code MUST match the `status` field

### Requirement: Read-only command

`harness validate` MUST NOT mutate the filesystem, MUST NOT execute third-party code, and MUST NOT make network requests.

#### Scenario: No mutation

- GIVEN a project directory
- WHEN `harness validate` runs against it
- THEN the directory contents MUST be unchanged

#### Scenario: Unsupported `apiVersion` exits nonzero and writes no file

- GIVEN a document declaring an unsupported `apiVersion`
- WHEN `harness validate` runs
- THEN it MUST exit non-zero without creating or modifying any file
