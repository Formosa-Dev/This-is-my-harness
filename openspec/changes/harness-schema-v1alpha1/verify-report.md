# Verification Report — harness-schema-v1alpha1

**Change**: `harness-schema-v1alpha1` · **Phase**: F2 (F2-01..F2-04) · **Surface**: Spec (`schemas/`, `scripts/schemas/`, `docs/adr/`)
**Mode**: Standard (`strict_tdd: false`, no test runner — `openspec/config.yaml`)
**Verifier**: independent re-execution on a clean `main` (working tree clean; `origin/main == HEAD`, 0 ahead / 0 behind)
**Environment**: Node v22.17.0 · npm 10.8.2 · `ajv` 8.20.0 · Python 3.12.10 · `jsonschema` 4.26.0 · PyYAML 6.0.3 · Rust absent (R3)

> Re-run note: every gate and probe in this report was executed by the verifier against the committed tree. None of the apply-progress claims was trusted without re-execution.

---

## Completeness

| Metric | Value |
|--------|-------|
| Tasks total | 27 |
| Tasks complete | 27 (`[x]`) |
| Tasks incomplete | 0 |
| Work units committed | 5/5 (`bc91d6f`, `d6653a8`, `e875fbb`, `1f4b244`, `57b43dc`) |

---

## Gate Results (independent re-execution)

| # | Command | Real output | Exit |
|---|---------|-------------|------|
| 1 | `python scripts/schemas/check_schemas.py` | `OK schemas=3 defs=7 refs=38 open=9 vendor=0 unresolved=0 cycle=0` | 0 |
| 2 | `npm run validate:schemas` | `PASS schemas=3 positive=5 negative=9` | 0 |
| 3 | `python scripts/schemas/cross_check.py` | `PASS schemas=3 positive=5 negative=9` | 0 |
| 4 | `python scripts/schemas/check_schemas.py --guards` | `GUARDS OK model_no_component=1 install_leaf_refs=14 cycle=0 unresolved=0` | 0 |
| 5 | `python scripts/schemas/check_schemas.py --trace` | `TRACE OK rows=40 untraced=0` | 0 |
| 6 | `python scripts/schemas/check_schemas.py --adr` | `ADR OK headings=7` | 0 |
| 7 | `python scripts/schemas/check_schemas.py --freeze` | `FREEZE BLOCKED r3=pending` | 0 |
| 8 | `python scripts/schemas/check_schemas.py --trust` | `TRUST OK` | 0 |
| 9 | `python scripts/schemas/check_schemas.py --self-test` | `SELFTEST OK cases=5` | 0 |
| 10 | `python scripts/schemas/check_schemas.py --readme` | `README OK sections=7` | 0 |

**10/10 gates match the expected output exactly and exit 0.** `refs=38` verified independently (harness 19 + install-plan 14 + model-contract 5). `schemas=3` is confirmed as the count of root schemas exercised by the corpus, not the stale `4` pin.

---

## Spec Compliance Matrix

Legend: ✅ COMPLIANT · ⚠️ PARTIAL · ❌ FAILING

### schema-strategy

| Requirement | Scenario | Evidence | Result |
|---|---|---|---|
| Single machine-readable source of truth | A shared concept is defined once | L0 duplicate check + registry; `--self-test` `duplicate-definition` | ✅ |
| Single machine-readable source of truth | A parallel hand-written type is a divergence | No parallel types exist; rule stated in README/ADR-0002 | ✅ |
| Language-neutral artifacts | Rust + JS consumer reach same result | L1 (ajv) + L2 (Python jsonschema) agree on corpus; Rust unavailable (R3) | ⚠️ PARTIAL (R3) |
| `$id` namespace | Namespace is a schema address, not a host | `schemas/README.md` `$id` map + `identity.md` §8 OPEN | ✅ |
| Shared `$defs` and reference strategy | No duplicate definition | L0 `check_duplicates` + `--guards` | ✅ |
| Shared `$defs` and reference strategy | A cyclic reference chain is rejected | `--self-test` `forced-cycle`; L0 `cycle=0` | ✅ |
| Offline-resolvable references | Offline resolution succeeds | `registry.json` local map; L1/L2 run offline | ✅ |
| Offline-resolvable references | A broken `$ref` fails loudly | Adversarial mutation C → L0 `FAIL unresolved … (not declared in registry.json)`, exit 1 | ✅ |
| Closure for REQUIRED objects | Closure holds under composition | `unevaluatedProperties:false` on root/`metadata`/`spec`; negatives `unknown-toplevel`/`unknown-metadata` | ✅ |
| Closure for REQUIRED objects | OPEN positions stay permissive | Probes 3/4/8: unknown capability, `remote`, extra field all accepted; OPEN markers present | ✅ |
| F1 traceability, no weakened MUST | Every constraint is traceable | `--trace` 40 rows, untraced=0 | ✅ |
| F1 traceability, no weakened MUST | **A weakening constraint is rejected** | **`hooks` items not closed — inline executable content validates (probe 6)** | ❌ FAILING |
| §58 open items remain OPEN | OPEN is annotated, not decided | `open=9` markers; manifest filename, media type, host, capability internals, profiles, secrets | ✅ |
| §58 open items remain OPEN | Fixing a §58 item is rejected | Only `const` in the set is `apiVersion`; `artifactType` pattern; host variable | ✅ |
| Schema-versus-semantic boundary | The boundary is documented | README semantic table with F2-xx owners | ⚠️ PARTIAL (`inline executable content` absent from the list) |
| Verification evidence and trust signals | A negative example fails | 9 negatives; mutation B proves non-vacuity | ✅ |
| Verification evidence and trust signals | Trust statement is precise | `--trust` OK; README trust section names only real checks | ✅ |
| Artifact risk and default autonomy | Risk and autonomy are stated | README + ADR: risk A / autonomy 0 / no mutation | ✅ |

