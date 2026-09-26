<!--
Thanks for contributing to This is my Harness.
Fill in every section. PRs that skip the checklist, or that declare/ imply v1.0
stability, will be sent back. See CONTRIBUTING.md and GOVERNANCE.md.
-->

## Summary

<!-- What does this PR do, in one or two sentences? -->

## Related issue / RFC

<!-- Link the issue and/or RFC. Use "Closes #N" to auto-close. -->
- Closes #
- RFC: <!-- rfcs/NNNN-*.md, or "N/A" -->

## One work unit

<!-- Confirm this PR is a single, focused, reviewable work unit. -->
This PR changes: <!-- e.g. "the glossary definition of `capability`" -->

## What changed

| File | Change |
| --- | --- |
|  |  |

## Type of change

- [ ] Documentation only
- [ ] Specification (`spec/`)
- [ ] JSON Schema (`schemas/`)
- [ ] Capability extension (`extensions/`)
- [ ] Toolchain / packaging (`packages/`)
- [ ] Runtime adapter (`adapters/`)
- [ ] Conformance / fixtures (`conformance/`)
- [ ] CI / tooling
- [ ] Governance / process

## Risk class

<!-- Risk class governs approval requirements. See spec/glossary.md and SECURITY.md. -->
- [ ] **A — Passive** (instructions/configuration, no execution)
- [ ] **B — Tooling** (MCP servers, plugins, integrations)
- [ ] **C — Executable** (hooks, scripts, commands)
- [ ] **D — Privileged** (user scope, admin/sudo, secrets, broad access)

## Spec version and compatibility

- Targets `apiVersion`: <!-- thisismyharness.dev/v1alpha1 -->
- Compatibility impact: <!-- none / additive / incompatible (requires RFC + version bump) -->
- Capability loss reported explicitly: <!-- yes / N/A -->

## Verification

<!-- How did you verify this? Commands run, fixtures checked, cross-checks against the sources. -->
```
<commands + observed result>
```

## Rollback plan

<!-- Required for anything touching install/apply/revert, trust or security. Otherwise "N/A". -->

---

## Contributor checklist

- [ ] I read [`CONTRIBUTING.md`](../CONTRIBUTING.md), [`GOVERNANCE.md`](../GOVERNANCE.md) and the [glossary](../spec/glossary.md).
- [ ] This PR is **one focused work unit** and links its issue/RFC.
- [ ] I used the controlled vocabulary from `spec/glossary.md`.
- [ ] Requirements use RFC 2119 keywords; scenarios use Given/When/Then where applicable.
- [ ] I did **not** introduce a competing skill or MCP format, and did **not** hard-code a model vendor into the Core.
- [ ] I did **not** conflate runtime with model.
- [ ] No third-party scripts, hooks, MCP servers or setup commands were auto-executed; no credentials were requested or stored.
- [ ] Filesystem mutations (if any) are scoped, snapshot-first, non-destructive and reversible.
- [ ] Markdown lint passes; schema validation passes where applicable.
- [ ] Every commit is **DCO signed-off** (`git commit -s`).
- [ ] No `Co-Authored-By` or AI-attribution trailers.
- [ ] **No v1.0 declaration.** This PR does **not** declare, imply or ship Harness Spec v1.0 stability (no v1.0 before the Reference Harnesses — see README and CONTRIBUTING).

<!-- The "no v1.0" checkbox above is a required gate, not a formality. -->
