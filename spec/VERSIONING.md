# Versioning Policy

This document defines how the This is my Harness specification and its artifacts are versioned, and the compatibility and migration rules that follow.

> Status: pre-alpha. The current schema `apiVersion` is `thisismyharness.dev/v1alpha1`. Nothing here is stable until the v1.0 gate is met.

---

## Two independent version series

The project versions **two different things**, and they MUST NOT be conflated:

| Series | What it versions | Example | Governed by |
| --- | --- | --- | --- |
| **Spec version** | The `apiVersion` of the specification and its JSON Schema | `thisismyharness.dev/v1alpha1` | The Core maintainers; follows this document. |
| **Package version** | A specific published harness artifact | `1.2.0` | The harness author; declared in the manifest `metadata.version`. |

A package version says **which release of one harness** you have. A spec version says **which generation of the standard** that harness conforms to. They move independently: a harness author MAY publish `2.0.0` of their harness without any change to the spec, and the spec MAY publish a new `apiVersion` without touching any package.

---

## Spec versioning

### Format

The spec uses a Kubernetes-style `apiVersion` string:

```
thisismyharness.dev/v1alpha1
```

- **Group:** `thisismyharness.dev`
- **Version:** `v<major><stability><minor>` — e.g. `v1alpha1`, `v1beta1`, `v1`.

### Stability stages

| Stage | Meaning | Compatibility promise |
| --- | --- | --- |
| `v1alpha1` | Early draft. **Current stage.** May change incompatibly between alpha releases. | None. |
| `v1beta1` | Feature-complete, still subject to breaking change with notice. | Best-effort, documented in a migration note. |
| `v1` | Stable. | Stability guarantees below apply. |

The spec is currently at **`v1alpha1`**. Its purpose is explicitly to let the Reference Harnesses break the design before stabilization.

### The v1.0 gate

Declaring `v1` (stable) is **prohibited** until the criteria in [GOVERNANCE.md](GOVERNANCE.md) and the project's launch criteria are met — at minimum:

- the Reference Harnesses (Web Agent, Developer, Local Hybrid, Multi-Agent, Workflow Automation) are implemented and used in real scenarios;
- the Core can represent them without runtime-specific hacks;
- a real validator, a real plan/install path and a tested rollback exist;
- at least three runtime adapters are demonstrated at different levels;
- a conformance suite and a threat model exist.

Documentation or branding is not a substitute for any of the above.

### Changes to `apiVersion`

- An **incompatible** change to the specification MUST bump the version and MUST be recorded through an RFC.
- Adding an **optional** field that old consumers can safely ignore MAY be done within the same alpha version.
- Removing or reinterpreting a field, changing a default, or tightening validation is **incompatible**.
- Deprecations MUST be announced for at least one version before removal, with a migration note.

---

## Package versioning

Harness packages MUST use [Semantic Versioning 2.0.0](https://semver.org/):

```
MAJOR.MINOR.PATCH
```

- **MAJOR** — incompatible change to the harness's declared interface, components or permissions.
- **MINOR** — backwards-compatible addition (for example a new optional component or capability).
- **PATCH** — backwards-compatible fix or documentation change.

Because the spec is `v1alpha1`, package authors SHOULD treat the spec version as part of their compatibility surface: a package that bumps its supported `apiVersion` is making a MAJOR-level change.

### Declaring the spec version

The manifest declares the spec generation it targets:

```yaml
apiVersion: thisismyharness.dev/v1alpha1
kind: Harness
metadata:
  name: example
  version: 1.2.0
```

The `apiVersion` is the **spec** version; `metadata.version` is the **package** version. A validator MUST reject a manifest whose `apiVersion` is not one it supports.

---

## Extension versioning

Extensions are versioned independently of the Core, using the same `apiVersion` shape under the extension's own namespace. An extension declares the Core `apiVersion` range it is compatible with. A change to an extension's meaning or shape MUST follow the same incompatible/semver rules as the Core, scoped to that extension.

---

## Artifact versioning and distribution

- Published artifacts are **immutable** and content-addressed. A tag MUST resolve to exactly one digest; a digest MUST always resolve to the same bytes.
- Signatures and attestations are attached as OCI referrers and are verifiable independently of any social application.
- A version that has been published MUST NOT be silently replaced. Corrections are new versions.

---

## Compatibility and migration rules

1. **Fail loudly, never silently.** A consumer that cannot understand a manifest MUST reject it with a typed error; it MUST NOT guess or partially apply.
2. **Explicit capability loss.** When an adapter cannot honor a capability, it MUST report the loss. Silent degradation is prohibited.
3. **Migration notes are mandatory** for every incompatible change: what changed, why, what a consumer must do, and a worked example.
4. **Reversibility.** Migrations MUST be reversible where the underlying operation is a managed write; snapshot and rollback apply to migrations as they do to installs.
5. **Deprecation is explicit.** Deprecated fields MUST be marked in the schema and documented, with a removal version stated.

---

## Summary

- Spec version (`apiVersion`) and package version (`metadata.version`) are **independent**.
- Current stage: `thisismyharness.dev/v1alpha1` — **no stability guarantees**.
- Incompatible spec changes bump the version and require an RFC.
- Packages use SemVer.
- **v1.0 is prohibited before the Reference Harnesses.**
