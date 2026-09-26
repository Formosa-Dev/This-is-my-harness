# Package Layout and Component Discovery

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [manifest](../manifest/README.md) · [component types](component-types.md) · [permissions](../core/permissions.md) · [risk classes](../core/risk-classes.md)

This document defines the package layout and how components are discovered. It answers the package-structure and component-discovery parts of §2.1 and §2.3. It defines **semantics**, not the concrete descriptor schema.

---

## 1. Root layout

A harness package is a directory tree. Its root contains:

| Entry | Required | Semantics |
| --- | --- | --- |
| `harness.yaml` | REQUIRED | The canonical manifest. Exactly one. See [`../manifest/README.md`](../manifest/README.md). |
| `README.md` | OPTIONAL (SHOULD) | Human-facing documentation. Carries no machine semantics. |
| Recognized component directories | OPTIONAL | The directories enumerated in §3. |
| Other files and directories | OPTIONAL | Package content that is not a typed component. |

1. A package MUST contain exactly one `harness.yaml` at its root.
2. A package MUST NOT be required to use every recognized directory. The only required entry is the manifest.
3. A file or directory does **not** become a component merely by existing. A component has a declared type and meaning (see [`glossary.md`](../glossary.md), "Component", and [`component-types.md`](component-types.md)).

---

## 2. Discovery rules

Components are discovered by two mechanisms, applied together.

1. **Explicit declaration.** An entry in the manifest's `spec.components` is authoritative. When a component is declared explicitly, the explicit declaration governs and conventional discovery MUST NOT contradict it.
2. **Conventional discovery.** A component not explicitly declared MAY be discovered by convention inside a recognized directory, as specified per directory in §3.
3. Where explicit and conventional discovery produce the same component identity with different content, the package MUST be rejected as ambiguous. The resolver MUST NOT silently prefer one.

General discovery constraints:

1. Component paths MUST be relative to the package root and MUST NOT contain `..` or otherwise escape the package root. Path traversal MUST be blocked (see [`glossary.md`](../glossary.md), "Apply / Revert", and decision record §50).
2. Symbolic links inside a package MUST NOT resolve outside the package root. Symlink escape MUST be blocked.
3. Package paths SHOULD be lowercase ASCII so that a package behaves identically on case-sensitive and case-insensitive filesystems. If two discovered entries collide under case-insensitive comparison, the package MUST be rejected.
4. Entries whose name begins with `.` MUST be ignored by conventional discovery unless explicitly declared.
5. Discovery MUST be deterministic: the same package tree MUST produce the same ordered set of components. Consumers MUST NOT depend on filesystem iteration order.
6. A `README.md` inside a recognized directory is human documentation and MUST NOT be treated as a component.

---

## 3. Recognized directories

Each row defines the normative semantics of one recognized directory, the component type it conventionally holds, and how entries are discovered.

| Directory | Holds | Component type | Required | Conventional discovery |
| --- | --- | --- | --- | --- |
| `instructions/` | Passive instruction and context content. | Instruction (recognized content, not a typed component) | OPTIONAL | Each entry is one instruction unit, projected by the adapter. No execution. |
| `skills/` | Portable task knowledge. | Skill | OPTIONAL | Each child directory is one skill; content follows the adopted portable skill standard rather than a competing format. |
| `agents/` | Agent definitions. | Agent | OPTIONAL | Each entry is one agent descriptor. |
| `rules/` | Passive constraints. | Policy | OPTIONAL | Each entry is one passive policy (risk class A). |
| `tools/` | Tool declarations. | composed through the MCP capability | OPTIONAL | Each entry is one tool descriptor. A tool is composed through the adopted tool-interop standard; see §4. |
| `mcp/` | MCP server declarations. | MCP | OPTIONAL | Each entry declares one MCP server (risk class B or higher). |
| `models/` | Model and router declarations. | Model, Router | OPTIONAL | Each entry is one model descriptor; a Router is a decision-model specialization (see §4). |
| `services/` | Managed service declarations. | Service | OPTIONAL | Each entry declares one service with a lifecycle. |
| `workflows/` | Workflow declarations. | Workflow | OPTIONAL | Each entry is one workflow descriptor. The model MUST NOT force a proprietary DSL where an adopted standard resolves it. |
| `policies/` | Policies. | Policy | OPTIONAL | Each entry is one policy descriptor; risk class depends on what the policy can cause. |
| `memory/` | Memory configuration. | Memory (recognized content) | OPTIONAL | Passive backend/configuration declarations. No execution by itself. |
| `hooks/` | Executable extension points. | executable content | OPTIONAL | Each entry MUST be attached to a typed component and MUST declare risk class C or higher. |
| `scripts/` | Executable scripts. | executable content | OPTIONAL | Each entry MUST be attached to a typed component and MUST declare risk class C or higher. |
| `adapters/` | Adapter declarations shipped by the package. | Adapter declaration | OPTIONAL | Each entry declares an adapter; the declaration is data consumed by the Core and MUST NOT execute by itself. |

---

## 4. Documented asymmetries

The following asymmetries exist between the package directory list and the canonical component taxonomy. They are recorded so that the schema phase resolves them deliberately rather than by accident.

1. **`tools/` versus the component taxonomy.** The canonical component taxonomy in §15 of the decision record lists no `Tool` type. Therefore `tools/` does not introduce a competing component type; its entries are composed through the adopted tool-interop standard and are typed as part of that capability. A package MUST NOT treat `tools/` as a new format.
2. **`Router` versus a dedicated directory.** The taxonomy lists `Router` as a component type, but there is no `routers/` directory in the recognized layout. A Router component MUST therefore be declared explicitly in `spec.components`, conventionally with its descriptor under `models/`, because a router is a decision-model specialization.
3. **`instructions/` and `memory/` are recognized content, not typed components.** They affect what is projected into the runtime but are not addressable, composable elements in the sense of [`component-types.md`](component-types.md). They MAY still carry a risk class where they can cause a write.
4. **`hooks/` and `scripts/` are executable content, not a component type.** They MUST be attached to a typed component and MUST NOT be executed during preview. Their presence raises the effective risk class of the package (see [`../core/risk-classes.md`](../core/risk-classes.md)).

These asymmetries are open for correction through an RFC and MUST NOT be resolved by inventing a new format at implementation time.

---

## 5. Effective package properties

1. The **effective risk class** of a package is the maximum risk class over all its components and permissions. Risk classes are monotonic under composition; combining a lower-class and a higher-class element yields the higher class.
2. The **effective permissions** of a package are the union of the declared permissions of its components, subject to the conflict rules in [`../core/dependencies.md`](../core/dependencies.md).
3. A package that declares only `instructions/`, and declares no other component, is passive by construction.
4. A package MUST declare in `permissions` every component whose risk class exceeds Passive.

---

## 6. Summary

- One manifest at the root; every component directory is optional.
- Discovery is explicit-then-conventional; ambiguity is rejected, never silently resolved.
- Path traversal and symlink escape are blocked by construction.
- Discovery is deterministic and case-safe.
- Directories hold typed components or recognized content; a file is not a component by mere existence.
- Known asymmetries are documented and deferred to an RFC, not patched ad hoc.
