# Example harness packages

Reference harness packages that exercise the `thisismyharness.dev/v1alpha1`
package model. Every package here is **conformant**: it MUST validate under the
full pipeline (`harness validate <dir>` exits `0`).

These are reference implementations of the profiles and of the "a package can
remain simple" rule (README, *Package model*). They are also the conformant half
of the conformance suite; the non-conformant half lives in
[`../conformance/`](../conformance/).

## Contents

| Package | Profile | Exercises |
| --- | --- | --- |
| `minimal/` | none | The smallest valid harness: `harness.yaml` plus `instructions/` and `skills/`. (F3-01) |
| `developer/` | `developer` | Skills, a passive policy and a local model; a runtime requirement. (F3-02) |
| `web-agent/` | `web-agent` | An MCP component and a declared non-passive permission. (F3-02) |
| `local-hybrid/` | `local-hybrid` | A remote generative model plus a local decision model and a backend requirement. (F3-02) |
| `multi-agent/` | `multi-agent` | Two collaborating agent components. (F3-02) |
| `workflow-automation/` | `workflow-automation` | A workflow component plus a decision model. (F3-02) |
| `partial-instructions-only/` | none | A package that uses only `instructions/`. (F3-07) |
| `partial-mcp-only/` | none | A package that uses only `mcp/`. (F3-07) |
| `partial-workflows-only/` | none | A package that uses only `workflows/`. (F3-07) |

## Verify

```sh
harness validate examples/minimal
```

Full suite: see [`../conformance/README.md`](../conformance/README.md).

## License

The example packages are dedicated to the public domain under **CC0-1.0** as
conformance material; see [`LICENSE`](LICENSE).
