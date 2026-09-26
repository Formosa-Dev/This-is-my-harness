# Proposal: harness-schema-v1alpha1 (F2-01..F2-04)

> **Change:** `harness-schema-v1alpha1` · **Phase:** F2 · **Surface:** Spec (`schemas/`) · **Target `apiVersion`:** `thisismyharness.dev/v1alpha1`
> **Risk class:** A (Passive) · **Default autonomy:** 0 (Preview)
> **Normative source:** `spec/**` (F1, complete). This proposal is NON-NORMATIVE; RFC 2119 keywords below only restate F1 requirements.

## Intent

F1 fixed semantics in prose only; `schemas/` holds just `.gitkeep`. Without a machine-readable contract, the validator (F2-05..12), SDK and future web layer would each re-implement types, contradicting ADR-0001 and `openspec/config.yaml` ("JSON Schema is the single source of truth; language reversible"). This change ships that contract so every consumer resolves one source (§2.2, §2.6, §8, §21, §52, §54).

## Scope

### In Scope (F2-01..F2-04)
- `schemas/harness.v1alpha1.schema.json` — root manifest schema (F2-01).
- `schemas/README.md` + `schemas/defs/*.schema.json` — strategy, `$id` scheme, resolver map, F1→constraint traceability, schema-vs-semantic boundary (F2-02).
- `schemas/model-contract.schema.json` — Model (+ Router) contract (F2-03).
- `schemas/install-plan.schema.json` — 16-field Install Plan (F2-04).
- Minimal NON-NORMATIVE valid/invalid examples in the change folder (evidence per task).

### Out of Scope
- F2-05..F2-12 validator code, CLI, JSON output, migration stubs, validator docs.
- Resolving any §58 open item.
- `examples/` corpus (F3), capability registry (F2-07), `packages/*`, and any `spec/**` edit.

## Capabilities

### New
- `schema-strategy`: dialect, `$id` namespace, shared `$defs`, absolute-`$id` refs + resolver map, F1 traceability, semantic-vs-schema boundary.
- `manifest-schema`: the `harness.yaml` document shape (4 top-level fields, `metadata`, 8 optional `spec` sections).
- `model-contract-schema`: input/output, capabilities, resources, execution location, lifecycle, fallback, version, license, artifact source.
- `install-plan-schema`: the deterministic, serializable Install Plan contract.

### Modified
None.

## Approach

Exploration's recommended path (§14):
1. **Dialect** Draft 2020-12 — `unevaluatedProperties` is required for closed REQUIRED objects (`manifest/README.md` §5.3) composed from shared refs.
2. **Layout** F2-pinned filenames + `schemas/README.md` + `defs/` (identity, capability, requirement, permission, component, dependency, distribution).
3. **`$id`** `https://thisismyharness.dev/schemas/v1alpha1/…` — a schema namespace, explicitly distinct from the §58-open harness host.
4. **Refs** absolute `$id` + a checked-in resolver map for offline validation; each shared concept defined exactly once.
5. **Strictness** `unevaluatedProperties:false` on REQUIRED objects; permissive and OPEN-marked where §58 is open.
6. **Traceability** `schemas/README.md` carries the F1-section→constraint matrix (R1 mitigation); the semantic-only list (exploration §10) is handed to F2-05..12.
7. **ADR** `docs/adr/0002-schema-strategy.md` records dialect/`$id`/modular defs + rejected alternatives (A1–A12).
8. **Single source** no parallel hand-written types; codegen *from* JSON Schema remains allowed.

## Affected Areas

| Area | Impact | Description |
|---|---|---|
| `schemas/README.md` | New | F2-02 strategy, `$id` map, resolver map, traceability |
| `schemas/harness.v1alpha1.schema.json` | New | Root manifest schema |
| `schemas/model-contract.schema.json` | New | Model/Router contract |
| `schemas/install-plan.schema.json` | New | Install Plan contract |
| `schemas/defs/*.schema.json` | New | Shared definitions |
| `docs/adr/0002-schema-strategy.md` | New | Schema-strategy decision |
| `openspec/changes/harness-schema-v1alpha1/` | New | proposal/spec/design/tasks + examples |
| `spec/**`, `examples/**`, `packages/**` | Untouched | F1 read-only; F3 fixtures; F2-05+ consumers |

## Guarantees That MUST NOT Regress

