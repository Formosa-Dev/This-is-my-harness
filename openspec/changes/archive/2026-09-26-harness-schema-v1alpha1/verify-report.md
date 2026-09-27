# Verification Report — harness-schema-v1alpha1 (post-fix re-verify)

**Change**: `harness-schema-v1alpha1` · **Phase**: F2 (F2-01..F2-04) · **Surface**: Spec (`schemas/`, `scripts/schemas/`, `docs/adr/`)
**Mode**: Standard (`strict_tdd: false`, no test runner — `openspec/config.yaml`)
**Verifier**: independent re-execution by the `sdd-verify` executor. This is the **re-verify after the fix pass** (commit `c735554`); it supersedes the previous FAIL report committed at `51d9f28` / Engram #4378.
**Environment**: Node v22.17.0 · npm 10.8.2 · `ajv` 8.20.0 · Python 3.12.10 · `jsonschema` 4.26.0 · PyYAML 6.0.3 · Rust absent (R3)
**Tree**: `HEAD = c735554`, working tree clean, `origin/main == HEAD` (0 ahead / 0 behind)

> Re-run note: every gate and every fix probe below was executed by the verifier against the committed tree at `c735554`. The fix report (Engram #4375) was **not** trusted; each claim was independently reproduced with the real engines (ajv 8.20.0 for L1, Python `jsonschema` 4.26.0 + `referencing` for L2).

---

## Completeness

| Metric | Value |
|--------|-------|
| Implementation tasks complete | 34 `[x]` (27 original + fix pass F.1–F.7) |
| Intentionally unchecked | 1 — `S.8/S.9` accepted backlog line (no code change) |
| Work units committed | 5/5 (`bc91d6f`, `d6653a8`, `e875fbb`, `1f4b244`, `57b43dc`) + verify report `51d9f28` + fix `c735554` |

---

## Gate Results (independent re-execution, post-restore)

| # | Command | Real output | Exit |
|---|---------|-------------|------|
| 1 | `python scripts/schemas/check_schemas.py` | `OK schemas=3 defs=7 refs=38 open=9 vendor=0 unresolved=0 cycle=0` | 0 |
| 2 | `npm run validate:schemas` | `PASS schemas=3 positive=7 negative=16` | 0 |
| 3 | `python scripts/schemas/cross_check.py` | `PASS schemas=3 positive=7 negative=16` | 0 |
| 4 | `python scripts/schemas/check_schemas.py --guards` | `GUARDS OK model_no_component=1 install_leaf_refs=14 cycle=0 unresolved=0` | 0 |
| 5 | `python scripts/schemas/check_schemas.py --trace` | `TRACE OK rows=40 untraced=0` | 0 |
| 6 | `python scripts/schemas/check_schemas.py --adr` | `ADR OK headings=7` | 0 |
| 7 | `python scripts/schemas/check_schemas.py --trust` | `TRUST OK` | 0 |
| 8 | `python scripts/schemas/check_schemas.py --self-test` | `SELFTEST OK cases=5` | 0 |
| 9 | `python scripts/schemas/check_schemas.py --readme` | `README OK sections=7` | 0 |
| 10 | `python scripts/schemas/check_schemas.py --freeze` | `FREEZE BLOCKED r3=pending` | 0 |

**10/10 gates match the expected output exactly and exit 0.** The corpus grew as declared: `positive=7` (was 5), `negative=16` (was 9). All gates were re-run a second time after the adversarial mutations were restored — no regression.

---

## Fix Confirmations (independent)

### C1 (was CRITICAL) — inline executable hooks content is now rejected by BOTH engines — **CONFIRMED**

`schemas/install-plan.schema.json` `properties.hooks.items` now sets `unevaluatedProperties: false`.

| Check | Engine | Real result |
|---|---|---|
| `install-plan.hooks-inline.json` rejected | L2 `jsonschema` 4.26.0 | `[('/hooks/0', 'unevaluatedProperties')]` — exactly the required `{path, keyword}` |
| `install-plan.hooks-inline.json` rejected | L1 `ajv` 8.20.0 | `valid? false`, errors `[["/hooks/0","unevaluatedProperties"],["/hooks/0","unevaluatedProperties"]]` |
| Closure is **load-bearing** (remove `unevaluatedProperties:false` from `hooks` items, deep copy) | L2 `jsonschema` | `[]` → instance now **validates** (closure proven necessary, not vacuous) |
| Closure is **load-bearing** | L1 `ajv` (fresh Ajv, same override) | `valid? true` |

The fixture is wired into `examples/corpus.json` with `expect {path:"/hooks/0", keyword:"unevaluatedProperties"}`.

### W2 (was WARNING) — declared conflict with a `resolution` validates; `resolution` is OPTIONAL — **CONFIRMED**

- `install-plan.conflict-resolution.json` (conflict carrying `resolution`) → L2 `[]`, L1 `valid? true`.
- Same instance with `resolution` removed → L2 `[]`, L1 `valid? true` → `resolution` is genuinely optional. `{path, reason}` remain the only `required` members.

### W3 (was WARNING) — `unsupportedCapabilities` entries require the `required` flag — **CONFIRMED**

- `required: ["capability", "blocksApply", "required"]` in the schema.
- Removing `required` from an entry → L2 `[('/unsupportedCapabilities/0', 'required')]`, L1 `valid? false` with `["/unsupportedCapabilities/0","required"]`.
- Both committed positives carry `required` (gate 2 still `positive=7`).

### W4 (was WARNING) — delta specs reconciled, schema NOT weakened — **CONFIRMED**

- `specs/install-plan-schema/spec.md` §"enumerates all sixteen fields" now names `hooks` and `verificationSteps` (was `hooksScripts`/`verification`).
- `specs/model-contract-schema/spec.md` now states the model `license` is **OPTIONAL**, OPEN by §58/Q7, and binds `version` (SemVer) as the required member; the SPDX shape is delegated to the semantic boundary.
- **No weakening**: `git show c735554 -- schemas/install-plan.schema.json` contains only additive deltas — `resolution` added as OPTIONAL, `"required"` added to the `unsupportedCapabilities` required list, `unevaluatedProperties:false` added to `hooks` items. No `required` entry was removed; `model-contract.schema.json` and `harness.v1alpha1.schema.json` were not touched by the fix commit.

### W5 / W6 (were WARNING) — semantic-boundary rows exist — **CONFIRMED**

`schemas/README.md` §Semantic boundary now contains:
- `Model `license` SPDX identifier shape | … | F2-12`
- `Model `license` vs package `metadata.license` contradiction (Q7) | … | F2-10`

### S7 (was SUGGESTION) — new corpus fixtures exist and are genuinely exercised — **CONFIRMED**

`positive=7`, `negative=16` reported by both gates. Newly committed and wired into `corpus.json`:

- Positives: `model-contract.service.json` (`executionLocation: remote` + full six-action `lifecycle`), `install-plan.conflict-resolution.json`.
- Negatives: `install-plan.hooks-inline.json`, `install-plan.timestamp.json`, `manifest.wrong-type-metadata.json`, `manifest.aggregate-root.json`, `manifest.unknown-apiversion.json`, `manifest.unknown-kind.json`, `model-contract.partial-lifecycle.json`.

The runners iterate the corpus and resolve each file from disk, so a missing or unwired fixture would fail loudly — the 7/16 counts are real, not declared.

---

## Adversarial Evidence (non-vacuity re-check)

All mutations were applied to committed fixtures, the gates were run, and the tree was restored to `c735554` (`git checkout --`); final `git status --porcelain` is empty.

| Mutation | Target | Real result | Exit |
|---|---|---|---|
| **P** — corrupt a positive | `install-plan.conflict-resolution.json` `scope` → `"global"` | L1: `FAIL … positive did not validate: data/scope must be equal to one of the allowed values`; L2: `FAIL … #/scope keyword=enum message='global' is not one of ['project','user']` | 1 / 1 |
| **N1** — repair a negative | `install-plan.hooks-inline.json`: delete inline `command` + `scriptBody` | L1 + L2: `FAIL … expected failure but instance validated` | 1 / 1 |
| **N3** — shift the failure | `install-plan.hooks-inline.json`: remove inline fields **and** `runtime` | L1 + L2: `FAIL … # keyword=required expected={"path":"/hooks/0","keyword":"unevaluatedProperties"}` (precise `{path, keyword}` mismatch) | 1 / 1 |
| **N2** — retarget the error | `install-plan.hooks-inline.json`: `hooks[0].path` → `123` | Gates still `PASS` — the expected `{/hooks/0, unevaluatedProperties}` error is still present (corpus matching is "any error matches"). Correct semantics, **not** vacuity. | 0 / 0 |
| Control | restore all | `check_schemas.py` → `OK … vendor=0 unresolved=0 cycle=0` | 0 |

**§58 re-check (independently confirmed)**: exactly **one** `const` exists in the whole set — `harness.v1alpha1.schema.json` line 11 `"const": "thisismyharness.dev/v1alpha1"`. Every other match is a structural `enum` (kind, profile, execution location, risk class, autonomy, lifecycle action, compatibility level, trust label, component type); no vendor token, no fixed host, no media-type `const` (`artifactType` remains a pattern; `canonicalIdentifier` keeps the host a variable). `open=9` preserved. No §58 item was resolved.

**Protected paths**: `git diff --name-only 5f4ec55..c735554` touches only `schemas/**`, `scripts/schemas/**`, `docs/adr/**`, `package*.json`, and `openspec/changes/harness-schema-v1alpha1/**`. **No** `spec/**`, **no** repo-root `examples/**`, **no** `packages/**`. The delta files under `openspec/changes/.../specs/*/spec.md` are the change's own delta specs (the W4 reconciliation target), not the normative F1 `spec/**`.

---

## Spec Compliance Matrix (updated)

Legend: ✅ COMPLIANT · ⚠️ PARTIAL · ❌ FAILING

### schema-strategy

| Scenario | Before | After | Evidence |
|---|---|---|---|
| A weakening constraint is rejected | ❌ FAILING | ✅ | C1 closure + both-engine rejection + load-bearing proof |
| Schema-versus-semantic boundary documented | ⚠️ PARTIAL | ✅ | Traceability row + two new boundary rows (W5/W6) |
| Rust + JS consumer reach same result | ⚠️ | ⚠️ | R3 still OPEN (Rust absent) — expected |
| All other scenarios | ✅ | ✅ | unchanged (`--trace` 40 rows, `--guards`, `--self-test`) |

### manifest-schema

| Scenario | Before | After |
|---|---|---|
| Wrong type rejected / unknown `apiVersion` / unknown `kind` / aggregate root | ✅ (UNTESTED) | ✅ (fixtures: `manifest.wrong-type-metadata`, `manifest.unknown-apiversion`, `manifest.unknown-kind`, `manifest.aggregate-root`) |
| Malformed optional `spec` section rejected | ⚠️ PARTIAL | ⚠️ PARTIAL (still no committed fixture; behaviour expressed by section types) |
| All other scenarios | ✅ | ✅ |

### model-contract-schema

| Scenario | Before | After |
|---|---|---|
| Complete service lifecycle validates / partial lifecycle rejected | ⚠️ PARTIAL | ✅ (`model-contract.service.json`, `model-contract.partial-lifecycle.json`) |
| Contradictory license flagged | ⚠️ PARTIAL | ✅ (README boundary row, W6) |
| Model license OPTIONAL per §58/Q7 | drifted | ✅ (delta spec reconciled, W4) |
| All other scenarios | ✅ | ✅ |

### install-plan-schema

| Scenario | Before | After |
|---|---|---|
| A declared conflict validates | ❌ FAILING | ✅ (W2 `resolution` OPTIONAL) |
| Inline executable content is rejected | ❌ FAILING | ✅ (C1 `hooks` closure) |
| Non-deterministic field rejected | ✅ (UNTESTED) | ✅ (`install-plan.timestamp.json`) |
| All other scenarios | ✅ | ✅ |

**Summary**: all **3** previously ❌ FAILING scenarios are now ✅; all previously ad-hoc-only ✅ scenarios are now backed by committed fixtures. Remaining: the R3-dependent language-neutral row (⚠️, expected) and one pre-existing PARTIAL (no fixture for a malformed optional `spec` section — not a fix target).

---

## Findings

### CRITICAL

None. C1 is resolved and independently reproduced.

### WARNING

None. W2, W3, W4, W5 and W6 are resolved as shown above.

### SUGGESTION

- **S-a (residual hardening)** — The C1 fix closed `hooks` items only, which is exactly the vector named by the F1 `MUST` ("hooks and scripts … not inline executable content"). The sibling plan item objects (`mcp`, `services`, `adaptations`, `unsupportedCapabilities`, `verificationSteps`) remain without `unevaluatedProperties:false`. This is **not** a weakened F1 `MUST` (the requirement scopes hooks/scripts), so it is not a failure; closing them would harden the plan shape uniformly.
- **S8** — `--freeze` exits 0 while the state is `BLOCKED`. Correct by design (the gate certifies the blocker is *recorded*), but it cannot be a hard CI gate. Consider a `--require-frozen` flag. Accepted backlog.
- **S9** — `--trace` verifies anchor existence, not anchor strength. A row citing a section that does not authorize the constraint's strength still counts as traced. Accepted backlog.
- **S7-residual** — add one negative fixture for a malformed optional `spec` section to close the last un-fixtured scenario. Non-blocking.

---

## R3 Status

**OPEN / `r3=pending` (expected, confirmed).** `--freeze` → `FREEZE BLOCKED r3=pending` (exit 0). Rust `jsonschema` Draft 2020-12 / `unevaluatedProperties` support remains unverified because Rust is absent. L1 (`ajv` 8.20.0) + L2 (`jsonschema` 4.26.0 + `referencing`) agree on the same 7 positive / 16 negative corpus — two independent 2020-12 engines, which is interim portability evidence, **not** a guarantee. Recorded in `docs/adr/0002-schema-strategy.md` and `schemas/README.md`. The change must not be frozen until F2-05 confirms Rust support.

**Stale pin confirmation**: `tasks.md` pins `refs=11` / `schemas=4`; real values are `refs=38` / `schemas=3`. The implementation chose correctness over the stale counts and states the real numbers in `verification.md`. Informational, not a failure.

---

## Verdict

**PASS** — 10/10 gates match the expected output and exit 0 on a clean `c735554`; C1 is fixed and proven load-bearing under **both** independent engines; W2–W6 are resolved without weakening any F1 `MUST`; the S7 fixtures exist and are genuinely exercised (7/16). No protected path was modified and no §58 position was decided. The only non-green items are the deliberately recorded R3 pre-freeze blocker and accepted backlog suggestions (S8/S9/S-a), none of which is a verification failure.

**Recommended next**: `sdd-archive` for the change (delta specs → base specs), leaving the freeze gate blocked on R3/F2-05 as designed.

---

## Skill Resolution

`injected` — the orchestrator supplied a `## Project Standards (auto-resolved)` block; no registry or SKILL.md file needed loading.
