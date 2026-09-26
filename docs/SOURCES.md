# Sources of Truth

This document records where the This is my Harness project's authoritative inputs live, which of them govern the current thesis, and how they relate. It exists so that any contributor (human or agent) can trace a claim in the repository back to its origin.

Last updated: 2026-09-26.

---

## Governing sources (current thesis: standard-first)

These documents define the project as it is understood today: an **open standard plus toolchain first**, with the social/registry/desktop layers **downstream**.

| Source | Language | Role |
| --- | --- | --- |
| [`This is my Harness — README v0.2.md`](../This%20is%20my%20Harness%20%E2%80%94%20README%20v0.2.md) | English | Public and technical explanation of the standard and the repository. Drives the README, repository structure, license and contributing language. |
| [`This is my Harness — Decisiones Post-Documento 2, Estándar, Ecosistema y Reference Harnesses v0.1.md`](../This%20is%20my%20Harness%20%E2%80%94%20Decisiones%20Post-Documento%202%2C%20Est%C3%A1ndar%2C%20Ecosistema%20y%20Reference%20Harnesses%20v0.1.md) | Spanish | Consolidated decisions **§1–§62**. Key sections: §44 (Labs/RFC pipeline), §50 (filesystem security), §51 (supply chain), §52 (repository structure), §53 (license), §54 (implementation order), §55 (launch criteria), §57 (what not to do), §58 (open questions). |

> **Note on language.** The public repository artifacts (README, CONTRIBUTING, SECURITY, GOVERNANCE, glossary, templates, and all specified behavior) are written in **English**, because this is a global open standard. The consolidated decisions source is in Spanish; it is an internal working document, and the English artifacts derived from it are what the project publishes.

---

## Lineage (historical documents)

The project evolved through several documents. Only the governing sources above define the **current** thesis.

| Document | Role in lineage |
| --- | --- |
| **Document 1** | Initial product / registry, public profile, static-first, publishing. |
| **Document 2** — [`Formosa.dev Harnesses — Social Network + Harness Desktop Runtime Manager — Product & Architecture v0.1.md`](../Formosa.dev%20Harnesses%20%E2%80%94%20Social%20Network%20%2B%20Harness%20Desktop%20Runtime%20Manager%20%E2%80%94%20Product%20%26%20Architecture%20v0.1.md) | Social network + Harness Desktop + zero-friction + one-command, runtime manager, monetization, backend. **Not the primary source for the current thesis** (it predates the shift to standard-first). |
| **Document 3 / README v0.2** | Condensed public/technical explanation of the repository and the standard. |
| **Document 4** (the "Decisiones Post-Documento 2 …" source above) | Exhaustive consolidation of post-Document-2 decisions: redefinition as a standard for agentic systems, System One / local models, web/agent protocols, OCI distribution and Reference Harnesses. |

---

## Duplicate artifact — PENDING REMOVAL

The repository root contains **two byte-identical copies** of the lineage document:

- `Formosa.dev Harnesses — Social Network + Harness Desktop Runtime Manager — Product & Architecture v0.1.md`
- `Formosa.dev Harnesses — Social Network + Harness Desktop Runtime Manager — Product & Architecture v0.1 (1).md`

They are confirmed identical by SHA-256:

```
4FF9920C088EC2DA76B817DAA59CBDACB4848B1BB5620109154EA8E6C49A2F2B
```

> **Status: PENDING — do not delete yet.**
>
> The ` (1).md` copy is a redundant duplicate that **should be removed**, but removal is intentionally deferred until **after the foundational commit** exists, so the deletion is recoverable from git history. Until then, treat the two files as **one document**. This is a deliberate, documented exception to avoiding irreversible operations: at the time of writing, the repository has **zero commits**, so a deletion now could not be restored.
>
> **Action required (post-foundational-commit):** delete the ` (1).md` copy in a dedicated commit, and update this section to record the removal.

---

## Machine-readable sources of truth

Project state for agents is tracked in **Engram** (project `this-is-my-harness`), not in this repository:

| Observation | Topic key | Role |
| --- | --- | --- |
| Product plan | `architecture/product-plan` | Product layers, architecture, canonical build order, language recommendation. |
| Master build index | `build-progress` | Phase index F0–F17, the authoritative statement of current build state. |
| Phase observations | `build-progress/F0` … `build-progress/F17` | Per-phase, per-task progress. Update tasks **only** in the phase observation. |

The master build index is the source of truth for **what is built and what is not**. This file (`docs/SOURCES.md`) is the source of truth for **provenance**.

---

## Repository-local decisions

Architecture decisions derived from these sources are recorded as ADRs in [`docs/adr/`](adr/). The first is [`docs/adr/0001-core-language.md`](adr/0001-core-language.md), which records the Harness Core language decision. Proposals that change the standard's semantics go through [`rfcs/`](../rfcs/).
