# Governance

This document describes how decisions are made in the This is my Harness project: the roles, the decision process, and the criteria for accepting capabilities into the Core versus an extension.

This project is incubated by **Formosa.dev** and is intended to be built in public as a technical forum.

---

## Scope of governance

Governance covers:

- the Harness Core Specification and its `apiVersion`s;
- the JSON Schema in `schemas/` (the single source of truth);
- capability extensions in `extensions/`;
- the Runtime Adapter Contract and the Install Protocol;
- conformance requirements and the toolchain that enforces them;
- this repository's process (RFCs, ADRs, releases).

Governance does **not** cover downstream consumers such as a future registry, website or desktop application, except insofar as they MUST consume conformant artifacts.

---

## Principles

1. **The standard is technical, not social.** The social/registry layer consumes conformant artifacts; it MUST NOT define the standard's semantics.
2. **Core small, extensions strong.** `Standardize composition, not every primitive.`
3. **Adopt existing open standards** rather than inventing incompatible equivalents.
4. **Correctness and reversibility over breadth.** Explicit contracts, fixtures, conformance and threat modeling come before features.
5. **No v1.0 before the Reference Harnesses.**
6. **Everything is auditable.** Decisions record their rationale and the alternatives they rejected.

---

## Roles

| Role | Responsibilities |
| --- | --- |
| **Contributor** | Anyone who submits an issue, RFC, PR, fixture or review. No special permissions. |
| **Labs participant** | A contributor working in [`labs/`](labs/). Experiments carry no stability promise. |
| **RFC author** | A contributor who has opened an RFC in [`rfcs/`](rfcs/). Owns shepherding it to a decision. |
| **Maintainer** | Reviews and merges pull requests, triages issues, guides RFCs. Listed in [MAINTAINERS.md](MAINTAINERS.md). |
| **Core maintainer** | Maintains the specification and `schemas/`. Owns correctness of the standard itself. |
| **Security maintainer** | Owns [SECURITY.md](SECURITY.md) and the supply-chain controls. |
| **Steering** | Resolves decisions that maintainers cannot reach consensus on, and finalizes `apiVersion` bumps. |

> Names for all roles are currently **Formosa.dev maintainers (TBD)**. See [MAINTAINERS.md](MAINTAINERS.md). Until named maintainers exist, the incubating organization acts as the interim decision body.

---

## Decision process

### Default: lazy consensus

Most changes proceed by **lazy consensus**: a pull request that follows the contribution flow and receives no sustained objection is merged. Silence is consent, given a reasonable review window.

### RFCs for significant changes

An RFC ([`rfcs/`](rfcs/)) is **required** for:

- a change to the Harness Core Specification;
- a new capability extension or a change to an existing one;
- a change to the Install Protocol, the Runtime Adapter Contract, or conformance rules;
- anything that changes compatibility or migration semantics.

An RFC is **accepted** when it has rough consensus among maintainers and no unresolved blocking objection. The RFC MUST document its rationale and its rejected alternatives.

### ADRs for architecture decisions

Decisions about the implementation — language, repository layout, tooling, distribution mechanics — are recorded as **Architecture Decision Records** in [`docs/adr/`](docs/adr/). An ADR records context, the decision, its rationale, **and the alternatives that were rejected**. See [`docs/adr/0000-template.md`](docs/adr/0000-template.md).

### Escalation

If maintainers cannot reach consensus, the issue is escalated to Steering. Steering's decision is recorded with its rationale. Any decision MAY be revisited by a new RFC that presents new evidence.

### Consensus is not unanimity

A single objection does not block a change if the objection is addressed or shown to be outweighed by technical reasons. Conversely, a maintainer MUST NOT merge over a credible, unaddressed security or correctness objection.

---

## Core vs extension

A capability is a candidate for the **Core** only if **all** of the following hold:

1. It is required by nearly every harness, or by the fundamental identity, packaging or installation model.
2. It can be specified **without vendor lock-in** and without hard-coding a specific implementation.
3. It can be **validated** mechanically (schema/conformance).
4. It is stable enough that representing the Reference Harnesses does not require special-casing it.
5. It does not duplicate an existing open standard.

Otherwise it belongs in an **extension**, which is versioned, optional and independently specified.

### Explicit anti-goals

- Do not invent another skill format when Agent Skills suffices.
- Do not invent another MCP.
- Do not hard-code Laya, Jev or any other model vendor into the Core.
- Do not conflate runtime with model.
- Do not declare all runtimes "compatible" for marketing reasons.
- Do not execute scripts without preview and policy.
- Do not publish fictitious installation commands.
- Do not declare v1.0 before the Reference Harnesses.
- Do not let the social layer define the standard's semantics.
- Do not promote every ecosystem novelty to the Core.

---

## Releases and versioning

- The spec `apiVersion` follows the rules in [`spec/VERSIONING.md`](spec/VERSIONING.md) and is independent of package versions.
- The artifact status is declared **alpha/labs** until the v1.0 gate is met. Declaring v1.0 before that is prohibited.
- Releases are immutable and content-addressed; signatures are verified before Apply.

---

## Changing this document

Changes to `GOVERNANCE.md` or `MAINTAINERS.md` require an RFC or an explicit maintainer decision recorded with rationale. As the project matures, governance SHOULD move toward a documented, versioned charter with named maintainers across organizations.