### manifest-schema

| Requirement | Scenario | Evidence | Result |
|---|---|---|---|
| Four top-level fields, all REQUIRED | A minimal valid manifest validates | `manifest.minimal.yaml` positive | ✅ |
| Four top-level fields, all REQUIRED | A missing REQUIRED field fails | `manifest.missing-apiversion.json` → `required` | ✅ |
| Four top-level fields, all REQUIRED | An unknown top-level field is rejected | `manifest.unknown-toplevel.json` → `unevaluatedProperties` | ✅ |
| Four top-level fields, all REQUIRED | A wrong type is rejected | Ad-hoc probe 1 → `/metadata type` (no committed fixture) | ✅ (UNTESTED fixture) |
| `apiVersion` single version | Supported version accepted | positive; `const` | ✅ |
| `apiVersion` single version | Range/unknown fails | Ad-hoc mutation A → `/apiVersion const`, exit 1 (no committed fixture) | ✅ (UNTESTED fixture) |
| `kind` recognized | Recognized accepted | positive | ✅ |
| `kind` recognized | Unknown kind rejected | `enum`; ad-hoc (no committed fixture) | ✅ (UNTESTED fixture) |
| `metadata` closed | name+version required | schema `required`; slug/SemVer negatives | ✅ |
| `metadata` closed | Malformed slug / invalid SemVer / unknown field rejected | `manifest.bad-slug`, `bad-semver`, `unknown-metadata` | ✅ |
| `spec` closed, 8 OPTIONAL | Empty spec valid | positive | ✅ |
| `spec` closed, 8 OPTIONAL | Malformed optional section rejected | schema section types; no committed fixture | ⚠️ PARTIAL |
| Single-entrypoint shape | One document validates | positive | ✅ |
| Single-entrypoint shape | Aggregate of manifests rejected | Ad-hoc probe 2 → `"" type` (no committed fixture) | ⚠️ PARTIAL |
| `extends` shape + cycle boundary | Ordered extends validates | `manifest.rich.yaml` | ✅ |
| `extends` shape + cycle boundary | Cyclic dependency is semantic | Listed in README boundary (F2-06); schema does not claim it | ✅ |
| No F1 MUST weakened | A semantic-only rule is listed | Permission coverage in README boundary | ✅ |

### model-contract-schema

