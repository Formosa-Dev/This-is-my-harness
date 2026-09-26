# THIS IS MY HARNESS

## README source document · v0.2

An open standard and toolchain for packaging, distributing, validating and running portable agentic systems.  
Define once. Run anywhere.  
This is my Harness is an open project incubated by Formosa.dev. Its primary purpose is not to create another prompt marketplace or another incompatible agent format. Its purpose is to standardize what a harness is, how it is packaged, how its capabilities are declared, how it is distributed, how it is adapted to different runtimes, and how another machine can install and run it safely.

# What is a harness?

A harness is a portable, declarative package that describes how models, agents, tools, context, interfaces, policies, workflows and runtime services work together.  
A harness can be small — for example, instructions plus a few skills — or it can represent a complete agentic environment containing local and remote models, MCP servers, browser interfaces, workflows, decision models, observability and runtime-specific adapters.  
The standard should make both ends reproducible: the author can publish the system as an artifact, and another developer can resolve, validate, inspect, install, run and revert it.

# The core problem

Today agentic systems are fragmented across vendor-specific files, local configuration, repositories, model runtimes and emerging protocols.

* Codex has its own instruction, configuration and skill surfaces.  
* Claude Code has its own instructions, rules, skills, subagents, hooks, MCP and plugin model.  
* OpenCode, Cursor, Gemini CLI and other runtimes expose overlapping but different concepts.  
* Modern agentic applications also combine MCP, WebMCP, interactive MCP Apps, agent-to-agent protocols, UI protocols, task systems, local models, hosted models and external services.

This is my Harness should not force all of those primitives into a proprietary replacement. It should provide the composition, packaging, installation and conformance layer above them.

# Design rule: standardize composition, not every primitive

When a useful open standard already exists, a harness should reference and compose it instead of inventing an incompatible equivalent.  
The standard is therefore designed to integrate capabilities such as:

* MCP for agent-to-tool and agent-to-resource integration.  
* WebMCP for tools exposed directly by live web applications.  
* MCP Apps for interactive UI delivered by MCP servers.  
* AG-UI for agent ↔ application interaction and event/state transport.  
* A2UI for declarative agent-generated user interfaces.  
* A2A for agent-to-agent discovery, delegation and collaboration.  
* Agent Skills / SKILL.md for portable task knowledge.  
* MCP Tasks for durable asynchronous work.  
* Arazzo for deterministic API workflows.  
* OpenTelemetry GenAI conventions for observability where applicable.  
* OCI/ORAS for content-addressed artifact distribution.  
* Sigstore/Cosign for signatures and attestations.  
* System One decision models for fast structured decisions alongside generative LLMs.

# What this repository standardizes

## Harness Core Specification

* Identity and naming.  
* Versioning.  
* Manifest semantics.  
* Package layout.  
* Component discovery.  
* Requirements.  
* Permissions and risk declarations.  
* Dependencies.  
* Distribution metadata.  
* Compatibility and conformance metadata.

## Capability Specifications

Capabilities extend the core without bloating it. Examples:

* models.generative  
* models.system-one  
* interfaces.mcp  
* interfaces.webmcp  
* interfaces.a2a  
* interfaces.agui  
* interfaces.a2ui  
* ui.mcp-apps  
* knowledge.agent-skills  
* workflows.arazzo  
* workflows.mcp-tasks  
* observability.opentelemetry  
* distribution.oci  
* runtime.codex  
* runtime.claude-code  
* runtime.opencode

## Runtime Adapter Contract

Adapters translate the canonical harness model into native runtime behavior without pretending that every runtime supports the same capabilities.

* detect()  
* inspectExisting()  
* capabilities()  
* planApply()  
* apply()  
* verify()  
* launch()  
* revert()

Compatibility loss must always be explicit. A conversion must never silently discard meaningful behavior.

## Install Protocol

The install protocol defines how a harness moves from a canonical identifier or URL to a verified local installation:  
Resolve → Manifest → Version → Artifact → Integrity/Signature → Capability Resolution → Runtime Adapter → Install Plan → Snapshot → Apply → Verify → Launch.  
The command is an implementation of the standard, not the standard itself.

## Conformance

A project should not be able to claim compatibility only by adding a badge. Compatibility should be testable.  
Planned commands:  
harness validate .  
harness test \--runtime codex  
harness conformance  
Conformance should verify schema validity, package structure, permission declarations, adapter behavior, fixtures and capability-specific requirements.

# Canonical manifest

A harness should have one canonical machine-readable entrypoint. Working name: harness.yaml.  
Conceptual example:  
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
The concrete schema is not stable yet. The v1alpha phase exists specifically so real reference harnesses can break the design before v1.0 is declared.

# Package model

A harness is not required to use every directory, but the standard should define the semantics of recognized component types.  
Conceptual structure:  
my-harness/  
├─ harness.yaml  
├─ README.md  
├─ instructions/  
├─ skills/  
├─ agents/  
├─ rules/  
├─ tools/  
├─ mcp/  
├─ models/  
├─ services/  
├─ workflows/  
├─ policies/  
├─ memory/  
├─ hooks/  
├─ scripts/  
└─ adapters/  
A package can remain simple. The standard must not require complexity where none is needed.

