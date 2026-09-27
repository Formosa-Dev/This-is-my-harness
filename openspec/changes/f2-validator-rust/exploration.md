# Exploration: f2-validator-rust (F2-05..F2-12)

> **Change:** `f2-validator-rust` · **Phase:** F2 · **Scope:** F2-05..F2-12 — the real validator (Rust) + `harness validate`.
> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1` · **Language:** English. NON-NORMATIVE exploration. Normative sources: `spec/**` (F1), `schemas/**` (F2-01..04), `docs/adr/0001`, `docs/adr/0002`.
> **Predecessor:** `harness-schema-v1alpha1` archived at `openspec/changes/archive/2026-09-26-harness-schema-v1alpha1/` (verify **PASS**, commit `c735554`/`dea72dd`; Engram `sdd/harness-schema-v1alpha1/*`).
> **Environment verified this session:** `cargo 1.98.1` / `rustc 1.98.1` (`stable-x86_64-pc-windows-gnu`, self-contained, **no MSVC, no MinGW**), Node `v22.17.0`, Python `3.12.10`, `ajv@8.20.0`, `jsonschema 4.26.0`, `PyYAML 6.0.3`.

## 1. Executive summary

- **R3 is RESOLVED** (pre-freeze blocker cleared): the Rust `jsonschema` crate **v0.58.1** (+ `referencing 0.58.1`), run offline with `default-features = false` and `.offline()`, validates our ACTUAL Draft 2020-12 schemas and reproduces the ajv L1 / Python-jsonschema L2 corpus result **exactly**: **7/7 positives valid, 16/16 negatives rejected, 0 keyword mismatches**, including every `unevaluatedProperties` case. Real command + output in §3.
- **One environment caveat** (not a crate limitation): the installed `windows-gnu` toolchain ships `dlltool` but **no GNU assembler `as`**, so any crate using `raw-dylib` (`windows-link`, `getrandom 0.3`) fails at import-lib generation. The spike worked only after pinning `ahash =0.8.11`, `parking_lot =0.12.4`, `parking_lot_core =0.9.11`. **This gap will recur for `harness-core` (tokio, etc.) and must be fixed at the environment level before F4.** See §3.3 / §10.
- **Regression found:** archiving the schema change BROKE the existing gates that hardcode its path. `npm run validate:schemas` → `FAIL missing: openspec/changes/harness-schema-v1alpha1/examples/corpus.json` (exit 1); `python scripts/schemas/cross_check.py` → `FileNotFoundError` (exit 1). `check_schemas.py` (L0) still passes. The corpus to reuse now lives under the archive (§8.2).
- The change is well-bounded: greenfield `packages/**` (only `.gitkeep`), a proven crate, a positional path argument, and a documented semantic boundary. The main design decisions are the **validation-layer split**, the **fixture home after archive**, and the **explicit deferral** of graph/registry/trust checks that need F4/F7/F15.

## 2. Current state

- **Spec layer (F1)** complete: `spec/core/*` (identity, versioning, requirements, permissions, risk-classes, dependencies, distribution, compatibility, conformance-metadata, profiles, core-vs-extension), `spec/manifest/README.md`, `spec/package/{component-types,layout}.md`, `spec/install-protocol/README.md` (§4 = 16 plan fields, §7 risk/autonomy), `spec/adapter-contract/README.md`.
- **Schema layer (F2-01..04)** archived. `schemas/` = 3 root schemas + 7 `defs/` + `README.md` (dialect, `$id` map, resolver map, strictness, §58 table, F1 traceability matrix, **semantic boundary** table, R3 record) + `registry.json` (`$id`→repo path, 10 entries). Corpus = **7 positive / 16 negative** at `…archive/2026-09-26-harness-schema-v1alpha1/examples/`.
- **Toolchain (F2-05..12)**: `packages/{core,validator,sdk,cli}` contain only `.gitkeep`. No `Cargo.toml` anywhere. No test runner configured (`openspec/config.yaml` `strict_tdd: false`; `verify` says "once tooling exists, run `cargo test` / `cargo clippy` / `cargo fmt --check`").
- **CI** (`.github/workflows/ci.yml`): markdown lint + a placeholder that only asserts `schemas/**/*.json` parse. The comment says "Once a real validator exists (F2), replace this with JSON Schema metaschema + instance validation."
- **Grep-confirmed surface contract**: `MAINTAINERS.md` assigns `packages/core/` = resolver/planning/snapshot/path-safety/hashing; `packages/validator/` = "`harness validate` and schema conformance"; `packages/cli/` = "`harness use/validate/test/conformance`, JSON output, typed states". `README.md` L101 names the conceptual commands `harness validate .`, `harness test --runtime`, `harness conformance`.

## 3. R3 spike (MANDATORY — executed, real output)

### 3.1 Candidate and verdict

| Candidate | Draft 2020-12 | `unevaluatedProperties` | Offline `$id` registry | Verdict |
|---|---|---|---|---|
| **`jsonschema` 0.58.1** (+ `referencing` 0.58.1) | Yes | Yes (proven) | Yes (`Registry`) | **SELECTED — R3 RESOLVED** |
| `boon` | unverified | unverified | unverified | Rejected (unproven; different API) |
| `valico` | No (Draft-07 era) | No | No | Rejected |
| hand-rolled over `serde_json` | n/a | would re-implement the dialect | n/a | Rejected (risk, effort, correctness) |

### 3.2 Exact commands and REAL output

Spike project (throwaway, **outside the repo**) at `C:\Users\Asus\AppData\Local\Temp\opencode\r3-spike`.

`Cargo.toml` deps: `jsonschema = { version = "0.58", default-features = false }` (no `resolve-http`/`resolve-file` — structurally offline), `serde_json = "1"`, `serde_yaml = "0.9"`, plus the environment pins in §3.3.

```powershell
$sc = "$env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin\self-contained"
$env:PATH = "$sc;$env:PATH"
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run
```

Real output (trimmed to the result; full per-case list printed):

```
REGISTRY OK resources=10

