# ADR-0003: Rust validator architecture (layered, embedded, offline)

| Field | Value |
| --- | --- |
| **ADR number** | 0003 |
| **Title** | Rust validator architecture (layered, embedded, offline) |
| **Status** | Accepted |
| **Date** | 2026-09-26 |
| **Deciders** | Formosa.dev maintainers |
| **Related** | ADR-0001 (core language); ADR-0002 (schema strategy); change `f2-validator-rust` (F2-05..F2-12); Engram `sdd/f2-validator-rust/*` |

> This ADR records a decision about the implementation of the toolchain that reads the standard's machine-readable layer. It does not decide the standard's semantics: the normative source is `spec/**` (F1), and `schemas/**` derives from it.

---

## Context

ADR-0001 makes JSON Schema the single, language-neutral source of truth. ADR-0002 fixed the schema set as Draft 2020-12, split into modular `$defs` under a versioned `$id` namespace, resolved offline through a checked-in map. Neither ADR decided how the *validator* that consumes those schemas would be built, and until this change `packages/**` held no code: nothing validated an artifact, and **risk R3** (does the chosen Rust engine support Draft 2020-12 with `unevaluatedProperties` under composition?) blocked the schema freeze.

The constraints in force:

- The validator is an **adapter**: it MUST read `schemas/**` and never re-declare a shape already defined there (ADR-0001 / ADR-0002).
- Validation MUST run **offline** and **read-only**: no network, no filesystem mutation, no execution of third-party code (risk class A, default autonomy 0).
- A `$ref` whose `$id` is absent from the registry MUST fail **loudly**, never be silently skipped.
- A check the validator does not perform MUST surface as a not-evaluated warning, never as a silent pass.
- Diagnostics MUST be a **stable, own-namespace** contract, because the frozen corpus `{path, keyword}` expectations and future CI/agents depend on them.
- The workspace is greenfield; F4 will add a resolver/planner (`harness-core`) that must be able to depend on the validator **without a cycle**.

## Decision

We will implement the validator as a **layered, embedded, offline** Rust workspace.

Concretely:

1. **Layered pipeline.** Five layers run in the spec's fixed order — structural → version → semantic → capability → filesystem. Each layer owns its rules, aggregates its diagnostics, and never short-circuits the others.
2. **Separate `harness-validator` crate.** A decoupled library crate holds all validation logic; a thin `harness` binary (the `cli` crate) owns argument parsing, output formatting and exit codes. A placeholder `harness-core` is reserved for F4 and depends on the validator, not the reverse.
3. **Compile-time embedding.** `build.rs` reads `schemas/registry.json` and every mapped schema at build time and emits a generated `&[(&str $id, &str json)]` table via `include_str!`. `schemas/` stays the single source; no schema is copied or re-typed.
4. **Offline registry.** `jsonschema` is pinned with `default-features = false` (no HTTP/file resolvers) and used through a lazily built `jsonschema::Registry` (`.offline()`); an unknown `$id` fails at registry preparation.
5. **YAML 1.2, fail-closed.** A YAML 1.2 parser is used; duplicate keys fail closed, multiple documents are rejected, and implicitly typed scalars are not silently coerced.
6. **Typed code namespace.** Every diagnostic is `{path, code, message}` where `code` is drawn from the validator's own namespace (`schema.required`, `version.unsupported`, …), never a raw library string, and `path` is a JSON Pointer matching the frozen corpus.

## Rationale

- **Layered buys precise ownership and honest scope.** Each check has exactly one home, so the authoritative IN/PARTIAL/OUT matrix can be published and the validator can truthfully report a deferred check as a not-evaluated warning naming its owning phase. A single merged pass would blur which layer owns a failure and tempt the validator into implying checks it does not make.
- **A separate crate buys decoupling and testability.** The library is testable without the CLI, and F4's `harness-core` can depend on it with no cycle. The cost is one extra crate in the workspace.
- **Compile-time embedding buys hermetic behaviour.** The binary carries the schemas, so there is no runtime path where the embedded schema and `schemas/` can drift, and the build fails if a registry path is missing. It also keeps `schemas/` the single source of truth rather than generating types from it (which would invert ADR-0001).
- **An offline registry buys a loud failure mode.** With the resolvers compiled out, an unresolved `$ref` cannot be answered over the network; it fails at preparation, which is exactly the spec's "fail loudly, never skip" rule.
- **YAML 1.2 closes a latent divergence.** The L1 `ajv` engine parses YAML 1.2 while PyYAML is 1.1; pinning the validator to 1.2 removes the ambiguity by construction, and fail-closed duplicate keys remove the "last key wins" trap.
- **An own code namespace buys a stable contract.** Corpus expectations, the `--json` shape and future CI/agents key on `code` and `path`; a raw library keyword would churn whenever the engine version changes.
- **R3 is recorded as confirmed.** The `jsonschema 0.58.1` engine (with `referencing 0.58.1`), built `default-features = false` and used with `.offline()`, reproduced the frozen corpus offline: 7/7 positives valid, 16/16 negatives rejected, 0 keyword mismatches, including every `unevaluatedProperties` case. This is executed evidence, not a safety claim.

