# Design: f2-validator-rust (F2-05..F2-12)

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1` · **Surface:** Toolchain (`packages/**`) · **Risk class:** A (Passive) · **Autonomy:** 0 (Preview) · **Language:** English.
> **NON-NORMATIVE.** Normative sources: `spec/**` (F1), `schemas/**`, `docs/adr/0001`, `docs/adr/0002`. This design MUST NOT weaken any F1 `MUST`.
> **R3:** recorded CONFIRMED from the executed spike (`jsonschema 0.58.1`, 7/7 valid, 16/16 rejected, 0 keyword mismatch, offline).

## Technical Approach

Ports-and-adapters: the JSON Schema set (ADR-0002) is the **port**; this change adds the Rust **adapter** that resolves the same `$id`s offline, plus a read-only CLI. A root Cargo workspace holds a decoupled `harness-validator` library, a thin `harness` binary and a placeholder `harness-core` (reserved for F4). Validation is a five-layer pipeline over one embedded schema registry, emitting typed diagnostics that match the corpus' golden `{path, keyword}` expectations. Nothing mutates the filesystem; no network is reachable from the validator.

## 1. Cargo workspace layout

| Path | Crate | Kind | Owns |
|---|---|---|---|
| `Cargo.toml` | workspace | — | `members = ["packages/*"]`, `[workspace.dependencies]`, `resolver = "2"` |
| `Cargo.lock` | — | committed | Binary workspace reproducibility |
| `rust-toolchain.toml` | — | committed | `channel = "stable"`; documents the `as` prerequisite |
| `packages/core` | `harness-core` | lib (placeholder) | Reserved: resolver/planning/session/hashing/path-safety (F4+). Depends on nothing yet. |
| `packages/validator` | `harness-validator` | lib | Schema embedding, the five layers, diagnostics, `Report`. |
| `packages/cli` | `harness` | bin | Argument parsing, output formatting, exit codes. Depends on `harness-validator`. |
| `packages/sdk` | — | reserved | No Rust; `.gitkeep` only. |

Directions: `harness → harness-validator`; `harness-core → harness-validator` (future); **no cycles**; `harness-validator` depends on no workspace crate. Shared pins live only in `[workspace.dependencies]`.

## 2. Module map (`harness-validator`)

| Module | Responsibility |
|---|---|
| `src/lib.rs` | Public `validate(&Source, &Options) -> Report`; the only public entry; pure (no process exit, no stdout). |
| `src/diagnostics.rs` | `Diagnostic { path, code, message }`, `Severity`, `Code`, `Report { status, errors, warnings }`; deterministic sort. |
| `src/registry.rs` | Embedded schema table + one lazily-built `jsonschema::Registry` (offline). |
| `src/parse.rs` | bytes → `serde_json::Value` (JSON/YAML); duplicate-key fail-closed; multi-doc rejection; BOM rule. |
| `src/document.rs` | Kind detection + `apiVersion` read + root-schema selection. |
| `src/structural.rs` | L1 — JSON Schema layer; maps `jsonschema` errors → `{path, code}`. |
| `src/semantic/mod.rs` | Orchestrates the document-local rules and short-circuit policy. |
| `src/semantic/{identity,dependencies,components,permissions}.rs` | Cross-section rules (see §4). |
| `src/capability.rs` | capability layer — known-capability lookup (embedded registry, F2-07). |
| `src/version.rs` | version layer — supported `apiVersion` set + migration stub (F2-11). |
| `src/pathsafe.rs` | L5 — declared-path safety (`..`, absolute, symlink escape). **Added** to the exploration map because L-filesystem needs a home. |
| `tests/corpus.rs` | Drives the promoted fixture corpus (primary gate). |
| `tests/{adversarial.rs,cli.rs}` | Adversarial inputs; `assert_cmd` CLI smoke + golden `--json`. |

## 3. Schema embedding + offline resolution

**Choice:** `packages/validator/build.rs` reads `schemas/registry.json` and every mapped `schemas/**` file at **compile time**, emitting a generated `&[(&str $id, &str json)]` table via `include_str!` (no `include_dir`, no `include`-macro dependency). `registry.rs` builds `jsonschema::Registry::new()`, `.add($id, schema)?` for all 10 ids, `.prepare()?` **once** (`OnceLock`), and validates each document from a wrapper `{ "$ref": $id }` via `draft202012::options().with_registry(&registry).offline().build(&wrapper)`.

**Offline guarantee:** `jsonschema` is pinned `default-features = false` (no `resolve-http`/`resolve-file`), so a `$ref` whose `$id` is absent from the registry **fails at registry `.prepare()`** — loudly, at startup/first use, never over the network. `build.rs` additionally fails the build if a registry path is missing from disk. `schemas/` stays the single source (ADR-0002); no schema is copied.

## 4. Validation layers — authoritative IN/PARTIAL/OUT matrix

Layer order (spec-normative): **structural → version → semantic → capability → filesystem**. Every `schemas/README.md` semantic-boundary row maps to **exactly one** layer/task:

| # | Semantic-boundary row | Layer | Task | Scope | Notes |
|---|---|---|---|---|---|
| 1 | Supported `apiVersion` set + migration map | version | F2-11 | **IN** | supported `{thisismyharness.dev/v1alpha1}`; static `Migration::None` stub; §58 OPEN |
| 2 | Unknown `kind` unless compatible extension present | document | F2-06 | **PARTIAL** | v1alpha1 `kind` is a closed structural enum; extension-kind lookup needs a registry → F4 |
| 3 | Reference resolution (scoped refs, digest) | semantic (identity) | F2-06 | **PARTIAL** | ref grammar + host consistency + short-ref-forbidden now; digest/network → F4/F7 |
| 4 | Dependency-graph analysis (cycles, conflicts, duplicate identity) | semantic (dependencies) | F2-06 | **PARTIAL** | ordered `extends`, kind-composition, self/known-set cycle now; full resolver → F4 |
| 5 | Permission coverage above Passive | semantic (permissions) | F2-06 | **IN** | every non-Passive component/permission declared |
| 6 | Effective risk, monotonicity, autonomy floor | semantic (permissions) | F2-05/06 | **IN** | effective = max over parts; autonomy ≥ floor(risk) |
| 7 | Capability existence vs extension registry | capability | F2-07 | **IN** | needs known-capability source (Q6); unknown → explicit code |
| 8 | Package discovery, path safety, symlink containment | filesystem | F2-08 | **PARTIAL** | declared-path traversal now; tree discovery/symlink escape on a real tree → F4 |
| 9 | Install-Plan determinism + binding to inputs | — | — | **OUT → F4** | document *shape* is L1; determinism is a generator property |
| 10 | Trust-label precision / declared-vs-verified | — | — | **OUT → F7/F15** | needs conformance evidence |
| 11 | `environmentVariableNames` names, not values | semantic (components) | F2-06/F2-12 | **IN** | structural `pattern` rejects `KEY=value`; add value-like heuristic |
| 12 | Digest / immutability semantics | — | — | **OUT → F7** | registry behaviour |
| 13 | Model `license` SPDX identifier shape | semantic (components) | F2-12 | **IN** | F1 fixes no grammar → documented heuristic, marked OPEN (Q7/§58) |
| 14 | Model `license` vs package `metadata.license` (Q7) | semantic (components) | F2-10 | **PARTIAL** | checked when both instances provided; else `semantic.not_evaluated` warning, never guessed |

This matrix **supersedes** the `schemas/README.md` "Owner" column, whose task numbers contradict `build-progress/F2` (e.g. it assigns path-safety to F2-08, the CLI task). Apply publishes this one matrix in `schemas/README.md` + `packages/validator/README.md`.

**Out of scope (named, never silent):** L9/L10/L12 above plus adapters, `harness use`, `packages/sdk`, and any §58 resolution.

## 5. Diagnostics contract

`Diagnostic { path: String, code: String, message: String }`. Our own `code` namespace (never a raw library keyword); `path` is a JSON Pointer identical to ajv `instancePath` (`""` for the document root), so the archived corpus `expect` blocks stay golden.

| Prefix | Codes |
|---|---|
| `parse.` | `invalid`, `duplicate_key`, `multiple_documents` |
| `document.` | `unknown_kind`, `kind_ambiguous` |
| `schema.` | `unresolved_ref` (an unresolvable offline `$ref`), plus one per mapped JSON Schema keyword: `required`, `type`, `pattern`, `enum`, `const`, `unevaluatedProperties`, `minItems`, `anyOf`, `oneOf`, `format`, `minimum`, `maximum` |
| `semantic.` | `identity_ref_short_forbidden`, `identity_host_mismatch`, `kind_composition`, `dependency_cycle`, `dependency_order`, `permission_coverage`, `effective_risk`, `autonomy_below_floor`, `env_value_like`, `license_shape`, `license_conflict`, `not_evaluated` (warning) |
| `capability.` | `unknown` |
| `version.` | `missing`, `unsupported` |
| `path.` | `traversal`, `absolute`, `symlink_escape` |
| `io.` / `usage.` | `read_failed`, `ambiguous_input` |

**Ordering (deterministic):** errors before warnings, then `path` (lexicographic), then `code`, then `message`. `schema.` keywords not in the map fall back to `schema.other` so the namespace never leaks library strings.

## 6. CLI contract

`harness validate <path>` with `clap` (derive). `<path>` is a **file** (validated directly) or a **directory** (single candidate document; the filename list is an explicitly **non-normative working name**, honouring §58). `--kind <manifest|model-contract|install-plan>` overrides detection; `--json` switches output.

| Condition | Exit | `status` |
|---|---|---|
| No error diagnostics | `0` | `valid` |
| ≥1 error diagnostic, including an unsupported or missing `apiVersion` (evaluated & invalid, Q9) | `1` | `invalid` |
| Could not evaluate: unknown/ambiguous kind, unreadable path, unparsable bytes, bad usage | `2` | `error` |

`--json` shape (stdout; **stable contract**, marked experimental):

```json
{ "status": "valid", "errors": [ { "path": "/metadata/name", "code": "schema.pattern", "message": "..." } ], "warnings": [] }
```

Human output → **stderr**; `--json` → **stdout** (pipes/agents). **Read-only:** opens inputs read-only, writes no files, no temp files, no network (`jsonschema` offline), no third-party execution; exits without mutation. `Report`/`Diagnostic`/exit taxonomy are the shared contract for future `harness test`/`conformance`.

**Kind detection:** `--kind` wins; else the document is matched by root-schema discriminators (`kind ∈ {Harness,Component,Preset}` for manifest; `executionLocation`/`lifecycle`/`router` for model-contract; `environmentVariableNames`/`risk`/`snapshot`/`verificationSteps` for install-plan); 0 or >1 matches → `document.unknown_kind`/`document.kind_ambiguous` → exit 2.

## 7. Corpus + test strategy

The durable corpus is **promoted (moved, not copied)** from `openspec/changes/archive/2026-09-26-harness-schema-v1alpha1/examples/` to `packages/validator/tests/fixtures/` (7 positive / 16 negative + `corpus.json`), so `cargo test` is hermetic; the archive remains the audit trail.

| Layer | What to test | Approach |
|---|---|---|
| Corpus (primary gate) | 7/7 positive valid; 16/16 negative rejected with matching `{path, code}` | `tests/corpus.rs` parses `corpus.json`, maps `keyword → code`, asserts. |
| Adversarial | dup YAML keys, anchors/aliases, implicit typing, broken `$ref`, unknown apiVersion/kind/capability, dependency cycle, permission-coverage violation, autonomy below floor, `KEY=value` env, inline hook, plan timestamp, `..`/absolute/symlink path | `tests/adversarial.rs`, each with explicit expected `{path, code}`. |
| CLI E2E | exit `0/1/2`, stdout/stderr split, `--json` shape | `assert_cmd`; golden `--json` snapshot for one positive + one negative. |
| Quality | fmt + lints | `cargo fmt --check`, `cargo clippy` (CI runs the same). |

**Destructive class for this change = malformed/adversarial *input*** (§60 config): the validator mutates nothing, so there is no crash-on-apply surface.

## 8. YAML

**Choice:** `serde-saphyr` (YAML **1.2**, over `saphyr`; actively maintained) as the primary parser; `yaml_serde` (YAML Organization) is the documented fallback if the `windows-gnu` build fails. `serde_yaml` and `serde_yml` are **rejected** (deprecated/unmaintained). Policy: **YAML 1.2 only** (L1's `yaml` npm engine is 1.2; L2 PyYAML is 1.1 — the divergence is closed by picking 1.2), **duplicate keys fail-closed** (`parse.duplicate_key`), **no silent coercion** (an implicitly-typed scalar surfaces as a `schema.type`/`pattern` diagnostic), multiple documents rejected, BOM accepted only at stream start.

## 9. Dependency pins / toolchain

| Item | Value |
|---|---|
| Toolchain | `stable-x86_64-pc-windows-gnu`, `cargo 1.98.1` (verified locally) |
| MSRV | `1.85` (the `jsonschema` crate floor; we run 1.98.1) |
| `jsonschema` | `0.58.1`, `default-features = false` (offline) |
| Transitive pins (gnu `as` gap) | `ahash =0.8.11`, `parking_lot =0.12.4`, `parking_lot_core =0.9.11` — these remove the `raw-dylib` edges (`getrandom 0.3`, `windows-link`) that fail because `windows-gnu` ships `dlltool` but **no GNU `as`** |
| R3 evidence | 7/7 valid · 16/16 rejected · 0 keyword mismatch · offline (`jsonschema` spike output) |
| F4 prerequisite | Fix the environment (install MinGW-w64 `as`/binutils, or VS Build Tools + MSVC) before F4 pulls `tokio`; the pins above are a documented stopgap, not the fix |

## 10. ADR-0003 (decision record)

Recorded as `docs/adr/0003-rust-validator-architecture.md` (next free number), following `docs/adr/0000-template.md`:

| Option | Tradeoff | Decision |
|---|---|---|
| Layered pipeline vs one merged check | Layered = precise ownership, testable per layer, honest IN/PARTIAL/OUT; one pass = fewer hops, murkier diagnostics | **Layered** |
| Separate `harness-validator` crate vs module in `harness-core` | Separate = decoupled, independently testable, no future cycle; cost = one crate | **Separate crate** |
| Compile-time embed vs runtime disk load vs codegen types | Embed = hermetic single binary, schemas stay the source; disk = divergence risk; codegen = a language becomes the source (violates ADR-0001) | **Embed** |
| `jsonschema` offline registry vs runtime resolver | Offline = hermetic, fails loudly on unknown `$ref`; runtime = network/drift | **Offline `Registry`** |
| YAML 1.2 (`serde-saphyr`) vs YAML 1.1 (PyYAML parity) | 1.2 matches L1 and the pinned dialect; 1.1 would reopen the latent divergence | **1.2** |
| Typed `code` namespace vs library keywords | Typed = stable agent/CI contract; raw = churn | **Own namespace** |

## 11. Sequence / flow

```text
harness validate <path> --json
  → read bytes (read-only)                        [io.read_failed → 2]
  → parse (JSON | YAML 1.2, dup-key fail-closed)  [parse.* → 2]
  → document: detect kind + read apiVersion        [document.* → 2]
  → structural   (embedded schema Registry)        [schema.*]
  → version      (supported set + migration stub)  [version.*]  ← supersedes the structural /apiVersion issue (one issue → one code); unsupported/missing = invalid (exit 1)
  → semantic     (identity/deps/components/permissions) [semantic.*]
  → capability   (known-set lookup)                [capability.*]
  → filesystem   (declared-path safety)            [path.*]
  → deterministic sort → Report
  → exit 0 (valid) | 1 (invalid) | 2 (error)
```

Dedupe rule: when the version gate emits `version.missing`/`version.unsupported`, the structural `schema.required`/`schema.const` on `/apiVersion` for the same document is suppressed so one issue yields one code.

## File Changes

| File | Action | Description |
|---|---|---|
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | Create | Root workspace, committed lock, toolchain pin |
| `packages/core/Cargo.toml`, `src/lib.rs` | Create | `harness-core` placeholder |
| `packages/validator/Cargo.toml`, `build.rs`, `src/**`, `README.md` | Create | `harness-validator` crate + docs |
| `packages/validator/tests/**` (fixtures + corpus) | Create | Promoted corpus + adversarial + CLI tests |
| `packages/cli/Cargo.toml`, `src/main.rs` | Create | `harness` bin — `validate` |
| `scripts/schemas/{validate.mjs,cross_check.py}` | Modify | Fix hardcoded archive path (keep as independent cross-checks) |
| `.github/workflows/ci.yml` | Modify | Placeholder job → `cargo test` / `clippy` / `fmt --check` |
| `schemas/README.md`, `docs/adr/0002-schema-strategy.md` | Modify | `r3=pending → confirmed`; publish authoritative matrix |
| `docs/adr/0003-rust-validator-architecture.md` | Create | ADR §10 |

## Migration / Rollout

Additive only: no existing behavior replaced, no install/apply/revert/trust/supply-chain surface touched, no credential path. Rollback = `git revert` of the range; the JS/Python gates stay as independent cross-checks so revert loses no coverage.

## Open Questions

- [ ] **Q6** capability known-set source — recommend a checked-in `extensions/registry.json` (or `schemas/capabilities.json`) with a documented empty set; **§58 stays OPEN** (does not close Core-vs-extension).
- [x] **Q9** unsupported or missing `apiVersion` = **exit 1**, `status: invalid` — **RATIFIED, the spec wins** (`specs/harness-cli/spec.md` "Exit-code taxonomy (Q9)"; `specs/harness-validator/spec.md` "Supported version gate"). The earlier cannot-evaluate recommendation is withdrawn; §6 and §11 are corrected so no downstream agent implements an exit-2 classification for the version layer. §58 stays OPEN.
- [ ] **Q10** `--json` stability — recommended: publish as experimental-but-stable shape, version it with the validator crate.
- [ ] **Q13** `windows-gnu` `as` fix ownership — must be resolved before F4 (installation, not the pins).
- [ ] **Q7** YAML crate final pin — confirm `serde-saphyr` builds on `windows-gnu` in the apply spike; else `yaml_serde`.