== POSITIVES (expect: valid) ==
  VALID    examples/positive/manifest.minimal.yaml
  VALID    examples/positive/manifest.rich.yaml
  VALID    examples/positive/model-contract.json
  VALID    examples/positive/router.json
  VALID    examples/positive/model-contract.service.json
  VALID    examples/positive/install-plan.json
  VALID    examples/positive/install-plan.conflict-resolution.json

== NEGATIVES (expect: invalid + expected {path,keyword}) ==
  REJECTED  examples/negative/manifest.missing-apiversion.json  (matched /required)
  REJECTED  examples/negative/manifest.unknown-toplevel.json  (matched /unevaluatedProperties)
  REJECTED  examples/negative/manifest.unknown-metadata.json  (matched /metadata/unevaluatedProperties)
  REJECTED  examples/negative/manifest.bad-slug.json  (matched /metadata/name/pattern)
  REJECTED  examples/negative/manifest.bad-semver.json  (matched /metadata/version/pattern)
  REJECTED  examples/negative/manifest.wrong-type-metadata.json  (matched /metadata/type)
  REJECTED  examples/negative/manifest.aggregate-root.json  (matched /type)
  REJECTED  examples/negative/manifest.unknown-apiversion.json  (matched /apiVersion/const)
  REJECTED  examples/negative/manifest.unknown-kind.json  (matched /kind/enum)
  REJECTED  examples/negative/model-contract.name-only.json  (matched /required)
  REJECTED  examples/negative/model-contract.bad-execution-location.json  (matched /executionLocation/enum)
  REJECTED  examples/negative/model-contract.partial-lifecycle.json  (matched /lifecycle/required)
  REJECTED  examples/negative/install-plan.missing-field.json  (matched /required)
  REJECTED  examples/negative/install-plan.env-value.json  (matched /environmentVariableNames/0/pattern)
  REJECTED  examples/negative/install-plan.hooks-inline.json  (matched /hooks/0/unevaluatedProperties)
  REJECTED  examples/negative/install-plan.timestamp.json  (matched /unevaluatedProperties)

