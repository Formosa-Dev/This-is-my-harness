# Tasks: harness-schema-v1alpha1 (F2-01..F2-04)

> Module `schemas` · Phase **F2** · Risk **A** / autonomy **0** · Normative source `spec/**` (F1). Examples under the change folder are **NON-NORMATIVE**. Do NOT edit `spec/**`, `examples/**`, `packages/**`. Verification = the exact commands below; code-like artifacts MUST be run before commit.

## Review Workload Forecast

| Field | Value |
|-------|-------|
| Estimated changed lines | ~1 900–2 300 |
| 400-line budget risk | High |
| Chained PRs recommended | Yes |
| Suggested split | PR1 harness → PR2 defs → PR3 manifest → PR4 model+plan → PR5 cross-check/ADR |
| Delivery strategy | ask-on-risk |
| Chain strategy | `stacked-to-main` (recommended; confirm) |

Decision needed before apply: Yes
Chained PRs recommended: Yes
Chain strategy: stacked-to-main
400-line budget risk: High

### Suggested Work Units

| Unit | Goal | PR | Base |
|------|------|----|------|
| 1 | Verification harness (L0/L1) + `package.json` + registry + strategy draft | PR 1 | `main` |
| 2 | Shared `schemas/defs/*` (7) + full resolver map + defs meta-negatives | PR 2 | PR 1 |
| 3 | Manifest root schema + 2 positive / 5 negative instances | PR 3 | PR 2 |
| 4 | Model-contract (+Router) + Install Plan + 3 positive / 4 negative | PR 4 | PR 3 |
| 5 | L2 cross-check + F1 traceability matrix + ADR-0002 + freeze gate | PR 5 | PR 4 |

Alternative chain: `feature-branch-chain` with a `harness-schema-v1alpha1` tracker branch (better rollback). Confirm before apply. Every PR ≤ ~500 lines and independently verified.

## Phase 1 — F2 `schemas`: verification harness + wiring (PR 1)

- [x] 1.1 Create `package.json` + lockfile (private; devDeps `ajv@^8`, `ajv-formats@^3`, `yaml@^2`; script `validate:schemas`). Files: `package.json`, `package-lock.json`. Deps: none. **Verify:** `npm install`; `npm ls ajv` → prints `ajv@8.x`. @2026-09-26
- [x] 1.2 Create `schemas/registry.json` (`base` = `https://thisismyharness.dev/schemas/v1alpha1/`, empty `$id`→path map). Deps: none. **Verify:** `python -c "import json;print(json.load(open('schemas/registry.json'))['base'])"` → prints the base URI. @2026-09-26
- [x] 1.3 Create `scripts/schemas/check_schemas.py` (L0, stdlib only: parse; `$schema` = 2020-12; every `$ref` ∈ registry and on disk; vendor scan of `const`/`enum`; §58 OPEN markers; README anchors; cycle check). Deps: 1.2. **Verify:** `python scripts/schemas/check_schemas.py` → exit 1, `FAIL missing: schemas/harness.v1alpha1.schema.json` (RED). @2026-09-26
- [x] 1.4 Create `scripts/schemas/validate.mjs` (ajv `dist/2020`, YAML+JSON load, corpus-driven) + `openspec/changes/harness-schema-v1alpha1/examples/corpus.json` (empty lists). Deps: 1.1. **Verify:** `npm run validate:schemas` → exit 1, `FAIL corpus empty: positive=0` (RED). @2026-09-26
- [x] 1.5 **Adversarial (pairs 1.3):** add `--self-test` to `check_schemas.py` with fixtures for vendor `const`, unresolved `$ref`, missing OPEN marker, forced cycle. Deps: 1.3. **Verify:** `python scripts/schemas/check_schemas.py --self-test` → `SELFTEST OK cases=4` exit 0. @2026-09-26
- [x] 1.6 Draft `schemas/README.md`: dialect, `$id` map, resolver map, strictness table, §58 OPEN table, semantic-boundary list (F2-05..12 owner per row), Install-Plan local-only note. Deps: 1.2. **Verify:** `python scripts/schemas/check_schemas.py --readme` → `README OK sections=7`. @2026-09-26

