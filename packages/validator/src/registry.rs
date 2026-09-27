//! Embedded schema registry (design §3).
//!
//! The `$id` -> schema bodies table is generated at compile time by `build.rs`
//! from `schemas/registry.json` and `schemas/**` (`include_str!`). One
//! `jsonschema::Registry` is built lazily **once** (`OnceLock`), then every
//! document is validated from a per-document wrapper `{ "$ref": $id }` built
//! with `draft202012` options, `with_registry`, and `.offline()`.
//!
//! **Offline guarantee:** the retriever is the offline one, so a `$ref` whose
//! `$id` is absent from the registry cannot be fetched — it fails loudly at
//! build-of-validator time (the caller maps that to `schema.unresolved_ref`).

use std::fmt;
use std::sync::OnceLock;

use jsonschema::{Registry, RegistryBuilder};
use serde_json::Value;

include!(concat!(env!("OUT_DIR"), "/schema_table.rs"));

/// A failure to build the embedded registry.
#[derive(Debug, Clone)]
pub struct RegistryError {
    message: String,
}

impl RegistryError {
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for RegistryError {}

static REGISTRY: OnceLock<Result<Registry<'static>, String>> = OnceLock::new();

fn build() -> Result<Registry<'static>, String> {
    let mut builder: RegistryBuilder<'static> = Registry::new();
    for &(id, json) in SCHEMA_TABLE {
        let value: Value = serde_json::from_str(json)
            .map_err(|e| format!("embedded schema `{id}` is not valid JSON: {e}"))?;
        builder = builder
            .add(id, value)
            .map_err(|e| format!("embedded registry entry `{id}` is not a valid URI: {e}"))?;
    }
    builder
        .prepare()
        .map_err(|e| format!("embedded schema registry failed to prepare: {e}"))
}

/// The embedded registry, built once and reused.
///
/// # Errors
///
/// Fails loudly if the embedded table is inconsistent (invalid JSON, invalid
/// `$id`, or an unresolvable `$ref`); it never falls back to an empty registry.
pub fn registry() -> Result<&'static Registry<'static>, RegistryError> {
    match REGISTRY.get_or_init(build) {
        Ok(registry) => Ok(registry),
        Err(message) => Err(RegistryError {
            message: message.clone(),
        }),
    }
}

/// Number of `$id`s embedded from `schemas/registry.json`.
#[must_use]
pub fn schema_count() -> usize {
    SCHEMA_TABLE.len()
}

/// The embedded `$id`s, in deterministic (sorted) order.
pub fn schema_ids() -> impl Iterator<Item = &'static str> {
    SCHEMA_TABLE.iter().map(|&(id, _)| id)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HARNESS_ID: &str = "https://thisismyharness.dev/schemas/v1alpha1/harness.schema.json";

    #[test]
    fn embedded_table_is_complete() {
        assert_eq!(
            schema_count(),
            10,
            "schemas/registry.json must embed 10 $ids"
        );
        assert!(schema_ids().any(|id| id == HARNESS_ID));
    }

    #[test]
    fn resolves_an_embedded_ref_offline() {
        // No network is reachable from this test; resolution comes only from the
        // embedded registry. `metadata.name`/`version` go through absolute `$ref`s
        // into `defs/identity.schema.json`.
        let instance = serde_json::json!({
            "apiVersion": "thisismyharness.dev/v1alpha1",
            "kind": "Harness",
            "metadata": { "name": "demo-harness", "version": "1.0.0" },
            "spec": {}
        });
        let diagnostics = crate::structural::validate(HARNESS_ID, &instance)
            .expect("the embedded registry must resolve every $ref offline");
        assert!(diagnostics.is_empty(), "unexpected: {diagnostics:?}");
    }
}