## Rejected alternatives

| Alternative | Description | Why rejected |
| --- | --- | --- |
| One merged check pass | Validate everything in a single traversal | Murkier diagnostics, no per-layer ownership and no honest way to mark scope IN/PARTIAL/OUT. The layered pipeline was chosen. |
| Validator as a module in `harness-core` | Put the validation logic inside the F4 planning crate | Couples validation to the planner, makes the library untestable in isolation, and risks a dependency cycle when `harness-core` grows. A separate crate was chosen. |
| Runtime disk load of schemas | Read `schemas/**` from disk at run time | Introduces a path where the loaded schema and the shipped `schemas/` can diverge, and makes the binary non-hermetic. Compile-time embedding was chosen. |
| Code-generated types from JSON Schema | Generate Rust structs from the schemas | Inverts ADR-0001: a language would become the authoritative type source. Codegen *from* JSON Schema stays allowed; embedding was chosen for validation. |
| Runtime `$ref` resolver | Resolve unknown `$id`s over the network or filesystem | Non-hermetic, non-reproducible and silently drift-prone. The offline `Registry` was chosen so an unknown `$ref` fails loudly. |
| YAML 1.1 (PyYAML parity) | Use a YAML 1.1 parser to match the L2 engine | Would reopen a latent divergence between engines and keep implicit-typing traps. YAML 1.2 was chosen. |
| Library keywords as codes | Emit the engine's raw keyword as the diagnostic `code` | Churns with the engine version and leaks a library string into a contract that CI, agents and the corpus depend on. The own namespace was chosen. |
| `boon`, `valico`, or a hand-rolled validator | Alternative Rust or bespoke engines | `valico` is Draft-07 era with no `unevaluatedProperties`; `boon` was unproven for our dialect; a hand-rolled validator would re-implement the dialect and add correctness risk. `jsonschema` 0.58.1 was proven against the actual corpus. |

## Consequences

### Positive

- Each check has one owner, so the coverage matrix is publishable and a deferral is explicit rather than an implied pass.
- The binary is hermetic and offline; an unknown `$ref` fails loudly instead of reaching the network.
- The library is independently testable and `harness-core` can build on it without a cycle.
- Diagnostics are a stable, versioned contract keyed on `path` and `code`.
- R3 is cleared with executed evidence, unblocking the schema freeze.

### Negative

- More modules and a resolver boundary to maintain than a single-file validator.
- Compile-time embedding means a schema change requires a rebuild (acceptable for a validator binary).
- The `windows-gnu` toolchain on this machine ships no GNU assembler (`as`), so `raw-dylib` crates fail; exact transitive pins are a documented **stopgap** until the environment is fixed before F4 (Q13).

### Neutral / follow-up

- `harness-core` (F4) will add resolver/planning/session/hashing/path-safety; it is a placeholder today.
- The JS/Python gates (`scripts/schemas/validate.mjs`, `scripts/schemas/cross_check.py`) remain as independent cross-checks; `cargo test` is the primary gate.
- The `--json` shape is published as experimental-but-stable and versioned with the crate (Q10).
- Every §58 item stays OPEN; the validator resolves none of them.

## Status of related decisions

This decision is reversible: the validator is additive, replaces no existing behaviour, and its schema dependency is declarative, so a successor ADR can supersede it and the crate can be restructured. Risk **R3** is **confirmed** (see ADR-0002 and `schemas/README.md`): the dialect and composition-aware closure were executed against the Rust implementation language and reproduced the corpus offline. The separate environment prerequisite — the missing GNU `as` — is recorded separately as Q13 and is NOT part of R3. Revision is triggered by a change to the layering, the crate boundary, the embedding strategy, the offline registry, the YAML dialect, or the code namespace.

## References

- `spec/**` — normative source (F1), in particular `spec/manifest/README.md`, `spec/core/permissions.md`, `spec/core/risk-classes.md`, `spec/core/dependencies.md`, `spec/core/versioning.md`, `spec/install-protocol/README.md`.
- `docs/adr/0001-core-language.md` — language neutrality and JSON Schema as the single source of truth.
- `docs/adr/0002-schema-strategy.md` — draft, `$id` namespace, modular `$defs`, offline resolver; the R3 record.
- `schemas/README.md` — strategy, `$id` map, resolver map, the authoritative layer-to-task matrix, the R3 record.
- `packages/validator/README.md` — what the validator checks, the exit codes, the `--json` shape and the full `code` catalogue.
- `openspec/changes/f2-validator-rust/` — proposal, exploration (the executed R3 spike), design §9/§10, tasks.