## Phase 2 — F2 `schemas`: shared `$defs` (PR 2)

- [x] 2.1 `schemas/defs/identity.schema.json` (slugName 1–64 no `/`, owner, scopedReference, canonicalIdentifier **host-variable OPEN**, semver, versionRange) + `schemas/defs/capability.schema.json` (capabilityId pattern, capabilityRequirement `{capability, preferred?, alternatives?}`). Deps: 1.3. **Verify:** L0 → `OK ... defs=2`. @2026-09-26
- [x] 2.2 `schemas/defs/requirement.schema.json` (runtime, runtimeVersion, modelCapabilities, `hardware` closed 7 fields, services, backends) + `schemas/defs/permission.schema.json` (permission, scope, riskClass A–D, autonomyLevel 0–3). Deps: 2.1. **Verify:** L0 → `OK ... defs=4`. @2026-09-26
- [x] 2.3 `schemas/defs/component.schema.json` (componentDescriptor + 8-type discriminator; **no `Tool` type**) + `schemas/defs/dependency.schema.json` (dependencyReference, override; ordered constraint noted). Deps: 2.1. **Verify:** L0 → `OK ... defs=6`. @2026-09-26
- [x] 2.4 `schemas/defs/distribution.schema.json` (digest, `artifactType` **pattern OPEN**, provenance, compatibilityLevel enum `native/adapted/partial/untested/unsupported`, trustLabel). Deps: 2.1. **Verify:** L0 → `OK ... defs=7 open=6`. @2026-09-26
- [x] 2.5 Populate `schemas/registry.json` with every `$id`→repo path (7 defs). Deps: 2.1–2.4. **Verify:** L0 → `OK schemas=0 defs=7 refs=0 open=6 vendor=0 unresolved=0 cycle=0`. @2026-09-26
- [x] 2.6 **Adversarial (pairs 2.1–2.5):** inject a vendor `const`, a duplicate definition, a forced component↔model cycle stub, and a broken `$ref`; assert each is reported. Deps: 2.5. **Verify:** each injection → exit 1 with the matching `FAIL vendor|duplicate|cycle|unresolved`; clean tree → exit 0. @2026-09-26

## Phase 3 — F2-01: manifest root schema (PR 3)

- [x] 3.1 `schemas/harness.v1alpha1.schema.json` root: `unevaluatedProperties:false`; 4 REQUIRED (`apiVersion`, `kind`, `metadata`, `spec`); `apiVersion` `const`; `kind` enum `[Harness, Component, Preset]`; closed `metadata` (`name` slug 1–64 no `/`, `version` SemVer, optional owner/description/license/author/homepage/repository/keywords). Deps: 2.5. **Verify:** L0 → `OK ... schemas=1 refs=3`. @2026-09-26
- [x] 3.2 Same file: closed `spec` with 8 OPTIONAL sections (`profile`, `components`, `requirements`, `permissions`, `extends`, `distribution`, `compatibility`, `conformance`); `extends` ordered array + optional `override`. Deps: 3.1. **Verify:** L0 → `OK ... refs=11`. @2026-09-26
- [x] 3.3 Positives `openspec/changes/harness-schema-v1alpha1/examples/positive/{manifest.minimal.yaml,manifest.rich.yaml}` (`spec:{}`; rich exercises all 8 sections). Deps: 3.2. **Verify:** `npm run validate:schemas` → `PASS ... positive=2`. @2026-09-26
- [x] 3.4 **Adversarial (pairs 3.1–3.2):** 5 negatives `examples/negative/manifest.*.json` — missing `apiVersion`, unknown top-level, unknown `metadata` field, bad slug, bad SemVer — each with expected `{path, keyword}`. Deps: 3.2. **Verify:** `npm run validate:schemas` → `PASS ... negative=5`; each record matches its expected path/keyword. @2026-09-26
- [x] 3.5 Wire corpus `positive`/`negative` entries. Deps: 3.3, 3.4. **Verify:** `npm run validate:schemas` → exit 0, `PASS schemas=1 positive=2 negative=5`. @2026-09-26