- The schema MUST NOT weaken **any** F1 `MUST`; a schema contradicting `spec/**` is wrong by definition (#4363).
- §58 items stay **OPEN**: manifest filename, OCI media type, canonical host, model-capability contract, Core/extension split, profiles versioning, secrets, policy language.
- **env var NAMES only** (`permissions.md` §4); no value channel, no credential storage.
- Install Plan stays deterministic (no timestamp), never uploads a local path (§48); conflicts and unsupported capabilities are reported, never resolved by the schema.
- No vendor in any `const`/`enum`/example (`STYLE.md` §5); Runtime ≠ Model are separate defs; no absolute `--yes` bypass (§24).

## Rollback Plan

Additive, declarative files only: no product code, no `spec/**` change, no migration, no data mutation. A bad schema is reverted with `git revert` of the change commits; the ADR records the decision so a successor can supersede it. Until F2-05 exists, no runtime or downstream artifact depends on these files, so deletion is also safe.

## Open Questions (carried — NOT resolved here)

Q1 `$id` host vs §58 harness host · Q2 flat names vs versioned dir · Q3 `apiVersion` `const` vs pattern · Q4 `kind` enum vs namespaced pattern · Q5 profile enum vs extension profiles · Q6 model I/O free-form JSON Schema vs typed descriptor · Q7 model vs package license · Q8 Install Plan envelope · Q9 strictness in OPEN positions · Q10 absolute `$id`+resolver vs relative refs · Q11 examples in change folder vs F3 · Q12 does `$id` bake in `harness.yaml`. Each is decided in spec/design.

## Traceability

Full matrix is a `schemas/README.md` deliverable. Anchors: manifest top-level/`metadata` → `manifest/README.md` §2–§5 · identity → `core/identity.md` §2 · versions/`apiVersion` → `core/versioning.md` §2–§4 + `VERSIONING.md` · requirements → `core/requirements.md` §2 · permissions/scope → `core/permissions.md` §1–§2, §4 · risk/autonomy → `core/risk-classes.md` §1–§4 · deps/`extends` → `core/dependencies.md` §1–§6 · distribution/digest → `core/distribution.md` §2–§5 · compatibility levels → `core/compatibility.md` §2 · conformance → `core/conformance-metadata.md` §2 · profiles → `core/profiles.md` §4 · components/model/router/service → `package/component-types.md` §4–§5, §10 · layout → `package/layout.md` §1–§5 · Install Plan 16 fields → `install-protocol/README.md` §4 + §7.1 · style → `STYLE.md` §1, §5.

## Validation Strategy

No test runner exists (`strict_tdd: false`, #4282), so code-like artifacts MUST be run before commit:
- **JSON parse + metaschema** check each schema against Draft 2020-12.
- **Positive examples** (minimal + rich manifest, model `input: state / output: {route, confidence}`, one Install Plan) MUST validate.
- **Negative examples** (missing REQUIRED field, unknown field in a closed object, bad slug/SemVer, vendor `const`) MUST fail.
- **Resolver map** resolves every declared `$id` offline to the intended def.
- **Dialect support** confirmed in the chosen Rust `jsonschema` and JS/TS `ajv` v8+ **before freeze** (R3).
- **Traceability review**: every non-OPEN constraint cites an F1 anchor; no `MUST` weakened.

## Risks

| Risk | Likelihood | Mitigation |
|---|---|---|
| Schema/spec divergence (R1) | High | F1→constraint matrix; review against F1 before freeze |
| Silently fixing §58 (R2) | Med | Explicit OPEN `description`; proposal lists touched open items |
| Dialect/tooling mismatch (R3) | Med | Verify 2020-12 support before freeze |
| Offline `$ref` resolution (R4) | Med | Checked-in resolver map |
| Vendor leakage (R5) | Low | No vendor in schema; examples vendor-free |
| F2-01 example unvalidatable (R11) | Med | Author completed minimal/rich examples |

## Success Criteria

- [ ] `schemas/README.md`, the three root schemas and `defs/` exist; strategy + resolver map + traceability documented.
- [ ] Valid examples validate; negative examples fail (recorded evidence).
- [ ] Every shared concept defined once; no cycle (`model-contract` does not ref the component descriptor).
- [ ] Every §58-touched position marked OPEN; no F1 `MUST` weakened (review).
- [ ] ADR-0002 records dialect/`$id`/defs + rejected alternatives.
- [ ] No `spec/**`, `packages/**` or `examples/**` change.

## Dependencies

- F1 (`spec/**`) complete — the normative source.
- ADR-0001 (`spec/`+`schemas/` language-neutral; JSON Schema the single source).
- Draft 2020-12 support in the chosen validators, confirmed before freeze.
