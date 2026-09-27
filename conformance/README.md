# Conformance fixtures

The conformance suite for the `thisismyharness.dev/v1alpha1` package model.
Fixtures are the **unit tests of the standard**: a conformant package MUST
validate, and a non-conformant package MUST be rejected with the diagnostic the
index records.

The normative sources are [`../spec/`](../spec/) and [`../schemas/`](../schemas/).
Everything in this directory is **non-normative** test material.

## Layout

```text
conformance/
  README.md                 this guide
  LICENSE                   CC0-1.0 (the fixtures are conformance material)
  fixtures/
    index.json              the single index: metadata + expected diagnostics
    invalid/                non-conformant packages (F3-03)
    path-safety/            path-traversal / absolute / symlink-escape (F3-04)
    plans/                  golden install-plan diffs, pending F4 (F3-05)
```

Conformant packages live under [`../examples/`](../examples/) and are indexed
here too; the suite has exactly **one** index so a fixture is described in one
place.

## Run the suite

One command runs every fixture and reports pass/fail:

```sh
cargo test -p harness-validator --test conformance -- --nocapture
```

It is part of the workspace suite (`cargo test --workspace`) and therefore part
of the CI gate. The runner is
[`../packages/validator/tests/conformance.rs`](../packages/validator/tests/conformance.rs).

A single fixture can also be checked by hand with the CLI:

```sh
harness validate examples/minimal --json
```

## Categories

| Category | `conformant` | Expected | Meaning |
| --- | --- | --- | --- |
| `conformant` | `true` | `status: valid` | A package that MUST validate with no errors. |
| `invalid` | `false` | `status: invalid` or `error` | A package rejected with the listed `{path, code}`. |
| `path-safety` | `false` | `status: invalid` | A declared path that MUST be rejected. |
| `plan` | `false` | `status: valid` | An Install Plan that MUST validate; the diff is pending F4. |

`runner` selects how the case is evaluated: `manifest` (default), `symlink`, or
`plan`.

## Adding a fixture

1. Create the package directory. For a conformant package use `examples/<slug>/`;
   for a non-conformant one use `conformance/fixtures/invalid/<slug>/` or
   `conformance/fixtures/path-safety/<slug>/`.
2. Add the manifest as `harness.yaml` (or set `manifest` in the index; plan
   fixtures use `plan.json`). Start the file with
   `# SPDX-License-Identifier: CC0-1.0`.
3. Add the **expected** diagnostics to `index.json`:
   `expected.status` and one `{path, code}` per error the validator MUST emit.
   Use the validator's own code namespace, never a raw JSON Schema keyword
   (`schema.pattern`, not `pattern`).
4. Bump every affected number under `counts` in `index.json`. The runner fails
   when the declared counts and the fixtures present disagree.
5. Run the suite and confirm your case passes. Every other fixture must stay
   green.

### Naming conventions

- Fixture `id`: lowercase, hyphenated, and category-prefixed where useful
  (`example-minimal`, `invalid-manifest-field-missing`, `path-traversal`,
  `plan-created`).
- Directory name: the id without its category prefix where it reads better
  (`invalid/manifest-field-missing`).
- A conformant fixture directory contains a `harness.yaml`; a plan fixture
  contains `plan.json` and `expected.json`.

### Expected diagnostics

`expected.diagnostics` is a **subset** match: the runner requires every listed
`{path, code}` to be present among the report's errors, and does not fail on
additional diagnostics. This keeps a fixture stable when an unrelated check is
added, while still proving the intended error fires at the intended JSON
Pointer.

For the `symlink` runner the comparison is on the `path.symlink_escape` code
only; the pointer is asserted against the typed declared-path position.

## The `plan` fixtures are pending F4

The install-plan diff engine is **F4** and does not exist yet. The three
`plans/` fixtures therefore ship as **data only**:

- `plan.json` is validated against the Install Plan schema (this runs today);
- `expected.json` records the golden `created` / `modified` / `conflict`
  projection and sets `pending_f4: true`.

The runner proves the projection is coherent with the plan and stops there. It
**does not** compute or compare a runtime diff, and nothing here simulates the
plan engine. When F4 lands, the same fixtures gain the executable comparison.

## License

The fixtures are dedicated to the public domain under **CC0-1.0** as
conformance material; see [`LICENSE`](LICENSE). Each manifest carries an
`SPDX-License-Identifier: CC0-1.0` header.

## Relation to the promoted validator corpus

[`../packages/validator/tests/fixtures/`](../packages/validator/tests/fixtures/)
holds a separate, smaller corpus (7 positive / 16 negative) that drives the
schema and validator unit tests and the L1/L2 cross-checks. That corpus is
**not** this suite and MUST NOT be modified from here. This suite is broader: it
exercises whole packages, the five profiles, partial layouts, path safety, the
index, and the golden plan diffs.
