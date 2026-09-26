# Glossary — Controlled Vocabulary

This document is the **normative** controlled vocabulary for This is my Harness. Every specification, schema, RFC and implementation **MUST** use these terms with the meanings defined here. Where a term is capitalized in other documents (for example, *Harness*), it refers to this glossary.

Each entry gives a normative definition followed by a **NOT** contrast: the common misuse to avoid.

> Status: pre-alpha, targeting `apiVersion: thisismyharness.dev/v1alpha1`. Definitions will evolve until the v1.0 gate is met.

---

## Harness

**Definition.** A portable, declarative package that describes how models, agents, tools, context, interfaces, policies, workflows and runtime services work together, together with the metadata that lets another machine resolve, validate, inspect, install, run and revert it. A harness has exactly one canonical machine-readable manifest.

**NOT.** A prompt, a single configuration file, a runtime, a model, a repository, or a marketplace listing. A harness *describes a composition*; it is not any one of its parts, and it is not a vendor-specific project format.

---

## Runtime

**Definition.** The environment or agent in which the user actually works and where a harness is materialized and executed. Examples: Codex, Claude Code, OpenCode, Cursor, Gemini CLI. A runtime is targeted by an adapter and consumes a runtime-specific projection of the canonical model.

**NOT.** A model. A runtime is the host and its native surfaces; it is not a cognitive capability. Do not use "runtime" to mean a model provider, and do not hard-code any single runtime as the standard.

---

## Model

**Definition.** A cognitive capability executed within or around the system. Examples: generative model, System One decision model, embedding model, reranker, classifier, vision model, speech model, reward/evaluator model. A model component declares an input contract, an output contract, capabilities, resource requirements, execution location, lifecycle, fallback, version and license.

**NOT.** A runtime. A harness may combine multiple models and one runtime. Also not a mandatory brand: the specification prefers capability requirements over hard-coded vendor names.

---

## Adapter

**Definition.** A runtime-specific implementation of the Runtime Adapter Contract that translates the canonical harness model into native runtime behavior — and back. An adapter declares its capabilities and reports capability loss explicitly.

**NOT.** A model, a runtime, or a place to hide standard semantics. Provider- and runtime-specific logic belongs in the adapter, never in the Core or in a UI. An adapter MUST NOT silently discard meaningful behavior.

---

## Capability

**Definition.** A named, versioned unit of functionality that a harness may require or provide, expressed as a contract independent of any concrete vendor implementation (for example `models.system-one`, `interfaces.mcp`, `workflows.arazzo`). Capabilities may live in the Core or in an extension.

**NOT.** A brand, a product feature, or a badge. `models.system-one` is a capability; "Laya" is one possible implementation of it. A capability is not proof that anything was verified — that is conformance.

---

## Extension

**Definition.** An independently versioned capability specification that adds functionality to the Core without bloating it. Extensions carry their own `apiVersion`, compatibility rules and conformance requirements.

**NOT.** A second standard, a fork, or an excuse to promote every ecosystem novelty into the Core. An extension MUST compose an existing open standard where one exists rather than inventing a competing format.

---

## Profile

**Definition.** A coherent, expected subset of capabilities for a use case that specializes UX, validators and defaults **over the same Core**. The initial profiles are `developer`, `web-agent`, `local-hybrid`, `multi-agent` and `workflow-automation`.

**NOT.** A separate standard, a fork of the specification, or a marketplace category. Profiles specialize; they do not fragment.

---

## Component

**Definition.** A typed, addressable element of a harness package, with defined semantics — for example a skill, model, router, policy, agent, MCP server or workflow. The standard defines the recognized component types and their semantics without requiring any harness to use all of them.

**NOT.** An arbitrary file, a folder, or a dependency. A component has a declared type and meaning; a file placed in a package does not become a component merely by existing.

---

## Manifest

**Definition.** The single canonical machine-readable entrypoint of a harness (working name `harness.yaml`). It declares `apiVersion`, `kind`, `metadata` and `spec` — including profile, components, requirements, permissions, distribution and compatibility. It describes the composition of the system; it does not replace a runtime's native files.

**NOT.** A build file, a lockfile, an arbitrary config, or a runtime's own configuration. There MUST be exactly one canonical manifest, and it is not a place to encode vendor-specific behavior.

---

## Install Protocol

**Definition.** The normative pipeline that moves a harness from a canonical identifier or URL to a verified local installation:

`Canonical ID/URL → Resolver → Manifest → Version → Artifact → Hash/Signature → Capability Resolution → Runtime Adapter → Install Plan → Snapshot → Apply → Verify → Launch`

**NOT.** A specific command. `harness use` is *one implementation* of the protocol; the protocol is the standard. Do not confuse the CLI surface with the protocol's semantics.

---

## Install Plan

**Definition.** A deterministic, serializable intermediate contract, produced before any mutation, describing: runtime and runtime version, project, scope, harness and version, files to create/modify, conflicts, adaptations, unsupported capabilities, MCP, hooks/scripts, services, required environment-variable names, risk, snapshot and verification steps. It is the shared contract between web, desktop, CLI and agents.

