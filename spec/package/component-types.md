# Component and Artifact Types

> **Status:** pre-alpha · `apiVersion: thisismyharness.dev/v1alpha1`
> **Normative:** This document is normative. The key words MUST, MUST NOT, SHOULD, SHOULD NOT and MAY are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174).
> **See also:** [glossary](../glossary.md) · [STYLE](../STYLE.md) · [package layout](layout.md) · [manifest](../manifest/README.md) · [risk classes](../core/risk-classes.md)

This document defines the artifact kinds and the component taxonomy. It answers §15, extended by the first-class treatment of models (§7–§8), routers (§9) and services (§13).

Two axes MUST NOT be conflated:

- An **artifact kind** states *what a published artifact is* (a whole harness, a reusable component, or a preset).
- A **component type** states *what a typed element inside a package is* (a skill, model, router, policy, agent, MCP server, workflow or service).

A component is a typed, addressable element with defined semantics. A file is not a component merely because it exists.

---

## 1. Artifact kinds

| Artifact kind (`kind`) | Definition | Extends what | Runnable on its own |
| --- | --- | --- | --- |
| **Full Harness** | A complete composition that declares a runtime requirement and can be resolved, planned, installed, verified and reverted. | Components, Presets, and base Full Harnesses. | Yes, given a compatible runtime. |
| **Component** | A reusable typed element intended to be composed into a harness or into another component. | Other Components. | No; it has no standalone runtime requirement. |
| **Preset** | A curated selection and set of defaults over components. | Components and Presets. | Only when it fully satisfies a Full Harness's requirements. |

Rules:

1. A Full Harness MUST declare a runtime requirement (see [`../core/requirements.md`](../core/requirements.md)).
2. A Component MUST NOT extend a Full Harness. A component composes other components; it does not inherit a runnable environment.
3. A Preset MUST NOT introduce new semantics. It MAY select, default and parameterize components, and MUST NOT define component behavior that is not defined by a component type.
4. A Preset that is installable MUST declare or inherit a complete set of requirements; otherwise a resolver MUST reject it as non-installable.
5. The `kind` used in the manifest is defined in [`../manifest/README.md`](../manifest/README.md), §2.2.

---

## 2. Component taxonomy

The recognized component types are the following. This list is exhaustive for the current `apiVersion`; a new type MUST be introduced through the extension process (see [`../core/core-vs-extension.md`](../core/core-vs-extension.md)), not by ad hoc convention.

| Component type | Purpose | Conventional directory | Typical risk class |
| --- | --- | --- | --- |
| **Skill** | Portable task knowledge. | `skills/` | A (Passive) |
| **Model** | A cognitive capability with an input contract, output contract, capabilities and resources. | `models/` | B (Tooling) when it runs as a service; A otherwise |
| **Router** | A decision-model specialization that routes, scores, gates or selects. | `models/` (explicit declaration) | B (Tooling) |
| **Policy** | A declarative constraint that governs behavior, including passive rules. | `policies/`, `rules/` | A–C depending on what it can cause |
| **Agent** | A delegated actor with declared capabilities and, where applicable, delegation paths. | `agents/` | B–C |
| **MCP** | A declaration of an MCP server and the tools/resources it exposes. | `mcp/`, `tools/` | B (Tooling) or higher |
| **Workflow** | A declarative execution graph or sequence. | `workflows/` | B–C |
| **Service** | A managed process with a lifecycle. | `services/` | B–D |

Notes:

1. The "typical risk class" column is informative; the **effective** risk class of a component MUST be the maximum class its declared operations require (see [`../core/risk-classes.md`](../core/risk-classes.md)).
2. A component MUST declare its type. A component whose type cannot be determined MUST cause rejection.
3. A component MUST declare every capability it needs. Undeclared capability use is a conformance failure.

---

## 3. Skill

**Definition.** Portable task knowledge, adopting the existing portable skill standard rather than defining a competing format.

1. A Skill component MUST be distributed using the adopted standard's content and MUST NOT define a new skill format.
2. A Skill is passive by default (risk class A). A skill that causes a write or an execution is not passive and MUST declare the corresponding risk class.
3. Discovery of a Skill follows the conventional rule in [`layout.md`](layout.md), §3.

---

## 4. Model

**Definition.** A cognitive capability executed within or around the system. A model is not a runtime. A harness MAY combine multiple models and one runtime.

A Model component MUST be more than a name. It MUST declare:

