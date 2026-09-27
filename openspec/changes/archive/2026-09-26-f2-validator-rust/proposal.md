# Proposal: f2-validator-rust (F2-05..F2-12) — the real validator and `harness validate`

> **Change:** `f2-validator-rust` · **Phase:** F2 · **apiVersion:** `thisismyharness.dev/v1alpha1` · **Risk class:** A (Passive) · **Default autonomy:** 0 (Preview) · **Status:** pre-alpha.
> **Surfaces:** Toolchain (`packages/**`); Spec read-only (`schemas/**`; minimal R3 bookkeeping in `schemas/README.md` + `docs/adr/0002`). No Adapters, no Distribution.
> **Does NOT alter** the manifest, package layout, install protocol, or compatibility model.
> **Normative language:** RFC 2119. Sources: `spec/**` (F1), `schemas/**`, `docs/adr/0001`, `docs/adr/0002`; motivation §2.6, §4, §22, §24, §47, §50, §52, §58.

## Intent

F1 is complete and F2-01..04 froze the schemas, yet **nothing validates an artifact**: `packages/**` are empty, the L1/L2 gates were throwaway cross-checks now broken by archiving, and **R3** (Rust `jsonschema` support for Draft 2020-12 + `unevaluatedProperties`) blocks the schema freeze. This change implements the Rust validator (ADR-0001) and `harness validate`, making the standard testable and clearing R3 with executed evidence (7/7 valid, 16/16 rejected, 0 keyword mismatch, offline).

## Scope

### In Scope
- Root Cargo workspace: `packages/core` (placeholder lib), `packages/validator`, `packages/cli`; `packages/sdk` reserved.
- `harness-validator` lib crate: structural (JSON Schema), semantic (document-local), capability, version, filesystem (declared-path) layers.
- `harness` CLI bin: `harness validate <path>` (file or directory), exit codes `0/1/2`, `--json` to stdout, human output to stderr, **READ-ONLY**.
- Corpus-driven `cargo test` (7 positive / 16 negative) + adversarial input fixtures.
- `apiVersion` gate + static migration stub (**§58 stays OPEN**).
- `packages/validator/README.md`: what "valid" means, exit codes, `--json` shape + `code` catalogue, coverage, trust statement.
- R3 bookkeeping: `r3=pending → confirmed` in `schemas/README.md` + `docs/adr/0002`; publish **ONE** authoritative layer→task matrix.

### Out of Scope
- Graph/cycle resolution across harnesses, digest/immutability, trust/declared-vs-verified, Install-Plan determinism → **F4/F5/F7/F15**.
- Resolving any §58 item; `packages/sdk` code; adapters; any mutation (`harness use`).
- JS/Python gates: **kept as independent cross-checks** (their archive-path regression is fixed here). They MUST NOT be retired until `cargo test` is the documented primary gate.

## Capabilities

### New Capabilities
- `harness-validator`: layered schema/semantic/capability/version validation with typed diagnostics and a stable `Report` contract.
- `harness-cli`: the `harness validate` command — path semantics, exit-code taxonomy, `--json` output.

### Modified Capabilities
- `schema-strategy`: R3 freeze record flips to `confirmed`; the semantic-boundary Owner column is superseded by the authoritative layer→task matrix.

## Approach

- **Workspace/crates.** `harness-validator` is its own crate (decoupled, testable, lets `harness-core` depend on it later without a cycle). All logic in the library; the CLI owns args/output/exit codes.
- **Engine.** `jsonschema 0.58` (`default-features=false`) + `.offline()`; one `Registry` built from `schemas/registry.json`; a `$ref` absent from the registry **MUST** fail loudly (no network).
- **Schema loading.** Embed `schemas/**` + `registry.json` at compile time (`build.rs` / `include_str!`); schemas stay in `schemas/` as the single source (ADR-0002).
- **Parsing.** YAML **1.2** parser (evaluate `serde-saphyr` / `yaml_serde`; `serde_yaml` and `serde_yml` are deprecated); duplicate keys fail-closed; no silent coercion; multi-doc rejected.
- **Layers.** structural → semantic (document-local) → capability → version → filesystem, governed by the authoritative IN/PARTIAL/OUT matrix (§6).
- **Diagnostics.** Typed `{path, code, message}` in our own `code` namespace; `path` is a JSON Pointer matching ajv `instancePath` so corpus expectations stay golden.
- **CLI contract.** Exit `0` valid · `1` invalid · `2` input/usage/IO; `--json` `{status, errors[], warnings[]}` to stdout; read-only (risk A, autonomy 0).
- **Toolchain prerequisite.** `windows-gnu` ships `dlltool` but no GNU `as`; `raw-dylib` crates fail. Resolve (install MinGW `as` / MSVC) or carry transitive pins — **MUST** be fixed before F4.

## Affected Areas