## Phase 4 — F2-03/F2-04: model contract + Install Plan (PR 4)

- [x] 4.1 `schemas/model-contract.schema.json`: `input`/`output` embedded JSON Schema (internals **OPEN**), `capabilities`, `resources` → hardware `$id`, `executionLocation` enum `[local, remote]`, service lifecycle 6 actions required when service-backed, `fallback`, `version` semver, `license` SPDX **OPEN**, `artifactSource` `{provider, digest, reference}` (no fixed provider), optional `router` block; **MUST NOT `$ref` the component descriptor**. Deps: 2.5. **Verify:** L0 → `OK ... schemas=2 cycle=0`. @2026-09-26
- [x] 4.2 `schemas/install-plan.schema.json`: 16 REQUIRED fields, closed plan/`risk`/`snapshot`/file/conflict entries, `environmentVariableNames` name-only pattern, **no timestamp**, leaf defs only (never harness root). Deps: 2.5. **Verify:** L0 → `OK ... schemas=3 refs=11`. @2026-09-26
- [x] 4.3 Positives `examples/positive/{model-contract.json,router.json,install-plan.json}` — canonical `input: state` / `output: {route, confidence}`. Deps: 4.1, 4.2. **Verify:** `npm run validate:schemas` → `PASS ... positive=5`. @2026-09-26
- [x] 4.4 **Adversarial (pairs 4.1–4.2):** 4 negatives — name-only model, bad `executionLocation`, plan missing one of 16, `environmentVariableNames` entry carrying a value (`KEY=sk-...`). Deps: 4.1, 4.2. **Verify:** `npm run validate:schemas` → `PASS ... negative=9`. @2026-09-26
- [x] 4.5 Cycle/leaf guards + full corpus. Deps: 4.3, 4.4. **Verify:** `python scripts/schemas/check_schemas.py --guards` → `GUARDS OK`; `npm run validate:schemas` → `PASS schemas=4 refs=11 positive=5 negative=9` exit 0 (L1 `schemas=4` = 3 roots + the `defs/` bundle, defined in the runner); mutate one negative → `FAIL <file> #/<ptr> keyword=<k> expected=<v>` exit 1. @2026-09-26

## Phase 5 — F2 `schemas`: cross-check, traceability, ADR, freeze (PR 5)

- [x] 5.1 `scripts/schemas/cross_check.py` (L2: `jsonschema>=4.18` + `referencing.Registry`). Deps: 4.5. **Verify:** `python -m pip install "jsonschema>=4.18"`; `python scripts/schemas/cross_check.py` → same `PASS ...` line, exit 0. @2026-09-26
- [x] 5.2 Complete the F1→constraint traceability matrix in `schemas/README.md` (every non-OPEN constraint → F1 anchor; a weakening constraint is rejected, `spec/**` never edited). Deps: 4.5. **Verify:** `python scripts/schemas/check_schemas.py --trace` → `TRACE OK rows=<n> untraced=0`. @2026-09-26
- [x] 5.3 `docs/adr/0002-schema-strategy.md` per `docs/adr/0000-template.md` (dialect, `$id`, modular defs, absolute-refs+resolver; rejected A1–A12). Deps: 4.5. **Verify:** `python scripts/schemas/check_schemas.py --adr` → `ADR OK headings=7`. @2026-09-26
- [x] 5.4 Record R3 as a pre-freeze blocker (Rust `jsonschema` 2020-12/`unevaluatedProperties` support unverified — Rust absent here) in ADR + `README.md`; note L1/L2 parity as the interim proof. Deps: 5.1, 5.3. **Verify:** `python scripts/schemas/check_schemas.py --freeze` → `FREEZE BLOCKED r3=pending` (expected until F2-05 confirms). @2026-09-26
- [x] 5.5 Trust statement + final three-layer gate. Deps: 5.2, 5.4. **Verify:** `python scripts/schemas/check_schemas.py` (L0), `npm run validate:schemas` (L1), `python scripts/schemas/cross_check.py` (L2) all exit 0; `--trust` → `TRUST OK` (no "100% safe"/conformance claim); evidence appended to the change. @2026-09-26
