# ADR-0002: Schema strategy for the harness contract (dialect, `$id`, modular `$defs`, resolver)

| Field | Value |
| --- | --- |
| **ADR number** | 0002 |
| **Title** | Schema strategy for the harness contract (dialect, `$id`, modular `$defs`, resolver) |
| **Status** | Accepted |
| **Date** | 2026-09-26 |
| **Deciders** | Formosa.dev maintainers |
| **Related** | ADR-0001 (core language); decision record §58 (open items); change `harness-schema-v1alpha1` (F2-01..F2-04); Engram `sdd/harness-schema-v1alpha1/*` |

> This ADR records a decision about the implementation of the standard's machine-readable layer. It does not decide the standard's semantics: the normative source is `spec/**` (F1), and `schemas/**` derives from it. The full derivation and the F1→constraint traceability matrix live in `schemas/README.md`.

---

## Context

Phase F1 fixed the harness semantics in prose (`spec/**`) but shipped no machine-readable contract: `schemas/` contained only `.gitkeep`. Phase F2 must derive a JSON Schema set from F1 without weakening a single `MUST`, while leaving every item that decision record §58 keeps OPEN explicitly unresolved.

The constraints in force:

- `spec/manifest/README.md` §5.3 requires a validator to reject an unknown field in a REQUIRED object, so closed objects are mandatory.
- The schema set composes shared concepts through `$ref`/`allOf`, so a same-level-only closure keyword cannot work.
- Consumers span languages and runtimes: the Rust validator (F2-05..F2-12), a future SDK and a future web layer must all resolve the same definition.
- Validation must be possible offline and on this machine, where Rust, `ajv` and Python `jsonschema` were originally absent and no test runner exists (`strict_tdd: false`).
- ADR-0001 makes JSON Schema the single source of truth; no language may become the authoritative type source.
- §58 open items (media type, canonical host, manifest filename, model capability internals, capability registry, policy/workflow shape, secrets, profile versioning) MUST NOT be resolved by accident.

## Decision

We will ship the harness contract as JSON Schema **Draft 2020-12**, split into **modular shared `$defs`** under a **versioned `$id` namespace**, referenced by **absolute `$id`** and resolved offline through a **checked-in resolver map**.

Concretely:

1. **Dialect.** `"$schema": "https://json-schema.org/draft/2020-12/schema"`, with **`unevaluatedProperties: false`** on every closed REQUIRED object.
2. **`$id` namespace.** Base `https://thisismyharness.dev/schemas/v1alpha1/`; the generation lives in the `$id` path and filenames stay flat. The host is a *schema-identity namespace*, explicitly not the §58-open harness-resolution host.
3. **Modular `$defs`.** Seven definition files under `schemas/defs/` (`identity`, `capability`, `requirement`, `permission`, `component`, `dependency`, `distribution`), each shared concept defined exactly once, plus three root schemas (`harness.v1alpha1.schema.json`, `model-contract.schema.json`, `install-plan.schema.json`).
4. **Absolute refs + resolver.** Every cross-file `$ref` uses the absolute `$id`; `schemas/registry.json` maps every `$id` to its repo path so any consumer resolves the same URI to the same file with no network.
5. **Structural guards.** `model-contract` MUST NOT reference the component descriptor (one-way edge), and `install-plan` references leaf `defs/` schemas only and never the manifest root.
6. **§58 stays OPEN.** Every sensitive position carries an explicit `OPEN` marker naming §58 and is deliberately permissive.

## Rationale

- **`unevaluatedProperties` is the only closure keyword that survives composition.** The manifest root, `metadata`, `spec` and the other closed objects are assembled from shared `$ref`s; `additionalProperties: false` sees only properties declared at the same level and would break under `allOf`, while `unevaluatedProperties: false` evaluates after every contribution. This directly satisfies `spec/manifest/README.md` §5.3.
- **Modular `$defs` keeps a shared concept single-sourced.** The manifest, the model contract and the Install Plan all reuse the same identity, capability, requirement, permission, component, dependency and distribution definitions, so a Rust consumer, an SDK and a web consumer cannot drift; it also lets consumers pull only what they need and bumps independently.
- **Absolute `$id` + resolver map buys one canonical URI.** Online consumers fetch the `$id`; offline consumers resolve the checked-in map. Relative-only refs are hermetic but couple the schemas to the directory layout and give no canonical-URI story.
- **Versioning in the `$id` path defers re-pathing.** A future `v1beta1` adds parallel files; the alpha files stay addressable. This is additive within a generation, matching `spec/VERSIONING.md`.
- **Leaving §58 OPEN avoids freezing by accident.** Hardcoding a media type, host or filename would silently turn an open decision into a normative one, which `spec/STYLE.md` §3.5 forbids.
- **Costs.** Modular files mean more `$ref` hops and a resolver map to keep in sync; 2020-12 is a newer dialect than Draft-07, so tooling support must be confirmed (risk R3, below).

## Rejected alternatives