== SPIKE RESULT ==
positive: valid=7 invalid=0 (expected valid=7)
negative: rejected=16 keyword_mismatch=0 accepted=0 (expected rejected=16)
MATCH_L1_L2=YES
```

How it resolved refs: build `jsonschema::Registry::new().add($id, schema_json)? … .prepare()?` for all 10 `registry.json` ids, then build a validator per corpus entry from the wrapper `{ "$ref": $id }` via `jsonschema::draft202012::options().with_registry(&registry).offline().build(&wrapper)`. `error.instance_path()` produced pointers identical to ajv `instancePath`; the keyword was taken from the last segment of `error.absolute_keyword_location()`.

- **Verdict:** R3 **RESOLVED** — the current `jsonschema` crate supports Draft 2020-12 + `unevaluatedProperties` under composition, resolves our absolute `$id`s offline, and matches both existing engines on the full corpus.

### 3.3 The environment caveat (important, reproducible)

First build failed, not in our code:

```
error: error calling dlltool 'dlltool.exe': program not found
...
dlltool could not create import library with ...dlltool.exe -d ...bcryptprimitives.dll_imports.def ...
```

Root cause, reproduced with `dlltool --verbose`:

```
dlltool.exe: run: ...\self-contained\as --64 -o th.o th.s
dlltool.exe: No such file or directory
dlltool.exe: CreateProcess
```

The bundled `rust-mingw` self-contained dir contains `dlltool.exe`, `ld.exe`, `x86_64-w64-mingw32-gcc.exe` — but **no `as.exe`** anywhere on the machine. `dlltool` needs `as`. Any crate using `raw-dylib` therefore fails: in our graph `ahash → getrandom 0.3` and `referencing → parking_lot → parking_lot_core → windows-link` both use it. Pinning `ahash =0.8.11` (→ `getrandom 0.2`, `windows-targets` prebuilt libs) and `parking_lot =0.12.4` + `parking_lot_core =0.9.11` (→ `windows-targets 0.52`) removed both `raw-dylib` edges and the build then succeeded in **39 s**. `cargo tree -i windows-link` → "did not match any packages"; `cargo tree -i getrandom@0.3.4` → "nothing to print" (host target).

**Implication:** this is a project-level Rust environment prerequisite, not a `jsonschema` defect. `harness-core` (F4+) will pull `tokio`/`parking_lot`/`getrandom` latest and hit the same wall. Resolution options (pick before F4): (a) install a real MinGW-w64 `as`/binutils so `dlltool` works, (b) install VS Build Tools and switch to `x86_64-pc-windows-msvc`, or (c) accept pins as a stopgap. **(a) or (b) recommended; (c) is fragile.**

## 4. Workspace layout proposal

Greenfield; `packages/*` empty. Recommend a **root Cargo workspace** matching `README.md`'s repository map and `MAINTAINERS.md` ownership:

```
/ Cargo.toml                      # [workspace] members = ["packages/*"], [workspace.dependencies]
/ Cargo.lock                      # committed (binary workspace)
/ rust-toolchain.toml             # optional: pin stable + document the `as` prerequisite
  packages/
    core/      Cargo.toml         # crate `harness-core`  (resolver/planning/mutation — future; placeholder lib now)
    validator/ Cargo.toml         # crate `harness-validator` (schema loading + structural/semantic/capability + diagnostics)
    cli/       Cargo.toml         # crate `harness` (bin) — `harness validate <path>`, `--json`
    sdk/                          # reserved (no Rust yet)
```

**Validator module map (`packages/validator`):**

```
src/lib.rs              # validate(input: &Source, opts) -> Report ; public API, no I/O decisions
src/diagnostics.rs      # Diagnostic { path, code, message }, Severity, Report { status, errors, warnings }
src/registry.rs         # embed schemas + registry.json (include_str!); build jsonschema::Registry once
src/parse.rs            # bytes -> serde_json::Value (JSON or YAML), duplicate-key policy, multi-doc rejection
src/document.rs         # detect kind (manifest | model-contract | install-plan) + apiVersion gate
src/structural.rs       # JSON Schema layer -> diagnostics (maps jsonschema errors -> {path, code})
src/semantic/mod.rs     # orchestrates document-local semantic rules
src/semantic/identity.rs        # owner/slug/ref grammar & host-consistency, short-ref rejection
src/semantic/dependencies.rs    # ordered extends, kind rules, self/known-set cycle detection
src/semantic/components.rs      # per-type rules, requirements<->components coherence, deps vs layout
src/semantic/permissions.rs     # coverage, effective risk (max), monotonicity, autonomy floor
src/capability.rs       # capability known-set lookup; unknown -> explicit typed error
src/version.rs          # supported apiVersion set + migration-map stub
tests/corpus.rs         # drives the 7/16 corpus under cargo test (primary gate)
```

**Tradeoff — `packages/validator` as its own crate vs a module inside `harness-core`:** separate crate keeps the schema/diagnostic layer decoupled and independently testable, and lets `harness-core` depend on it later without a cycle; the cost is one more crate in the workspace. Recommend **separate crate**. `packages/cli` owns argument parsing, output formatting and exit codes; all validation logic lives in the library so it is testable without a subprocess.

## 5. Schema loading (offline, single source)

Proven in the spike. Three options:

| Option | Pros | Cons |
|---|---|---|
| **Embed via `include_str!`/`include_dir!`** (recommended) | Hermetic single binary; no runtime path; unresolved `$ref` fails at first use; schemas stay in `schemas/` as the ONE source (no copy) | Rebuild required when a schema changes (fine); path is compile-time relative to the crate |
| Read from disk at runtime (repo path / env `HARNESS_SCHEMA_DIR`) | No recompile | Distributed binary needs the schemas shipped + a resolvable path; can diverge from source |
| Codegen Rust types from JSON Schema | Type-safe | Violates ADR-0001 (a language would become a second source); JSON Schema stays the source, codegen *from* it only |

Recommended: `packages/validator/build.rs` (or `include_str!` directly) reads `../../schemas/**` at compile time into a `&[(&str /*$id*/, &str /*json*/)]` table derived from `registry.json`; build the `jsonschema::Registry` once and reuse. Use `default-features = false` + `.offline()` so a `$ref` not present in the registry **fails loudly** at build (satisfies the schema-strategy requirement "a broken `$ref` fails loudly", no network).

## 6. Validation layers

Each row of the `schemas/README.md` **semantic boundary** table is mapped to a layer and an explicit in-scope decision. `IN` = in this change; `PARTIAL` = document-local subset now, graph part deferred; `OUT` = deferred (needs a phase/registry that does not exist yet).

| Layer | Responsibility | Owner task |
|---|---|---|
| **L-structural** | All JSON-Schema-expressible constraints (the whole schema set) | F2-05 |
| **L-semantic** | Document-local cross-section rules (see table) | F2-06 |
| **L-capability** | Known-capability lookup; unknown → typed error | F2-07 |
| **L-version** | Supported `apiVersion` set + migration-map stub; unsupported/missing → typed error | F2-11 |
| **L-filesystem** | Declared-path traversal (`..`, absolute, symlink) within a provided project path | F2-08 |

Boundary-row mapping:

| Semantic-boundary row | Layer | Scope now | Notes |
|---|---|---|---|
| Supported `apiVersion` set + migration map | L-version | **IN** (F2-11) | supported set `{thisismyharness.dev/v1alpha1}`; static migration stub; §58 stays OPEN |
| Unknown `kind` unless compatible extension present | L-semantic | **PARTIAL** (F2-06) | v1alpha1 kinds are a closed enum (structural); extension-kind lookup needs a registry → defer |
| Reference resolution (scoped refs, digest resolution) | L-semantic | **PARTIAL** (F2-06) | grammar + host-consistency + "short ref forbidden in a published manifest" now; digest/network → OUT (F4/F7) |
| Dependency-graph analysis (cycles, conflicts, duplicate identity) | L-semantic | **PARTIAL** (F2-06) | ordered `extends`, kind-composition rules, self-cycle + cycle over any resolved set provided; full resolver → OUT (F4) |
| Permission coverage for capabilities above Passive | L-semantic | **IN** (F2-06) | cross-section: every non-Passive component/permission is declared |
| Effective risk, monotonicity, autonomy floor | L-semantic | **IN** (F2-05/F2-06) | effective = max over parts; autonomy ≥ floor(risk); no bypass |
| Capability existence against the extension registry | L-capability | **IN** (F2-07) | **needs a known-capability source** (open question Q6); unknown → explicit error |
| Package discovery, path safety, symlink containment | L-filesystem | **PARTIAL** (F2-08) | declared-path traversal in a document now; discovery/symlink-escape on a real tree → OUT (F4) |
| Install-Plan determinism and binding to inputs | (plan) | **OUT** (F4) | document *shape* is L-structural; determinism is a generator property |
| Trust-label precision / declared-vs-verified compatibility | — | **OUT** (F7/F15) | needs conformance evidence |
| `environmentVariableNames` are names, not values | L-semantic | **IN** (F2-12/F2-06) | structural `pattern` already rejects `KEY=value`; add a value-like heuristic + clear code |
| Digest / immutability semantics | — | **OUT** (F7) | registry behaviour |
| Model `license` SPDX identifier shape | L-semantic | **IN** (F2-12) | F1 fixes no grammar → documented heuristic, marked OPEN (§58/Q7) |
| Model `license` vs package `metadata.license` contradiction (Q7) | L-semantic | **PARTIAL** (F2-10) | cross-document; checked when both instances are provided, else reported as not-evaluated, never guessed |

**Finding:** the README's "Owner" column is **inconsistent with `build-progress/F2` task numbers** (e.g. it assigns "package discovery, path safety" to F2-08, which the backlog defines as the CLI command; "plan determinism" to F2-09, which is the corpus). The new change's spec/design MUST publish one authoritative layer→task matrix and re-state deferrals, so the boundary table stops implying capabilities the change cannot deliver.

## 7. CLI surface

- **Command:** `harness validate <path>` where `<path>` is a file (a manifest / model-contract / install-plan document) **or** a directory (project root → discover the manifest; discovery rules explicitly deferred, so for now a directory validates the single manifest it finds). Also accept `-` / stdin if cheap; optional `--kind` override when a filename is ambiguous.
- **Exit codes (proposed, stable contract):** `0` = valid; `1` = invalid (≥1 error diagnostic); `2` = input/usage/IO error (unreadable path, unparsable bytes, unsupported `apiVersion`, unknown document kind). Rationale: distinguishing "the artifact is invalid" from "I could not evaluate it" matters to agents/CI. (Decision to ratify in spec — see §11 Q9.)
- **`--json` output** (agent-facing; stdout):

```json
{
  "status": "valid",
  "errors": [
    { "path": "/metadata/name", "code": "schema.pattern", "message": "…" }
  ]
}
```

  `status ∈ {valid, invalid, unsupported}` (or `error` for exit-2 conditions); `errors[]` is the typed diagnostic list; add `warnings[]` (e.g. not-evaluated cross-document checks) without breaking the shape. Diagnostics are **typed and stable**: a `code` is ours (`schema.required`, `schema.type`, `semantic.capability_unknown`, `version.unsupported`, `path.traversal`, …), never a raw library string; `path` is a JSON Pointer matching ajv `instancePath` so the corpus expectations stay the golden data.
- **Human output:** grouped, path-first, to **stderr**; `--json` goes to **stdout** so scripts/agents can pipe it.
- **Relationship to future commands:** `validate` is read-only (risk A, autonomy 0 — no mutation, no execution, consistent with `schemas/README.md` risk posture). The same `Diagnostic`/`Report` types and exit-code taxonomy MUST back the future `harness test` / `harness conformance` / `harness use`, so the JSON output is the shared contract. `harness use` (mutation) is not introduced here.

## 8. Corpus + test strategy

1. **Reuse the existing corpus** (7 positive / 16 negative), now located under the archive: `openspec/changes/archive/2026-09-26-harness-schema-v1alpha1/examples/`.
2. **Fixture-home decision (open — §11 Q1):**
   - (a) point Rust tests at the immutable archived path — zero duplication, but couples tests to the archive layout;
   - (b) copy into `openspec/changes/f2-validator-rust/fixtures/` — self-contained change, duplicates 23 files;
   - (c) **promote to a stable, non-archived home** as the first fixture collection (e.g. `packages/validator/tests/fixtures/` for hermetic `cargo test`, or `conformance/` per F3) and keep the archive as audit trail.
   Recommended: **(c) into `packages/validator/tests/fixtures/`** so `cargo test` is hermetic; move (not delete) the archived copy stays as audit trail.
3. **`cargo test` driver:** `tests/corpus.rs` parses `corpus.json`, and for each entry validates with the embedded registry and asserts `positive ⇒ status valid` and `negative ⇒ status invalid AND some diagnostic matches {path, keyword}` (keyword mapped to our `code`). This replaces L1/L2 as the primary gate. Add `assert_cmd` for a thin CLI smoke test (exit codes + `--json` shape).
4. **Golden expectations:** the corpus `expect` blocks are the golden data; add a golden `--json` snapshot for one positive and one negative to freeze the output contract.
5. **Adversarial/destructive cases** (the config requires adversarial coverage; this change mutates nothing, so the applicable class is **malformed/adversarial input**, not crash-on-apply): malformed YAML (duplicate keys, anchors/aliases, implicit typing), broken `$ref`, unknown `apiVersion`, unknown `kind`, unknown capability, dependency cycle, permission-coverage violation, autonomy below floor, `KEY=value` env entry, inline hook/`command`, plan `timestamp`, declared path with `..`/absolute/symlink. Each MUST have a negative fixture with an explicit expected `{path, code}`.
6. **Fix or retire L1/L2:** see §10 R-regression.

## 9. Migration / versioning stubs (F2-11)

- Read `apiVersion` **before** schema validation. Missing → `version.missing` typed error (distinct from the schema's `required`). Not in the supported set → `version.unsupported` naming the supported set; never crash, never silently accept.
- Migration map: a static `&[(supported: &str, migration: Migration)]` with `Migration::None` for `thisismyharness.dev/v1alpha1`, plus a documented extension point. **§58 stays OPEN** — do not invent a `v1beta1`, a media type, a host or a migration algorithm. The map is a stub that returns "no migration needed" for the one supported generation.
- The schema's `apiVersion` `const` still rejects a wrong value structurally; the version layer exists to give a *clearer* message and to own the supported-set/migration policy (F2-11), per the boundary table.

## 10. Docs plan (F2-12)

`packages/validator/README.md` MUST state, precisely and without overclaiming:
- **What "valid" means:** the layers actually run (structural / semantic / capability / version / filesystem) and, explicitly, **what is NOT checked** (graph resolution, registry/immutability, trust labels, declared-vs-verified compatibility, plan determinism) with the owning phase.
- **Exit codes** table (0/1/2) and the **`--json` shape** with the **diagnostic `code` catalogue**.
- **Coverage** table (schema / estructura / permisos / adapter / fixtures / capability) — naming exactly what each layer guarantees.
- **Install Plan is local-only** (mirrors `schemas/README.md`; never uploaded).
- **Trust statement:** no "100% safe", no conformance claim; only what was verified (phases, layers, corpus result).
- How to run `cargo test` / `cargo clippy` / `cargo fmt --check`, and how to add a fixture.
- Cross-reference: resolving R3 flips `r3=pending → confirmed` in `schemas/README.md` + ADR-0002 (**but the explore phase is forbidden from editing `schemas/**`; the change itself must decide whether to make that minimal bookkeeping edit** — §11 Q4).

## 11. Open questions

1. **Fixture home** after archive: archived path vs copy into the change vs promote to `packages/validator/tests/fixtures/` (§8.2). Recommendation: promote; move-not-copy.
2. **Crate boundary:** `packages/validator` as a separate crate vs a module of `harness-core`. Recommendation: separate crate.
3. **Schema embedding** vs runtime disk load (§5). Recommendation: embed via `include_str!`.
4. **R3 bookkeeping:** does *this* change edit `schemas/README.md` + `docs/adr/0002` (`r3=pending → confirmed`) and flip the `--freeze` marker, or is that a separate doc change? The phase rules forbid `schemas/**` edits during explore; apply must decide.
5. **Semantic scope:** ratify the IN/PARTIAL/OUT split in §6, and fix the README Owner-column inconsistency.
6. **Capability registry source (F2-07):** where do "known capabilities" come from? `extensions/*` are empty `.gitkeep` dirs. Options: a checked-in `schemas/capabilities.json` (or `extensions/registry.json`), the capability-list in `README.md` L87, or "all namespaced ids are shape-valid; only registry-declared ones are 'known'". §58 keeps Core-vs-extension capabilities OPEN — this change must not close it.
7. **YAML crate + version** and duplicate-key policy (§12).
8. **`validate <path>` semantics:** file vs directory discovery; kind detection; multi-document files; `--kind` override.
9. **Exit-code taxonomy** (0/1/2) and whether `unsupported apiVersion` is exit 1 or 2.
10. **`--json` stability:** is it a public contract now? Version it / mark experimental?
11. **L1/L2 fate:** fix the broken archive paths, or retire JS/Python gates in favor of `cargo test`?
12. **CI:** replace the placeholder `schema-validation` job with `cargo test` (needs a Rust setup on ubuntu + the workspace).
13. **Windows-gnu `as` gap ownership** (§3.3) — environment fix before F4; who/what.
14. **MSRV / toolchain pin:** `jsonschema` MSRV 1.85, we have 1.98; add `rust-toolchain.toml`?

## 12. YAML handling (hazard review)

- **Avoid** `serde_yaml` (`0.9.34+deprecated`) and **also** `serde_yml` — `cargo search` reports `serde_yml = "0.0.13"  # DEPRECATED — serde_yml is unmaintained`.
- **Candidates:** `serde-saphyr 1.3.0` (serde bindings over `saphyr 0.1.0`, "fully YAML 1.2 compliant"), `yaml_serde 0.10.7` ("maintained by The YAML Organization"), `serde_yaml_neo 0.11.0`.
- **Hazards to handle explicitly:** duplicate keys (spec says error; implementations vary — choose **fail-closed**); anchors/aliases (expansion / billion-laughs); implicit typing (`yes/no/on/off` → bool in YAML 1.1, `1.10` → float, `2020-12-01` → date); tabs; multiple documents in one stream; BOM.
- **Latent L1/L2 divergence already present:** L1 uses the `yaml` npm package (**YAML 1.2**); L2 uses `PyYAML safe_load` (**YAML 1.1**). The two engines can disagree on booleans/sexagesimals. The current 2 YAML positives happen to pass both, so there is no observed divergence — but the Rust validator must pick one version (recommend **1.2**), document it, and its tests must include an implicit-typing fixture.
- Because our schemas type the string fields as `type: string` with `pattern`s, an implicitly-typed value surfaces as a clear `type`/`pattern` diagnostic; still, the parser MUST NOT silently coerce.

## 13. Risks

- **R-toolchain (HIGH):** `windows-gnu` has no `as`; `raw-dylib` crates fail. Project-level, will recur in F4+. Must be fixed or pinned deliberately.
- **R-regression (MED):** archiving broke `npm run validate:schemas` and `cross_check.py`. Until addressed, no gate validates the corpus. Fix path constants (small) or retire in favor of `cargo test`.
- **R-scope (MED):** the semantic-boundary Owner column over-promises; a "semantic validator" cannot do graph/registry/trust checks without a resolver/registry. Must publish the IN/PARTIAL/OUT matrix.
- **R-diagnostic-stability (MED):** `{path, code, message}` becomes an agent/CI contract; leaky library keywords would churn it. Enforce our own `code` namespace.
- **R-yaml (MED):** crate choice + YAML 1.1 vs 1.2 + duplicate-key policy; L1/L2 already disagree on version.
- **R-verification-platform (MED):** only Windows-gnu is available locally; CI is ubuntu. Cross-platform build must be proven before claiming portability.
- **R-review-budget (MED):** new workspace + crate + CLI + tests + docs is likely **>400 lines** → chained PRs (see delivery).
- **R-§58 (LOW):** accidentally resolving a §58-open item (capability registry, migration) while "implementing" the validator.

## 14. Alternatives considered

1. **Keep the JS/Python gates, skip Rust** — rejected: ADR-0001 mandates Rust Core; single-binary distribution; F2-05..12 are explicitly the Rust validator.
2. **`jsonschema` vs `boon` vs `valico` vs hand-rolled** — `jsonschema 0.58.1` proven by execution; others unverified/lacking 2020-12.
3. **Embed schemas vs disk load vs codegen** — embed recommended (§5).
4. **Separate `harness-validator` crate vs module in `harness-core`** — separate recommended.
5. **One monolithic binary crate vs workspace (`core`/`validator`/`cli`)** — workspace, per `README.md`/`MAINTAINERS.md`.
6. **Corpus: archived path vs change copy vs promoted fixtures** — promote recommended (§8.2).
7. **YAML: `serde-saphyr` vs `yaml_serde` vs deprecated crates** — YAML 1.2 parser required (§12).
8. **Testing: library integration tests vs CLI-only** — library tests + thin `assert_cmd` smoke test.

## 15. Recommended approach (for propose)

- Root **Cargo workspace**; crates `harness-core` (placeholder), **`harness-validator`**, **`harness`** (CLI); shared `[workspace.dependencies]`.
- **Crate:** `jsonschema 0.58` (`default-features = false`) + `.offline()`; `serde`/`serde_json`; a **YAML 1.2** parser. **Record R3 as CONFIRMED with the exact spike evidence** (crate + version + 7/16 result), and record the toolchain `as` prerequisite separately.
- **Schema loading:** embed `schemas/**` + `registry.json` at compile time; build one `Registry`; unresolved ref fails loudly.
- **Layers:** structural (F2-05) → semantic document-local (F2-06) → capability (F2-07) → version (F2-11) → filesystem, documented with the IN/PARTIAL/OUT matrix from §6.
- **Diagnostics:** typed `{path, code, message}` with a stable `code` namespace; `--json` = `{status, errors, warnings?}`; exit `0/1/2`.
- **`harness validate <path>`:** file or directory; read-only; human to stderr, `--json` to stdout.
- **Tests:** promote the corpus to `packages/validator/tests/fixtures/` and drive it under `cargo test`; keep the archive as audit trail; add negative fixtures per semantic rule and an adversarial input set (§8.5).
- **Explicit deferrals:** graph resolver, digest/immutability, trust/conformance, plan determinism → named phases (F4/F7/F15), never silently missing.
- **Fix the L1/L2 regression** (path constants to the archive) — or retire the gates in favor of `cargo test`; do not leave the corpus ungated.
- **Docs:** `packages/validator/README.md` per §10.
- **Delivery:** chained PRs — (1) workspace + crates + toolchain/doc prerequisites; (2) schema embedding + registry + structural layer + corpus runner; (3) semantic layer; (4) capability + version layers; (5) CLI + `--json` + exit codes; (6) docs + adversarial fixtures + R3 bookkeeping.

## 16. Ready for proposal

**YES.** R3 is resolved with executable evidence; the crate, workspace, loading strategy, layer split, CLI contract and corpus plan all have a recommended path with rejected alternatives. The propose phase must ratify: the IN/PARTIAL/OUT semantic matrix; the fixture home; the capability-registry source; the YAML crate/version; the L1/L2 fate; and the R3/toolchain bookkeeping ownership.

## Affected areas

- `packages/core/`, `packages/validator/`, `packages/cli/` — new Rust crates (greenfield).
- Root `Cargo.toml`, `Cargo.lock`, optional `rust-toolchain.toml` — new.
- `scripts/schemas/{validate.mjs,cross_check.py}` — broken by the archive (fix or retire).
- `.github/workflows/ci.yml` — placeholder → `cargo test`.
- `openspec/changes/f2-validator-rust/**` — this change's artifacts + fixtures.
- `schemas/**`, `docs/adr/0002*` — **read-only here**; possible minimal R3 bookkeeping in apply (decision pending).
- Engram: `build-progress/F2` (#4271), `sdd/harness-schema-v1alpha1/*` (context), `sdd/f2-validator-rust/*` (new).
