# harness-validator

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1` · **Risk class:** A (Passive) · **Default autonomy:** 0 (Preview) · **READ-ONLY**.
> **NON-NORMATIVE documentation.** Normative sources: `spec/**` (F1), `schemas/**`, `docs/adr/0001`, `docs/adr/0002`. This document describes what the validator checks and, just as importantly, what it does not. It weakens no F1 `MUST` and makes no safety or conformance claim.

`harness-validator` is the Rust adapter over the frozen JSON Schema set (ADR-0002). It embeds `schemas/**` and `schemas/registry.json` at build time, resolves every `$ref` offline, and evaluates one artifact document (a manifest, a model contract, or an Install Plan) through five layers, returning a typed `Report`. `harness validate` is the thin CLI over this library.

## What "valid" means

`valid` means **zero error diagnostics from the layers that ran**. It does not mean the document is safe, secure, or conformant.

The five layers run in a fixed order; every layer whose preconditions hold runs and aggregates its diagnostics (there is no short-circuit on the first error):

| Layer | What it checks | Module |
|---|---|---|
| structural | The document against its embedded root schema (Draft 2020-12; `unevaluatedProperties` closure under composition). | `src/structural.rs` |
| version | `apiVersion` is inside the supported set `{thisismyharness.dev/v1alpha1}`; a static no-op migration stub. Applies to manifests (the documents that declare `apiVersion`). | `src/version.rs` |
| semantic | Document-local rules: reference grammar and host consistency; ordered `extends`, kind composition and cycles; `environmentVariableNames` value-likeness; model `license` shape and conflict; permission coverage; effective risk; autonomy floor. | `src/semantic/` |
| capability | Every declared capability id exists in the checked-in registry `schemas/capabilities.json`. | `src/capability.rs` |
| filesystem | Declared paths do not escape the provided project root (absolute, `..`, or symlink). | `src/pathsafe.rs` |

A check the validator does **not** perform is reported as a `semantic.not_evaluated` **warning** naming the owning phase. It is never reported as passing, and a warning never changes the status.

The validator is a **document-local** checker: it never resolves a graph, never contacts a registry, never reaches the network, and never mutates the filesystem.

## Exit codes

| Exit | `status` | Condition |
|---|---|---|
| `0` | `valid` | No error diagnostics. |
| `1` | `invalid` | At least one error diagnostic from an evaluated layer, including an unsupported or missing `apiVersion` (Q9) or an unknown capability. The document was evaluated and failed. |
| `2` | `error` | The document could not be evaluated: an unreadable path or I/O error, unparsable bytes, duplicate keys, multiple documents in one stream, an undetectable or ambiguous kind, a broken capability registry, or a usage error. |

## `--json` contract

With `--json`, `harness validate` writes **exactly one** JSON object to stdout:

```json
{
  "status": "valid",
  "errors": [],
  "warnings": []
}
```

- `status` is one of `valid | invalid | error`.
- `errors` holds every error diagnostic as `{path, code, message}`; `path` is a JSON Pointer (`""` is the document root), identical to the ajv `instancePath` the frozen corpus records.
- `warnings` holds the `semantic.not_evaluated` notices.

Human output goes to **stderr** and stdout stays empty; `--json` suppresses human output. The process exit code always equals `status` (`valid` → 0, `invalid` → 1, `error` → 2). The shape is an experimental-but-stable contract, versioned with the crate. The `message` field is human-facing and never normative: scripts and agents key on `code` and `path`.

## Diagnostic `code` catalogue

Every code is drawn from the validator's own namespace; a raw library keyword never becomes a code. `message` is not part of the contract.

Codes emitted before any layer can run (cannot evaluate):

| Code | Meaning |
|---|---|
| `parse.invalid` | The bytes are not well-formed JSON or YAML 1.2. |
| `parse.duplicate_key` | A mapping declares the same key twice (fail-closed), or has a cyclic alias. |
| `parse.multiple_documents` | A YAML stream contains more than one document. |
| `document.unknown_kind` | No root-schema discriminator matched the document. |
| `document.kind_ambiguous` | More than one root-schema discriminator matched. |
| `io.read_failed` | The input path could not be read. |
| `usage.ambiguous_input` | Directory discovery found zero or more than one candidate manifest (full discovery is F4). |
| `capability.registry_unavailable` | The checked-in capability registry is missing or malformed: a loud failure, never "all known". |
| `schema.unresolved_ref` | A `$ref` could not be resolved offline. |

Structural codes (`schema.*`) are one per mapped JSON Schema keyword:

`schema.required`, `schema.type`, `schema.pattern`, `schema.enum`, `schema.const`, `schema.unevaluatedProperties`, `schema.additionalProperties`, `schema.unevaluatedItems`, `schema.additionalItems`, `schema.minItems`, `schema.maxItems`, `schema.uniqueItems`, `schema.contains`, `schema.minLength`, `schema.maxLength`, `schema.minimum`, `schema.maximum`, `schema.exclusiveMinimum`, `schema.exclusiveMaximum`, `schema.multipleOf`, `schema.minProperties`, `schema.maxProperties`, `schema.propertyNames`, `schema.anyOf`, `schema.oneOf`, `schema.not`, `schema.format`, `schema.contentEncoding`, `schema.contentMediaType`, `schema.falseSchema`. A keyword outside this map falls back to `schema.other`, so the namespace never leaks a library string.

Semantic codes (`semantic.*`):

| Code | Severity | Meaning |
|---|---|---|
| `semantic.identity_ref_short_forbidden` | error | A short (non-canonical) reference is forbidden. |
| `semantic.identity_host_mismatch` | error | A reference's host contradicts the document's host. |
| `semantic.kind_composition` | error | An illegal kind composition expressed through `extends`. |
| `semantic.dependency_cycle` | error | A dependency cycle (self-reference or within the known set). |
| `semantic.dependency_order` | error | The declared `extends` order is violated. |
| `semantic.permission_coverage` | error | A capability above Passive has no declared permission. |
| `semantic.effective_risk` | error | The effective risk (maximum over parts) is misdeclared. |
| `semantic.autonomy_below_floor` | error | The declared autonomy is below the floor for the risk class. |
| `semantic.env_value_like` | error | An `environmentVariableNames` entry looks like a value (`KEY=value`). The value is never echoed into the Report. |
| `semantic.license_shape` | error | A model `license` value is not identifier-shaped (documented heuristic, OPEN §58). |
| `semantic.license_conflict` | error | The model `license` contradicts the package `metadata.license` (checked only when both instances are present). |
| `semantic.not_evaluated` | warning | A deferred check was not performed; the message names the owning phase. Never a pass. |

Capability, version and path codes:

| Code | Meaning |
|---|---|
| `capability.unknown` | A declared capability id is absent from the registry; the message names the id. |
| `version.missing` | The document declares no `apiVersion`. |
| `version.unsupported` | `apiVersion` is outside the supported set; evaluated-and-invalid (exit 1, Q9). |
| `path.traversal` | A declared path escapes the project root with `..`. |
| `path.absolute` | A declared path is absolute. |
| `path.symlink_escape` | A declared path resolves, through a symlink, outside the project root. |

One issue yields one code: when the version gate emits `version.missing` or `version.unsupported`, the superseded structural `schema.required`/`schema.const` on `/apiVersion` is suppressed.

## Coverage: the authoritative layer to task matrix

This is the ONE authoritative layer to task matrix (the ratified Q5). The copy in `schemas/README.md` and this copy MUST stay identical; where any other table disagrees, this matrix prevails.

| Check (semantic-boundary row) | Layer | Scope | F2 task | Owning phase | Notes |
|---|---|---|---|---|---|
| All schema-expressible constraints (types, required, patterns, enums, consts, closure) | structural | IN | F2-05 | F2 | The JSON Schema layer; engine errors are mapped to the validator's own `schema.*` codes. |
| Supported `apiVersion` set and migration map | version | IN | F2-11 | F2 | Supported set `{thisismyharness.dev/v1alpha1}`; static no-op migration stub; §58 OPEN. |
| Unknown `kind` unless a compatible extension is present | document | PARTIAL | F2-06 | F2 (extension lookup → F4) | `kind` is a closed structural enum; extension-kind lookup needs a registry → F4. |
| Reference resolution (scoped refs, digest) | semantic (identity) | PARTIAL | F2-06 | F2 (digest/network → F4/F7) | Reference grammar, host consistency and short-ref-forbidden are checked; digest/network are not. |
| Dependency-graph analysis (cycles, conflicts, duplicate identity) | semantic (dependencies) | PARTIAL | F2-06 | F2 (full resolver → F4) | Ordered `extends`, kind composition and self/known-set cycles are checked; the full resolver is not. |
| Permission coverage for capabilities above Passive | semantic (permissions) | IN | F2-06 | F2 | Cross-section rule across `components` and `permissions`. |
| Effective risk, monotonicity and the autonomy floor | semantic (permissions) | IN | F2-05/06 | F2 | Effective = maximum over parts; autonomy ≥ floor(risk). |
| Capability existence against the extension registry | capability | IN | F2-07 | F2 | Resolved against the checked-in `schemas/capabilities.json`; unknown → `capability.unknown`. |
| Package discovery, path safety and symlink containment | filesystem | PARTIAL | F2-08 | F2 (whole-tree discovery → F4) | Declared-path traversal is checked now; whole-tree discovery and containment are F4. |
| Install-Plan determinism and binding to inputs | — | OUT | — | F4 | The plan document shape is structural; determinism is a generator property. |
| Trust-label precision and declared-vs-verified compatibility | — | OUT | — | F7/F15 | Requires conformance evidence. |
| `environmentVariableNames` are names, not values | semantic (components) | IN | F2-06/F2-12 | F2 | The structural `pattern` rejects `KEY=value`; a value-like heuristic adds `semantic.env_value_like`. |
| Digest / immutability semantics | — | OUT | — | F7 | Tag-to-digest and published immutability are registry behaviour. |
| Model `license` SPDX identifier shape | semantic (components) | IN | F2-12 | F2 | F1 (`package/component-types.md` §4.8) fixes no grammar; a documented heuristic, marked OPEN (§58). |
| Model `license` vs package `metadata.license` (Q7) | semantic (components) | PARTIAL | F2-10 | F2 | Checked only when both instances are provided; otherwise a not-evaluated warning, never guessed. |

## Deliberately not evaluated yet

These are named so that absence is never mistaken for a pass. Each emits a `semantic.not_evaluated` warning when a document needs it.

- Graph resolution across harnesses (the full `extends` resolver, digest resolution, duplicate identity across a registry) → **F4**.
- Install-Plan determinism and binding to inputs → **F4**.
- Digest and immutability semantics → **F7**.
- Trust-label precision and declared-vs-verified compatibility → **F7/F15**.
- Whole-tree package discovery and tree-wide symlink containment → **F4** (only the single discovered manifest and its declared paths are checked today).
- Extension-kind lookup against an extension registry → **F4**.
- Every decision-record §58 item (definitive manifest filename, media type, canonical host, capability Core-versus-extension, and so on) → **OPEN**; the validator resolves none of them.

## Install Plan is local-only

An Install Plan carries a local project path and is local-machine data. It MUST NOT be uploaded, transmitted or published by the validator or by any consumer; the validator only ever reads it. This restates the Verified Use rule (§48) and risk R6.

## Trust statement

The validator proves exactly this: that a document parses, that it satisfies the five layers above over the embedded, offline schema set, and that the frozen corpus passes and fails as declared. It does **not** prove that any harness is safe, secure or conformant, and it is not a security guarantee. It performs no mutation and executes no third-party code. The artifact is risk class A (Passive) with default autonomy level 0 (Preview).

## Running the validator

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

The CLI surface:

```text
harness validate <path>                    # file or directory; human output on stderr
harness validate --json <path>             # one JSON object on stdout
harness validate --kind manifest <path>    # override kind detection
```

On this development machine the toolchain is `stable-x86_64-pc-windows-gnu`, whose bundled `rust-mingw` ships `dlltool` but no GNU assembler (`as`). The workspace currently works around the resulting `raw-dylib` failure with exact transitive version pins; the environment itself must be fixed before F4 (Q13). See `Cargo.toml` and `rust-toolchain.toml`.

## Adding a fixture

The durable corpus lives in `packages/validator/tests/fixtures/`. `corpus.json` lists 7 positive and 16 negative entries with `{schema, file, expect}`; `positive/` holds instances that MUST validate and `negative/` holds instances that MUST be rejected with the expected `{path, keyword}`. To add a case, place the instance under `positive/` or `negative/`, add its entry to `corpus.json` with the `schema` `$id` and `file` (and, for a negative, its `expect {path, keyword}`), then run `cargo test -p harness-validator --test corpus`. Fixtures not listed in `corpus.json` (for example `negative/capability.unknown.json`) are driven directly by `tests/adversarial.rs` and do not change the 7/16 golden counts.

## Determinism

For the same input bytes and options the Report is identical on every run. Diagnostics are totally ordered: layer, then errors before warnings, then JSON Pointer, then `code`, then `message`.