# Models are first-class components

A harness may include or depend on more than one model. Runtime and model are different concepts.  
Runtime examples: Codex, Claude Code, OpenCode, Cursor.  
Model capability examples: generative, System One decision, embedding, reranker, classifier, vision, speech, evaluator.  
The standard should prefer capability requirements over hard-coded brands whenever possible.  
Example concept:  
requires: typed-decision-model  
preferred implementation: local Laya-compatible provider  
alternative implementation: hosted Jev-compatible provider  
This allows a harness to preserve its architecture even when the concrete model implementation changes.

# System One \+ frontier LLM

One important reference architecture uses a fast structured decision model together with a generative frontier model.  
The System One component can handle repetitive decisions such as routing, scoring, confidence gates, tool selection, retrieval prioritization, risk checks or escalation decisions. The generative LLM remains responsible for tasks that benefit from deeper reasoning, coding or language generation.  
A harness should be able to describe:

* the decision interface and typed outputs;  
* the preferred local or remote implementation;  
* hardware/resource requirements;  
* confidence thresholds;  
* fallback behavior;  
* which decisions escalate to the frontier LLM.

The standard should describe capabilities and contracts, not assume one specific vendor or model forever.

# Local execution and hardware resolution

A complete harness may contain local models and managed services. Harness Core should be able to evaluate the target machine before installation.

* Operating system and architecture.  
* CPU.  
* RAM.  
* GPU and available VRAM where relevant.  
* Required runtimes and package managers.  
* Model backends such as ONNX Runtime or another supported engine.  
* Disk/download requirements.

The resolver may then select a compatible implementation: local on a capable workstation, hosted on a smaller machine, or another implementation satisfying the same capability contract.

# Services are first-class components

Some capabilities run as managed services rather than static files.  
Examples include local model servers, MCP servers, vector databases, browser services and embedding services.  
Harness Core should be able to manage a declared service lifecycle: install, start, health-check, stop, update and remove.  
Local network services should bind to localhost by default unless the manifest explicitly declares and the user approves broader exposure.

# Composable harnesses

The long-term standard should support composition without forcing it into the first stable core.  
A harness may eventually extend or depend on reusable components such as:

* a base coding environment;  
* a security policy layer;  
* a local decision router;  
* a frontend skill collection;  
* a shared MCP service.

Dependency resolution must detect cycles, version conflicts, incompatible permissions and capability conflicts before Apply.

# One command from zero to running

The user-facing toolchain is being designed around one stable logical interface:  
harness use \<canonical-url-or-owner/slug\>  
The target experience is not merely “install our CLI”. It is “move from a published harness to a running environment with minimal manual configuration”.  
A system-specific bootstrap for Windows, macOS and Linux should be able to install/verify Harness, preserve the requested action, resolve the harness, detect the project and available runtime, generate an Install Plan, create a restore point, apply, verify and optionally launch the runtime.  
Until real package IDs and signed release channels exist, installation commands in documentation must remain clearly marked as conceptual.

# Built for humans and AI agents

The same canonical harness should be consumable from:

* Web: discovery and inspection.  
* Desktop: project/runtime selection, diff, Apply, switching and rollback.  
* CLI: reproducible installation and automation.  
* AI agents: resolve the official install guide and delegate mutation to Harness Core.

An AI agent should not manually reinvent where every file belongs when Harness Core is available.  
Agent-oriented CLI output should have a machine-readable JSON mode and should stop with a typed needs\_confirmation state when a requested operation exceeds the user's policy.

# Zero-friction is a product requirement, not permission to hide risk

The target for a passive, compatible harness is close to: paste → enter → run.  
But autonomy is risk-classed.

* Metadata resolution and Install Plan generation can be automatic.  
* Passive project-scope changes can use a simple approval policy.  
* Trusted low-risk harnesses may receive reduced friction if the user opts in.  
* Hooks, scripts, broad MCP access, user-scope changes, runtime installation and privileged operations require stronger approval.  
* Admin/sudo operations remain subject to operating-system confirmation.

Every managed write must remain inspectable and reversible.

# Install Plan, snapshot and rollback

Before mutation, Harness Core generates a deterministic Install Plan describing the runtime, target project, scope, files, conflicts, adaptations, unsupported capabilities, services, executable components, required environment-variable names, risk level and verification steps.  
Before Apply, Harness Core creates a snapshot of managed state.  
Rollback is not an emergency feature. It is part of the normal product contract.

# Distribution

The project should avoid inventing a bespoke binary transport if a mature content-addressed ecosystem already exists.  
OCI registries are a strong candidate for distributing immutable harness artifacts through OCI/ORAS semantics, using a dedicated harness artifact media type, content digests and referrers for signatures/attestations.  
The public registry can provide discovery and metadata while the actual versioned package is distributed as an immutable artifact.  
Signatures and attestations should be verifiable independently of the social application.

# Reference Harnesses before v1.0

The standard should not be declared stable based only on a design document. Before v1.0, we should build and use intentionally different reference harnesses that stress the model.

## Web Agent Harness