**NOT.** The mutation itself, a diff of uncertain provenance, or a marketing summary. An Install Plan MUST be exact and MUST be generated before Apply. It MUST NOT contain or require executing third-party code.

---

## Snapshot

**Definition.** A recorded, restorable capture of managed state taken **before** a mutation is applied, so that the operation can be reverted losslessly. Snapshotting is part of the normal contract, not an emergency feature.

**NOT.** A backup of the whole machine, a git commit, or an optional nicety. A snapshot covers exactly the managed state the operation will touch, and rollback MUST be possible without damaging unmanaged files.

---

## Apply / Revert

**Definition.** **Apply** is the controlled materialization of an Install Plan: scoped writes performed against a prior snapshot, with atomic writes, a project lock, an operation journal and crash recovery. **Revert** restores the state captured in a snapshot, returning managed state to its prior condition.

**NOT.** Silent or best-effort file writes. Apply MUST NOT overwrite conflicts silently, MUST NOT delete unmanaged files, and MUST be crash-recoverable. Revert is a normal operation, not a destructive escape hatch.

---

## Scope

**Definition.** The reach of a mutation. **Project scope** limits changes to the target project directory and is the **default**. **User scope** touches the user's global environment and is a higher-risk operation.

**NOT.** A permission level that can be assumed. **Project scope is the default; user scope requires an exact preview of affected paths plus an explicit warning and approval.** Do not treat both scopes as interchangeable.

---

## Risk class

**Definition.** A classification that governs UX and autonomy for a harness or operation:

- **A — Passive:** instructions/configuration with no execution.
- **B — Tooling:** MCP servers, plugins, integrations.
- **C — Executable:** hooks, scripts, commands.
- **D — Privileged:** user scope, admin/sudo, secrets, broad system access.

**NOT.** A measure of trustworthiness or a marketing label. A risk class states what an artifact *can do*, not that it is safe. Higher classes require stronger approval; there MUST be no absolute `--yes` that bypasses Elevated operations.

---

## Autonomy level

**Definition.** The degree of friction applied to an operation, from 0 to 3:

- **0 — Preview:** resolve metadata, download the manifest, compute the plan, analyze the diff. No mutation.
- **1 — Safe Apply:** passive changes within project scope with simple confirmation/policy.
- **2 — Trusted Harness:** reduced friction, opt-in, for a previously authorized publisher/harness on low-risk updates.
- **3 — Elevated:** scripts, hooks, user scope, runtime installation, admin/sudo or sensitive access require explicit confirmation. The operating system always remains the final authority.

**NOT.** A permission the user can grant blindly, or a way to skip review. Autonomy is granted by policy, is risk-classed, and MUST remain inspectable and reversible at every level.

---

## Canonical identifier

**Definition.** A stable, globally unique identifier for a harness — and, by extension, for a specific resolved version — usable by both humans and machines. A conceptual shape is `https://<host>/h/<owner>/<slug>`, resolving to a manifest, a version and an immutable artifact.

**NOT.** A local path, a transient URL, an unversioned nickname, or a bare name. The canonical identifier MUST resolve reproducibly to a specific artifact by digest; different versions MUST NOT share one identifier without a version qualifier.

---

## Conformance

**Definition.** The verifiable property that a harness, extension or adapter satisfies the specification: schema validity, package structure, permission declarations, adapter behavior, fixtures and capability-specific requirements. Conformance is established by tests, not by declaration.

**NOT.** A badge, a self-declaration, or a marketing claim. A project MUST NOT be able to claim compatibility by adding a badge. Where behavior was not tested, conformance MUST NOT be claimed.

---

## Trust label

**Definition.** A precise report of exactly what was verified for an artifact: `manifest valid`, `hash verified`, `signature verified`, `author identity verified`, `maintainer reviewed`, `runtime tested`.

**NOT.** A safety guarantee. There is no "100% safe". A trust label MUST show exactly which checks passed and MUST NOT imply more assurance than was actually established.

---

## Labs

**Definition.** The pre-RFC experimentation area (`labs/`) where frontier ideas are tried without a stability promise. An idea advances via the pipeline: `Labs → RFC → extension candidate → conformance → stable`.

**NOT.** A permanent home for a feature, a place to bypass review, or a commitment. Code and designs in Labs MAY be removed at any time.

---

## Reference Harness

**Definition.** One of the intentionally different, end-to-end harnesses used to stress the standard before v1.0: Web Agent, Developer, Local Hybrid, Multi-Agent and Workflow Automation. They are **conformance torture tests**, not marketing demos.

**NOT.** A demo, a template gallery, or proof that the spec is complete. If a reference harness requires runtime-specific hacks, the **specification** must change. The spec MUST NOT be declared v1.0 before these harnesses have been implemented and used in real scenarios.
