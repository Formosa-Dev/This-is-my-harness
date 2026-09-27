# Verification — harness-schema-v1alpha1 Phase 5 (PR 5)

> **Status:** pre-alpha · **Date:** 2026-09-26 · **Scope:** tasks 5.1–5.5 (L2 cross-check, F1 traceability matrix, ADR-0002, freeze gate, trust statement).
> **Environment:** Node v22.17.0 · npm 10.8.2 · Python 3.12.10 · `jsonschema` 4.26.0 · PyYAML 6.0.3.
> Every command below was executed in this repository; the output is pasted verbatim. These artifacts are NON-NORMATIVE machine/documentation outputs; the normative source is `spec/**`.

## Final three-layer gate (5.5)

| Layer | Command | Output | Exit |
|---|---|---|---|
| L0 structural | `python scripts/schemas/check_schemas.py` | `OK schemas=3 defs=7 refs=38 open=9 vendor=0 unresolved=0 cycle=0` | 0 |
| L1 schema (ajv) | `npm run validate:schemas` | `PASS schemas=3 positive=5 negative=9` | 0 |
| L2 cross-check | `python scripts/schemas/cross_check.py` | `PASS schemas=3 positive=5 negative=9` | 0 |
| Trust | `python scripts/schemas/check_schemas.py --trust` | `TRUST OK` | 0 |

L1 and L2 are two independent Draft 2020-12 engines agreeing on the same corpus (five positive instances pass, nine negative instances fail), which is the interim portability proof for risk R3.

## Task evidence

### 5.1 — L2 cross-check (`scripts/schemas/cross_check.py`)

```
$ python -m pip install "jsonschema>=4.18"   # exit 0 — already satisfied
$ python -m pip show jsonschema
Name: jsonschema
Version: 4.26.0

$ python scripts/schemas/cross_check.py
PASS schemas=3 positive=5 negative=9         # exit 0
```

### 5.2 — F1 traceability matrix (`schemas/README.md`)

```
$ python scripts/schemas/check_schemas.py --trace
TRACE OK rows=40 untraced=0                  # exit 0
```

Every non-OPEN constraint row cites an existing `spec/**.md` file and a section anchor. A row with no anchor, or an anchor naming a `spec/**.md` file that does not exist, is counted as untraced and fails the check. §58-OPEN positions are deliberately not constraints and are not traced here; they are listed and marked OPEN. `spec/**` was not edited.

### 5.3 — ADR-0002 (`docs/adr/0002-schema-strategy.md`)

```
$ python scripts/schemas/check_schemas.py --adr
ADR OK headings=7                            # exit 0
```

The ADR follows `docs/adr/0000-template.md` (Context, Decision, Rationale, Rejected alternatives, Consequences, Status of related decisions, References) and records the dialect, `$id` scheme, modular `$defs` and absolute-refs-plus-resolver decision, with rejected alternatives A1–A12.

### 5.4 — pre-freeze blocker R3

```
$ python scripts/schemas/check_schemas.py --freeze
FREEZE BLOCKED r3=pending                    # exit 0
```

R3 (Rust `jsonschema` 2020-12 / `unevaluatedProperties` support unverified — Rust is not installed on this machine) is recorded in both the ADR and `schemas/README.md`. The freeze gate is expected to read `FREEZE BLOCKED r3=pending` until F2-05 confirms Rust support; L1/L2 parity is the interim proof. The check exits 0 because it certifies the blocker is *recorded*; an actual freeze requires the gate to read `FREEZE OK`.

### 5.5 — trust statement

`schemas/README.md` carries a `## Trust statement` naming only the checks actually performed, with no absolute-safety claim and no conformance claim. `--trust` scans the README and the ADR for overclaims and passes (`TRUST OK`, exit 0).

## Non-vacuity (adversarial) proof

The new checks were proven to detect failures on throwaway fixtures (not on repository files):

- trace: an untraced row → `FAIL TRACE rows=2 untraced=2` (exit 1).
- adr: a two-heading document → `FAIL ADR headings=2` (exit 1).
- freeze: a missing `r3` record → `FAIL freeze: r3 state not recorded` (exit 1).
- trust: a `100% safe` claim → `FAIL trust: overclaim` (exit 1).
- control: a correct fixture → all four pass (exit 0).

## Invariants

`cycle=0`, `unresolved=0`, `vendor=0`, and exit 0 across L0/L1/L2. The real schema counts are `schemas=3` and `refs=38`; older task text that pins `schemas=4` / `refs=11` is stale and is superseded by this evidence.
