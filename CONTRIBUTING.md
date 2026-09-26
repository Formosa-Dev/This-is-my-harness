# Contributing to This is my Harness

Thanks for helping build an open standard and toolchain for portable agentic systems. This document explains **how work moves from an idea to a stable part of the standard**, and the rules that govern contributions.

By participating, you agree to our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## Before you start

- Read the [README](README.md) and the controlled vocabulary in [`spec/glossary.md`](spec/glossary.md). Terminology matters here.
- Search existing [RFCs](rfcs/) and issues before proposing something new.
- This repository is **pre-alpha**. The spec is `v1alpha1` and is expected to change.

### The non-negotiable rule: no v1.0 before the Reference Harnesses

> **Do not declare, imply or ship Harness Spec v1.0 stability.** The spec MUST NOT be declared stable until the Reference Harnesses have been implemented and used in real scenarios and the Core can represent them without runtime-specific hacks.

Every pull request must confirm this via a checkbox in the [PR template](.github/pull_request_template.md). A PR that declares or implies v1.0 stability will be closed.

---

## The contribution pipeline

New ideas normally start in **Labs** or an **RFC**, prove themselves in a **reference harness**, and only then move toward a **stable extension**:

```
Labs → RFC → extension candidate → conformance → stable
```

| Stage | Where it lives | What it means | What it takes to advance |
| --- | --- | --- | --- |
| **Labs** | [`labs/`](labs/) | Free-form experiments. No stability promise. May be removed at any time. | A working experiment and a clear problem statement. |
| **RFC** | [`rfcs/`](rfcs/) | A written proposal for a change to the spec or a new capability. Copied from [`rfcs/0000-template.md`](rfcs/0000-template.md). | Rough consensus among maintainers; rationale and rejected alternatives documented. |
| **Extension candidate** | [`extensions/`](extensions/) | The capability is specified as a versioned extension with an `apiVersion`. | A written specification plus example manifests. |
| **Conformance** | [`conformance/`](conformance/) | Fixtures and checks prove the extension can be validated and reproduced. | Passing conformance fixtures. |
| **Stable extension** | `extensions/` (without `alpha`) | The extension is versioned and its compatibility rules are documented. | Demonstrated in at least one reference harness and required conformance tests. |

Architecture decisions that are not proposals for the spec itself — for example, implementation language, repository layout, tooling — are recorded as **ADRs** in [`docs/adr/`](docs/adr/) using [`docs/adr/0000-template.md`](docs/adr/0000-template.md). An ADR MUST state its rationale **and the alternatives it rejected**.

### Core vs extension

The Core is deliberately small. A capability belongs in the Core only if it is required by nearly every harness and can be specified without vendor lock-in. Everything else is an extension. See [GOVERNANCE.md](GOVERNANCE.md#core-vs-extension) for the acceptance criteria.

- **Never invent a competing skill or MCP format** when an existing open standard works.
- **Never hard-code a specific model vendor** into the Core (for example, Laya or Jev). Describe the *capability*; name implementations only as preferred or alternative.
- **Runtime ≠ Model.** Do not conflate the two.

---

## Ways to contribute

- Runtime-format research and portability edge cases.
- New capability-extension proposals (start with an RFC).
- Reference harnesses and conformance fixtures.
- System One / local-model experiments.
- WebMCP / MCP Apps / AG-UI / A2UI / A2A integrations.
- Security and supply-chain review (see [SECURITY.md](SECURITY.md)).
- Documentation, glossary and specification clarity.
- Cross-platform installation and toolchain work.

---

## Development workflow

1. **Open or find an issue** describing the change. For anything non-trivial, an RFC is expected first.
2. **Fork and branch.** Use a descriptive branch name, e.g. `docs/rfc-0007-system-one`.
3. **Make one focused change per pull request.** A change is one reviewable work unit.
4. **Follow the spec rules.** Requirements use RFC 2119 keywords (`MUST`, `MUST NOT`, `SHOULD`, `MAY`). Scenarios use Given/When/Then where applicable.
5. **Keep behavior inspectable and reversible.** Never auto-execute third-party scripts, hooks, MCP servers or setup commands. Never request or store runtime provider credentials.
6. **Verify.** Run the checks described in [`.github/workflows/ci.yml`](.github/workflows/ci.yml) locally where possible. Documentation-only changes must at minimum pass Markdown lint.
7. **Open the PR** using the template. Fill in every section, including the "no v1.0" checkbox.

### Commit conventions

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>
```

Common types: `docs`, `spec`, `feat`, `fix`, `chore`, `ci`, `refactor`, `test`.

- Keep the subject in the imperative mood and under ~72 characters.
- **Do NOT add `Co-Authored-By` or any AI attribution trailers.** Contributions are authored by the human contributor.
- One commit should tell one story.

### Developer Certificate of Origin (DCO)

Every commit MUST be signed off under the [Developer Certificate of Origin 1.1](https://developercertificate.org/). Add a sign-off line to each commit:

```
Signed-off-by: Your Name <you@example.com>
```

`git commit -s` adds this automatically. The sign-off certifies that you have the right to submit the contribution under the project's license.

### Licensing of contributions

By submitting a contribution, you agree that it is licensed to the project under the **Apache License 2.0** (the same license as the project), unless you explicitly state otherwise. See [`LICENSE`](LICENSE).

---

## Pull request checklist

The canonical checklist lives in [`.github/pull_request_template.md`](.github/pull_request_template.md). At minimum, a PR must:

- [ ] Link the relevant issue and/or RFC.
- [ ] Describe one focused work unit.
- [ ] Declare the **risk class** (A Passive, B Tooling, C Executable, D Privileged) of any behavior change.
- [ ] State the spec `apiVersion` it targets and any compatibility impact.
- [ ] Include a rollback plan for any change touching install, apply, revert, trust or security behavior.
- [ ] Pass Markdown lint and any applicable schema validation.
- [ ] Confirm it does **not** declare or imply v1.0 stability.
- [ ] Be DCO-signed-off.

---

## Review process

- Maintainers review pull requests following [GOVERNANCE.md](GOVERNANCE.md).
- Reviews focus on correctness, spec consistency, security invariants and reversibility — not on personal style preferences.
- Be kind and specific. See the [Code of Conduct](CODE_OF_CONDUCT.md).

---

## Security

Do **not** open a public issue for a vulnerability. Follow the disclosure process in [SECURITY.md](SECURITY.md).
