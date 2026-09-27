# Design: harness-schema-v1alpha1 (F2-01..F2-04)

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1` · **Surface:** Spec (`schemas/`) · **Risk class:** A (Passive) · **Autonomy:** 0
> **Language:** English. Keywords MUST/MUST NOT/SHOULD/MAY are RFC 2119/8174. **Normative source:** `spec/**` (F1, complete). This design is NON-NORMATIVE; it must not weaken any F1 MUST (#4363).

## Technical Approach

F1 fixes semantics in prose only; `schemas/` holds `.gitkeep`. This change ships the machine-readable contract as **four schema artifacts + shared `$defs`**, with **JSON Schema Draft 2020-12 as the single source of truth**. Ports-and-adapters: the schema set is the **port**; the Rust validator (F2-05..12), the SDK and the future web layer are **adapters** that resolve the same `$id`s. Verified on this machine: Node `v22.17.0`, Python `3.12.10`, git `2.51.0`; Rust, `ajv` and Python `jsonschema` are **NOT installed** (checked).

## 1. Schema set + file layout

F2-pinned names are evidence anchors and MUST NOT change.

| File | `$id` | Task | Role |
|---|---|---|---|
| `schemas/harness.v1alpha1.schema.json` | `…/schemas/v1alpha1/harness.schema.json` | F2-01 | Composition root of the manifest |
| `schemas/model-contract.schema.json` | `…/schemas/v1alpha1/model-contract.schema.json` | F2-03 | Model + Router contract |
| `schemas/install-plan.schema.json` | `…/schemas/v1alpha1/install-plan.schema.json` | F2-04 | 16-field Install Plan |
| `schemas/defs/identity.schema.json` | `…/v1alpha1/defs/identity.schema.json` | F2-02 | owner, slug, refs, semver, range |
| `schemas/defs/capability.schema.json` | `…/defs/capability.schema.json` | F2-02 | capabilityId, capabilityRequirement |
| `schemas/defs/requirement.schema.json` | `…/defs/requirement.schema.json` | F2-02 | runtime, modelCapabilities, hardware, services, backends |
| `schemas/defs/permission.schema.json` | `…/defs/permission.schema.json` | F2-02 | permission, scope, riskClass, autonomyLevel |
| `schemas/defs/component.schema.json` | `…/defs/component.schema.json` | F2-02 | componentDescriptor + 8-type discriminator |
| `schemas/defs/dependency.schema.json` | `…/defs/dependency.schema.json` | F2-02 | extends entries, override |
| `schemas/defs/distribution.schema.json` | `…/defs/distribution.schema.json` | F2-02 | digest, artifactType, provenance, compatibilityLevel, trustLabel |
| `schemas/README.md` | — | F2-02 | Strategy, `$id` map, resolver map, traceability, semantic boundary |
| `schemas/registry.json` | — | F2-02 | Checked-in `$id`→path resolver map (offline) |

**Versioned namespace decision.** Base `https://thisismyharness.dev/schemas/v1alpha1/`. Filenames stay **flat** (F2 pins them); the **generation lives in the `$id` path**, so a future `v1beta1` adds parallel files at `/v1beta1/` without re-pathing alpha. The `v1alpha1` in `harness.v1alpha1.schema.json` is human convenience — `$id` is authoritative and `README.md` MUST say so. The host is a **schema namespace**, explicitly **not** the §58-open harness-resolution host (`identity.md` §8).

## 2. `$defs` factoring + dependency graph

Each shared concept is defined **exactly once** and referenced by absolute `$id`. Cycle guards are structural, not conventional:

```text
harness (root) ──> identity, capability, requirement, permission,
                   component, dependency, distribution
component ───────> identity, capability, requirement#hardware, model-contract
model-contract ──> capability, requirement#hardware, identity#semver   (NOT component)
install-plan ────> permission, distribution, identity                   (leaf defs only; NOT harness root)
```

- **Guard 1:** `model-contract` MUST NOT `$ref` `component.schema.json`; a model's capabilities/resources reuse the leaf defs, so `component → model-contract` is a one-way edge.
- **Guard 2:** `install-plan` `$ref`s **leaf defs only** and never the manifest root, because it is consumed by external hosts (CLI/web/desktop/agents) with no manifest in hand.

**Q7 (license, not resolved here):** `distribution.md` §7 declares the package license once in `metadata.license`, yet `component-types.md` §4.8 requires a model license. The schema keeps an optional model `license` field carrying an **OPEN** `description` pointing at `distribution.md` §7; it does not decide duplicate-declaration policy.

## 3. Dialect + strictness

**Draft 2020-12** (`"$schema": "https://json-schema.org/draft/2020-12/schema"`). Reason: `manifest/README.md` §5.3 requires rejecting unknown fields in a REQUIRED object, and the defs are composed via `$ref`/`allOf`; `additionalProperties:false` sees only same-level properties and breaks under composition, while **`unevaluatedProperties:false`** evaluates after all contributions. Draft-07 cannot express this without inlining every property (killing the single-source strategy).

| Position | Strictness |
|---|---|
| manifest root, `metadata`, `spec` container | `unevaluatedProperties:false` |
| closed objects: `hardware`, each `permission` entry, `componentDescriptor` base, plan root, plan `risk`, plan `snapshot`, plan file/conflict entries | `unevaluatedProperties:false` |
| **§58-OPEN positions** | permissive, no closure |

OPEN / permissive per §58: model `input`/`output` **internals** (embedded JSON Schema, no new DSL); Policy/Workflow component payload; capability-registry internals; `artifactType` as a **pattern, never a media-type `const`**; `canonicalIdentifier` with the **host as a variable, never a fixed host**; an optional namespaced **extension branch** beside the 5 recognized profiles and the `kind` enum `[Harness, Component, Preset]`.

## 4. `$ref` resolution strategy

Absolute `$id` refs **plus** a checked-in resolver map (`schemas/registry.json`, mapping every `$id` to its repo path). Each consumer resolves the **same URI → the same definition**:

| Consumer | How it resolves offline |
|---|---|
| Rust validator (F2-05..12) | `jsonschema` crate; builds a registry from `registry.json`; no network. Dialect support confirmed **before freeze** (R3) |
| SDK (later) | Codegen **from** JSON Schema via the registry; never hand-written parallel types |
| Web / F17 | Either fetches the absolute `$id` or ships the registry bundle; same URIs |

Relative-only refs are rejected: hermetic but layout-coupled and no canonical-URI story.

## 5. F1 traceability matrix

Every non-OPEN constraint cites the F1 section that authorizes it (R1 mitigation; full matrix is the `schemas/README.md` deliverable).

| Schema constraint | F1 anchor |
|---|---|
| 4 REQUIRED top-level fields; `apiVersion` single value; `kind` recognized | `manifest/README.md` §2, §2.1, §2.2 |
| `metadata` closed; `name` REQUIRED slug 1–64 no `/`; `version` REQUIRED SemVer | `manifest/README.md` §3; `core/identity.md` §2; `core/versioning.md` §2 |
| `spec` REQUIRED, `{}` allowed, every section OPTIONAL; malformed section rejected | `manifest/README.md` §4, §4.1, §4.2 |
| Unknown field in REQUIRED object rejected | `manifest/README.md` §5.3 |
| requirements: runtime(1), runtimeVersion, modelCapabilities, hardware, services, backends | `core/requirements.md` §2 |
| permission/scope/riskClass/autonomyLevel; scope default `project` | `core/permissions.md` §1–§2, §4; `core/risk-classes.md` §1–§4 |
| `extends` **ordered**; override; no silent conflict resolution | `core/dependencies.md` §1–§6 |
| digest, artifactType (pattern), provenance, trust-label checks | `core/distribution.md` §2–§5 |
| compatibilityLevel ∈ native/adapted/partial/untested/unsupported | `core/compatibility.md` §2 |
| profile: 5 recognized, at most one | `core/profiles.md` §4 |
| componentDescriptor discriminates the **8** canonical types; no `Tool` type | `package/component-types.md` §2; `package/layout.md` §2–§4 |
| model: input/output, capabilities, resources(=hardware), executionLocation, lifecycle, fallback, version, license, artifactSource | `package/component-types.md` §4, §5 |
| Install Plan **16 fields**; risk records effective class + autonomy; digest recorded | `install-protocol/README.md` §4 (table), §7.1 |
| env-var **NAMES only**; descriptive fields carry no semantics; no vendor in Core | `core/permissions.md` §4; `manifest/README.md` §3.3; `STYLE.md` §5 |

## 6. §58 OPEN positions

Each position carries an explicit `description` marking it **OPEN** and pointing to §58; F2 MUST NOT silently resolve any (`STYLE.md` §3.5).

| §58 open item | Schema handling |
|---|---|
| Definitive manifest filename | Describe the **document shape**; `harness` in the `$id` is a working resource name, not a normative filename (Q12). |
| Definitive OCI media type | `artifactType` = **pattern**, never a media-type `const`, marked OPEN. |
| Canonical host / namespace | `canonicalIdentifier` pattern with **host as a variable**, never a fixed host. |
| Exact model-capability contract | `model-contract` stays **permissive** on `input`/`output` internals. |
| Core-vs-extension capabilities | Schema validates **shape only**; existence is F2-07. |
| Policy language / workflow graph | Components are **descriptors**, not DSLs. |
| Secrets handling | Only `environmentVariableNames` (names-only). |
| Profiles versioning | 5 recognized + optional namespaced extension branch (Q5). |

## 7. Validation strategy (CRITICAL)

No test runner exists (`strict_tdd:false`), so this code-like artifact MUST be run before commit. Layered, offline-capable, honest about what is installed today:

- **L0 — structural pre-check (works NOW, zero install).** A Python-stdlib checker (no third-party import) that: JSON-parses every schema; asserts `$schema` is 2020-12; asserts every `$ref` target exists in `registry.json` and on disk; asserts no vendor token in any schema `const`/`enum`; asserts each §58 position carries its OPEN marker. This is the guaranteed gate for F2-02 on this machine (Python 3.12.10 verified present; `jsonschema` verified **absent**).
- **L1 — full validation, primary.** Node 22 + `ajv` v8 in 2020-12 mode (`ajv/dist/2020`) driven by a checked-in runner. One-time `npm install --save-dev ajv@^8 ajv-formats@^3` (network once, then hermetic). **Positive MUST pass:** minimal manifest (`spec:{}`), rich manifest, model contract (`input: state` / `output: {route, confidence}`), a Router, one Install Plan. **Negative MUST fail with the expected path/keyword:** missing `apiVersion` → `#/required`; unknown field in a closed object → `#/metadata/unevaluatedProperties`; bad slug → `#/metadata/name/pattern`; bad SemVer → `#/metadata/version/pattern`; a vendor literal → L0 meta-check.
  - Expected PASS: `PASS schemas=4 refs=11 positive=5 negative=9` · exit `0`.
  - Expected FAIL: `FAIL <file> #/<ptr> keyword=<k> expected=<v>` · exit `1`.
- **L2 — independent cross-check (R3 mitigation).** Python `jsonschema>=4.18` using the `referencing` `Registry` (`python -m pip install "jsonschema>=4.18"`, one-time; verified **absent** today). Two independent implementations agreeing on the same corpus is the real proof that the 2020-12 features (`unevaluatedProperties`) are portable.
- **Evolution to F2-05..12.** L1/L2 are replaced by `cargo test` in `harness-core` using the Rust `jsonschema` crate; the negative-example→`{path, keyword, expected}` table becomes the validator's **diagnostic contract** (F2-05 typed errors); the small corpus moves to the F3 `examples/` fixture collection. Dialect support MUST be confirmed in Rust/ajv before freeze.

## 8. ADR-0002 — schema strategy

Recorded as `docs/adr/0002-schema-strategy.md` (a task deliverable, per `docs/adr/0000-template.md`).

| Option | Tradeoff | Decision |
|---|---|---|
| Modular `defs/` vs monolithic | Modular = independent review, per-consumer pull, per-def versioning | **Modular** |
| Draft 2020-12 vs Draft-07 | 2020-12 needs `unevaluated*` for closed composed objects; current stable | **2020-12** |
| Absolute `$id` + resolver map vs relative-only | Absolute = one canonical URI for web + offline map; relative = hermetic but layout-coupled | **Absolute + map** |
| JSON Schema as source vs code-first types | Code-first makes a language the source, violating ADR-0001 | **JSON Schema source**; codegen *from* it is allowed |
| Resolve §58 now vs leave OPEN | Resolving fixes media type/host/name by accident | **Leave OPEN** |

Rejected (full set A1–A12 in exploration §13): monolithic schema, Draft-07, relative-only refs, code-first types, resolving §58 now, OpenAPI Schema Object, JTD/RFC 8927, a separate `router.schema.json`, capability registry in-schema.

## 9. Flow: schema → examples → checker → (future) validator/SDK/web

```text
spec/** (F1, normative)
        │ derive (no MUST weakened)
        ▼
schemas/ defs + 3 root schemas  ── registry.json ($id → path)
        │                                   │
        │ (F3 expands)                      │ resolve offline
        ▼                                   ▼
 examples (minimal/rich/valid + negative)   │
        │                                   │
        ▼                                   ▼
 [L0 stdlib structural] → [L1 ajv 2020-12] → [L2 python jsonschema]
        │ pass/fail + {path, keyword, expected}
        ▼
 (F2-05..12) Rust validator `cargo test` ──► typed states ──► SDK ──► web (F17)
```

## File Changes

| File | Action | Description |
|---|---|---|
| `schemas/README.md` | Create | Strategy, `$id` map, resolver map, traceability matrix, semantic boundary, Install-Plan-local-only note (§48, R6) |
| `schemas/harness.v1alpha1.schema.json` | Create | Root manifest schema |
| `schemas/model-contract.schema.json` | Create | Model + Router contract |
| `schemas/install-plan.schema.json` | Create | 16-field Install Plan |
| `schemas/defs/*.schema.json` (7) | Create | Shared definitions |
| `schemas/registry.json` | Create | Offline `$id` resolver map |
| `docs/adr/0002-schema-strategy.md` | Create | ADR (§8) |
| `openspec/changes/harness-schema-v1alpha1/examples/**` | Create | Minimal/rich valid + negative instances (NON-NORMATIVE) |
| `spec/**`, `examples/**`, `packages/**` | Untouched | F1 read-only; F3 fixtures; F2-05+ consumers |

## Testing Strategy

| Layer | What to test | Approach |
|---|---|---|
| Structural (L0) | JSON parse, dialect, `$ref` integrity, vendor-free, OPEN markers | Python stdlib checker (runs now, zero install) |
| Schema (L1) | Valid corpus passes; negative corpus fails with exact `{path, keyword}` | `ajv` v8 Draft 2020-12 runner on Node 22 |
| Cross-impl (L2) | Same corpus under an independent engine | Python `jsonschema>=4.18` after freeze gate |
| Future (F2) | Typed diagnostics + golden fixtures | `cargo test` in `harness-core` (F2-05..12) |

## Migration / Rollout

Additive, declarative files only: no product code, no `spec/**` edit, no migration, no data mutation. A bad schema is reverted with `git revert`; the ADR preserves the decision so a successor can supersede it. Until F2-05 exists nothing depends on these files, so deletion is also safe. Within a generation `$defs` evolve **additive-only**; a breaking change opens a new `$id` generation directory with a mandatory migration note.

## Open Questions

- [ ] **Q3/Q4** — `apiVersion` `const` + `kind` enum on the Core branch vs namespaced extension branches sharing the defs (recommend: Core branch + extension schemas `$ref` the defs).
- [ ] **Q7** — model license vs the `distribution.md` §7 single-declaration rule (left OPEN).
- [ ] **Q8** — Install Plan strict 16 fields vs the 16 + a deliberate `apiVersion`/`kind`/binding envelope (recommend: the 16 required, extras only if explicit).
- [ ] **R3** — Rust `jsonschema` + `ajv` 2020-12/`unevaluatedProperties` support must be confirmed **before freeze**.
