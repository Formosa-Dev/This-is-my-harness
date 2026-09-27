# Validator fixtures

The promoted validation corpus and its instances, driven by
`packages/validator/tests/corpus.rs`. This is the **live** corpus home for the
Rust validator and, going forward, the single source the L1/L2 cross-checks
(`scripts/schemas/validate.mjs`, `scripts/schemas/cross_check.py`) resolve.

## Provenance

Copied from the archived change
`openspec/changes/archive/2026-09-26-harness-schema-v1alpha1/examples/`.

The archive rule forbids modifying an archived change, so the corpus was
**copied, not moved**: the archive copy stays frozen as the audit trail and this
directory is the durable, non-archived home the design calls for. The only edit
on promotion was rewriting each corpus `file` entry from `examples/positive/...`
/ `examples/negative/...` to `positive/...` / `negative/...`. The `schema` `$id`s
and the `expect { path, keyword }` blocks are byte-for-byte the archived ones.

## Layout

```
corpus.json         7 positive entries, 16 negative entries (golden expectations)
positive/           7 instances (5 JSON, 2 YAML) that MUST validate
negative/           16 instances that MUST be rejected with the expected {path, keyword}
```

`keyword` is the JSON Schema keyword the frozen L1 corpus recorded; the Rust
validator maps it to its own code namespace (`keyword` -> `schema.<keyword>`).

## Not normative

`corpus.json` is a NON-NORMATIVE validation corpus. The normative sources are
`spec/**` and `schemas/**`.