| Area | Impact | Description |
|------|--------|-------------|
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | New | Root workspace |
| `packages/validator/**` | New | `harness-validator` crate + `README.md` + `tests/fixtures/` |
| `packages/cli/**` | New | `harness` bin — `validate` |
| `packages/core/Cargo.toml` | New | Placeholder lib `harness-core` |
| `schemas/README.md`, `docs/adr/0002*` | Modified | R3 → `confirmed`; matrix correction |
| `scripts/schemas/{validate.mjs,cross_check.py}` | Modified | Fix archive-path regression |
| `.github/workflows/ci.yml` | Modified | Placeholder → `cargo test` |

## Guarantees that MUST NOT regress

- The validator **MUST NOT** weaken, contradict, or permit bypass of any F1 `MUST`.
- **§58 stays OPEN**; no new grammar, host, media type or migration algorithm is invented.
- `environmentVariableNames` are **NAMES only** — never request, echo or store values; no credential storage.
- No vendor name is hard-coded into the Core or any normative artifact.
- `harness validate` is **READ-ONLY**: no third-party execution, no filesystem mutation, no silent overwrite; declared-path traversal (`..`, absolute, symlink) is blocked.
- No trust claim beyond what was verified; never "100% safe"; no v1.0 claim.

## Traceability

| Layer | `schemas/README.md` boundary rows | F1 anchor | F2 task |
|---|---|---|---|
| L-structural | All schema-expressible constraints | `spec/manifest/README.md` §2, §5.3 | F2-05 |
| L-semantic | Effective risk/monotonicity/autonomy floor; permission coverage; refs; deps; env names; model license; Q7 | `permissions.md` §2/§4; `risk-classes.md` §3/§4; `dependencies.md` §2/§5; `identity.md` §2/§8; `component-types.md` §4.8 | F2-05/06/10/12 |
| L-capability | Capability existence vs extension registry | `requirements.md`; §4 | F2-07 |
| L-version | Supported `apiVersion` set + migration map | `versioning.md` §3; `manifest/README.md` §2.1 | F2-11 |
| L-filesystem | Package discovery, path safety, symlink containment | §50 | F2-08 |

## Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| Toolchain `as` gap blocks builds; recurs F4+ | High | Fix env before F4; document pins |
| Broken L1/L2 gates leave corpus ungated | Med | Fix paths in this change |
| Boundary Owner column over-promises | Med | Publish IN/PARTIAL/OUT matrix |
| Diagnostic `code`/`--json` churn | Med | Own namespace; golden snapshot |
| YAML 1.1/1.2 divergence | Med | Pin 1.2; implicit-typing fixture |
| Windows-only local vs ubuntu CI | Med | Prove cross-platform build in CI |
| >400 changed lines | Med | Chained PRs |
| Accidental §58 resolution | Low | Explicit OPEN guards |

## Rollback Plan

Additive crates + a new CLI; **nothing replaces existing behavior**. Revert by commit/PR; no install/apply/revert/trust/supply-chain surface is touched. The only spec-adjacent edits (`schemas/README.md` R3 line, `docs/adr/0002` status) revert in the same range. The JS/Python gates remain as independent cross-checks unless explicitly retired, so reverting loses no coverage.

## Dependencies

- Rust toolchain (present: `cargo 1.98.1`); `jsonschema 0.58.1`.
- A maintained YAML 1.2 crate (choice open, Q7).
- Corpus promoted from the archive to a stable home (Q1).
- MinGW `as` / MSVC decision before F4 (Q13).

## Open Questions (carried from exploration; MUST NOT be silently resolved)

Q1 fixture home after archive · Q2 validator crate boundary · Q3 schema embed vs disk · Q4 R3 bookkeeping ownership · Q5 ratify IN/PARTIAL/OUT + fix README Owner column · Q6 capability-registry source (F2-07) · Q7 YAML crate/version · Q8 `validate <path>` file-vs-dir + kind detection · Q9 exit-code taxonomy (unsupported `apiVersion` = 1 or 2) · Q10 `--json` stability · Q11 L1/L2 fate · Q12 CI `cargo test` · Q13 Windows `as` gap ownership · Q14 MSRV / toolchain pin.

## Validation Strategy

- `cargo test` drives the corpus: positive ⇒ valid; negative ⇒ invalid **and** a diagnostic matching `{path, keyword→code}`.
- Adversarial fixtures: malformed YAML (dup keys, anchors, implicit typing), broken `$ref`, unknown `apiVersion`/`kind`/capability, dependency cycle, permission-coverage violation, autonomy below floor, `KEY=value` env, inline hook, plan timestamp, `..`/absolute/symlink path — each with expected `{path, code}`.
- `assert_cmd` CLI end-to-end: exit codes `0/1/2`, `--json` shape + golden snapshot, stdout/stderr split.
- `cargo fmt --check` + `cargo clippy`.

## Success Criteria

- [ ] `cargo test` reproduces 7/7 positive + 16/16 negative with zero unexpected diagnostics.
- [ ] `harness validate <path>` returns 0/1/2 correctly and `--json` matches the documented shape.
- [ ] R3 = `confirmed`; the freeze gate reads `FREEZE OK`.
- [ ] No F1 `MUST` weakened; §58 still OPEN; no vendor hard-coded.
- [ ] `packages/validator/README.md` states exactly what is and is not checked.
