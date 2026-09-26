# Maintainers

This file lists the people responsible for reviewing and steering This is my Harness, and the areas they own. See [GOVERNANCE.md](GOVERNANCE.md) for roles and the decision process.

> **Status: TBD.** This is a pre-alpha project incubated by Formosa.dev. Named maintainers have not yet been recorded. Until names are assigned, the incubating organization acts as the interim decision body.

---

## Current maintainers

| Name | Role | Areas | Contact |
| --- | --- | --- | --- |
| Formosa.dev maintainers (TBD) | Interim maintainer body | All areas | TBD |

---

## Areas of ownership

The following areas need named owners. Until then, the interim maintainer body covers them.

| Area | Path | Description |
| --- | --- | --- |
| Specification | `spec/` | Harness Core Spec, manifest, package, install protocol, adapter contract. |
| Schema | `schemas/` | JSON Schema — the single source of truth. |
| Vocabulary | `spec/glossary.md` | Controlled vocabulary and normative definitions. |
| Core engine | `packages/core/` | Resolver, planning engine, snapshot/apply/verify/revert, path safety, hashing. |
| Validator | `packages/validator/` | `harness validate` and schema conformance. |
| SDK | `packages/sdk/` | Language bindings around the Core contract. |
| CLI | `packages/cli/` | `harness use/validate/test/conformance`, JSON output, typed states. |
| Adapters | `adapters/` | Runtime adapter contract implementations (Codex, Claude Code, OpenCode, …). |
| Extensions | `extensions/` | Versioned capability extensions. |
| Profiles | `profiles/` | developer, web-agent, local-hybrid, multi-agent, workflow-automation. |
| Conformance | `conformance/` | Conformance suite and fixtures. |
| Labs | `labs/` | Pre-RFC experiments. |
| RFCs | `rfcs/` | Proposal process. |
| Security | `SECURITY.md` | Vulnerability disclosure and supply-chain controls. |
| Documentation & governance | root `*.md`, `docs/` | Landing page, contributing, governance, ADRs. |

---

## Becoming a maintainer

There is no fixed formula, but the path is typically:

1. Contribute consistently and thoughtfully (RFCs, specs, fixtures, reviews).
2. Demonstrate sound judgment on correctness, security and reversibility.
3. Be nominated by an existing maintainer and confirmed by rough consensus.

Maintainers are expected to:

- review contributions fairly and specifically, per the [Code of Conduct](CODE_OF_CONDUCT.md);
- uphold the security invariants in [SECURITY.md](SECURITY.md);
- respect the Core-vs-extension criteria in [GOVERNANCE.md](GOVERNANCE.md);
- never declare v1.0 before the Reference Harnesses.

---

## Emeritus maintainers

_None yet._