| Requirement | Scenario | Evidence | Result |
|---|---|---|---|
| More than a name | Complete contract validates | `model-contract.json` positive | ✅ |
| More than a name | Name-only model rejected | `model-contract.name-only.json` → `required` | ✅ |
| I/O reuse JSON Schema, OPEN | Canonical example validates | `input: state` / `output: {route, confidence}` positive | ✅ |
| I/O reuse JSON Schema, OPEN | OPEN internals stay permissive | Probes 3/4 (extra internals accepted); OPN markers | ✅ |
| Capabilities are contracts, not vendors | Capability requirement validates | positives | ✅ |
| Capabilities are contracts, not vendors | Unknown capability accepted | Ad-hoc probe 3 → `[]`; `capabilityId` has no enum | ✅ |
| Capabilities are contracts, not vendors | Vendor `const`/`enum` rejected | L0 vendor scan; mutation D → `FAIL vendor`, exit 1; `--self-test` | ✅ |
| Resource contract shared | Resources resolve to shared hardware | `resources` `$ref` → `requirement#/$defs/hardware`; no redefinition | ✅ |
| Execution location local/remote | Recognized accepted | positives use `local`; probe 4 `remote` → `[]` | ✅ |
| Execution location local/remote | Unrecognized rejected | `bad-execution-location.json` → `enum` | ✅ |
| Lifecycle complete when service | Complete service lifecycle validates | **No positive model-contract fixture carries `lifecycle`** | ⚠️ PARTIAL |
| Lifecycle complete when service | Partial lifecycle rejected | Ad-hoc probe 5 → `/lifecycle required` ×4 | ✅ (UNTESTED fixture) |
| Fallback declared | Fallback accepted | positives | ✅ |
| Version and license | Version + license validate | `model-contract.json` (SemVer + `Apache-2.0`) | ✅ |
| Version and license | Contradictory license flagged | Rule stated in field descriptions; absent from README semantic table | ⚠️ PARTIAL |
| Artifact source is data | Artifact source validates | positives | ✅ |
| Artifact source is data | Fixed provider rejected | `provider` is a free string; no enum/const | ✅ |
| Router is a Model specialization | Router block validates | `router.json` positive | ✅ |
| Router is a Model specialization | No cycle with component descriptor | `--guards` `model_no_component=1`, `cycle=0` | ✅ |

### install-plan-schema

| Requirement | Scenario | Evidence | Result |
|---|---|---|---|
| Enumerates all sixteen fields | A complete plan validates | `install-plan.json` positive | ✅ |
| Enumerates all sixteen fields | A plan missing a field fails | `install-plan.missing-field.json` → `required` | ✅ |
| Deterministic and serializable | No timestamp in the core plan | Root closed; probe 9 → `"" unevaluatedProperties` | ✅ (UNTESTED fixture) |
| Deterministic and serializable | Non-deterministic field rejected | Probe 9; determinism listed as semantic on README boundary | ✅ |
| Env-variable names only | Valid names accepted | `install-plan.json` | ✅ |
| Env-variable names only | A value is rejected | `install-plan.env-value.json` → `/environmentVariableNames/0 pattern` | ✅ |
| Env-variable names only | Name-shaped value flagged | Listed semantic (F2-12); secrets §58 OPEN marker | ✅ |
| No local path leakage | Project path is local | `project` string; no upload field; root closed | ✅ |
| No local path leakage | Upload requirement rejected | No transmission destination exists | ✅ |
| Risk and autonomy recorded | Risk + autonomy recorded | `risk.effectiveClass`/`autonomyLevel`; positive | ✅ |
| Risk and autonomy recorded | Sub-floor flagged | Listed semantic (F2-05) on README boundary | ✅ |
| Conflicts/unsupported never silent | **A declared conflict validates** | **Conflict with an explicit resolution value is REJECTED (probe 7 → `/conflicts.0 unevaluatedProperties`)** | ❌ FAILING |
| Conflicts/unsupported never silent | Blocking condition stops Apply | `blocksApply` boolean + semantic boundary | ✅ |
| No executable third-party code | A hook is a reference | `hooks[*].path`; positive | ✅ |
| No executable third-party code | **Inline executable content rejected** | **`hooks` items open — inline `command`/`scriptBody`/`content` validates (probe 6)** | ❌ FAILING |

**Compliance summary**: 47 scenarios ✅ (7 of them behaviorally verified ad-hoc but without a committed fixture), 6 ⚠️ PARTIAL, 3 ❌ FAILING, 1 R3-dependent ⚠️.

---

## Adversarial Evidence (non-vacuity)

The gates are **not vacuous**; each was proven to fail on an injected defect and then restored (`git status` clean after every restore).

| Mutation | Target | Result | Exit |
|---|---|---|---|
| A — corrupt positive | `manifest.minimal.yaml` `apiVersion` → `v2alpha1` | L1 `FAIL examples/positive/manifest.minimal.yaml positive did not validate: data/apiVersion must be equal to constant`; L2 `#/apiVersion keyword=const` | 1 / 1 |
| B — repair negative | remove `unknownTopLevel` from `manifest.unknown-toplevel.json` | L1 + L2 `FAIL … expected failure but instance validated` | 1 / 1 |
| C — broken `$ref` | inject schema referencing `defs/nonexistent.schema.json` | L0 `FAIL unresolved: … (not declared in registry.json)` | 1 |
| D — vendor const | inject schema with `const: "openai"` | L0 `FAIL vendor: … /$defs/x/const = 'openai'` | 1 |
| Control | remove injected files | L0 `OK … vendor=0 unresolved=0 cycle=0` | 0 |

