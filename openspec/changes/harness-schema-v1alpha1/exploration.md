# Exploration: harness-schema-v1alpha1 (F2-01..F2-04)

> **Change:** `harness-schema-v1alpha1`
> **Phase:** F2 — `harness.yaml` schema + JSON Schema (+ validator, later tasks)
> **Scope of this exploration:** Phase F2 tasks **F2-01..F2-04** only — the toolchain-free schema layer.
> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Language:** English (public artifact). Normative keywords below are RFC 2119 only where quoting the spec; this document is a NON-NORMATIVE exploration and defines no requirements.
> **Source of truth:** `spec/**` (F1, complete) is normative. Any schema that contradicts it is wrong by definition.

---

## 1. Current State

Phase **F1 (Core Spec v1alpha) is COMPLETE**: `spec/` holds the normative semantics that this change must *derive from*, never redefine. The schema layer itself does not exist yet.

Facts verified on disk:

- `schemas/` contains **only `.gitkeep`** — no schema, no strategy document.
- `examples/` contains **only `.gitkeep`** — no instance manifests to validate against yet (fixtures are F3).
- `packages/{core,validator,sdk,cli}/` contain only `.gitkeep` — no validator, no SDK; there is **no test runner** (`strict_tdd: false`, testing-capabilities #4282).
- Phase F2 is defined in Engram `build-progress/F2` (#4271) with 12 tasks; **this change covers F2-01..F2-04**. F2-05..F2-12 (validator core, semantic checks, capability registry, CLI, output schema, migration stubs, docs) are explicitly **out of scope** and are the consumer of this layer.
- F2's DoD depends on the schema being the **single machine-readable source** that the validator, SDK and future web layer consume: "existe `harness.yaml` JSON Schema v1alpha1 + validador … el JSON Schema es fuente única consumible por toolchains y web" (#4271).
- ADR-0001 (`docs/adr/0001-core-language.md`) makes `spec/` and `schemas/` **language-neutral** and states *JSON Schema is the single source of truth*; the Rust Core is a reference implementation, not the definition.
- F1's apply-progress (#4363) recorded the governing risk for this change: **"F2's JSON Schema must not contradict these documents. Any schema convenience that weakens a MUST here is a spec violation."** It also recorded 7 README(EN)↔Decisiones(ES) conflicts already resolved in F1; the schema must honor the resolved side.
- The F1 index explicitly left **package-layout asymmetries for F2 to resolve deliberately** (`tools/` has no `Tool` component type; `Router` has no dedicated directory) — see §10 below.

### Normative surfaces this change must derive from (exact anchors)

| Schema concern | Normative source (F1) |
| --- | --- |
| Manifest top-level + `metadata` | `spec/manifest/README.md` §2, §3, §5 |
| `spec` section set | `spec/manifest/README.md` §4 (8 sections) |
| `owner`/`slug`/reference grammars | `spec/core/identity.md` §2 |
| SemVer + ranges + `apiVersion` | `spec/core/versioning.md` §2, §3, §4; `spec/VERSIONING.md` |
| Requirements (runtime, runtimeVersion, modelCapabilities, hardware, services, backends) | `spec/core/requirements.md` §2 |
| Permissions, scope | `spec/core/permissions.md` §1, §2, §4 |
| Risk classes + autonomy | `spec/core/risk-classes.md` §1–§4; `spec/glossary.md` "Autonomy level" |
| Dependencies / `extends` / conflicts | `spec/core/dependencies.md` §1–§6 |
| Distribution / digest / artifactType / trust label | `spec/core/distribution.md` §2–§5 |
| Compatibility levels | `spec/core/compatibility.md` §2 |
| Conformance metadata | `spec/core/conformance-metadata.md` §2 |
| Profiles | `spec/core/profiles.md` §4 |
| Component taxonomy + model contract | `spec/package/component-types.md` §1–§10 (model: §4, router: §5) |
| Package layout + discovery | `spec/package/layout.md` §1–§5 |
| Install Plan — 16 fields | `spec/install-protocol/README.md` §4 (authoritative table) + §7.1 |
| Adapter contract (11 ops) | `spec/adapter-contract/README.md` §2–§3 (semantic, not a JSON schema) |
| Style: English, RFC 2119, no vendors in Core | `spec/STYLE.md` §1, §5 |
| Open items | Decisiones §58 (Engram #4341, #4322 §5) |

---

## 2. Entity + Schema Inventory

### 2.1 Machine-readable contracts that need a schema (in F2-01..F2-04)

| # | Entity | Schema? | Owner task | F1 anchor |
| --- | --- | --- | --- | --- |
| E1 | `harness.yaml` manifest document (4 top-level fields) | **YES — root** | F2-01 | `manifest/README.md` §2 |
| E2 | `metadata` object | embedded/`$defs` | F2-01 | `manifest/README.md` §3; `identity.md` §2 |
| E3 | `spec` container (8 optional sections) | embedded | F2-01 | `manifest/README.md` §4 |
| E4 | `requirements` (6 kinds) | shared `$defs` | F2-01/02 | `requirements.md` §2 |
| E5 | `hardware` object (7 fields) | shared `$defs` | F2-01/02 | `requirements.md` §2.4 |
| E6 | `permissions` array + `scope` + `riskClass` + `autonomyLevel` | shared `$defs` | F2-01/02 | `permissions.md` §1–2; `risk-classes.md` |
| E7 | `extends` / dependency references | shared `$defs` | F2-01/02 | `dependencies.md` §1–§2 |
| E8 | `components` array (component descriptors, 8 types) | shared `$defs` | F2-01/02 | `layout.md` §2–§3; `component-types.md` §2 |
| E9 | `distribution` metadata (`digest`, `artifactType`, provenance, trust label) | shared `$defs` | F2-01/02 | `distribution.md` §2–§5 |
| E10 | `compatibility` (declared expectations + level enum) | shared `$defs` | F2-01/02 | `compatibility.md` §2–§3 |
| E11 | `conformance` metadata | shared `$defs` | F2-01/02 | `conformance-metadata.md` §2 |
| E12 | Identity primitives (`owner`, `slug`, `scopedReference`, `canonicalIdentifier`, `semver`, range) | shared `$defs` | F2-02 | `identity.md` §2; `versioning.md` §2, §4 |
| E13 | Capability identifier (shape only) | shared `$defs` | F2-02 | `glossary.md` "Capability"; `requirements.md` §2.3 |
| E14 | **Model contract** (§8) | **YES — standalone** | F2-03 | `component-types.md` §4–§5; Decisiones §8 |
| E15 | **Install Plan** (§21 / §4) | **YES — standalone** | F2-04 | `install-protocol/README.md` §4; glossary "Install Plan" |
| E16 | `profile` identifier | shared `$defs` | F2-01/02 | `profiles.md` §4 |
| E17 | Manifest name / filename (`harness.yaml`) | NOT a schema field | — | §58 OPEN |

### 2.2 Machine-readable contracts that must NOT get a schema in this change

| Entity | Why no schema here | Where it belongs |
| --- | --- | --- |
| Runtime Adapter Contract (11 ops) | It is a **behavioral function contract** with purpose/inputs/outputs/preconditions, not a serializable document. `adapter-contract/README.md` §2.6 says the surface syntax belongs to the schema artifact, but a function contract is not a JSON document. | A later adapter-interface schema (F6) if any; not F2-01..04. |
| Capability **registry** (which capability ids exist) | Schema validates *shape*; the **existence** check requires the extension registry. | F2-07. |
| Conformance **report** / validator diagnostics | Output contract of the validator. | F2-10. |
| Package **layout** (directories, discovery) | Filesystem semantics, not a document. | F2-06 (semantic). |
| Snapshot / operation journal | Runtime state; not a spec contract yet. | F5. |

### 2.3 Instance corpus

F2-01 evidence requires "valida el manifest de ejemplo del README" and F2-03 requires the `input: state / output: {route, confidence}` example; F2-04 requires "un plan de ejemplo". The README/Decisiones conceptual manifests are the source. **`examples/` is empty today and fixtures are F3** — so this change must include *minimal* example instances to prove the schemas validate, or explicitly defer them to F3. Recommended: include a tiny, NON-NORMATIVE example per schema under the change folder (not `examples/`, which is F3's), to satisfy each task's evidence without pre-empting F3.

---

## 3. Schema File Layout + Naming + `$id`

### 3.1 F2-pinned names (must be honored for traceability)

`build-progress/F2` (#4271) names three files by path:

- `schemas/harness.v1alpha1.schema.json` (F2-01)
- `schemas/model-contract.schema.json` (F2-03)
- `schemas/install-plan.schema.json` (F2-04)

These names are **evidence anchors**; the proposal should keep them exactly.

### 3.2 Recommended layout (proposal input)

```
schemas/
├── README.md                              # F2-02: strategy, dialect, $id scheme, ref resolution,
│                                          #         consumer contract, F1->schema traceability matrix,
│                                          #         explicit "semantic-only" boundary list
├── harness.v1alpha1.schema.json           # F2-01 — root manifest schema
├── model-contract.schema.json             # F2-03 — model (and base for Router)
├── install-plan.schema.json               # F2-04
└── defs/                                  # F2-02 — shared definitions, each referenced by $id
    ├── identity.schema.json               # owner, slug, name, refs, semver, version range
    ├── capability.schema.json             # capabilityId pattern + capability requirement (preferred/alternatives)
    ├── requirement.schema.json            # runtime, runtimeVersion, modelCapabilities, hardware, services, backends
    ├── permission.schema.json             # permission, scope, riskClass, autonomyLevel
    ├── component.schema.json              # component descriptor + type discriminator
    ├── dependency.schema.json             # extends entries, override
    └── distribution.schema.json           # digest, artifactType, provenance, trust-label checks
```

Rationale:

- The three top-level schemas are the F2 deliverables; `defs/` is the single place a shared concept is defined (F2-02's "single source of truth").
- Splitting `defs/` by concern (not one monolith) keeps each `$defs` file small, independently reviewable, and lets web/SDK pull only what they need.
- **No `v1alpha1` subdirectory** at this stage: F2 pins flat filenames. The generation is carried in the `$id` URI instead (§3.3), so a future `v1beta1` adds parallel files rather than re-pathing the alpha ones.

### 3.3 `$id` scheme

Recommended base: `https://thisismyharness.dev/schemas/v1alpha1/`.

| File | `$id` |
| --- | --- |
| `harness.v1alpha1.schema.json` | `https://thisismyharness.dev/schemas/v1alpha1/harness.schema.json` |
| `model-contract.schema.json` | `https://thisismyharness.dev/schemas/v1alpha1/model-contract.schema.json` |
| `install-plan.schema.json` | `https://thisismyharness.dev/schemas/v1alpha1/install-plan.schema.json` |
| `defs/<name>.schema.json` | `https://thisismyharness.dev/schemas/v1alpha1/defs/<name>.schema.json` |

Design notes:

- The `v1alpha1` **in the `$id` path** is the discriminator; the filename `harness.v1alpha1.schema.json` is human convenience. `schemas/README.md` must state this explicitly so the redundancy is deliberate, not accidental.
- `thisismyharness.dev` is already the **frozen `apiVersion` group** (`VERSIONING.md` "Group: thisismyharness.dev"). Using it as the schema `$id` host is consistent with an already-fixed string, and it is **not** a claim about the authoritative *harness-resolution* host — the §58 open item is about `(host, owner)` for canonical identifiers, a different concern. This distinction MUST be written down to avoid contradicting `identity.md` §8.
- `$id` URIs are the **canonical reference target**; `$ref`s use them (see §4.3). Whether the host actually serves the files at `v1alpha1` is a deployment detail; a checked-in resolver map makes offline validation hermetic.

---

## 4. `$ref` / `$defs` Strategy

### 4.1 Shared definitions (defined exactly once)

| Def | Home file | Referenced by |
| --- | --- | --- |
| `slugName`, `owner`, `scopedReference`, `canonicalIdentifier`, `shortReference`, `semver`, `versionRange` | `defs/identity.schema.json` | manifest (`metadata.name`, `metadata.owner`), `extends`, distribution, model version |
| `capabilityId`, `capabilityRequirement` (`preferred`/`alternatives`) | `defs/capability.schema.json` | `requirements.modelCapabilities`, model contract capabilities, permissions |
| `runtimeRequirement`, `runtimeVersionRequirement`, `modelCapabilitiesRequirement`, `hardware`, `serviceRequirement`, `backendRequirement` | `defs/requirement.schema.json` | manifest `requirements` |
| `scope`, `riskClass`, `autonomyLevel`, `permission` | `defs/permission.schema.json` | manifest `permissions`, install-plan `risk`/`scope`, service/component risk |
| `componentType`, `componentDescriptor` | `defs/component.schema.json` | manifest `spec.components` |
| `dependencyReference`, `override` | `defs/dependency.schema.json` | manifest `spec.extends` |
| `digest`, `artifactType`, `distributionMetadata`, `trustLabelCheck` | `defs/distribution.schema.json` | manifest `spec.distribution`, install-plan harness binding |
| `compatibilityLevel` | `defs/distribution.schema.json` or its own small file | manifest `compatibility`, install-plan adaptations/unsupported |
| `modelContract` | `model-contract.schema.json` | `defs/component.schema.json` (Model/Router components) |
| `installPlan` | `install-plan.schema.json` | external consumers only |

### 4.2 How the three schemas share

- `harness.v1alpha1.schema.json` is the **composition root**: it `$ref`s `defs/*` for every section.
- `model-contract.schema.json` is **standalone-validatable** (F2-03 evidence) *and* `$ref`-able from `component.schema.json` for a Model/Router descriptor. **Cycle avoidance:** `model-contract` MUST NOT `$ref` `component.schema.json`; a model's capabilities/requirements reuse the leaf defs (`capability.schema.json`, `requirement.schema.json`), not the component descriptor.
- `install-plan.schema.json` is consumed only by external hosts (CLI/web/desktop/agents), so it `$ref`s only leaf defs (`permission`, `distribution`, `identity`) and never the manifest root.

### 4.3 Reference form: absolute `$id` vs relative path

Two viable strategies:

- **Absolute `$ref` by `$id`** (`{"$ref": "https://thisismyharness.dev/schemas/v1alpha1/defs/identity.schema.json"}`)
  - Pros: one true source; a web consumer can resolve online; no path coupling.
  - Cons: offline/air-gapped resolution needs a resolver map keyed by `$id`; a moved host breaks refs.
- **Relative `$ref` by file path** (`{"$ref": "./defs/identity.schema.json"}`)
  - Pros: fully hermetic, no network.
  - Cons: couples schemas to the directory layout; a web consumer must reconstruct paths; weaker "one canonical URI" story.

**Recommendation:** absolute `$id` refs **plus** a checked-in resolver map (documented in `schemas/README.md`) so both online and offline consumers work. This is the concrete meaning of "language-neutral single source of truth" — the schema is addressed by URI, and any consumer resolves the same URI to the same definition.

### 4.4 Strictness mechanism (`unevaluatedProperties`)

`manifest/README.md` §5.3 requires a validator to **reject unknown fields in a REQUIRED object** unless a later schema version marks the position forward-compatible. Because shared defs are composed via `$ref`/`allOf`, `additionalProperties: false` is the *wrong* tool: it only sees properties declared at the same level and breaks under composition. The correct mechanism is **`unevaluatedProperties: false`** (Draft 2019-09+), which evaluates after all `allOf`/`$ref` contributions. This is a primary reason for the dialect choice in §5.

For positions that §58 leaves **OPEN** (model capability internals, policy language, workflow shape), the schema intentionally stays permissive and says so in `description`.

---

## 5. JSON Schema Dialect

**Recommended dialect:** JSON Schema **Draft 2020-12** (`"$schema": "https://json-schema.org/draft/2020-12/schema"`).

Why 2020-12 and not Draft-07:

1. **`unevaluatedProperties`/`unevaluatedItems`** (added 2019-09, refined in 2020-12) are required to satisfy `manifest/README.md` §5.3 (closed REQUIRED objects) *while* composing shared `$defs` via `allOf`/`$ref`. Draft-07's `additionalProperties: false` cannot express "closed object assembled from refs" without inlining every property — which would destroy the single-source strategy.
2. **`$defs`** is the canonical definitions keyword; the strategy in §4 depends on it.
3. **Precise `$id`/`$ref` URI semantics** (2020-12) make the multi-file `$id` scheme in §3.3 correct and portable.
4. **Current stable generation**; choosing the legacy draft in a greenfield standard would be a deliberate downgrade and would need a migration later.
5. **Tooling availability** is sufficient: both reference implementation families support it — Rust (`jsonschema` crate) and JS/TS (`ajv` v8+, `@hyperjump/json-schema`). Web/SDK consumption is viable.

Rejected: Draft-07 (no `unevaluated*`; see above), OpenAPI-3.x Schema Object (HTTP-API-flavored subset, not a general document validator), JSON Type Definition / RFC 8927 (cannot express conditional composition and cross-file `$defs` needed here).

A **dialect-support verification task** belongs in the proposal: both the Rust validator (F2-05) and any web consumer must be confirmed to support 2020-12 *before* the schemas are frozen.

---

## 6. Manifest Schema Design (F2-01)

Aligned **exactly** with `spec/manifest/README.md`. Four top-level fields, all REQUIRED (§2). Closed objects (`unevaluatedProperties: false`) per §5.3.

| Field | Required | Type | Constraint | Source |
| --- | --- | --- | --- | --- |
| `apiVersion` | MUST | string | `const: thisismyharness.dev/v1alpha1` (see Q3) | `manifest/README.md` §2.1; `versioning.md` §3 |
| `kind` | MUST | string | `enum: [Harness, Component, Preset]` (see Q4) | `manifest/README.md` §2.2 |
| `metadata` | MUST | object | closed; see below | §2, §3 |
| `spec` | MUST | object | closed; every child OPTIONAL | §2, §4 |

### 6.1 `metadata`

| Field | Required | Type | Constraint | Source |
| --- | --- | --- | --- | --- |
| `name` | MUST | string | `slug` grammar (1–64, `^[a-z0-9](?:[a-z0-9-]{0,62}[a-z0-9])?$`); **no `/`** | §3; `identity.md` §2 |
| `version` | MUST | string | SemVer 2.0.0 regex | §3; `versioning.md` §2 |
| `owner` | MAY | string | `owner` grammar (1–40) | §3; `identity.md` §2 |
| `description` | MAY | string | free text, no semantics | §3 |
| `license` | MAY | string | SPDX identifier shape (declared exactly once) | §3; `distribution.md` §7 |
| `author` | MAY | string | free text; **not** verified identity | §3 |
| `homepage` | MAY | string | URI (`format: uri`) | §3 |
| `repository` | MAY | string | URI | §3; `distribution.md` §6 |
| `keywords` | MAY | array | array of strings | §3 |

Rules to encode: descriptive fields carry no semantics (§3.3) — **schema cannot enforce "MUST NOT be used by a resolver"**; hand to F2-06. `metadata.name` MUST NOT contain an owner segment — encode via the slug regex (no `/`).

### 6.2 `spec` sections (all OPTIONAL)

`spec` MUST allow `{}` (minimal passive harness, §4.1). Closed object; each section is independently `$ref`ed.

| Section | Required | Shape | Defined in |
| --- | --- | --- | --- |
| `profile` | MAY | string, single value, enum of 5 recognized profiles (Q5) | `profiles.md` §4 |
| `components` | MAY | array of `componentDescriptor` | `layout.md` §2; `component-types.md` §2 |
| `requirements` | MAY | object: `runtime`(1), `runtimeVersion`, `modelCapabilities`, `hardware`, `services`, `backends` | `requirements.md` §2 |
| `permissions` | MAY | array of `permission` (capability, scope, riskClass, optional justification) | `permissions.md` §2; `risk-classes.md` |
| `extends` | MAY | **ordered** array of reference + optional `override` | `dependencies.md` §2 |
| `distribution` | MAY | object: `artifactType`, `digest`, provenance, source repository | `distribution.md` |
| `compatibility` | MAY | object: declared expectations + level enum (`native/adapted/partial/untested/unsupported`) | `compatibility.md` §2–§3 |
| `conformance` | MAY | object: artifact identity, target, spec apiVersion, adapter version, fixtures, per-capability result, result, tool identity, date | `conformance-metadata.md` §2 |

Notes:

- The schema **cannot** enforce `manifest/README.md` §4.3 ("a harness MUST declare, in `permissions`, every capability it needs above Passive") — that is a cross-section semantic rule. → F2-06.
- `requirements.runtime` is exactly one primary runtime for a Full Harness and MUST NOT appear on a Component (`requirements.md` §2.1) — conditional on `kind`, expressible in-schema with `if/then` on `kind`, but the cross-document rule is better enforced semantically; proposal should decide. (Recommend: express the easy `kind` conditions in-schema; leave graph rules to F2-06.)
- `tools/` and `Router` asymmetries (`layout.md` §4): `tools/` is not a component type; a `Router` is a Model specialization. The `componentDescriptor` discriminator must reflect the **8 canonical types** (Skill, Model, Router, Policy, Agent, MCP, Workflow, Service) and must NOT invent a `Tool` type.

### 6.3 Example instance that MUST validate (F2-01 evidence)

The README/Decisiones conceptual manifest:

```yaml
apiVersion: thisismyharness.dev/v1alpha1
kind: Harness
metadata:
  name: nextjs-production
  version: 1.2.0
  license: Apache-2.0
spec:
  profile: developer
  components: ...
  requirements: ...
  permissions: ...
  distribution: ...
  compatibility: ...
```

The `...` placeholders are not valid YAML values — the F2-01 example must be completed into a real minimal instance. Recommend a minimal valid manifest (e.g. `kind: Harness`, `spec: {}`) plus a richer one exercising each section, both NON-NORMATIVE, stored under the change folder.

---

## 7. Model Contract Schema Design (F2-03)

Derived from `spec/package/component-types.md` §4 (Model) and §5 (Router); Decisiones §8 lists the same ten items. The schema MUST validate the §8 example shape `input: state / output: {route, confidence}`.

| Contract element (§8 / §4) | Schema field | Shape | Notes |
| --- | --- | --- | --- |
| Input contract | `input` | object | Contains a `schema` (a JSON Schema fragment) and/or a `type` label. **Do not invent a DSL** — reuse JSON Schema for the embedded contract. |
| Output contract | `output` | object | Same shape as `input`; typed outcomes expressed as JSON Schema properties (e.g. `route`, `confidence`). |
| Capabilities | `capabilities` | array of `capabilityRequirement` | `$ref` `defs/capability.schema.json`; each MAY name `preferred` + `alternatives`. |
| Resource contract | `resources` | `$ref` `defs/requirement.schema.json#/$defs/hardware` | **Reuse** the hardware def — one definition of OS/arch/CPU/RAM/GPU/VRAM/disk. |
| Execution location | `executionLocation` | enum `[local, remote]` | §4.5 |
| Lifecycle | `lifecycle` | object: install/start/healthCheck/stop/update/remove | Only if the model runs as a service; reuse the Service lifecycle shape (`component-types.md` §10). |
| Fallback | `fallback` | object | Alternative implementation and/or escalation/failure behavior; no vendor required. |
| Version | `version` | `$ref` identity `semver` | §4.8 |
| License | `license` | string (SPDX shape) | **Conflict:** `distribution.md` §7 says package license is declared *once* in `metadata.license`; §4.8 requires a model to declare a version+license. Resolve deliberately (Q7). |
| Artifact source | `artifactSource` | object: `provider` + `digest` + `reference` | Vendor names allowed here as *data* (preferred/alternative implementation), not as Core normative text. |

### 7.1 Router base

`Router` is a Model specialization (`component-types.md` §5) that MUST declare typed outputs, thresholds and fallback/escalation. Recommended: `model-contract.schema.json` exposes a base `modelContract` def plus an optional `router` block (thresholds, escalation target, abstention/no-op outcome). A dedicated `router.schema.json` is *not* required by F2-03 and can come later; keeping one file preserves the evidence path. **Cycle guard:** model-contract must not `$ref` the component descriptor.

### 7.2 Vendor / capability nuance

`component-types.md` §4.1 and `STYLE.md` §5.3: a concrete implementation MAY be named only as preferred/alternative, never required. The *schema* must therefore model a capability requirement as `{ capability, preferred?, alternatives[] }` and must never add a `const`/`enum` containing a vendor. A vendor string in an **instance** is user data; a vendor in the **schema** would violate `STYLE.md` §5. `schemas/README.md` and every example must avoid vendor names.

---

## 8. Install Plan Schema Design (F2-04)

Authoritative field list: `spec/install-protocol/README.md` §4 table (**16 fields**, `MUST enumerate each`), reinforced by §7.1 (must record risk class **and** required autonomy level) and glossary "Install Plan". Install Plan is the **shared serializable contract** between web, desktop, CLI and agents.

| # | §21 / §4 field | Schema field | Shape | Constraint / source |
| --- | --- | --- | --- | --- |
| 1 | Runtime | `runtime` | `{ id: capabilityId, version: string }` | `runtime.<id>` capability; `requirements.md` §2.1 |
| 2 | Runtime version | `runtimeVersion` | string | resolved concrete version; `requirements.md` §2.2 |
| 3 | Project | `project` | `{ path: string, ... }` | target project; local-only field (see risk R6) |
| 4 | Scope | `scope` | enum `[project, user]` | `$ref` `defs/permission`; default `project` (`permissions.md` §1) |
| 5 | Harness / version | `harness` | `{ id: canonicalIdentifier, version: semver, digest: digest[] }` | digest MUST be recorded (`dependencies.md` §1.2; `distribution.md` §4.4) |
| 6 | Files created / modified | `files` | array of `{ path, action: create\|modify, digest?, size? }` | managed files only; `layout.md` §2 |
| 7 | Conflicts | `conflicts` | array of `{ path, reason, resolution }` | MUST NOT be resolved silently; `resolution` ∈ `none`/`declared-override` (`dependencies.md` §5) |
| 8 | Adaptations | `adaptations` | array of `{ capability, level: adapted\|partial, difference }` | `compatibility.md` §2; `adapter-contract` §6 |
| 9 | Unsupported capabilities | `unsupportedCapabilities` | array of `{ capability, required: bool, blocksApply: bool }` | required+unsupported MUST block Apply (`compatibility.md` §2.3) |
| 10 | MCP | `mcp` | array of component refs | `component-types.md` §8 |
| 11 | Hooks / scripts | `hooksScripts` | array of executable component refs | risk C+; `layout.md` §4.4 |
| 12 | Services | `services` | array of `{ ref, lifecycleActions[] }` | `component-types.md` §10 |
| 13 | Required environment-variable names | `environmentVariableNames` | array of strings, **NAMES ONLY** | `permissions.md` §4; pattern + name-only description; schema cannot fully prove no value |
| 14 | Risk | `risk` | `{ effective: riskClass, autonomyLevel: autonomyLevel }` | effective = max over parts; floor rule (`risk-classes.md` §3–§4; §7.1) |
| 15 | Snapshot | `snapshot` | `{ id, covers[] }` | state the snapshot will cover (`install-protocol` §3.10) |
| 16 | Verification steps | `verification` | array of `{ check, kind? }` | checks `verify` will run (`install-protocol` §3.12) |

Envelope question (Q8): should the plan carry plan-level `apiVersion`/`kind: InstallPlan`/`planHash`? §4.3 requires a stable machine-readable form and §4.6 requires it "bound to the inputs that produced it"; §4.10 says the plan MUST enumerate the 16 fields but does not explicitly forbid an envelope. Recommendation: keep the 16 as the **required core** and, if an envelope is added, mark any extra property explicitly and deliberately rather than by default.

Determinism concern: §4.2 requires identical inputs+environment → identical plan. A schema cannot enforce determinism, and a `generatedAt` timestamp would **break** it — so the schema should **not** include non-deterministic fields in the core plan. Hand to F2-05/06 (runtime test).

Local-only concern: the plan contains a local project path. `Verified Use` (`§48`) says a local path MUST NOT be sent anywhere. The schema is local-machine data; `schemas/README.md` and the validator docs (F2-12) must state that an Install Plan is never uploaded.

---

## 9. Schema Versioning + Migration + §58 Open Items

### 9.1 Version series

Two independent series (`spec/core/versioning.md` §1; `spec/VERSIONING.md`):

- **Spec version** = `apiVersion` = `thisismyharness.dev/v1alpha1` — which generation of the standard.
- **Package version** = `metadata.version` — SemVer of one package.

The schema is a **separate artifact targeting a spec generation**; it is not itself a harness package. The `$id` path carries the spec generation (`/v1alpha1/`), so a future schema for `v1beta1` lives at `/v1beta1/` without re-pathing alpha.

### 9.2 `$defs` evolution rules

- **Additive-only within a generation**: add an optional field/property; never remove or reinterpret (`VERSIONING.md` "Changes to `apiVersion`").
- **Breaking change → new generation directory** and a migration note (mandatory per `VERSIONING.md`).
- A shared `defs/*` change is a **contract change for every consumer**; the proposal should require that any change to a shared def carry the same review weight as a Core spec change.

### 9.3 §58 open items touching these schemas

| §58 open item | Schema impact | Handling |
| --- | --- | --- |
| **Definitive manifest name** (working: `harness.yaml`) | If the filename is open, the schema describes the *document shape*, not the filename. `$id` resource name `harness` would freeze a name early. | Describe document shape; document that `harness` in the `$id` is a working resource name; do not make the filename normative in the schema. |
| **Definitive OCI media type** | `artifactType` is semantically required but the media type is OPEN (`distribution.md` §3). | Schema constrains `artifactType` as a string **pattern**, never a `const` containing a media type; mark OPEN in `description`. |
| **Namespace / canonical identifier** | `identity.md` grammar is fixed, but the authoritative host is open. | `canonicalIdentifier` uses a **pattern with `host` as a variable**, never a fixed host. `$id` host (`thisismyharness.dev`) is a schema-namespace, explicitly not a harness host. |
| **Exact model capability contract** | Model I/O + capability internals are OPEN. | `model-contract.schema.json` stays **permissive** on input/output internals; mark OPEN. Do not lock a capability registry. |
| **Core vs extension capabilities** | Schema validates capability **shape**; existence is registry work. | `capabilityId` is a pattern; existence → F2-07. |
| **SemVer of spec vs package** | Two fields, two grammars. | Modeled as two distinct defs; never conflate. |
| **Policy language, workflow graph, secrets handling** | The manifest `policies`/`workflows` components and secrets must not lock a DSL or a value channel. | Components are descriptors, not DSLs; `environmentVariableNames` is names-only. |
| **Profiles versioning** | `spec.profile` is an enum of 5 today; extension profiles? | Model the 5 as recognized; allow a namespaced extension branch (Q5). |

**Rule for the proposal:** every §58-open position in the schema MUST carry an explicit `description` marking it OPEN and pointing to §58. F2 must not silently resolve an open item (this was an explicit F1 risk, #4363).

---

## 10. What JSON Schema CANNOT Express → Semantic Checks (F2-05..F2-12)

This boundary is a first-class deliverable of F2-02: the schema is necessary but **not sufficient** for conformance. The list below is handed to the validator tasks.

1. **Supported `apiVersion` set** and the migration map — schema can `const` one value; the supported set + unknown-version typed error is semantic (F2-11).
2. **Unknown `kind` unless a compatible extension is present** — needs the extension registry (`manifest/README.md` §2.2.4).
3. **Reference resolution** — scoped ref resolves against the declaring host; dependencies resolve to digests (`dependencies.md` §1, §3).
4. **Dependency graph analysis** — cycles, version/permission/capability/kind conflicts, identity ambiguity, duplicate identity (`dependencies.md` §5).
5. **Permission coverage** — every component above Passive MUST be declared in `permissions` (`manifest/README.md` §4.3; `risk-classes.md` §3).
6. **Effective risk / monotonicity** — max over parts; plan effective risk (`risk-classes.md` §3).
7. **Autonomy floor** — plan risk class implies a minimum autonomy; no absolute bypass (`install-protocol` §7).
8. **Capability existence + capability schema** — against the extension registry (`requirements.md` §2.3; F2-07).
9. **Package discovery semantics** — single manifest, explicit-vs-conventional ambiguity, determinism, case-insensitive collisions, `.`-prefixed ignore, `README.md` not a component (`layout.md` §1–§3).
10. **Path safety** — `..`/absolute paths, symlink escape, package-root containment (`layout.md` §2; §50).
11. **License declared exactly once** across the package (`distribution.md` §7).
12. **`environmentVariableNames` are names, not values** — a value can be shaped like a name; a secret/value scan is semantic (F2-12).
13. **Digest/immutability semantics** — tag→digest, published immutability (`distribution.md` §4; `versioning.md` §6).
14. **Declared vs verified compatibility** — expectations are informative, never presented as verified (`compatibility.md` §3.2, §5).
15. **SemVer range parsing + deterministic selection** — precedence and selection are algorithmic (`versioning.md` §2.1, §4).
16. **Plan MUST NOT contain/require executing third-party code** (`install-protocol` §4.5).
17. **Plan determinism** — identical inputs+environment → identical plan; a runtime test (`install-protocol` §4.2).
18. **Plan binding to inputs** — material change → new plan (`install-protocol` §4.6–§4.7).
19. **Trust label precision** — exactly which checks passed (`distribution.md` §5; `conformance-metadata.md` §5).
20. **Extension `apiVersion` range compatibility** — needs extension registry (`versioning.md` §4–§5).
21. **`metadata.owner` matches the publishing namespace** — registry context (`manifest/README.md` §3).
22. **Descriptive fields MUST NOT influence compatibility/permission decisions** — behavioral (`manifest/README.md` §3.3).
23. **Source document conflicts** already resolved in F1 (adapter 11 ops, license location, pipeline naming) must not be re-opened by schema convenience (#4363).
24. **YAML parsing normalization** — duplicate keys, anchors, implicit typing; the schema validates parsed JSON, the parser must be strict (semantic, F2-05).

---

## 11. Open Questions

- **Q1 — `$id` host.** Is freezing `thisismyharness.dev` as the schema `$id` namespace safe while §58 keeps the *harness* host open? (Recommended: yes, documented as a namespace distinction; verify.)
- **Q2 — File layout.** Flat versioned filenames (F2 evidence) vs a `schemas/v1alpha1/` directory. (Recommend flat + `$id` versioning.)
- **Q3 — `apiVersion` enforcement.** `const: thisismyharness.dev/v1alpha1` (fail-closed, blocks extension manifests) vs a pattern + validator-supported-set (F2-11). Recommendation: `const` on the Core branch; extension kinds get their own schema `$ref`ing the shared defs.
- **Q4 — `kind`.** `enum: [Harness, Component, Preset]` vs a namespaced pattern for extension kinds. `manifest/README.md` §2.2 says only `Harness` is fixed in Core and `Component`/`Preset` are recognized-but-later.
- **Q5 — `spec.profile`.** Enum of the 5 recognized profiles vs also allowing a namespaced extension profile.
- **Q6 — Model I/O contract.** Free-form embedded JSON Schema vs a typed descriptor. (Recommend embedded JSON Schema; no new DSL.)
- **Q7 — Model license vs package license.** `distribution.md` §7 says package license declared once in `metadata.license`; `component-types.md` §4.8 requires a model license. Duplicate-declaration rule to be decided.
- **Q8 — Install Plan envelope.** Strict 16 fields vs the 16 + plan-level `apiVersion`/`kind`/binding. (§4.10 says the plan MUST enumerate each of the 16.)
- **Q9 — Strictness in OPEN positions.** How permissive must the schema be where §58 is open, without failing `manifest/README.md` §5.3?
- **Q10 — Reference form.** Absolute `$id` refs + resolver map vs relative paths (offline vs canonical-URI trade-off).
- **Q11 — Where instances live.** Inline examples in the change folder vs deferring all fixtures to F3 (`examples/` is empty and is F3's).
- **Q12 — Manifest filename.** Does the schema `$id`/title bake in `harness.yaml` while §58 keeps the name open?

---

## 12. Risks

- **R1 — Schema/spec divergence (highest).** Any schema convenience that weakens a MUST is a spec violation (#4363). *Mitigation:* a traceability matrix in `schemas/README.md` mapping every constraint to an F1 section; review against F1 before freeze.
- **R2 — Silently fixing §58.** Hardcoding a media type, capability registry, manifest name or host would resolve an open item by accident. *Mitigation:* explicit OPEN notes in `description`; proposal must list every touched open item.
- **R3 — Dialect/tooling mismatch.** 2020-12 `unevaluatedProperties` support varies across Rust/TS validators and web consumers. *Mitigation:* verify the chosen validators before freeze (a proposal task).
- **R4 — `$ref` resolution in offline contexts.** Absolute `$id` refs without a resolver map fail air-gapped. *Mitigation:* checked-in resolver map + documented.
- **R5 — Vendor leakage into schemas.** Vendor names in a `const`/`enum`/example would violate `STYLE.md` §5. *Mitigation:* no vendor in schema; examples NON-NORMATIVE and vendor-free.
- **R6 — Install Plan privacy.** The plan carries a local project path; must never be uploaded (`§48`). *Mitigation:* document plan as local-only in `schemas/README.md` and F2-12.
- **R7 — Plan determinism broken by schema.** A `generatedAt`/timestamp field would violate §4.2. *Mitigation:* no non-deterministic fields in the core plan.
- **R8 — Strictness vs alpha velocity.** `unevaluatedProperties: false` everywhere may make alpha iteration painful, but §5.3 mandates fail-closed in REQUIRED objects. *Mitigation:* a documented forward-compatible escape hatch for explicitly-marked positions.
- **R9 — YAML vs JSON hazards.** `harness.yaml` is YAML; duplicate keys/anchors/implicit types can change the parsed data model. *Mitigation:* strict parser + semantic checks (F2-05); schema documents the JSON data model, not YAML syntax.
- **R10 — Poor error messages from composed schemas.** `allOf`+`unevaluatedProperties` can yield unhelpful diagnostics, but F2-05 requires precise `{path, keyword, expected}`. *Mitigation:* design refs to be shallow; F2-05 owns message quality.
- **R11 — F2-01 example is incomplete.** The README manifest uses `...` placeholders; validating it as-is is impossible. *Mitigation:* author completed minimal/rich examples in the change folder.

---

## 13. Alternatives Considered and Rejected

| # | Alternative | Why rejected |
| --- | --- | --- |
| A1 | **Single monolithic schema file** (all `$defs` inline) | Trivial resolution, but couples manifest/model/plan versioning; a change to any def bumps one artifact; web/SDK cannot consume only what they need. Rejected for modular shared defs. |
| A2 | **Draft-07** | No `unevaluatedProperties`; cannot express closed REQUIRED objects composed from shared refs (needed for `manifest/README.md` §5.3). Legacy choice in a greenfield standard. |
| A3 | **OpenAPI 3.x Schema Object** | HTTP-API-oriented subset; weaker as a general document validator. |
| A4 | **JSON Type Definition (RFC 8927)** | Simpler, but cannot express conditional composition/cross-file `$defs` needed for extension composition. |
| A5 | **Code-first types** (Rust structs or TS types as source; generate JSON Schema) | Makes a language the source of truth, contradicting ADR-0001 and `openspec/config.yaml` ("JSON Schema is the single source of truth; language decision reversible"). Codegen *from* JSON Schema remains allowed. |
| A6 | **`const` vs pattern `apiVersion`** (from Q3) | `const` is fail-closed but blocks extension manifests from the Core schema; recommended resolution is a `const` Core branch + extension schemas sharing the defs. |
| A7 | **Flat filenames vs versioned directory** (from Q2) | F2 evidence pins flat filenames; `$id` carries the generation. Directory versioning documented as the fallback. |
| A8 | **Relative `$ref` only** | Hermetic but couples schemas to layout and weakens the canonical-URI story for web. Recommended absolute `$id` + resolver map. |
| A9 | **Enrich the Install Plan beyond §21** | §4.10 says the plan MUST enumerate each of the 16 fields; extra fields should be deliberate, not default. |
| A10 | **One `model-contract` schema with Router as separate file** | F2-03 names a single `model-contract.schema.json`; Router is a Model specialization — keep one file with an optional `router` block. |
| A11 | **Put capability registry in schema** | Existence is registry work (F2-07); schema validates shape only, preserving Core-small. |
| A12 | **Resolve §58 open items now** (fix media type, host, manifest name) | Prohibited: `STYLE.md` §3.5 says open items MUST NOT be treated as normative; F1 left them OPEN. |

---

## 14. Recommended Approach (input to `sdd-propose`)

1. **Dialect:** JSON Schema **Draft 2020-12**, justified by `unevaluatedProperties` (closed REQUIRED objects composed from shared refs) and current tooling.
2. **Layout:** keep the F2-pinned filenames `schemas/harness.v1alpha1.schema.json`, `schemas/model-contract.schema.json`, `schemas/install-plan.schema.json`; add `schemas/README.md` (the F2-02 strategy artifact) and `schemas/defs/*.schema.json` shared definitions.
3. **`$id`: `https://thisismyharness.dev/schemas/v1alpha1/…`** — a schema namespace, explicitly distinct from the §58-open harness host. Document the distinction.
4. **`$ref` strategy:** absolute `$id` references + a checked-in resolver map for offline validation; each shared concept defined exactly once in `defs/`.
5. **Strictness:** `unevaluatedProperties: false` on REQUIRED objects (manifest §5.3); permissive and OPEN-marked where §58 is open.
6. **Traceability:** `schemas/README.md` carries an F1-section→constraint matrix; every non-OPEN constraint cites its source. This is the direct mitigation for R1.
7. **Semantic boundary:** ship the explicit list in §10 as the hand-off contract to F2-05..F2-12; no schema tries to be a validator.
8. **Single source of truth:** the schema is the source; the Rust validator, SDK and web consumer resolve the same `$id`s. No hand-written parallel type definitions.
9. **Scope discipline:** this change writes **schemas + strategy + minimal examples only**. No validator, no CLI, no fixtures corpus (F3), no capability registry.
10. **Do not resolve §58**: every touched open item is marked OPEN in-schema and listed in the proposal.

---

## 15. Affected Areas

- `schemas/` — new `README.md`, three root schemas, `defs/` (the change's main output).
- `spec/**` — read-only normative source; MUST NOT be edited by this change (if a schema reveals an F1 gap, that is a separate RFC/Core change per `STYLE.md` §6.3).
- `docs/adr/` — likely one new ADR recording the schema strategy (dialect, `$id`, modular defs, single-source) with rejected alternatives (A1–A12).
- `openspec/changes/harness-schema-v1alpha1/` — this exploration plus the coming proposal/spec/design/tasks.
- Engram — `build-progress/F2` (#4271), `architecture/product-plan` (#4322), `sdd/harness-schema-v1alpha1/*`.
- Future consumers (not edited now): `packages/validator` (F2-05+), `packages/sdk`, web layer (F17).

---

## 16. Ready for Proposal

**Yes.** The normative source (F1) is complete and stable enough to derive from; the change is well-bounded (4 tasks, schemas + strategy only); the main design decisions (dialect, layout, `$id`, ref strategy, strictness, semantic boundary) have a recommended path with rejected alternatives. The proposal should record the schema-strategy ADR and enumerate every §58-open item touched.
