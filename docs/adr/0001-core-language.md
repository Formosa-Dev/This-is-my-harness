# ADR-0001: Harness Core is implemented in Rust; the initial surface is CLI-only

| Field | Value |
| --- | --- |
| **ADR number** | 0001 |
| **Title** | Harness Core is implemented in Rust; the initial surface is CLI-only |
| **Status** | Accepted |
| **Date** | 2026-09-26 |
| **Deciders** | Project owner (confirmed) |
| **Related** | `architecture/product-plan` (Engram #4322), README v0.2, Decisiones Post-Documento 2 v0.1 |

---

## Context

This is my Harness is an open standard plus a toolchain. The toolchain's central job is to be the **mutation authority over the local filesystem when processing untrusted third-party input**: resolving a harness, planning an install, snapshotting, applying, verifying and reverting managed files — with path-traversal and symlink-escape protection, hash verification, OCI artifact distribution and signature verification.

An earlier plan assumed a Tauri desktop application with a Rust `harness-core` library and a React/TypeScript UI. The project's thesis has since shifted to **standard-first, social-last**: the specification and toolchain must stand on their own, and the social/registry/web/desktop layers are downstream consumers. That shift does not by itself mandate any implementation language, but it does change the priority order and the tolerated costs.

The relevant constraints:

- Distribution must eventually be a **single cross-platform binary** that can be bootstrapped without requiring an external language runtime.
- The Core must be hardened against **untrusted artifacts** (malicious manifests, crafted paths, symlink escapes, tampered blobs).
- Filesystem mutation, locking, snapshotting and atomic writes must be **robust and crash-recoverable**.
- The project must remain **reversible** on this decision: `spec/` and `schemas/` are language-neutral (JSON Schema is the single source of truth), so an implementation can be replaced without changing the standard.
- Contributor onboarding speed matters, and the surrounding agentic ecosystem (WebMCP, MCP Apps, AG-UI, A2UI) is predominantly JS/TS.

---

## Decision

We will implement **Harness Core in Rust** as:

- **`harness-core`** — a Rust library containing the resolver, planning engine, snapshot/apply/verify/revert, path-safety, hashing, OCI client and signature verification.
- **`harness`** — a Rust CLI producing a **single cross-platform binary**, with a machine-readable JSON output mode and typed states.

The **initial surface is CLI-only**. There is no desktop application and no TypeScript surface in the initial scope:

- `spec/` and `schemas/` remain **language-neutral**; JSON Schema is the single source of truth.
- **TypeScript is deferred** to the social/downstream phase (F17) and to Labs experiments.
- The **Desktop is deferred**: it is treated as a future *client* of the same Core, not as part of the Core.

---

## Rationale

- **The crown jewel is mutation authority over the filesystem.** Parsing and acting on untrusted third-party input is exactly where memory safety and careful I/O pay off. Rust gives strong guarantees here without a garbage collector.
- **Single-binary distribution.** A statically linked binary avoids "install a runtime first", which is central to the one-command bootstrap goal.
- **Deterministic, crash-safe I/O.** Rust's ecosystem supports robust atomic writes, file locking and hashing, and the language makes error handling explicit.
- **Reversibility is preserved by the standard.** Because `spec/` and `schemas/` are language-neutral, choosing Rust for the reference implementation does not bind the *standard* to Rust. A conformant implementation in another language remains possible.
- **Code signing and supply chain** are simpler when the shipped artifact is a native binary rather than a bundled runtime.

---

## Rejected alternatives

| Alternative | Description | Why rejected |
| --- | --- | --- |
| **TypeScript-first (Node/Bun)** | Implement the Core, validator and CLI in TypeScript; distribute via Bun compile or Node SEA. | Faster iteration and a JS/TS-native agentic ecosystem, but requires an external runtime for distribution (unless single-file compiled), and filesystem/snapshot hardening for untrusted input must be built more carefully by hand. Distribution and safety, the two properties that matter most for the Core, are weaker. |
| **Hybrid (Rust core + TypeScript surface)** | Rust only for mutation/snapshot/path-safety/hash/OCI/verify; TypeScript for CLI UX, validator orchestration, SDK and web. | Genuinely attractive for velocity, but introduces an IPC/JSON/WASM interface boundary, two toolchains and two CI matrices before there is a working standard. It risks "logic split across two languages" and slows the pre-alpha phase where correctness of the Core is what matters. Revisited later as an *addition* (SDK/surface), not as a replacement for the Core. |
| **Keep the original Tauri/React desktop as the primary surface** | Ship the desktop app first, with the CLI as a secondary interface. | The desktop UI is a downstream consumer of the same Core. Building it first inverts the standard-first thesis, multiplies the surface under test, and delays the reproducibility proof that the reference harnesses must provide. The Desktop remains a planned future client. |

---

## Consequences

**Positive**

- The Core is implemented once, in the language best suited to safe mutation and cross-platform distribution.
- `spec/` and `schemas/` stay language-neutral, so the decision remains reversible from the standard's perspective.
- The CLI is scriptable and agent-consumable from day one (JSON output, typed states).

**Negative**

- Contributor onboarding is slower and iteration is less immediate than in TypeScript.
- The agentic ecosystem integrations (WebMCP, MCP Apps, AG-UI, A2UI) are JS/TS-native; integrating them will require care at the boundary and possibly a later TS surface.
- A future desktop client will need an interface to the Core (CLI JSON or a stable library/FFI contract), which is additional work deferred to the downstream phase.

**Neutral / follow-up**

- A TypeScript SDK and any Desktop client remain valid, and are deferred to F17 / downstream.
- If measurement later shows Rust is the wrong trade-off for velocity, the language-neutral spec keeps a port viable; the standard does not change.
- This ADR MUST be revisited if the initial surface stops being CLI-only, or if a second implementation of the Core appears.

---

## References

- `This is my Harness — README v0.2.md` — "Suggested repository structure", "Initial implementation priority", "Project status".
- `This is my Harness — Decisiones Post-Documento 2, Estándar, Ecosistema y Reference Harnesses v0.1.md` — §4 (small core / strong extensions), §6 (Runtime ≠ Model), §50 (filesystem security), §54 (implementation order), §57 (what not to do).
- Engram `architecture/product-plan` (#4322) — §2 module map and §4 language recommendation.
- [`spec/VERSIONING.md`](../spec/VERSIONING.md) — language-neutral spec versioning.
