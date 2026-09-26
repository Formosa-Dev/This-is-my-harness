# This is my Harness

**An open standard and toolchain for packaging, distributing, validating and running portable agentic systems.**

> **Define once. Run anywhere.**

This is my Harness is an open project incubated by [Formosa.dev](https://formosa.dev.ar), a technology community from Formosa, Argentina. Its primary purpose is not to create another prompt marketplace or another incompatible agent format. Its purpose is to standardize what a *harness* is, how it is packaged, how its capabilities are declared, how it is distributed, how it is adapted to different runtimes, and how another machine can install and run it safely.

The ambition is global; the origin remains part of the project.

---

## Status: pre-alpha

This repository is at the **specification, scaffold and reference-implementation stage**.

- There is **no stable Harness Spec v1.0**.
- There is **no production CLI** and **no official installer**.
- There is **no compatibility guarantee, no published package, and no signed release channel**.
- Nothing in this repository is a released product.

Early work favors explicit contracts, fixtures, conformance, threat modeling, inspectable behavior and real reference harnesses over premature breadth.

> **Install commands shown anywhere in this project are conceptual** until real package IDs and signed release channels exist. Do not treat any example command as a supported install path.

---

## The rule: no v1.0 before the Reference Harnesses

> **The Harness Spec MUST NOT be declared v1.0 until the Reference Harnesses have been implemented and used in real scenarios and the core can represent them without runtime-specific hacks.**

The standard MUST NOT be declared stable based only on a design document or on branding. The reference harnesses (Web Agent, Developer, Local Hybrid, Multi-Agent, Workflow Automation) exist to *stress the model and break the schema*. If representing a capability requires special-case logic inside the Core or inside a UI, the abstraction — not the harness — is what must change.

This rule is enforced in the contribution flow: every pull request includes a checkbox confirming it does not declare, imply or ship v1.0 stability. See [`CONTRIBUTING.md`](CONTRIBUTING.md) and the [pull request template](.github/pull_request_template.md).

---

## What is a harness?

A **harness** is a portable, declarative package that describes how models, agents, tools, context, interfaces, policies, workflows and runtime services work together.

A harness can be small — instructions plus a few skills — or it can represent a complete agentic environment containing local and remote models, MCP servers, browser interfaces, workflows, decision models, observability and runtime-specific adapters. The standard exists to make both ends reproducible: the author can publish the system as an artifact, and another developer can resolve, validate, inspect, install, run and revert it.

---

## The core problem

Today agentic systems are fragmented across vendor-specific files, local configuration, repositories, model runtimes and emerging protocols.

- Codex has its own instruction, configuration and skill surfaces.
- Claude Code has its own instructions, rules, skills, subagents, hooks, MCP and plugin model.
- OpenCode, Cursor, Gemini CLI and other runtimes expose overlapping but different concepts.
- Modern agentic applications also combine MCP, WebMCP, interactive MCP Apps, agent-to-agent protocols, UI protocols, task systems, local models, hosted models and external services.

This is my Harness does **not** force those primitives into a proprietary replacement. It provides the composition, packaging, installation and conformance layer **above** them.

### Design rule: standardize composition, not every primitive

When a useful open standard already exists, a harness references and composes it instead of inventing an incompatible equivalent. The standard is therefore designed to integrate capabilities such as:

| Area | Adopted standard |
| --- | --- |
| Agent ↔ tool / resource | MCP |
| Tools exposed by live web apps | WebMCP |
| Interactive UI delivered by MCP servers | MCP Apps |
| Agent ↔ application event/state transport | AG-UI |
| Agent-generated declarative UI | A2UI |
| Agent ↔ agent discovery and delegation | A2A |
| Portable task knowledge | Agent Skills / `SKILL.md` |
| Durable asynchronous work | MCP Tasks |
| Deterministic API workflows | Arazzo |
| Observability | OpenTelemetry GenAI conventions |
| Content-addressed artifact distribution | OCI / ORAS |
| Signatures and attestations | Sigstore / Cosign |
| Fast structured decisions | System One decision models |

---

## What this repository standardizes

### Harness Core Specification

Identity and naming · versioning · manifest semantics · package layout · component discovery · requirements · permissions and risk declarations · dependencies · distribution metadata · compatibility and conformance metadata.

### Capability specifications

Capabilities extend the core without bloating it: `models.generative`, `models.system-one`, `interfaces.mcp`, `interfaces.webmcp`, `interfaces.agui`, `interfaces.a2ui`, `interfaces.a2a`, `ui.mcp-apps`, `knowledge.agent-skills`, `workflows.arazzo`, `workflows.mcp-tasks`, `observability.opentelemetry`, `distribution.oci`, `runtime.codex`, `runtime.claude-code`, `runtime.opencode`, and more.

### Runtime Adapter Contract

Adapters translate the canonical harness model into native runtime behavior **without pretending that every runtime supports the same capabilities**. Compatibility loss MUST always be explicit; a conversion MUST never silently discard meaningful behavior.

### Install Protocol

`Canonical ID/URL → Resolver → Manifest → Version → Artifact → Hash/Signature → Capability Resolution → Runtime Adapter → Install Plan → Snapshot → Apply → Verify → Launch`

The command is an implementation of the protocol, not the standard itself.

### Conformance

Compatibility must be testable, not declarative. A project should not be able to claim compatibility by adding a badge. Planned commands (conceptual): `harness validate .`, `harness test --runtime <runtime>`, `harness conformance`.

---

## Roadmap (high level)

> Order matters: the **standard and the toolchain come first**; the social layer is built **on top** and comes last. No version is declared stable until the reference harnesses have broken the schema.

1. **Foundations** — repository scaffold, governance, glossary, ADR/RFC process, versioning policy.
2. **Core Spec v1alpha** — identity, versioning, manifest semantics, package layout, component types, requirements, permissions/risk, dependencies, distribution and compatibility metadata.
3. **Manifest + schema + validator** — `harness.yaml`, JSON Schema, `harness validate`.
4. **Fixtures and examples** — conformant and non-conformant package corpora.
5. **Harness Core planning engine** — resolver, deterministic Install Plan, capability resolution.
6. **Snapshot / Apply / Verify / Revert** — atomic writes, journal, crash recovery, rollback.
7. **First runtime adapter (Codex)** plus adapter conformance.
8. **Canonical identifier + immutable artifact** — OCI/ORAS distribution and signature verification.
9. **One-command bootstrap** on one OS, preserving Pending Intent.
10. **Agent-native install** — machine-readable install guide and JSON CLI states.
11. **Reference Harnesses** — Developer, Web Agent, Local Hybrid, Multi-Agent, Workflow Automation (torture tests for the standard).
12. **More runtime adapters** — Claude Code, OpenCode.
13. **Cross-platform distribution**, code signing and updater.
14. **Conformance suite, threat model and supply-chain controls.**
15. **v1.0 candidate** — only when the launch criteria are met.
16. **Social registry, web and desktop** — downstream consumers of the standard.

---

## Repository map

```
.
├─ spec/                 Harness Core Specification (language-neutral)
│  ├─ core/              identity, versioning, requirements, risk, deps
│  ├─ manifest/          canonical `harness.yaml` semantics
│  ├─ package/           package layout and component types
│  ├─ install-protocol/  resolve → plan → snapshot → apply → verify → revert
│  └─ adapter-contract/  the runtime adapter operations
├─ extensions/           versioned capability extensions
│  ├─ mcp/ webmcp/ mcp-apps/ system-one/ a2a/ ag-ui/ a2ui/
│  └─ agent-skills/ arazzo/ opentelemetry/ oci/
├─ schemas/              JSON Schema — the single source of truth (language-neutral)
├─ packages/             toolchain: core/ (engine), validator/, sdk/, cli/
├─ adapters/             runtime adapters: codex/, claude-code/, opencode/
├─ profiles/             developer, web-agent, local-hybrid, multi-agent, workflow-automation
├─ labs/                 pre-RFC experiments
├─ rfcs/                 requests for comments (see rfcs/0000-template.md)
├─ conformance/          conformance suite and fixtures
├─ examples/             example harnesses
├─ docs/                 ADRs (docs/adr/) and provenance (docs/SOURCES.md)
├─ openspec/             SDD workspace for this repository
├─ CONTRIBUTING.md
├─ SECURITY.md
├─ GOVERNANCE.md
├─ MAINTAINERS.md
├─ CODE_OF_CONDUCT.md
├─ LICENSE
└─ README.md
```

The social / registry / web / desktop layers are **downstream consumers** of the standard and do **not** define its semantics. They are intentionally out of this repository's technical core.

---

## How to contribute

New ideas normally start in **Labs** or an **RFC**, prove themselves in a **reference harness**, and only then move toward a **stable extension**:

```
Labs → RFC → extension candidate → conformance → stable
```

See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full flow, the DCO requirement and the pull request checklist. All participation is covered by our [Code of Conduct](CODE_OF_CONDUCT.md).

Useful contributions include runtime-format research, new capability-extension proposals, reference harnesses, conformance fixtures, portability edge cases, System One / local-model experiments, WebMCP / MCP Apps / AG-UI / A2UI integrations, A2A and task-orchestration experiments, security and supply-chain review, cross-platform installation work, and observability and evals.

---

## Exploratory integrations

> **Status: exploratory — not implemented, not supported, no commitment.** Nothing described in this section exists yet. It is published so the direction is visible and open to discussion.

### ADE integration — ORCA

An **Agent Development Environment (ADE)** such as **ORCA** lets a user choose, from a list of built-in agents (for example Codex, Cursor, Hermes and Claude), which agents to deploy inside its own ecosystem and technology.

The intended integration: once the necessary This is my Harness components are installed, ORCA should be able to **detect** that installation and list the **harnesses a user has loaded** (installed and/or published) as **first-class deployable agents**, alongside ORCA's built-in agents. A published harness should be selectable and deployable from inside an ADE, not only from the CLI, Web or Desktop.

This is a **discovery + deployment** integration and is **not specified yet**. Open questions include: ORCA's extension surface (plugin API, MCP, or a config-based agent registry); how ORCA defines a deployable agent; whether ORCA behaves as a *host/surface* like the Desktop or as a *runtime* like Codex — which decides whether This is my Harness treats it as a discovery client or as one more runtime adapter; and whether "loaded" means published harnesses, locally installed harnesses, or both.

Security invariants are unchanged: exposing harnesses as agents MUST NOT bypass preview, snapshot/rollback, or the execution policy (risk classes A–D, autonomy levels 0–3).

If you are building ORCA or another ADE and want to collaborate on this, open an [RFC](rfcs/0000-template.md).

---

## License

Licensed under the **Apache License 2.0**. See [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE).

Apache-2.0 is the current choice for the open specification and tooling layer: it supports broad commercial and open-source adoption while including an explicit patent grant. Different hosted services or future commercial components may use different licensing where appropriate; that does not require closing the public specification.

---

## North Star

> One harness. Multiple agents. One standard way to package, distribute and run it.
> **Define once. Run anywhere.**

The deeper technical objective is to make complete agentic architectures portable: models, agents, tools, context, interfaces, policies, workflows and services — packaged as a reproducible, inspectable and reversible system.

*Built from Formosa. Open to builders everywhere.*