**Independent behavior probes** (`jsonschema` Draft 2020-12 + `referencing.Registry`, separate from the corpus):

| Probe | Instance | Schema verdict |
|---|---|---|
| 1 | `metadata` as a string | `/metadata type` ✅ (wrong type rejected) |
| 2 | root is a list of manifests | `"" type` ✅ (aggregate rejected) |
| 3 | capability not known to Core (`vendorx.some-future-capability`) | `[]` ✅ (accepted, registry concern) |
| 4 | `executionLocation: remote` | `[]` ✅ |
| 5 | model with partial `lifecycle` | `/lifecycle required` ×4 ✅ |
| 6 | `hooks` entry with inline `command`/`scriptBody`/`content` | `[]` ❌ **spec requires failure** |
| 7 | conflict with an explicit `resolution` value | `/conflicts.0 unevaluatedProperties` ❌ **spec requires pass** |
| 8 | unsupported capability without a `required` flag | `[]` ⚠️ spec says the flag MUST be marked |
| 9 | plan carrying `timestamp` | `"" unevaluatedProperties` ✅ |

**§58 re-check**: only one `const` exists in the whole set (`apiVersion`). `artifactType` is a pattern (no media-type const), `canonicalIdentifier` keeps the host as a variable, `input`/`output` are permissive, `capabilityId` has no enum, `versionRange` has no grammar, `payload`/`override` are open, `environmentVariableNames` keeps secrets OPEN. No §58 item was resolved. `open=9` confirmed.

**No F1 MUST weakened (spot-check)**: traceability rows verified against `spec/**` — `compatibility.md` §2 (5 levels), `profiles.md` §4 (5 profiles), `component-types.md` §2 (8 types, no `Tool`), `component-types.md` §4 (9 model elements), `permissions.md` §4 (names only, §58-OPEN secrets), `manifest/README.md` §5.3 (unknown field rejected), `risk-classes.md` §1/§4 (A–D + no bypass), `STYLE.md` §5 (vendor neutrality) all match the schema constraints.

**Protected paths**: `spec/**`, repo-root `examples/**`, `packages/**` were **not modified** by the change. `git show --name-only` for all 8 change commits reports clean; `git diff 5f4ec55..57b43dc` touches only `schemas/`, `scripts/schemas/`, `docs/adr/`, `package*.json`, and `openspec/changes/harness-schema-v1alpha1/**` (42 files, +4720, additive only).

---

## Findings

### CRITICAL

**C1 — `install-plan.schema.json` does not reject inline executable content in `hooks` (weakening of F1 `install-protocol/README.md` §4 Rule 5).**
The `hooks` item object (`install-plan.schema.json` lines 179–198) declares `path`/`interpreter`/`riskClass` but sets **no `unevaluatedProperties: false`**, so a plan entry carrying an inline `command`/`scriptBody`/`content` validates. This contradicts the change's own requirement and scenario ("The plan contains no executable third-party code" → "GIVEN a plan carrying an inline command or script body WHEN it is validated THEN it MUST fail") and F1 §4 Rule 5 ("The Install Plan MUST NOT contain or require the execution of third-party code"). It is also **not** listed in the README semantic-boundary table, so no downstream phase owns the check. The traceability row for this constraint cites the correct anchor, but `--trace` only verifies that the anchor file exists — it cannot detect that the schema under-enforces it, which is exactly the "weakening constraint" the schema-strategy spec says MUST be rejected before freeze.
*Evidence*: probe 6 → `[]`; `python scripts/schemas/check_schemas.py` still `OK`; `npm run validate:schemas`/`cross_check.py` still `PASS` (no fixture covers it).
*Remedy (either)*: (a) add `unevaluatedProperties: false` to the `hooks` item schema (and a negative fixture `install-plan.inline-hook.json` expecting `{path:"/hooks/0", keyword:"unevaluatedProperties"}`), and consider closing the sibling item objects (`mcp`, `services`, `adaptations`, `unsupportedCapabilities`, `verificationSteps`); or (b) if deferral is intended, add an explicit semantic-boundary row naming the owning phase. Option (a) is preferred because it is schema-expressible and the spec scenario demands validation failure.

### WARNING

**W2 — Conflict entries cannot carry an explicit resolution value.**
`conflicts` items are closed to `{path, reason}`, so probe 7 yields `/conflicts.0 unevaluatedProperties`. The delta scenario "A declared conflict validates" explicitly requires a conflict with "a reason and an explicit resolution value" to pass. F1 §4 only mandates the reason and that conflicts are not resolved silently, so this is a delta-spec-vs-schema divergence rather than an F1 weakening: reconcile the requirement text or add an optional `resolution` property.

