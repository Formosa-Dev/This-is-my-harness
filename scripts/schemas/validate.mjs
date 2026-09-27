#!/usr/bin/env node
// L1 schema validation runner (ajv, JSON Schema Draft 2020-12).
//
// Corpus-driven: reads the NON-NORMATIVE corpus under the change folder and
// validates every positive instance (MUST pass) and every negative instance
// (MUST fail with the expected {path, keyword}). Schemas are resolved from the
// checked-in resolver map so validation runs offline.
//
// Usage: npm run validate:schemas
// Exit code is 0 when the corpus passes and 1 on the first failure.

import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import Ajv2020 from "ajv/dist/2020.js";
import addFormats from "ajv-formats";
import YAML from "yaml";

const HERE = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(HERE, "..", "..");
// The corpus folder starts live under `changes/` and is later moved under
// `changes/archive/`. Resolve it dynamically so this gate keeps working after
// the change is archived (hardcoding the path silently broke the gate once).
const CHANGE_NAME = "harness-schema-v1alpha1";
function findChangeDir() {
  const live = ["openspec", "changes", CHANGE_NAME];
  if (existsSync(join(ROOT, ...live, "examples", "corpus.json"))) return live.join("/");
  const archiveRoot = join(ROOT, "openspec", "changes", "archive");
  if (existsSync(archiveRoot)) {
    for (const entry of readdirSync(archiveRoot)) {
      if (entry.endsWith(`-${CHANGE_NAME}`)) {
        const rel = ["openspec", "changes", "archive", entry];
        if (existsSync(join(ROOT, ...rel, "examples", "corpus.json"))) return rel.join("/");
      }
    }
  }
  return live.join("/");
}
const CHANGE = findChangeDir();
const CORPUS_REL = `${CHANGE}/examples/corpus.json`;
const REGISTRY_REL = "schemas/registry.json";

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8"));
}

function loadInstance(path) {
  const text = readFileSync(path, "utf8");
  return path.endsWith(".json") ? JSON.parse(text) : YAML.parse(text);
}

function resolveInstance(file) {
  const fromRoot = join(ROOT, file);
  if (existsSync(fromRoot)) return fromRoot;
  const fromChange = join(ROOT, CHANGE, file);
  if (existsSync(fromChange)) return fromChange;
  return fromRoot;
}

function main() {
  const registryPath = join(ROOT, REGISTRY_REL);
  if (!existsSync(registryPath)) {
    console.log(`FAIL missing: ${REGISTRY_REL}`);
    process.exit(1);
  }
  const corpusPath = join(ROOT, CORPUS_REL);
  if (!existsSync(corpusPath)) {
    console.log(`FAIL missing: ${CORPUS_REL}`);
    process.exit(1);
  }

  const registry = readJson(registryPath);
  const corpus = readJson(corpusPath);
  const positive = Array.isArray(corpus.positive) ? corpus.positive : [];
  const negative = Array.isArray(corpus.negative) ? corpus.negative : [];

  if (positive.length === 0) {
    console.log("FAIL corpus empty: positive=0");
    process.exit(1);
  }

  const ajv = new Ajv2020({ allErrors: true, strict: false });
  addFormats(ajv);

  for (const [id, path] of Object.entries(registry.ids || {})) {
    const schemaPath = join(ROOT, path);
    if (!existsSync(schemaPath)) {
      console.log(`FAIL registry path missing: ${path}`);
      process.exit(1);
    }
    ajv.addSchema(readJson(schemaPath), id);
  }

  const schemasUsed = new Set();

  const compile = (schemaId, file) => {
    if (typeof schemaId !== "string") {
      console.log(`FAIL ${file} corpus entry missing schema $id`);
      process.exit(1);
    }
    const validate = ajv.getSchema(schemaId);
    if (!validate) {
      console.log(`FAIL schema not registered: ${schemaId}`);
      process.exit(1);
    }
    schemasUsed.add(schemaId);
    return validate;
  };

  let passed = 0;
  for (const entry of positive) {
    const validate = compile(entry.schema, entry.file);
    const instance = loadInstance(resolveInstance(entry.file));
    if (!validate(instance)) {
      console.log(
        `FAIL ${entry.file} positive did not validate: ${ajv.errorsText(validate.errors)}`,
      );
      process.exit(1);
    }
    passed += 1;
  }

  let failed = 0;
  for (const entry of negative) {
    const validate = compile(entry.schema, entry.file);
    const instance = loadInstance(resolveInstance(entry.file));
    if (validate(instance)) {
      console.log(`FAIL ${entry.file} expected failure but instance validated`);
      process.exit(1);
    }
    const expect = entry.expect || {};
    const errors = validate.errors || [];
    const matches = errors.some(
      (error) =>
        (expect.path === undefined || error.instancePath === expect.path) &&
        (expect.keyword === undefined || error.keyword === expect.keyword),
    );
    if (!matches) {
      const first = errors[0] || {};
      console.log(
        `FAIL ${entry.file} #${first.instancePath || ""} keyword=${first.keyword} expected=${JSON.stringify(expect)}`,
      );
      process.exit(1);
    }
    failed += 1;
  }

  console.log(
    `PASS schemas=${schemasUsed.size} positive=${passed} negative=${failed}`,
  );
  process.exit(0);
}

main();
