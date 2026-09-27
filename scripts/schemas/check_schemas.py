#!/usr/bin/env python3
"""L0 structural pre-check for the harness schema set (F2).

Standard library only. This is the guaranteed gate on a machine without
``jsonschema`` or ``ajv`` installed. It performs the checks JSON Schema cannot
make about itself:

1. every schema parses as JSON and declares the Draft 2020-12 dialect;
2. every ``$ref`` targets an ``$id`` declared in ``schemas/registry.json`` AND a
   file that exists on disk (offline resolver integrity);
3. no two schema documents declare the same ``$id`` and no two ``$id``s map to
   the same repo path (duplicate definition);
4. every registry entry maps to a file that exists on disk;
5. no vendor token appears in any schema ``const``/``enum`` (STYLE.md §5);
6. every §58-sensitive position carries an explicit OPEN marker;
7. the reference graph contains no cycle;
8. ``schemas/README.md`` documents the required strategy sections.

Modes::

    python scripts/schemas/check_schemas.py            # full L0 check
    python scripts/schemas/check_schemas.py --self-test # 5 adversarial fixtures
    python scripts/schemas/check_schemas.py --readme    # README sections only
    python scripts/schemas/check_schemas.py --guards    # cycle/leaf structural guards
    python scripts/schemas/check_schemas.py --trace     # F1 traceability matrix (PR 5)
    python scripts/schemas/check_schemas.py --adr       # ADR-0002 headings (PR 5)
    python scripts/schemas/check_schemas.py --freeze    # R3 freeze gate (exit 1 while blocked)
    python scripts/schemas/check_schemas.py --trust     # trust statement (PR 5)

Exit code is 0 on success and 1 on any failure.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path

# --- Constants ---------------------------------------------------------------

BASE_DEFAULT = "https://thisismyharness.dev/schemas/v1alpha1/"
DIALECT = "https://json-schema.org/draft/2020-12/schema"

REGISTRY_PATH = "schemas/registry.json"
HARNESS_ROOT = "schemas/harness.v1alpha1.schema.json"
MODEL_CONTRACT_PATH = "schemas/model-contract.schema.json"
INSTALL_PLAN_PATH = "schemas/install-plan.schema.json"
README_PATH = "schemas/README.md"

# Structural cycle/leaf guards (F2-03/F2-04). The composition edges are defined
# by design.md §2 and enforced here, not by convention:
#   Guard 1: model-contract MUST NOT `$ref` the component descriptor (the
#            component -> model-contract edge is one-way; a back-reference would
#            close the cycle component <-> model-contract).
#   Guard 2: install-plan `$ref`s leaf `defs/` schemas only and never the manifest
#            root, because external hosts consume a plan with no manifest in hand.
HARNESS_ROOT_SUFFIX = "harness.schema.json"
COMPONENT_DEF_SUFFIX = "defs/component.schema.json"
DEFS_PREFIX = "defs/"

# Vendor tokens that MUST NOT appear in a schema `const`/`enum` (STYLE.md §5).
# Match is case-insensitive and word-bounded to avoid false positives.
VENDOR_TOKENS = (
    "laya",
    "jev",
    "openai",
    "anthropic",
    "claude",
    "gemini",
    "llama",
    "mistral",
    "cohere",
    "bedrock",
)

# §58-sensitive property names: the property subschema MUST carry an OPEN marker
# naming §58 (no fixed media type, no fixed canonical host, no fixed range
# grammar, no fixed capability registry, no fixed policy/workflow payload, no
# fixed override syntax). One occurrence per concept; no nested re-declaration.
OPEN_SENSITIVE_KEYS = frozenset(
    {
        "artifactType",
        "canonicalIdentifier",
        "versionRange",
        "capabilityId",
        "payload",
        "override",
    }
)

# File-scoped OPEN rules: (schema filename suffix, sensitive property names).
OPEN_FILE_RULES = (
    ("model-contract.schema.json", frozenset({"input", "output"})),
)

# README sections the strategy document MUST contain (F2-02 deliverable).
README_SECTIONS = (
    ("dialect", ("dialect",)),
    ("$id-map", ("$id",)),
    ("resolver-map", ("resolver",)),
    ("strictness", ("strictness", "strict")),
    ("§58-open-items", ("58",)),
    ("semantic-boundary", ("semantic",)),
    ("install-plan-local-only", ("local-only", "local only", "never uploaded")),
)

# --- PR 5 deliverables (cross-check, traceability, ADR, freeze, trust) --------

ADR_PATH = "docs/adr/0002-schema-strategy.md"
TRACE_HEADING = "f1 traceability matrix"
TRUST_HEADING = "trust statement"

# The ADR template defines exactly these seven `##` sections; ADR-0002 MUST
# follow docs/adr/0000-template.md rather than invent a shape of its own.
ADR_HEADINGS = (
    "context",
    "decision",
    "rationale",
    "rejected alternatives",
    "consequences",
    "status of related decisions",
    "references",
)

# Machine-readable state marker for risk R3 (dialect support in the Rust
# validator, F2-05). `pending` blocks an actual freeze; `confirmed` clears it.
R3_STATE_RE = re.compile(r"r3\s*=\s*(pending|confirmed)", re.IGNORECASE)

# Absolute-safety / conformance overclaims the trust statement MUST NOT make.
FORBIDDEN_CLAIMS = (
    "100% safe",
    "100 percent safe",
    "100% secure",
    "guaranteed safe",
    "guarantees safety",
    "guaranteed secure",
    "proves conformance",
    "proven conformance",
    "certified conformant",
    "guarantees conformance",
    "fully conformant",
    "is conformant",
    "are conformant",
)


# --- Walkers -----------------------------------------------------------------


def iter_nodes(obj, ptr=""):
    """Depth-first walk yielding ``(json-pointer, node)`` for every value."""
    yield ptr, obj
    if isinstance(obj, dict):
        for key, value in obj.items():
            yield from iter_nodes(value, ptr + "/" + key)
    elif isinstance(obj, list):
        for index, value in enumerate(obj):
            yield from iter_nodes(value, ptr + "/" + str(index))


def _strip_fragment(ref):
    return ref.split("#", 1)[0]


def _has_vendor(value):
    low = value.lower()
    return any(
        re.search(r"\b" + re.escape(token) + r"\b", low) for token in VENDOR_TOKENS
    )


# --- Individual checks -------------------------------------------------------


def check_dialect(docs):
    """Every schema MUST declare the Draft 2020-12 dialect."""
    failures = []
    for name, data in docs:
        if data.get("$schema") != DIALECT:
            failures.append(
                f"FAIL dialect: {name} $schema={data.get('$schema')!r} "
                f"(expected {DIALECT})"
            )
    return failures


def scan_vendor(docs):
    """No vendor token in any `const`/`enum` string (STYLE.md §5)."""
    failures = []
    for name, data in docs:
        for ptr, node in iter_nodes(data):
            if not isinstance(node, dict):
                continue
            const = node.get("const")
            if isinstance(const, str) and _has_vendor(const):
                failures.append(f"FAIL vendor: {name} #{ptr}/const = {const!r}")
            enum = node.get("enum")
            if isinstance(enum, list):
                for value in enum:
                    if isinstance(value, str) and _has_vendor(value):
                        failures.append(
                            f"FAIL vendor: {name} #{ptr}/enum = {value!r}"
                        )
    return failures


def check_refs(base, known_ids, docs, root):
    """Every `$ref` MUST resolve in registry.json AND exist on disk."""
    failures = []
    for name, data in docs:
        for ptr, node in iter_nodes(data):
            if not (isinstance(node, dict) and isinstance(node.get("$ref"), str)):
                continue
            ref = node["$ref"]
            target = _strip_fragment(ref)
            if target == "":
                continue  # local ref (e.g. "#/$defs/x") inside the same document
            if target in known_ids:
                path = known_ids[target]
                if (Path(root) / path).exists():
                    continue
                failures.append(
                    f"FAIL unresolved: {name} #{ptr} -> {ref} "
                    f"(registered but file missing: {path})"
                )
                continue
            if target.startswith(base):
                failures.append(
                    f"FAIL unresolved: {name} #{ptr} -> {ref} "
                    "(not declared in registry.json)"
                )
                continue
            if (Path(root) / target).exists():
                continue
            failures.append(f"FAIL unresolved: {name} #{ptr} -> {ref}")
    return failures


def check_duplicates(docs, known_ids=None):
    """No two schema documents MAY declare the same `$id`; paths MUST be unique."""
    failures = []
    seen_ids = {}
    for name, data in docs:
        sid = data.get("$id")
        if isinstance(sid, str):
            if sid in seen_ids and seen_ids[sid] != name:
                failures.append(
                    f"FAIL duplicate: $id {sid!r} declared by {seen_ids[sid]} and {name}"
                )
            else:
                seen_ids[sid] = name
    path_to_id = {}
    for sid, path in (known_ids or {}).items():
        if path in path_to_id and path_to_id[path] != sid:
            failures.append(
                f"FAIL duplicate: path {path!r} registered by "
                f"{path_to_id[path]!r} and {sid!r}"
            )
        else:
            path_to_id[path] = sid
    return failures


def check_registry_paths(known_ids, root):
    """Every registered `$id` MUST map to a file that exists on disk."""
    failures = []
    for sid, path in (known_ids or {}).items():
        if not (Path(root) / path).exists():
            failures.append(f"FAIL registry: {sid} -> missing file {path}")
    return failures


def _open_targets(name, data):
    """Yield ``(ptr, property_name, subschema)`` for OPEN-sensitive positions."""
    basename = os.path.basename(name)
    file_keys = [keys for suffix, keys in OPEN_FILE_RULES if basename.endswith(suffix)]
    for ptr, node in iter_nodes(data):
        if not isinstance(node, dict):
            continue
        for key, value in node.items():
            if not isinstance(value, dict):
                continue
            if key in OPEN_SENSITIVE_KEYS or any(key in keys for keys in file_keys):
                yield ptr, key, value


def check_open(docs):
    """Every §58-sensitive position MUST carry an explicit OPEN marker."""
    failures = []
    for name, data in docs:
        for ptr, key, value in _open_targets(name, data):
            text = json.dumps(value)
            if "OPEN" in text and "58" in text:
                continue
            failures.append(
                f"FAIL open: {name} #{ptr}/{key} missing OPEN/§58 marker"
            )
    return failures


def count_open(docs):
    total = 0
    for name, data in docs:
        for _ptr, _key, value in _open_targets(name, data):
            text = json.dumps(value)
            if "OPEN" in text and "58" in text:
                total += 1
    return total


def count_refs(docs):
    total = 0
    for _name, data in docs:
        for _ptr, node in iter_nodes(data):
            if isinstance(node, dict) and isinstance(node.get("$ref"), str):
                total += 1
    return total


def find_cycles(edges):
    """Return a list of human-readable cycles in a node->targets adjacency map."""
    white, grey, black = 0, 1, 2
    color = {}
    stack = []
    cycles = []

    def visit(node):
        color[node] = grey
        stack.append(node)
        for target in edges.get(node, ()):
            state = color.get(target, white)
            if state == grey:
                start = stack.index(target)
                cycles.append(" -> ".join(stack[start:] + [target]))
            elif state == white:
                visit(target)
        stack.pop()
        color[node] = black

    for node in list(edges):
        if color.get(node, white) == white:
            visit(node)
    return cycles


def build_edges(docs):
    id_to_rel = {}
    for rel, data in docs:
        if isinstance(data.get("$id"), str):
            id_to_rel[data["$id"]] = rel
    edges = {}
    for rel, data in docs:
        targets = set()
        for _ptr, node in iter_nodes(data):
            if isinstance(node, dict) and isinstance(node.get("$ref"), str):
                target = _strip_fragment(node["$ref"])
                if target in id_to_rel:
                    targets.add(id_to_rel[target])
        edges[rel] = targets
    return edges


def collect_refs(data):
    """Return the list of absolute `$ref` targets (fragments stripped) in a doc."""
    targets = []
    for _ptr, node in iter_nodes(data):
        if isinstance(node, dict) and isinstance(node.get("$ref"), str):
            target = _strip_fragment(node["$ref"])
            if target != "":
                targets.append(target)
    return targets


# --- Cycle / leaf guards (F2-03/F2-04) --------------------------------------


def run_guards(root):
    """Assert the structural guards of design.md §2 plus graph integrity.

    Runs against the real schema tree: the model contract MUST NOT reference the
    component descriptor, the Install Plan MUST reference leaf ``defs/`` schemas
    only, and the reference graph MUST be acyclic and fully resolvable.
    """
    registry_file = Path(root) / REGISTRY_PATH
    if not registry_file.exists():
        print(f"FAIL missing: {REGISTRY_PATH}")
        return 1
    registry = json.loads(registry_file.read_text(encoding="utf-8"))
    base = registry.get("base", BASE_DEFAULT)
    known_ids = registry.get("ids", {}) or {}

    docs, _roots, _defs = discover_docs(root)
    by_name = {name: data for name, data in docs}
    failures = []

    # Guard 1: model-contract MUST NOT `$ref` the component descriptor.
    model = by_name.get(MODEL_CONTRACT_PATH)
    if model is None:
        failures.append(f"FAIL guard: missing {MODEL_CONTRACT_PATH}")
    else:
        component_id = base + COMPONENT_DEF_SUFFIX
        for target in collect_refs(model):
            if target == component_id:
                failures.append(
                    "FAIL guard: model-contract must not $ref the component "
                    f"descriptor ({component_id})"
                )

    # Guard 2: install-plan `$ref`s leaf defs only and never the harness root.
    plan = by_name.get(INSTALL_PLAN_PATH)
    if plan is None:
        failures.append(f"FAIL guard: missing {INSTALL_PLAN_PATH}")
    else:
        harness_id = base + HARNESS_ROOT_SUFFIX
        leaf_count = 0
        for target in collect_refs(plan):
            if target == harness_id:
                failures.append(
                    "FAIL guard: install-plan must not $ref the manifest root "
                    f"({harness_id})"
                )
            elif target.startswith(base + DEFS_PREFIX):
                leaf_count += 1
            else:
                failures.append(
                    f"FAIL guard: install-plan $ref not a leaf def: {target}"
                )

    # Guard 3: the reference graph MUST be acyclic.
    cycles = find_cycles(build_edges(docs))
    failures += [f"FAIL guard cycle: {cycle}" for cycle in cycles]

    # Guard 4: every `$ref` MUST resolve (registry + disk).
    unresolved = check_refs(base, known_ids, docs, root)
    failures += [f"FAIL guard {failure}" for failure in unresolved]

    if failures:
        for failure in failures:
            print(failure)
        return 1

    print(
        "GUARDS OK "
        f"model_no_component=1 install_leaf_refs={leaf_count} "
        f"cycle={len(cycles)} unresolved={len(unresolved)}"
    )
    return 0


# --- README check ------------------------------------------------------------
def readme_sections(root):
    path = Path(root) / README_PATH
    if not path.exists():
        return None
    headings = [
        line.strip().lower()
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip().startswith("## ")
    ]
    haystack = " \n ".join(headings)
    return [
        name for name, keys in README_SECTIONS if any(key in haystack for key in keys)
    ]


def run_readme(root):
    found = readme_sections(root)
    if found is None:
        print(f"FAIL missing: {README_PATH}")
        return 1
    if len(found) == len(README_SECTIONS):
        print(f"README OK sections={len(README_SECTIONS)}")
        return 0
    missing = [name for name, _ in README_SECTIONS if name not in found]
    print(f"FAIL README sections={len(found)} missing={','.join(missing)}")
    return 1


# --- PR 5 gates --------------------------------------------------------------


def read_sections(path):
    """Split a markdown file into ``{lowercased `## ` heading: [body lines]}``."""
    sections = {}
    current = None
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith("## "):
            current = line[3:].strip().lower()
            sections[current] = []
        elif current is not None:
            sections[current].append(line)
    return sections


def _parse_table(lines):
    """Yield non-header, non-separator markdown table rows as cell lists."""
    rows = []
    for line in lines:
        stripped = line.strip()
        if not stripped.startswith("|"):
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if all(re.fullmatch(r":?-{2,}:?", cell) for cell in cells):
            continue
        if cells and cells[0].lower() in ("schema constraint", "constraint"):
            continue
        rows.append(cells)
    return rows


def run_trace(root):
    """Every non-OPEN constraint row MUST cite a real F1 anchor in spec/**.

    The matrix is the mitigation for R1 (schema/spec divergence). Rows without an
    anchor, or whose anchor names a ``spec/**.md`` file that does not exist, or
    that carries no section reference, count as untraced. A weakening constraint
    is rejected by review against the cited anchor; this check guarantees no row
    is left unanchored.
    """
    path = Path(root) / README_PATH
    if not path.exists():
        print(f"FAIL missing: {README_PATH}")
        return 1
    sections = read_sections(path)
    if TRACE_HEADING not in sections:
        print(f"FAIL trace: missing '{TRACE_HEADING}' section in {README_PATH}")
        return 1

    rows = _parse_table(sections[TRACE_HEADING])
    untraced = []
    for row in rows:
        if len(row) < 2 or not row[0]:
            untraced.append("(malformed row)")
            continue
        anchor = row[1]
        match = re.search(r"`(spec/[^`]+\.md)`", anchor)
        if not match or "§" not in anchor:
            untraced.append(row[0])
            continue
        if not (Path(root) / match.group(1)).exists():
            untraced.append(f"{row[0]} -> missing {match.group(1)}")

    if untraced:
        for item in untraced:
            print(f"FAIL trace: untraced constraint: {item}")
        print(f"FAIL TRACE rows={len(rows)} untraced={len(untraced)}")
        return 1

    print(f"TRACE OK rows={len(rows)} untraced=0")
    return 0


def run_adr(root):
    """ADR-0002 MUST follow docs/adr/0000-template.md (exactly seven sections)."""
    path = Path(root) / ADR_PATH
    if not path.exists():
        print(f"FAIL missing: {ADR_PATH}")
        return 1
    headings = [
        line[3:].strip().lower()
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.startswith("## ")
    ]
    missing = [name for name in ADR_HEADINGS if name not in headings]
    if missing or len(headings) != len(ADR_HEADINGS):
        print(
            f"FAIL ADR headings={len(headings)} "
            f"missing={','.join(missing) if missing else 'none'}"
        )
        return 1
    print(f"ADR OK headings={len(headings)}")
    return 0


def run_freeze(root):
    """Gate the freeze on risk R3 (Rust dialect support) being recorded.

    R3 is recorded in both the ADR and the README with a machine-readable state
    marker, and this is a real freeze gate: while the state is ``pending`` the
    freeze is blocked and the command FAILS (exit 1); only ``confirmed`` passes
    (``FREEZE OK r3=confirmed``, exit 0). Confirmation requires executed evidence
    (the Rust ``jsonschema`` engine reproducing the frozen corpus offline), not a
    declaration, so the gate refuses to pass while a blocker is open. The gate
    also fails closed if the two records disagree or a record is missing.
    """
    states = {}
    for rel in (ADR_PATH, README_PATH):
        path = Path(root) / rel
        if not path.exists():
            print(f"FAIL missing: {rel}")
            return 1
        match = R3_STATE_RE.search(path.read_text(encoding="utf-8"))
        if not match:
            print(f"FAIL freeze: r3 state not recorded in {rel}")
            return 1
        states[rel] = match.group(1).lower()

    unique = set(states.values())
    if len(unique) != 1:
        print(f"FAIL freeze: r3 state mismatch {states}")
        return 1
    state = unique.pop()
    if state == "confirmed":
        print("FREEZE OK r3=confirmed")
        return 0
    print("FREEZE BLOCKED r3=pending")
    return 1


def run_trust(root):
    """README MUST carry a trust statement and MUST NOT overclaim safety."""
    path = Path(root) / README_PATH
    if not path.exists():
        print(f"FAIL missing: {README_PATH}")
        return 1
    sections = read_sections(path)
    body = "\n".join(sections.get(TRUST_HEADING, [])).strip()
    if not body:
        print(f"FAIL trust: missing non-empty '{TRUST_HEADING}' section in {README_PATH}")
        return 1

    offenders = []
    for rel in (README_PATH, ADR_PATH):
        doc = Path(root) / rel
        if not doc.exists():
            continue
        low = doc.read_text(encoding="utf-8").lower()
        for claim in FORBIDDEN_CLAIMS:
            if claim in low:
                offenders.append(f"{rel}: {claim!r}")
    if offenders:
        for offender in offenders:
            print(f"FAIL trust: overclaim {offender}")
        return 1

    print("TRUST OK")
    return 0


# --- L0 run ------------------------------------------------------------------


def discover_docs(root):
    """Load every ``schemas/*.schema.json`` (roots) and ``schemas/defs/*.schema.json``."""
    docs = []
    roots = []
    defs = []
    for path in sorted((Path(root) / "schemas").glob("*.schema.json")):
        rel = path.relative_to(root).as_posix()
        docs.append((rel, json.loads(path.read_text(encoding="utf-8"))))
        roots.append(rel)
    for path in sorted((Path(root) / "schemas" / "defs").glob("*.schema.json")):
        rel = path.relative_to(root).as_posix()
        docs.append((rel, json.loads(path.read_text(encoding="utf-8"))))
        defs.append(rel)
    return docs, roots, defs


def run_l0(root):
    registry_file = Path(root) / REGISTRY_PATH
    if not registry_file.exists():
        print(f"FAIL missing: {REGISTRY_PATH}")
        return 1
    registry = json.loads(registry_file.read_text(encoding="utf-8"))
    base = registry.get("base", BASE_DEFAULT)
    known_ids = registry.get("ids", {}) or {}

    docs, roots, defs = discover_docs(root)
    if not docs:
        # The schema tree is empty: name the primary deliverable. Defs-only trees
        # are accepted (PR 2), but a wholly empty tree is RED by design.
        print(f"FAIL missing: {HARNESS_ROOT}")
        return 1

    failures = []
    failures += check_dialect(docs)
    failures += check_refs(base, known_ids, docs, root)
    failures += check_duplicates(docs, known_ids)
    failures += check_registry_paths(known_ids, root)
    failures += scan_vendor(docs)
    failures += check_open(docs)
    cycles = find_cycles(build_edges(docs))
    failures += [f"FAIL cycle: {cycle}" for cycle in cycles]

    found = readme_sections(root)
    if found is None:
        failures.append(f"FAIL missing: {README_PATH}")
    elif len(found) != len(README_SECTIONS):
        missing = [name for name, _ in README_SECTIONS if name not in found]
        failures.append(f"FAIL README sections={len(found)} missing={','.join(missing)}")

    if failures:
        for failure in failures:
            print(failure)
        return 1

    print(
        "OK "
        f"schemas={len(roots)} defs={len(defs)} refs={count_refs(docs)} "
        f"open={count_open(docs)} vendor=0 unresolved=0 cycle=0"
    )
    return 0


# --- Self-test ---------------------------------------------------------------


def run_self_test():
    """Each adversarial fixture MUST be detected by its dedicated check."""
    cases = [
        (
            "vendor-const",
            bool(scan_vendor([("fixture.json", {"const": "openai"})])),
        ),
        (
            "unresolved-ref",
            bool(
                check_refs(
                    BASE_DEFAULT,
                    {},
                    [("fixture.json", {"$ref": BASE_DEFAULT + "defs/missing.schema.json"})],
                    "nonexistent-selftest-root",
                )
            ),
        ),
        (
            "missing-open-marker",
            bool(
                check_open(
                    [
                        (
                            "model-contract.schema.json",
                            {"properties": {"input": {"type": "object"}}},
                        )
                    ]
                )
            ),
        ),
        (
            "forced-cycle",
            bool(find_cycles({"a": ["b"], "b": ["a"]})),
        ),
        (
            "duplicate-definition",
            bool(
                check_duplicates(
                    [
                        ("a.schema.json", {"$id": BASE_DEFAULT + "defs/x.schema.json"}),
                        ("b.schema.json", {"$id": BASE_DEFAULT + "defs/x.schema.json"}),
                    ],
                    {},
                )
            ),
        ),
    ]
    failed = [name for name, detected in cases if not detected]
    if failed:
        print(f"SELFTEST FAIL cases={len(cases)} failed={','.join(failed)}")
        return 1
    print(f"SELFTEST OK cases={len(cases)}")
    return 0


# --- CLI ---------------------------------------------------------------------


def main(argv=None):
    parser = argparse.ArgumentParser(description="L0 structural pre-check (F2)")
    parser.add_argument("--self-test", action="store_true", help="run adversarial fixtures")
    parser.add_argument("--readme", action="store_true", help="check README sections only")
    parser.add_argument(
        "--guards",
        action="store_true",
        help="assert the cycle/leaf structural guards (model-contract has no component edge; install-plan refs leaf defs only)",
    )
    parser.add_argument(
        "--trace",
        action="store_true",
        help="check the F1 traceability matrix in the README (PR 5)",
    )
    parser.add_argument(
        "--adr",
        action="store_true",
        help="check docs/adr/0002-schema-strategy.md against the ADR template (PR 5)",
    )
    parser.add_argument(
        "--freeze",
        action="store_true",
        help="R3 freeze gate: exit 1 while the dialect blocker is pending (PR 5)",
    )
    parser.add_argument(
        "--trust",
        action="store_true",
        help="check the README trust statement for overclaims (PR 5)",
    )
    args = parser.parse_args(argv)

    root = Path(__file__).resolve().parents[2]
    if args.self_test:
        return run_self_test()
    if args.readme:
        return run_readme(root)
    if args.guards:
        return run_guards(root)
    if args.trace:
        return run_trace(root)
    if args.adr:
        return run_adr(root)
    if args.freeze:
        return run_freeze(root)
    if args.trust:
        return run_trust(root)
    return run_l0(root)


if __name__ == "__main__":
    sys.exit(main())
