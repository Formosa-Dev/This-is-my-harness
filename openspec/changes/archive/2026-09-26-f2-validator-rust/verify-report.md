# Verify Report — `f2-validator-rust` (F2-05..F2-12)

**Verdict: PASS** · verified inline by the orchestrator (the `sdd-verify` sub-agent could not run: provider `Insufficient Balance`).
**Date**: 2026-09-26 · **Revision verified**: `3375efa` (6/6 work units, all 41 tasks `[x]`).
**Mode**: strict TDD inactive (`strict_tdd: false`, no test runner); verification = executing the real gates.

All commands below were re-run independently. Outputs are the real captured results.

---

## 1. Rust quality gates

| Command | Real output | Exit |
| --- | --- | --- |
| `cargo build --workspace` | `Finished dev profile ... in 1.03s` | 0 |
| `cargo test --workspace` | cli **15** · validator lib **66** · adversarial **37** · corpus **2** — all `ok`, 0 failed | 0 |
| `cargo fmt --all -- --check` | (no output) | 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | `Finished ... in 0.62s` (no warnings) | 0 |

The corpus suite asserts the frozen expectations: `positive=7 negative=16 mismatches=0`.

## 2. Schema gates (L0 / L1 / L2)

| Command | Real output | Exit |
| --- | --- | --- |
| `python scripts/schemas/check_schemas.py` | `OK schemas=3 defs=7 refs=38 open=9 vendor=0 unresolved=0 cycle=0` | 0 |
| `check_schemas.py --freeze` | `FREEZE OK r3=confirmed` | 0 |
| `check_schemas.py --trace` | `TRACE OK rows=40 untraced=0` | 0 |
| `check_schemas.py --adr` | `ADR OK headings=7` | 0 |
| `check_schemas.py --trust` | `TRUST OK` | 0 |
| `check_schemas.py --guards` | `GUARDS OK model_no_component=1 install_leaf_refs=14 cycle=0 unresolved=0` | 0 |
| `check_schemas.py --readme` | `README OK sections=7` | 0 |
| `npm run validate:schemas` (L1, ajv) | `PASS schemas=3 positive=7 negative=16` | 0 |
| `python scripts/schemas/cross_check.py` (L2, jsonschema) | `PASS schemas=3 positive=7 negative=16` | 0 |

L1 and L2 (two independent JSON Schema engines) agree on the same corpus.

## 3. CLI end-to-end (`target/debug/harness.exe`) — exit-code taxonomy

| Case | Exit | Diagnostic |
| --- | --- | --- |
| valid `manifest.minimal.yaml` | **0** | `{"status":"valid","errors":[],"warnings":[]}` |
| `manifest.bad-slug.json` | **1** | `{path:"/metadata/name", code:"schema.pattern"}` |
| `manifest.unknown-apiversion.json` | **1** | `{path:"/apiVersion", code:"version.unsupported"}` ← **Q9** |
| `manifest.missing-apiversion.json` | **1** | `{path:"/apiVersion", code:"version.missing"}` ← **Q9** |
| `manifest.unknown-kind.json` | **2** | `{code:"document.unknown_kind"}` |
| `path.traversal.json` | **1** | `{path:"/spec/components/0/path", code:"path.traversal"}` |
| unparsable YAML (synthetic) | **2** | `{code:"parse.invalid"}` |
| directory without a canonical manifest name | **2** | `{code:"io.read_failed"}` |

- `--json` → documented object on **stdout**; human mode → **stderr** (verified: stdout length = 0 in human mode).
- **READ-ONLY proven**: after invoking `validate`, `git status --short packages/validator/tests/fixtures` is empty.

## 4. Adversarial non-vacuity (independent mutation)

Reproduced the positive manifest as a temp copy and mutated it:

| Instance | Exit | Observation |
| --- | --- | --- |
| byte-copy of a positive | **0** | validates |
| same copy + one injected unknown top-level field | **1** | `code: schema.unevaluatedProperties` |

Same base, one change → different verdict: the corpus/tests are **not vacuous**.

## 5. Spec compliance (walked against the delta specs)

- `harness-validator` — schema loading/resolution offline; the five layers; typed `{path, code, message}` diagnostics; deterministic ordering; READ-ONLY; corpus reuse. **Satisfied.**
- `harness-cli` — `harness validate <path>`; file vs directory; `--kind`; exit 0/1/2; human→stderr, `--json`→stdout; the exact `--json` shape. **Satisfied.**
- `schema-strategy` (MODIFIED) — **R3 resolved/confirmed**: the Rust `jsonschema 0.58.1` engine reproduces the ajv/L2 corpus offline (`7/16`, 0 keyword mismatches); `--freeze` is now a real gate returning `FREEZE OK r3=confirmed`; the single authoritative layer→task matrix is published and the `schemas/README.md` Owner column was reconciled to it. **Satisfied.**

## 6. Invariants checked

- **§58 stays OPEN**: `open=9` preserved; only `apiVersion` is a `const`; no host / media-type fixed.
- **Q9 (spec wins)**: unsupported/missing `apiVersion` → exit **1**. Confirmed at the CLI; confirmed `design.md` no longer ties `apiVersion`/`version.*` to exit 2 (the remaining `exit 2` in the design is the correct kind-detection case).
- **Protected paths untouched**: `git status --short -- spec openspec/changes/archive` is empty.
- **No vendor names in normative requirements**: the only match is the deliberate, pre-existing `spec/glossary.md` example — `models.system-one` is a capability and `"Laya"` is *one* possible implementation (a "NOT a brand" contrast), not a hardcoded requirement.

## 7. Findings

- **CRITICAL**: none.
- **WARNING**: none.
- **SUGGESTION** (carried, out of scope): (a) the repo-wide markdown-lint baseline was already red before this change (pre-existing ADR/openspec lint issues) — not a regression; (b) the `windows-gnu` toolchain still lacks GNU `as`, so the current dependency pins are a stopgap — this is a separate, explicitly-open environment prerequisite (**Q13**) that must be resolved before F4 (which will pull `tokio`), and is deliberately kept distinct from R3.

## 8. Carried open items

- **Q13** — `windows-gnu` missing `as` / `raw-dylib`: pins are a stopgap; a real environment fix (install MinGW binutils or migrate to MSVC + VS Build Tools) is owned before F4.
- §58 open positions (capability-registry *contents*, OCI media type, canonical host, manifest filename, migration algorithm) remain OPEN by design.
- The validator's OUT-of-scope checks (graph/cycle resolution across harnesses, digest/immutability, trust/declared-vs-verified, Install-Plan determinism) are surfaced as `semantic.not_evaluated` warnings naming their owning phase (F4/F7/F15) — no silent pass.