Tests WebMCP \+ MCP \+ MCP Apps \+ agent UI protocols \+ browser fallback \+ optional System One decision components.

## Developer Harness

Tests Codex/Claude Code/OpenCode adapters, Agent Skills, repo retrieval, System One routing/scoring, policy gates, tests and development workflows.

## Local Hybrid Harness

Tests frontier LLM \+ local decision model \+ hardware detection \+ local/remote capability substitution.

## Multi-Agent Harness

Tests A2A, MCP, durable tasks, delegation and approvals.

## Workflow Automation Harness

Tests OpenAPI/Arazzo workflows, MCP, Tasks, decision gates and observability.  
These are conformance torture tests, not marketing demos only. If the core model requires runtime-specific hacks to represent them, the specification must change before v1.0.

# Profiles are specializations, not separate standards

A profile defines a coherent expected subset of capabilities for a use case.

* developer  
* web-agent  
* local-hybrid  
* multi-agent  
* workflow-automation

Profiles let us specialize UX, validators and reference implementations while keeping one underlying Harness Standard.

# Labs → extension → stable

The project should provide an explicit path for frontier ideas:  
labs → reference harness → real-world use → extension candidate → conformance tests → stable extension.  
This keeps This is my Harness innovative without turning every new library or social-media trend into permanent core specification.

# Social registry is built on top of the standard

Public profiles, search, votes, comments, saves, follows, collections, feeds and Verified Use remain important product layers, but they are downstream consumers of the Harness Standard.  
The technical repository should make the standard and toolchain understandable even if the social application did not exist.  
The registry should prefer conformant, versioned artifacts rather than arbitrary uploads.

# Suggested repository structure

.  
├─ spec/  
│  ├─ core/  
│  ├─ install-protocol/  
│  └─ adapter-contract/  
├─ extensions/  
│  ├─ mcp/  
│  ├─ webmcp/  
│  ├─ mcp-apps/  
│  ├─ system-one/  
│  ├─ a2a/  
│  ├─ ag-ui/  
│  ├─ a2ui/  
│  ├─ agent-skills/  
│  ├─ arazzo/  
│  └─ opentelemetry/  
├─ schemas/  
├─ packages/  
│  ├─ core/  
│  ├─ validator/  
│  ├─ sdk/  
│  └─ cli/  
├─ adapters/  
│  ├─ codex/  
│  ├─ claude-code/  
│  └─ opencode/  
├─ profiles/  
├─ labs/  
├─ conformance/  
├─ examples/  
├─ rfcs/  
├─ CONTRIBUTING.md  
├─ SECURITY.md  
├─ LICENSE  
└─ README.md

# Initial implementation priority

The first milestone is not the social feed. It is proving that a standardized harness can be packaged and reproduced.

* 1\. Core vocabulary and v1alpha manifest.  
* 2\. JSON Schema \+ validator.  
* 3\. Package structure and fixtures.  
* 4\. Harness Core planning engine.  
* 5\. Codex adapter.  
* 6\. Snapshot / Apply / Verify / Revert.  
* 7\. Canonical identifier and immutable artifact.  
* 8\. One-command bootstrap on one operating system.  
* 9\. AI-agent install guide and JSON CLI output.  
* 10\. Developer Reference Harness.  
* 11\. Web Agent Reference Harness.  
* 12\. Claude Code and OpenCode adapters.  
* 13\. Cross-platform distribution.  
* 14\. Remaining reference harnesses.  
* 15\. Conformance suite.  
* 16\. Only then: v1.0 candidate.

# License

Apache License 2.0 is the current project choice for the open specification/tooling layer. It supports broad commercial and open-source adoption while including an explicit patent license.  
Different hosted services or future commercial components may use different licensing where appropriate; that does not require closing the public specification.

# Project status

Pre-alpha / specification and reference-implementation stage.  
There is no stable Harness Spec v1.0, production CLI, compatibility guarantee or official installer yet.  
Early work should favor explicit contracts, fixtures, conformance, threat modeling, inspectable code, reversible behavior and real reference harnesses over premature breadth.

# Contributing

The project is intended to be built in public and should function as a technical forum for improving portable agentic systems.  
Useful contributions include:

* runtime-format research;  
* new capability-extension proposals;  
* reference harnesses;  
* conformance fixtures;  
* portability edge cases;  
* System One/local-model experiments;  
* WebMCP/MCP Apps/AG-UI/A2UI integrations;  
* A2A and task orchestration experiments;  
* security and supply-chain review;  
* cross-platform installation work;  
* observability and evals.

New ideas should normally start in Labs or an RFC, prove themselves in a reference harness, and only then move toward a stable extension.

# Origin

This project is incubated by Formosa.dev, a technology community from Formosa, Argentina.  
Built from Formosa. Open to builders everywhere.  
The ambition is global, but the origin remains part of the project.

# North Star

One harness. Multiple agents. One standard way to package, distribute and run it.  
Define once. Run anywhere.  
The deeper technical objective is to make complete agentic architectures portable: models, agents, tools, context, interfaces, policies, workflows and services — packaged as a reproducible, inspectable and reversible system.

