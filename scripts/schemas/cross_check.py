#!/usr/bin/env python3
"""L2 independent cross-check for the harness schema set (F2 / PR 5).

Runs the SAME NON-NORMATIVE corpus as the L1 ``ajv`` runner under an independent
engine: Python ``jsonschema`` + the ``referencing`` ``Registry``. Two independent
2020-12 implementations agreeing on the same corpus is the real proof that the
features the schema set depends on (``unevaluatedProperties`` under composition,
absolute ``$id`` resolution) are portable. It is the interim mitigation for risk
R3 while the Rust validator (F2-05) is not present on this machine.

Schemas are resolved from the checked-in resolver map (``schemas/registry.json``),
so the cross-check runs offline and resolves the same ``$id`` to the same
definition as every other consumer.

Usage::

    python scripts/schemas/cross_check.py

Prints the same ``PASS schemas=<n> positive=<p> negative=<n>`` line as L1 and
exits 0 when the corpus passes; exits 1 on the first divergence.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import jsonschema
import yaml
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012

ROOT = Path(__file__).resolve().parents[2]
REGISTRY_REL = "schemas/registry.json"

_CHANGE_NAME = "harness-schema-v1alpha1"

# The live corpus home is the promoted fixtures directory. The archived change
# folder is retained as an audit-only fallback; the live change folder is the
# pre-archive fallback.
_FIXTURES_REL = "packages/validator/tests/fixtures"


def _find_corpus() -> tuple[Path, str]:
    """Return ``(instance_base, corpus_rel)`` for the live corpus.

    Prefers the promoted fixtures home, then falls back to the archived change
    (audit) and the live change folder. Hardcoding a path silently broke this
    gate when the change was archived, so resolution stays dynamic.
    """
    promoted = ROOT / _FIXTURES_REL
    if (promoted / "corpus.json").exists():
        return promoted, (promoted / "corpus.json").relative_to(ROOT).as_posix()
    archive_root = ROOT / "openspec" / "changes" / "archive"
    if archive_root.is_dir():
        for entry in sorted(archive_root.iterdir()):
            if entry.is_dir() and entry.name.endswith(f"-{_CHANGE_NAME}"):
                if (entry / "examples" / "corpus.json").exists():
                    return entry, (entry / "examples" / "corpus.json").relative_to(ROOT).as_posix()
    live = ROOT / "openspec" / "changes" / _CHANGE_NAME
    if (live / "examples" / "corpus.json").exists():
        return live, (live / "examples" / "corpus.json").relative_to(ROOT).as_posix()
    return promoted, (promoted / "corpus.json").relative_to(ROOT).as_posix()


CHANGE_DIR, CORPUS_REL = _find_corpus()


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def load_instance(path: Path):
    """Load a corpus instance as its parsed JSON data model (JSON or YAML)."""
    text = path.read_text(encoding="utf-8")
    if path.suffix == ".json":
        return json.loads(text)
    return yaml.safe_load(text)


def resolve_instance(file: str) -> Path:
    """Resolve a corpus file relative to the repo root or the change folder."""
    from_root = ROOT / file
    if from_root.exists():
        return from_root
    return CHANGE_DIR / file


def json_pointer(error) -> str:
    """Render a jsonschema error's location as the JSON Pointer L1 reports."""
    parts = list(error.absolute_path)
    if not parts:
        return ""
    return "/" + "/".join(str(part) for part in parts)


def build_registry(ids: dict):
    """Build a ``referencing`` registry from the ``$id`` -> repo path map."""
    registry = Registry()
    contents_by_id = {}
    for uri, rel in ids.items():
        path = ROOT / rel
        if not path.exists():
            print(f"FAIL registry path missing: {rel}")
            raise SystemExit(1)
        contents = load_json(path)
        contents_by_id[uri] = contents
        resource = Resource.from_contents(contents, default_specification=DRAFT202012)
        registry = registry.with_resource(uri, resource)
    return registry, contents_by_id


def main() -> int:
    registry_meta = load_json(ROOT / REGISTRY_REL)
    if not isinstance(registry_meta, dict) or not registry_meta.get("ids"):
        print(f"FAIL missing: {REGISTRY_REL}")
        return 1
    registry, contents_by_id = build_registry(registry_meta["ids"])

    corpus = load_json(ROOT / CORPUS_REL)
    positive = corpus.get("positive") or []
    negative = corpus.get("negative") or []
    if not positive:
        print("FAIL corpus empty: positive=0")
        return 1

    schemas_used = set()

    def validator_for(entry):
        schema_id = entry.get("schema")
        if not isinstance(schema_id, str):
            print(f"FAIL {entry.get('file')} corpus entry missing schema $id")
            raise SystemExit(1)
        if schema_id not in contents_by_id:
            print(f"FAIL schema not registered: {schema_id}")
            raise SystemExit(1)
        schemas_used.add(schema_id)
        return jsonschema.Draft202012Validator(
            contents_by_id[schema_id], registry=registry
        )

    passed = 0
    for entry in positive:
        validator = validator_for(entry)
        instance = load_instance(resolve_instance(entry["file"]))
        errors = sorted(validator.iter_errors(instance), key=lambda e: list(e.path))
        if errors:
            first = errors[0]
            print(
                f"FAIL {entry['file']} positive did not validate: "
                f"#{json_pointer(first)} keyword={first.validator} "
                f"message={first.message}"
            )
            return 1
        passed += 1

    failed = 0
    for entry in negative:
        validator = validator_for(entry)
        instance = load_instance(resolve_instance(entry["file"]))
        errors = sorted(validator.iter_errors(instance), key=lambda e: list(e.path))
        if not errors:
            print(f"FAIL {entry['file']} expected failure but instance validated")
            return 1
        expect = entry.get("expect") or {}

        def matches(error):
            if "path" in expect and json_pointer(error) != expect["path"]:
                return False
            if "keyword" in expect and error.validator != expect["keyword"]:
                return False
            return True

        if not any(matches(error) for error in errors):
            first = errors[0]
            print(
                f"FAIL {entry['file']} #{json_pointer(first)} "
                f"keyword={first.validator} expected={json.dumps(expect)}"
            )
            return 1
        failed += 1

    print(
        f"PASS schemas={len(schemas_used)} positive={passed} negative={failed}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