**W3 — `unsupportedCapabilities` does not require a "required" marker.**
The delta requirement says the entry "MUST mark whether the capability is required and whether it blocks Apply"; the schema requires only `capability` + `blocksApply` (probe 8 accepted an entry with no `required`). F1 §4 mentions only "whether it blocks Apply", so this is a delta-spec over-specification; either require the flag or amend the requirement text.

**W4 — Delta-spec field-name / requiredness drift vs the implemented schema.**
The delta requirement enumerates `hooksScripts` and `verification`; the schema implements `hooks` and `verificationSteps` (faithful to F1 §4's "Hooks / scripts" and "Verification steps" semantics). Additionally the delta lists `license` among the model-contract elements that MUST be required, while the schema leaves it optional — an intentional, explicitly `§58`/Q7-OPEN-justified relaxation ("the field is retained and left optional"). No functional gate fails, but the delta spec and the schema should be reconciled so downstream readers cannot grep for a field that does not exist.

**W5 — "SPDX identifier shape" for model `license` is not enforced.**
The delta requirement says `license` (SPDX identifier shape); the schema uses a bare `type: string` with no pattern. Low impact because F1 (`component-types.md` §4.8) does not fix an SPDX grammar, but the constraint is stated and unenforced.

**W6 — Model license-contradiction semantic check is not in the boundary table.**
The scenario "A contradictory license is flagged" is stated only in the `metadata.license` and model `license` field descriptions; the README semantic-boundary table has no row naming the owning phase for Q7 reconciliation.

### SUGGESTION

**S7 — Add committed fixtures for scenarios that are behaviourally satisfied but untested.** Wrong-type `metadata`, aggregate-root list, unknown `apiVersion`, unknown `kind`, `remote` execution location, full model service `lifecycle`, and plan `timestamp` all behave correctly when probed ad-hoc (probes 1–5, 9) but have no corpus entry, so they are not regression-protected. Adding them would raise the corpus from 5/9 to a fuller contract.

**S8 — `--freeze` exits 0 while the project state is claimed BLOCKED.** Correct and documented by design (the gate certifies the blocker is *recorded*; the real contract is the textual `FREEZE OK`), but it cannot be wired into CI as a hard blocking gate. Consider a distinct non-zero exit (or a `--require-frozen` flag) for automation.

**S9 — `--trace` verifies anchor existence, not anchor strength.** A row whose cited F1 section does not actually authorize the constraint's strength still counts as traced; weakening detection remains manual (acknowledged in `verification.md`). A machine-checkable subset (e.g. each non-OPEN row must name an existing file *and* a heading that exists in that file) would tighten this.

---

## R3 Status

**OPEN / `r3=pending` (expected, confirmed).** `--freeze` → `FREEZE BLOCKED r3=pending` (exit 0). The Rust `jsonschema` Draft 2020-12 / `unevaluatedProperties` support is unverified because Rust is absent. L1 (`ajv` 8.20.0) + L2 (`jsonschema` 4.26.0 + `referencing`) agree on the same 5 positive / 9 negative corpus — two independent engines, which is the interim portability evidence, **not** a guarantee. R3 is recorded consistently in both `docs/adr/0002-schema-strategy.md` (`## Status of related decisions`) and `schemas/README.md` (`## Pre-freeze blocker (R3)`). The change must not be frozen until F2-05 confirms Rust support.

**Stale pin confirmation**: `tasks.md` pins `refs=11` / `schemas=4`. Real values are `refs=38` / `schemas=3`. The implementation chose correctness over the stale counts; `verification.md` states the real numbers and marks the pins superseded. Not a failure.

---

## Verdict

**FAIL** — all 10 gates pass and the protected paths are untouched, but the mandated adversarial scenario "an attempt to weaken an F1 MUST" exposes a real gap: `install-plan.schema.json` accepts inline executable content in `hooks`, so the delta scenario "Inline executable content is rejected" and F1 `install-protocol/README.md` §4 Rule 5 are not enforced by schema or by any named semantic check. The failure is bounded and additive: closing the `hooks` item object plus one negative fixture clears it. The three additional delta-spec divergences (W2/W3/W4) should be reconciled before the change is archived.

**Recommended next**: `sdd-apply` (fix C1 + reconcile W2–W4), then re-run this verification; freeze remains blocked on R3/F2-05 regardless.