1. An **input contract** — the shape and semantics of its input.
2. An **output contract** — the shape and semantics of its output, including typed outcomes where applicable.
3. **Capabilities** — the capability contracts it satisfies (for example a generative capability or a structured decision capability).
4. A **resource contract** — the resources it needs (see [`../core/requirements.md`](../core/requirements.md)).
5. **Execution location** — local or remote.
6. **Lifecycle** — how it is started, health-checked and stopped, if it runs as a service.
7. **Fallback** — what happens when the model or its host is unavailable.
8. **Version** and **license**.
9. **Artifact source** — where the artifact or provider is obtained.

Rules:

1. A Model component MUST declare capabilities as **capability contracts**, not as hardcoded brands. A concrete implementation MAY be named only as a preferred or alternative implementation of a capability, and MUST NOT be required for conformance.
2. When more than one implementation satisfies the same capability contract, the resolver MUST select one based on declared requirements and constraints, and MUST report which implementation it selected. The selection MUST be explainable.

---

## 5. Router

**Definition.** A decision-model specialization that performs routing, scoring, confidence gating, tool selection or escalation decisions with typed outputs.

1. A Router MUST declare typed outputs (for example a selected route, a score, or an abstention/no-op outcome).
2. A Router MUST declare its thresholds and its fallback behavior, including which decisions escalate to a generative capability.
3. A Router is a Model specialization and MUST satisfy the Model declaration requirements in §4.
4. Because there is no dedicated directory for routers, a Router MUST be declared explicitly (see [`layout.md`](layout.md), §4).

---

## 6. Policy

**Definition.** A declarative constraint that governs behavior.

1. A Policy MUST declare the scope in which it applies (project or user) and the risk class of the operations it can permit, deny or require confirmation for.
2. A passive rule (for example an instruction-level constraint) is a Policy with risk class A.
3. A Policy MUST NOT itself execute code. A policy that requires code to be enforced is a Policy plus an executable component, and the executable component MUST be declared separately with its own risk class.
4. A Policy MAY be expressed in an adopted policy language; the choice of policy language is an open question (decision record §58) and MUST NOT be fixed by the Core at `v1alpha1`.

---

## 7. Agent

**Definition.** A delegated actor with declared capabilities and, where applicable, delegation paths and identity.

1. An Agent MUST declare its capabilities and its identity within the harness.
2. Where agents delegate to one another, the delegation paths MUST be declared, and delegation MUST be expressible without runtime-specific logic in the Core (see decision record §30).
3. An Agent MUST NOT be used to smuggle runtime-specific behavior into a portable package. Runtime-specific behavior belongs in an adapter.

---

## 8. MCP

**Definition.** A declaration of an MCP server and the tools and resources it exposes, or a tool declaration composed through the adopted tool-interop standard.

1. An MCP component MUST adopt the existing MCP semantics rather than defining a competing tool format.
2. An MCP component MUST declare the server's transport and the capabilities it provides.
3. An MCP component MUST declare its risk class. A server that can execute arbitrary commands MUST NOT be presented as Passive.
4. A local MCP service MUST bind to localhost by default. Broader exposure MUST be explicit and approved (see [`../core/permissions.md`](../core/permissions.md)).

---

## 9. Workflow

**Definition.** A declarative execution graph or sequence, optionally combining deterministic steps with agentic gates and human approval.

1. A Workflow MUST declare its inputs, its steps or nodes, its transitions and its terminal outcomes.
2. Where an adopted standard already describes the workflow (for example a deterministic API workflow standard), the Workflow MUST compose that standard rather than define a competing DSL.
3. The Core at `v1alpha1` is not required to provide a graphical workflow language, but the data model MUST NOT block composed workflows.
4. A Workflow that can execute code MUST declare the corresponding risk class.

---

## 10. Service

**Definition.** A capability that runs as a managed process rather than as a static file, with a managed lifecycle.

1. A Service MUST declare a lifecycle covering install, start, health-check, stop, update and remove.
2. A Service MUST declare its network binding. A local service MUST bind to localhost by default; exposure beyond localhost MUST be explicit and approved.
3. A Service MUST declare its resource requirements and its health-check.
4. A Service MUST declare its risk class. A service that runs with elevated privileges is risk class D.

---

## 11. Composition rules

1. Components MAY be composed by extends (see [`../core/dependencies.md`](../core/dependencies.md)).
2. The effective risk class of a composition is the maximum risk class over its parts; combining classes is monotonic.
3. Conflicting declarations of the same component identity MUST be detected before Apply and MUST NOT be silently resolved.
4. A component type that is not recognized MUST cause rejection unless an extension defining it is present and compatible.
