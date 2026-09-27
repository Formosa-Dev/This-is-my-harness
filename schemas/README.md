# Harness Schema Set — Strategy (F2)

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1` · **Risk class:** A (Passive) · **Default autonomy:** 0 (Preview)
> **NON-NORMATIVE strategy document.** The normative source is `spec/**` (F1). JSON Schema is the single machine-readable source of truth; this document explains how it is organised, resolved and verified. It does not weaken any F1 `MUST`.

This directory ships the machine-readable contract for the manifest, the model contract and the Install Plan. The Rust validator (F2-05..F2-12), the SDK and the future web layer are adapters that resolve the same `$id`s and never re-declare a shape already defined here.

## Dialect

**JSON Schema Draft 2020-12** (`"$schema": "https://json-schema.org/draft/2020-12/schema"`).

Draft 2020-12 is required because `spec/manifest/README.md` §5.3 mandates rejecting unknown fields in a required object, while the schema set composes shared `$defs` through `$ref`/`allOf`. The closure keyword is therefore `unevaluatedProperties: false`, which evaluates after every `allOf`/`$ref` contribution; `additionalProperties: false` only sees properties declared at the same level and breaks under composition. Draft-07 cannot express this without inlining every property, which would destroy the single-source strategy.

## `$id` map

Every schema and shared definition declares a unique `$id` under the base namespace `https://thisismyharness.dev/schemas/v1alpha1/`. The generation lives in the `$id` path, so a future `v1beta1` adds parallel files without re-pathing `v1alpha1`. The `v1alpha1` in `harness.v1alpha1.schema.json` is human convenience: the `$id` is authoritative and this document says so.

| File | `$id` | Role |
|---|---|---|
| `harness.v1alpha1.schema.json` | `…/v1alpha1/harness.schema.json` | Manifest composition root (F2-01) |
| `model-contract.schema.json` | `…/v1alpha1/model-contract.schema.json` | Model + Router contract (F2-03) |
| `install-plan.schema.json` | `…/v1alpha1/install-plan.schema.json` | 16-field Install Plan (F2-04) |
| `defs/identity.schema.json` | `…/v1alpha1/defs/identity.schema.json` | slug, owner, refs, SemVer, range |
| `defs/capability.schema.json` | `…/v1alpha1/defs/capability.schema.json` | capability id + requirement |
| `defs/requirement.schema.json` | `…/v1alpha1/defs/requirement.schema.json` | runtime, hardware, services, backends |
| `defs/permission.schema.json` | `…/v1alpha1/defs/permission.schema.json` | permission, scope, risk class, autonomy |
| `defs/component.schema.json` | `…/v1alpha1/defs/component.schema.json` | component descriptor + 8-type discriminator |
| `defs/dependency.schema.json` | `…/v1alpha1/defs/dependency.schema.json` | extends entries, override |
| `defs/distribution.schema.json` | `…/v1alpha1/defs/distribution.schema.json` | digest, artifact type, trust label |

**Namespace, not harness host.** `thisismyharness.dev` in the `$id` is the already-frozen `apiVersion` group used as a *schema-identity namespace*. It is explicitly **not** the canonical harness-resolution `(host, owner)` that decision record §58 leaves OPEN (`spec/core/identity.md` §8). Treating the `$id` host as the normative harness host MUST be rejected.

## Resolver map

`schemas/registry.json` is the checked-in `$id` → repo path map. Each consumer resolves the same URI to the same definition offline:

| Consumer | How it resolves offline |
|---|---|
| Rust validator (F2-05..F2-12) | Builds a registry from `registry.json`; no network. Dialect support confirmed before freeze (R3). |
| SDK (later) | Codegen *from* JSON Schema via the registry; never hand-written parallel types. |
| Web / F17 | Fetches the absolute `$id` or ships the registry bundle; same URIs. |

Relative-only refs are rejected: they are hermetic but layout-coupled and provide no canonical-URI story. A `$ref` whose target is not declared in `registry.json` or has no file on disk fails loudly (L0 check).

## Strictness

| Position | Strictness |
|---|---|
| manifest root, `metadata`, `spec` container | `unevaluatedProperties: false` |
| closed objects: `hardware`, each `permission` entry, component descriptor base, plan root, plan `risk`, plan `snapshot`, plan file/conflict entries | `unevaluatedProperties: false` |
| §58-OPEN positions | permissive, no closure, marked OPEN |

The closure is structural, not conventional. `component → model-contract` is a one-way edge (the model contract MUST NOT `$ref` the component descriptor), and the Install Plan `$ref`s leaf definitions only and never the manifest root.

## §58 OPEN items

Every position touching an item left OPEN by decision record §58 carries an explicit `OPEN` marker naming §58 and is never constrained to a fixed value.

| §58 open item | Schema handling |
|---|---|
| Definitive manifest filename | The schema describes the **document shape**; `harness` in the `$id` is a working resource name, not a normative filename. |
| Definitive OCI media type | `artifactType` is a **pattern**, never a media-type `const`, marked OPEN. |
| Canonical host / namespace | `canonicalIdentifier` uses a pattern with the **host as a variable**, never a fixed host. |
| Exact model-capability contract | The model contract stays **permissive** on `input`/`output` internals. |
| Core-vs-extension capabilities | The schema validates **shape only**; existence is F2-07. |
| Policy language / workflow graph | Components are **descriptors**, not DSLs. |
| Secrets handling | Only `environmentVariableNames` (names only). |
| Profiles versioning | Five recognized profiles plus an optional namespaced extension branch. |

## F1 traceability matrix

Every non-OPEN constraint in the schema set cites the F1 (`spec/**`) section that authorizes it. This is the direct mitigation for risk R1: a schema convenience that would permit, weaken or contradict an F1 `MUST` is a specification violation and is rejected before freeze, and the inconsistency is never resolved by editing `spec/**` inside this change. §58-OPEN positions are deliberately **not** constraints and are therefore not traced here; they are listed above and marked OPEN in-schema.

| Schema constraint | F1 anchor |
|---|---|
| Exactly four REQUIRED top-level fields; root object closed | `spec/manifest/README.md` §2; §5.3 |
| `apiVersion` is a single value, never a range | `spec/manifest/README.md` §2.1; `spec/core/versioning.md` §3 |
| `kind` restricted to `[Harness, Component, Preset]` | `spec/manifest/README.md` §2.2; `spec/package/component-types.md` §1 |
| `metadata` closed; `name` REQUIRED slug 1–64 with no `/` | `spec/manifest/README.md` §3; `spec/core/identity.md` §2 |
| `version` REQUIRED, valid SemVer | `spec/manifest/README.md` §3; `spec/core/versioning.md` §2 |
| Optional metadata fields (owner, description, license, author, homepage, repository, keywords) | `spec/manifest/README.md` §3 |
| `spec` REQUIRED object; `{}` valid; eight OPTIONAL sections | `spec/manifest/README.md` §4; §4.1; §4.2 |
| Unknown field in a REQUIRED object rejected (`unevaluatedProperties`) | `spec/manifest/README.md` §5.3 |
| `requirements`: runtime(1), runtimeVersion, modelCapabilities, hardware, services, backends | `spec/core/requirements.md` §2.1–§2.6 |
| `hardware` closed seven-field resource shape | `spec/core/requirements.md` §2.4 |
| `permission` entry: permission, scope, riskClass, autonomyLevel | `spec/core/permissions.md` §2; `spec/core/risk-classes.md` §1 |
| `scope` enum `[project, user]`, default `project` | `spec/core/permissions.md` §1 |
| `riskClass` enum A–D and `autonomyLevel` 0–3 | `spec/core/risk-classes.md` §1; §4 |
| `extends` ordered array; optional override; no silent conflict resolution | `spec/core/dependencies.md` §2; §5; §6 |
| `distribution` digest and provenance; a trust label is verified, not declared | `spec/core/distribution.md` §2; §4; §5; §6 |
| `compatibility.level` enum native/adapted/partial/untested/unsupported | `spec/core/compatibility.md` §2; §3 |
| `profile` single value; five recognized profiles | `spec/core/profiles.md` §4 |
| `componentDescriptor` discriminates the eight canonical types; no `Tool` type | `spec/package/component-types.md` §2; `spec/package/layout.md` §2; §4 |
| Component descriptor base closed; explicit declaration is authoritative | `spec/package/layout.md` §2; §3 |
| `conformance` metadata shape (artifact, target, result, tool, date) | `spec/core/conformance-metadata.md` §2; §3 |
| Model contract REQUIRED elements (input, output, capabilities, resources, executionLocation, fallback, version, artifactSource) | `spec/package/component-types.md` §4 |
| `executionLocation` enum `[local, remote]` | `spec/package/component-types.md` §4.5 |
| Model resources reuse the shared hardware definition (never redefined) | `spec/package/component-types.md` §4.4; `spec/core/requirements.md` §2.4 |
| Service-backed model lifecycle requires all six actions | `spec/package/component-types.md` §4.6; §10 |
| `capabilities` require at least one; contracts, never hardcoded brands | `spec/package/component-types.md` §4.3 |
| `artifactSource` `{provider, digest, reference}`; no fixed provider | `spec/package/component-types.md` §4.9; `spec/core/distribution.md` §2 |
| Router block: typed outputs, thresholds, fallback | `spec/package/component-types.md` §5 |
| Model contract MUST NOT reference the component descriptor (no cycle) | `spec/package/component-types.md` §5.3 |
| Install Plan requires all sixteen fields | `spec/install-protocol/README.md` §4 |
| Plan is deterministic; no timestamp in the core plan | `spec/install-protocol/README.md` §4 |
| `environmentVariableNames` names-only pattern; no values | `spec/core/permissions.md` §4; `spec/install-protocol/README.md` §4 |
| `risk` records effective class and required autonomy level | `spec/install-protocol/README.md` §7; `spec/core/risk-classes.md` §3; §4 |
| `conflicts` carry a reason and an explicit resolution | `spec/core/dependencies.md` §5 |
| `unsupportedCapabilities` mark required/blocksApply; a required unsupported capability blocks Apply | `spec/core/compatibility.md` §2 |
| Plan contains no executable third-party code; hooks and scripts are references | `spec/install-protocol/README.md` §4 |
| Plan never requires an upload or remote destination for the local project path | `spec/install-protocol/README.md` §4; §6 |
| No vendor token in any schema `const`/`enum` (vendor neutrality) | `spec/STYLE.md` §5 |
| `canonicalIdentifier` host is a variable, never a fixed host | `spec/core/identity.md` §2; §8 |
| Descriptive fields carry no semantics and never drive a decision | `spec/manifest/README.md` §3 |
| One manifest document; no aggregate or array root | `spec/manifest/README.md` §1 |

## Semantic boundary

JSON Schema is necessary but not sufficient for conformance. The checks below cannot be expressed by a schema and are owned by the downstream validation phases.

| Semantic check | Why the schema cannot express it | Owner |
|---|---|---|
| Supported `apiVersion` set and migration map | A `const` fixes one value; the supported set and the unknown-version typed error are runtime logic. | F2-11 |
| Unknown `kind` unless a compatible extension is present | Requires the extension registry. | F2-06 |
| Reference resolution (scoped refs, digest resolution) | Graph-level, needs the resolver and the registry. | F2-06 |
| Dependency-graph analysis (cycles, conflicts, duplicate identity) | The schema validates the `extends` shape only; the graph is semantic. | F2-06 |
| Permission coverage for capabilities above Passive | Cross-section rule across `components` and `permissions`. | F2-06 |
| Effective risk, monotonicity and the autonomy floor | A maximum over parts and a floor derived from risk class. | F2-05 |
| Capability existence against the extension registry | Existence is a registry lookup, not a shape. | F2-07 |
| Package discovery, path safety and symlink containment | Filesystem semantics, not a document shape. | F2-08 |
| Install-Plan determinism and binding to inputs | A runtime property of plan generation, not a schema constraint. | F2-09 |
| Trust-label precision and declared-vs-verified compatibility | Requires conformance evidence. | F2-10 |
| `environmentVariableNames` are names, not values | A secret value can be shaped like a name; needs a semantic scan. | F2-12 |
| Digest / immutability semantics | Tag-to-digest and published immutability are registry behaviour. | F2-07 |

## Install Plan is local-only

An Install Plan contains a local project path and is **local-machine data**. It MUST NOT be uploaded, transmitted or published by the schema layer or by any consumer. The schema never requires or introduces a remote destination for a plan. This note mirrors the Verified Use rule (§48) and risk R6; the validator documentation (F2-12) restates it.

---

## Pre-freeze blocker (R3)

Risk **R3** is a pre-freeze blocker recorded here and in `docs/adr/0002-schema-strategy.md`: the Rust `jsonschema` crate's support for Draft 2020-12 and `unevaluatedProperties` is **unverified on this machine**, because Rust is not installed here. The dialect is not frozen until F2-05 confirms support in the Rust validator.

Interim proof of portability: the L1 (`ajv` v8, 2020-12) and L2 (Python `jsonschema` + `referencing`) engines validate the same corpus to the same result, so two independent 2020-12 implementations agree on the features this schema set depends on.

Freeze gate state: `r3=pending`. An actual freeze requires the gate to read `FREEZE OK` (`python scripts/schemas/check_schemas.py --freeze`); the ADR and this document are updated when F2-05 confirms support.

## Verification (what is actually checked)

The schema set is verified in layers, and this section claims only what those layers check — never an absolute-safety claim and never a conformance claim:

- **L0** — `python scripts/schemas/check_schemas.py`: JSON parse, Draft 2020-12 dialect, `$ref` integrity against `registry.json` and disk, vendor-free `const`/`enum`, §58 OPEN markers, no reference cycle, README sections. Python standard library only.
- **L1** — `npm run validate:schemas`: ajv v8 in 2020-12 mode over the change-folder corpus (positive instances MUST pass; negative instances MUST fail with the expected `{path, keyword}`).
- **L2** — `python scripts/schemas/cross_check.py`: the same corpus under Python `jsonschema` + `referencing.Registry` as an independent engine.

## Trust statement

Verification of the schema layer proves only that the schemas parse, that the reference graph resolves, and that the corpus passes and fails as declared. It does not prove that any harness is safe, secure or conformant, and it is not a security guarantee. Each layer names exactly the checks it performs and claims nothing beyond them. The artifacts are risk class A (Passive), default autonomy 0 (Preview): the schema layer performs no mutation and executes no third-party code.

