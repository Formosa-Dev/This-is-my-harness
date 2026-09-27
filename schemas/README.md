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

## Verification (what is actually checked)

The schema set is verified in layers, and this section claims only what those layers check — never "100% safe" and never conformance:

- **L0** — `python scripts/schemas/check_schemas.py`: JSON parse, Draft 2020-12 dialect, `$ref` integrity against `registry.json` and disk, vendor-free `const`/`enum`, §58 OPEN markers, no reference cycle, README sections. Python standard library only.
- **L1** — `npm run validate:schemas`: ajv v8 in 2020-12 mode over the change-folder corpus (positive instances MUST pass; negative instances MUST fail with the expected `{path, keyword}`).
- **L2** — `python scripts/schemas/cross_check.py` (F2-05 / PR 5): the same corpus under Python `jsonschema` + `referencing.Registry` as an independent engine.

Verification of the schema layer proves only that the schemas parse, that the corpus passes/fails as declared, and that the structural checks above hold. It does not prove that any harness is safe or conformant.