| Alternative | Description | Why rejected |
| --- | --- | --- |
| A1 Monolithic schema | One file with every definition inlined | Couples manifest/model/plan versioning, forces one artifact bump for any change, and prevents per-consumer pulls. Modular defs chosen instead. |
| A2 Draft-07 | Previous stable dialect | Has no `unevaluatedProperties`; cannot express closed REQUIRED objects composed from shared refs without inlining every property (`spec/manifest/README.md` §5.3). |
| A3 OpenAPI 3.x Schema Object | HTTP-API-flavoured schema subset | Weaker as a general document validator and tied to an API-description model the standard does not use. |
| A4 JSON Type Definition (RFC 8927) | Simpler schema language | Cannot express the conditional composition and cross-file `$defs` the extension model needs. |
| A5 Code-first types | Rust structs or TypeScript types as the source, JSON Schema generated | Makes a language the source of truth, contradicting ADR-0001 and the "JSON Schema is the single source of truth" rule. Codegen *from* JSON Schema stays allowed. |
| A6 Pattern `apiVersion` on the Core branch | Accept any version-shaped string | Silently accepts unsupported generations instead of failing closed. The Core branch uses `const`; extension schemas share the `$defs` instead. |
| A7 Versioned directory layout | `schemas/v1alpha1/…` files instead of flat, versioned `$id`s | F2 pins the filenames as evidence anchors; the `$id` path already carries the generation, so a directory adds churn without benefit. |
| A8 Relative `$ref` only | `./defs/….schema.json` references | Hermetic but couples schemas to layout and removes the canonical-URI story web consumers need. |
| A9 Install Plan beyond the sixteen fields | Add plan-level envelope fields by default | `spec/install-protocol/README.md` §4 requires the sixteen fields to be enumerated; extras must be deliberate, never default. |
| A10 Separate `router.schema.json` | Split the Router contract into its own file | F2-03 names a single `model-contract.schema.json`; a Router is a Model specialization, so one file with an optional `router` block preserves the evidence path. |
| A11 Capability registry in-schema | Enumerate capability identifiers as an `enum` | Capability existence is registry work (F2-07); the schema validates shape only and must not close the identifier set. |
| A12 Resolve §58 now | Fix the media type, host and manifest filename in-schema | Prohibited: `spec/STYLE.md` §3.5 says open items MUST NOT be treated as normative. Every touched item stays OPEN and marked. |

## Consequences

**Positive**

- Unknown fields in REQUIRED objects fail loudly under composition, as `spec/manifest/README.md` §5.3 demands.
- Every shared concept has exactly one definition, resolved by URI across Rust, SDK and web consumers.
- Offline validation works from a checked-in map, so CI and air-gapped use are hermetic.
- A generation bump (`v1beta1`) is additive and never re-paths the alpha files.
- §58 open items are visibly open, not silently frozen.

**Negative**

- More files and `$ref` hops than a monolith, and `registry.json` must be updated whenever a schema is added.
- Draft 2020-12 is newer, so validator dialect support must be confirmed before freeze (risk R3).
- Composed `allOf`/`unevaluatedProperties` can yield less direct diagnostics until the validator owns typed errors (F2-05, risk R10).

**Neutral / follow-up**

- The Rust validator (F2-05..F2-12) replaces the interim L1/L2 checks with `cargo test` and becomes the diagnostic contract.
- The small corpus moves to the F3 fixtures collection.
- Shared `defs/` changes carry Core-spec review weight; within a generation evolution is additive-only.

## Status of related decisions

This decision is reversible: the schema set is declarative, additive and has no runtime consumer yet, so a successor ADR can supersede this one and the files can be re-generated. It is **no longer blocked** on risk **R3**, which is now **confirmed**: the `jsonschema 0.58.1` engine (with `referencing 0.58.1`), built `default-features = false` and used with `.offline()`, resolved this schema set's absolute `$id`s offline and reproduced the frozen corpus exactly — 7/7 positives valid, 16/16 negatives rejected, 0 keyword mismatches — matching the independent L1 (`ajv` v8) and L2 (Python `jsonschema` + `referencing`) engines. This records the executed evidence; it makes no safety or conformance claim. The freeze gate state is `r3=confirmed`, recorded here and in `schemas/README.md`. The missing GNU assembler (`as`) on the `windows-gnu` toolchain is a **separate, explicitly-open environment prerequisite** (Q13), **not part of R3**. Revision is triggered by a change to the dialect, the `$id` scheme, the resolver strategy, or the no-cycle/leaf guards, and by any schema constraint that review finds weaker than the F1 anchor it cites.

## References

- `spec/**` — normative source (F1), in particular `spec/manifest/README.md` §2–§5, `spec/core/identity.md` §2/§8, `spec/package/component-types.md` §4–§5, `spec/install-protocol/README.md` §4/§7.
- `docs/adr/0001-core-language.md` — language neutrality and JSON Schema as the single source of truth.
- `docs/adr/0000-template.md` — the ADR format followed here.
- `schemas/README.md` — strategy, `$id` map, resolver map, strictness, §58 table, F1 traceability matrix, trust statement, R3 record.
- `openspec/changes/harness-schema-v1alpha1/` — exploration §13 (alternatives A1–A12), design §3/§4/§8, tasks Phase 5.
- `schemas/registry.json` — the offline `$id` → repo path resolver map.
