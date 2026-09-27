# Tasks: f2-validator-rust (F2-05..F2-12)

> Phase F2 · `apiVersion: thisismyharness.dev/v1alpha1` · Risk class A (Passive) · autonomy 0 · READ-ONLY · Language: English.
> Sources: `proposal.md`, `specs/**` (authoritative), `design.md` (non-normative), `exploration.md`; Engram `sdd/f2-validator-rust/*`, `build-progress/F2` (#4271).
> **Toolchain (verified this session):** `cargo 1.98.1`, `rustc 1.98.1` (`stable-x86_64-pc-windows-gnu`). Locally invoke `& "$env:USERPROFILE\.cargo\bin\cargo.exe"`; prepend the self-contained dir to `PATH` (no GNU `as`):
> `$sc = "$env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin\self-contained"; $env:PATH = "$sc;$env:PATH"`.

## Review Workload Forecast

| Field | Value |
|-------|-------|
| Estimated changed lines | ~1,400–2,200 hand-written (Rust ~900–1,300, fixtures ~250–400, docs/schemas ~350–500) + generated `Cargo.lock` (~500–900) |
| 400-line budget risk | High |
| Chained PRs recommended | Yes |
| Suggested split | PR 1 → PR 2 → PR 3 → PR 4 → PR 5 → PR 6 (6 work units) |
| Delivery strategy | ask-on-risk |
| Chain strategy | pending (team choice; recommended `feature-branch-chain`) |

Decision needed before apply: Yes
Chained PRs recommended: Yes
Chain strategy: pending
400-line budget risk: High

### Suggested Work Units

| Unit | Goal | Likely PR | Base boundary |
|------|------|-----------|---------------|
| 1 | Workspace + 3 crate skeletons + gnu pins build green | PR 1 | feature/tracker branch |
| 2 | Embed schemas + structural layer + promoted corpus (7/16) | PR 2 | PR 1 branch |
| 3 | Document-local semantic layer | PR 3 | PR 2 branch |
| 4 | Capability + version layers; Q9 correction | PR 4 | PR 3 branch |
| 5 | Declared-path safety + `harness validate` CLI/`--json`/exit codes | PR 5 | PR 4 branch |
| 6 | Docs, ADR-0003, R3 bookkeeping, matrix, CI, L1/L2 repoint | PR 6 | PR 5 branch |

## Phase 1: Workspace & toolchain foundation (PR 1)

- [ ] 1.1 Create root `Cargo.toml` (`[workspace] members=["packages/*"]`, `resolver="2"`) with `[workspace.dependencies]`: `jsonschema = { version="=0.58.1", default-features=false }`, `serde`, `serde_json`, `clap`, `assert_cmd`, and gnu stopgap pins `ahash="=0.8.11"`, `parking_lot="=0.12.4"`, `parking_lot_core="=0.9.11"`. **Files:** `Cargo.toml`, `Cargo.lock` (committed). **Deps:** —. **Verify:** `cargo metadata --no-deps --format-version 1` exits 0 and lists the workspace; `cargo tree -i windows-link` → "did not match any packages".
- [ ] 1.2 Create `rust-toolchain.toml` (`channel="stable"`, minimal profile) + comment documenting the `windows-gnu` missing-`as` prerequisite (F4 blocker, §9). **Files:** `rust-toolchain.toml`. **Deps:** —. **Verify:** `cargo show`/`cargo --version` still resolves; `Get-Content rust-toolchain.toml` contains `channel = "stable"`.
- [ ] 1.3 Create `packages/core` placeholder lib `harness-core` (empty `src/lib.rs`) and delete `packages/core/.gitkeep`. **Files:** `packages/core/Cargo.toml`, `packages/core/src/lib.rs`. **Deps:** 1.1. **Verify:** `cargo build -p harness-core` exits 0.
- [ ] 1.4 Create `packages/validator/Cargo.toml` (lib `harness-validator`, pins referenced from workspace deps) + minimal `src/lib.rs`; `packages/cli/Cargo.toml` (bin `harness`, deps `harness-validator` + `clap` derive) + minimal `src/main.rs`; keep `packages/sdk/.gitkeep`. **Files:** `packages/validator/**`, `packages/cli/**`, `packages/sdk/.gitkeep`. **Deps:** 1.1. **Verify:** `cargo build --workspace` exits 0.
- [ ] 1.5 (verification/adversarial) Prove the pins remove `raw-dylib` edges and a clean locked build works: `cargo build --workspace --locked` then `cargo tree -i getrandom@0.3` (host). **Files:** —. **Deps:** 1.3, 1.4. **Verify:** build exits 0; `cargo tree -i getrandom@0.3` → "nothing to print"; `cargo tree -i parking_lot_core` shows `=0.9.11`.

## Phase 2: Embedding, parsing, structural layer, corpus (PR 2, F2-05 structural)

- [ ] 2.1 Write `packages/validator/build.rs`: read `../../schemas/registry.json`, emit a generated `&[(&str $id, &str json)]` table via `include_str!`; fail the build if any mapped path is missing. **Files:** `packages/validator/build.rs`. **Deps:** 1.4. **Verify:** `cargo build -p harness-validator` exits 0 and `$env:OUT_DIR`/`schema_table.rs` is generated.
- [ ] 2.2 Implement `src/registry.rs`: one `OnceLock` `jsonschema::Registry` from the embedded table, `.add()` all 10 `$id`s, `.prepare()`; per-document wrapper `{"$ref": $id}` via `draft202012::options().with_registry(&r).offline().build()`. **Files:** `packages/validator/src/registry.rs`. **Deps:** 2.1. **Verify:** unit test resolves a `ref` instance offline → valid.
- [ ] 2.3 Implement `src/parse.rs`: JSON + YAML **1.2** (`serde-saphyr`) → `serde_json::Value`; duplicate keys fail-closed (`parse.duplicate_key`), multi-doc rejected (`parse.multiple_documents`), BOM at stream start only, no silent coercion, bounded aliases. **Files:** `packages/validator/src/parse.rs`. **Deps:** 1.4. **Verify:** `cargo test -p harness-validator parse` → dup-key + multi-doc + implicit-typing cases pass.
- [ ] 2.4 Implement `src/document.rs`: read `apiVersion`; kind detection (manifest vs model-contract vs install-plan) via root-schema discriminators; 0/>1 match → `document.unknown_kind` (spec code). **Files:** `packages/validator/src/document.rs`. **Deps:** 2.2. **Verify:** unit test for unknown/ambiguous kind.
- [ ] 2.5 Implement `src/diagnostics.rs`: `Diagnostic{path,code,message}`, `Severity`, `Code`, `Report{status,errors,warnings}`; total ordering (layer → JSON Pointer → code); JSON-keyword→own-`code` map with `schema.other` fallback. **Files:** `packages/validator/src/diagnostics.rs`. **Deps:** 1.4. **Verify:** ordering unit test passes (errors before warnings, then path, then code).
- [ ] 2.6 Implement `src/structural.rs`: run the wrapper validator, map `error.instance_path()` → JSON Pointer and absolute/keyword → own `schema.*` code. **Files:** `packages/validator/src/structural.rs`. **Deps:** 2.2, 2.5. **Verify:** `metadata.name` pattern violation → `{path:"/metadata/name", code:"schema.pattern"}`.
- [ ] 2.7 Promote the corpus (move, not copy) `archive/2026-09-26-harness-schema-v1alpha1/examples/{corpus.json,positive/,negative/}` → `packages/validator/tests/fixtures/`; rewrite `file` paths to `positive/...`/`negative/...` (keep `schema` `$id`s and `expect` blocks golden). **Files:** `packages/validator/tests/fixtures/**`. **Deps:** 1.4. **Verify:** `git status --short` shows renames; `(Get-ChildItem packages/validator/tests/fixtures -Recurse -File).Count` → 28 (corpus + 23 instances + README if added).
- [ ] 2.8 Write `tests/corpus.rs`: parse `corpus.json`, map corpus `keyword`→own `code`, assert 7 positives → `status=valid`; 16 negatives → `status=invalid` **and** ≥1 diagnostic matching `{path, code}`. **Files:** `packages/validator/tests/corpus.rs`. **Deps:** 2.6, 2.7. **Verify:** `cargo test -p harness-validator --test corpus` → `positive=7 negative=16 mismatches=0`.
- [ ] 2.9 (verification/adversarial) Broken-registry fixtures: missing `$id` in `registry.json`/disk → build fails or `schema.unresolved_ref` + exit 2; assert multiple structural errors aggregate (no short-circuit). **Files:** `packages/validator/tests/adversarial.rs`. **Deps:** 2.6. **Verify:** `cargo test -p harness-validator --test adversarial` → both cases pass.

## Phase 3: Semantic layer (PR 3, F2-06)

- [ ] 3.1 Implement `src/semantic/mod.rs`: run document-local rules in fixed order; aggregate (never short-circuit); emit `semantic.not_evaluated` warning naming the owning phase (F4/F7/F15) for deferred checks. **Files:** `packages/validator/src/semantic/mod.rs`. **Deps:** 2.5. **Verify:** deferred-check fixture yields a warning, `status` unaffected.
- [ ] 3.2 Implement `src/semantic/identity.rs`: canonical-ref grammar, host consistency, short-ref-forbidden → `semantic.identity_ref_short_forbidden`, `semantic.identity_host_mismatch`. **Files:** `packages/validator/src/semantic/identity.rs`. **Deps:** 3.1. **Verify:** unit tests per code.
- [ ] 3.3 Implement `src/semantic/dependencies.rs`: ordered `extends`, kind-composition (`semantic.kind_composition`), self/known-set cycle (`semantic.dependency_cycle`), ordering (`semantic.dependency_order`); full resolver deferred → warning. **Files:** `packages/validator/src/semantic/dependencies.rs`. **Deps:** 3.1. **Verify:** cycle + order fixtures rejected with expected code.
- [ ] 3.4 Implement `src/semantic/components.rs`: per-type rules, requirements↔components coherence, env value-like (`semantic.env_value_like`; never echo the value), model `license` shape (`semantic.license_shape`, OPEN §58). **Files:** `packages/validator/src/semantic/components.rs`. **Deps:** 3.1. **Verify:** `KEY=value` entry → `semantic.env_value_like` at that pointer; Report contains no value.
- [ ] 3.5 Implement `src/semantic/permissions.rs`: permission coverage for non-Passive (`semantic.permission_coverage`), effective risk = max over parts (`semantic.effective_risk`), autonomy ≥ floor (`semantic.autonomy_below_floor`), monotonicity. **Files:** `packages/validator/src/semantic/permissions.rs`. **Deps:** 3.1. **Verify:** autonomy-below-floor + coverage fixtures rejected.
- [ ] 3.6 (verification/adversarial) Negative fixtures for each semantic rule with explicit expected `{path, code}` (dep cycle, coverage violation, autonomy floor, env value, license conflict/not-evaluated). **Files:** `packages/validator/tests/fixtures/negative/*.yaml|json`, `packages/validator/tests/adversarial.rs`. **Deps:** 3.2–3.5. **Verify:** `cargo test -p harness-validator` → all semantic cases green.

## Phase 4: Capability + version layers (PR 4, F2-07/F2-11) — Q9 resolved

- [ ] 4.1 Add checked-in `schemas/capabilities.json` (known-capability registry; documented default/empty set, **contents OPEN §58**); embed it in `build.rs`; missing/unreadable → loud failure, never "all known". **Files:** `schemas/capabilities.json`, `packages/validator/build.rs`. **Deps:** 2.1. **Verify:** `python scripts/schemas/check_schemas.py` still `OK`; removing the file makes `cargo build` fail.
- [ ] 4.2 Implement `src/capability.rs`: lookup each declared capability id; absent → `capability.unknown` error naming the id; registry missing → loud failure (exit 2). **Files:** `packages/validator/src/capability.rs`. **Deps:** 4.1. **Verify:** unknown-capability fixture → `capability.unknown`; missing-registry test fails loudly.
- [ ] 4.3 Implement `src/version.rs`: supported set `{thisismyharness.dev/v1alpha1}`; missing → `version.missing`; outside set → `version.unsupported` at `/apiVersion` naming the set; static `Migration::None` stub; **unsupported is an ERROR (evaluated & invalid, exit 1), NOT exit 2**; §58 OPEN. **Files:** `packages/validator/src/version.rs`. **Deps:** 2.4. **Verify:** unknown-apiVersion negative → `status=invalid`, `code=version.unsupported`; supported version accepted.
- [ ] 4.4 Implement the dedupe rule: when `version.missing`/`version.unsupported` fires, suppress the L1 `schema.required`/`schema.const` on `/apiVersion` (one issue → one code). **Files:** `packages/validator/src/version.rs`, `src/lib.rs`. **Deps:** 4.3. **Verify:** unknown-apiVersion fixture yields exactly one `version.*` diagnostic, no `schema.const`.
- [ ] 4.5 **(Q9 — SPEC WINS)** Correct `design.md` text so no downstream agent implements exit 2: §6 exit table + §11 flow must state unsupported/missing `apiVersion` → **exit 1**, `status: invalid`; align the layer order to the spec (`structural → version → semantic → capability → filesystem`); align code catalogue to spec (`parse.multiple_documents`, `document.unknown_kind`, add `schema.unresolved_ref`). **Files:** `openspec/changes/f2-validator-rust/design.md`. **Deps:** —. **Verify:** `Select-String -Path design.md -Pattern "version.*exit 2"` → no match; "version.unsupported" row reads exit 1 / invalid.
- [ ] 4.6 (verification/adversarial) CLI-level proof of Q9: unknown + missing `apiVersion` fixtures via `assert_cmd` → **exit 1** (not 2) with `version.unsupported`/`version.missing`. **Files:** `packages/cli/tests/cli.rs` (or `packages/validator/tests/cli.rs`). **Deps:** 4.3, 5.4. **Verify:** `cargo test --test cli` → both assertions pass.

## Phase 5: Declared-path safety + CLI (PR 5, F2-08/F2-10)

- [ ] 5.1 Implement `src/pathsafe.rs`: declared-path safety within a provided project root — reject `..` escape (`path.traversal`), absolute (`path.absolute`), symlink escaping root (`path.symlink_escape`); never follow links outside root. **Files:** `packages/validator/src/pathsafe.rs`. **Deps:** 2.5. **Verify:** traversal/absolute/symlink fixtures rejected with expected codes.
- [ ] 5.2 Implement the public entry `validate(&Source, &Options) -> Report` in `src/lib.rs` (pure: no process exit, no stdout) wiring all layers. **Files:** `packages/validator/src/lib.rs`. **Deps:** 2.6, 3.1, 4.2, 4.3, 5.1. **Verify:** `cargo test -p harness-validator` → full suite green.
- [ ] 5.3 Implement `packages/cli/src/main.rs` with `clap` derive: `harness validate <path>` (file or directory), `--json`, `--kind <kind>`; usage/missing-path → exit 2; human output → **stderr**, `--json` → **stdout**; read-only. **Files:** `packages/cli/src/main.rs`. **Deps:** 5.2. **Verify:** `harness validate <positive fixture>` exits 0, stdout empty; `--json` prints one object.
- [ ] 5.4 Wire the exit/status taxonomy and assert agreement: `valid→0`, `invalid→1`, `error→2`; `--json` `status ∈ {valid,invalid,error}`; unsupported/missing `apiVersion` → `invalid`/exit 1; unreadable/unparsable/unknown-kind/usage → `error`/exit 2. **Files:** `packages/cli/src/main.rs`. **Deps:** 5.3. **Verify:** exit code always equals `status` on every `--json` run.
- [ ] 5.5 Write `tests/cli.rs` (`assert_cmd`): exit `0/1/2`, stdout/stderr split, golden `--json` snapshot for one positive + one negative. **Files:** `packages/cli/tests/cli.rs`, `packages/cli/tests/snapshots/*`. **Deps:** 5.3, 5.4. **Verify:** `cargo test -p harness` → CLI suite green.
- [ ] 5.6 (verification/adversarial) Unreadable path, unparsable bytes, unknown kind, directory discovery, `..`/absolute/symlink path, inline hook and plan `timestamp` fixtures → expected exit code + `{path, code}`. **Files:** `packages/validator/tests/adversarial.rs`, `packages/validator/tests/fixtures/negative/*`. **Deps:** 5.1, 5.4. **Verify:** `cargo test -p harness-validator --test adversarial` → green.
- [ ] 5.7 (verification/adversarial) Read-only proof: hash the project dir contents before/after `harness validate`; assert unchanged, no file created/modified. **Files:** `packages/cli/tests/cli.rs`. **Deps:** 5.3. **Verify:** before/after listing identical; test exits 0.

## Phase 6: Docs, bookkeeping, CI, gates (PR 6, F2-09/F2-12)

- [ ] 6.1 Write `packages/validator/README.md` (F2-12): what "valid" means (layers run), what is **NOT** checked (graph/digest/trust/plan-determinism → F4/F7/F15), exit codes `0/1/2`, `--json` shape + full `code` catalogue, coverage table, Install-Plan-local-only note, precise trust statement (no "100% safe"/conformance), how to run `cargo test`/`clippy`/`fmt --check` and add a fixture. **Files:** `packages/validator/README.md`. **Deps:** 5.2. **Verify:** `npx markdownlint-cli2 packages/validator/README.md` → 0 errors.
- [ ] 6.2 Write `docs/adr/0003-rust-validator-architecture.md` per `docs/adr/0000-template.md` capturing design §10 options/tradeoffs (layered vs merged, separate crate, embed, offline registry, YAML 1.2, own code namespace). **Files:** `docs/adr/0003-rust-validator-architecture.md`. **Deps:** —. **Verify:** `npx markdownlint-cli2 docs/adr/0003-*.md` → 0 errors.
- [ ] 6.3 R3 bookkeeping: flip `r3=pending → confirmed` in `schemas/README.md` (§Pre-freeze blocker + resolver row) and `docs/adr/0002-schema-strategy.md` ("Status of related decisions"), citing the executed evidence (jsonschema 0.58.1, 7/7 valid, 16/16 rejected, 0 mismatch, offline). **Files:** `schemas/README.md`, `docs/adr/0002-schema-strategy.md`. **Deps:** 2.8. **Verify:** `python scripts/schemas/check_schemas.py --freeze` → `FREEZE OK` (exit 0).
- [ ] 6.4 Publish the ONE authoritative layer→task matrix (proposal Traceability table) in `schemas/README.md` (replace the semantic-boundary Owner column) and `packages/validator/README.md`; correct stale owners (path-safety stays F2-08; plan determinism → OUT F4; trust → OUT F7/F15; digest → OUT F7). **Files:** `schemas/README.md`, `packages/validator/README.md`. **Deps:** 6.1. **Verify:** `python scripts/schemas/check_schemas.py --readme` → `README OK sections=7`; owner numbers match the matrix (spot-check F2-08/F2-09/F2-10 rows).
- [ ] 6.5 Replace the CI placeholder `schema-validation` job with a Rust job: `cargo build --locked`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --all -- --check` on ubuntu (setup Rust stable). **Files:** `.github/workflows/ci.yml`. **Deps:** 5.5. **Verify:** `cargo fmt --all -- --check` and `cargo clippy --workspace -- -D warnings` both exit 0 locally.
- [ ] 6.6 Repoint the independent L1/L2 gates to the promoted corpus home: `scripts/schemas/validate.mjs` + `cross_check.py` resolve `packages/validator/tests/fixtures/corpus.json` (retain archive fallback for audit); keep them as cross-checks, not retired. **Files:** `scripts/schemas/validate.mjs`, `scripts/schemas/cross_check.py`. **Deps:** 2.7. **Verify:** `npm run validate:schemas` → `PASS schemas=3 positive=7 negative=16`; `python scripts/schemas/cross_check.py` → same.
- [ ] 6.7 (verification) Full-gate evidence run: `cargo test --workspace` + `cargo clippy` + `cargo fmt --check` + L0/L1/L2; confirm §58 still OPEN (`open=9`), no vendor hard-coded, no F1 `MUST` weakened, R3 confirmed. **Files:** —. **Deps:** 6.3–6.6. **Verify:** all commands exit 0; `check_schemas.py` prints `open=9 vendor=0`.
- [ ] 6.8 (verification/adversarial) Doc-consistency assertions: `design.md` no longer states exit 2 for `version.unsupported`; `schemas/README.md` Owner column equals the published matrix; README names the deferred checks with owning phases. **Files:** —. **Deps:** 4.5, 6.4. **Verify:** grep assertions pass (`version.unsupported` → invalid/exit 1; each `OUT` row names F4/F7/F15).

## Open Questions (carried — MUST NOT be silently resolved)

- **Q9 conflict:** RESOLVED here in favor of the spec (exit 1); task 4.5 corrects the design text.
- **Layer order + code catalogue drift:** spec order is `structural → version → semantic → capability → filesystem` (design §4/§11 differ); spec codes `parse.multiple_documents` / `document.unknown_kind` (design says `multi_document` / `kind_unknown`). Task 4.5 aligns; `kind_ambiguous` stays a design-only extension.
- **§58 items** (capability-registry contents, migration algorithm, media type, host, filename) stay OPEN; `schemas/capabilities.json` content is a documented default, not a normative closure.
- **`windows-gnu` `as` fix ownership (Q13):** pins are a stopgap; environment fix required before F4.

## Dependencies / order

`1 → 2 → {3, 4} → 5 → 6`. Phase 3 and Phase 4 may proceed in parallel after Phase 2; Phase 5 needs 4.3 (version → exit taxonomy) and 5.1; Phase 6 needs 5.5. Task 4.5 (design correction) is order-free but MUST land before any code reads the design for exit semantics.
