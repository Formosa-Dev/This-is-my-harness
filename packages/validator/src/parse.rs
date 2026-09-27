//! Parsing: bytes -> `serde_json::Value` (design §8, spec Q7).
//!
//! * **JSON** via `serde_json`, with a recursive visitor that makes duplicate
//!   object keys fail closed (`parse.duplicate_key`).
//! * **YAML 1.2** via `serde-saphyr`, duplicate mapping keys fail closed by
//!   policy, a multi-document stream is rejected (`parse.multiple_documents`),
//!   aliases are bounded, and implicit typing is preserved (no coercion).
//! * A UTF-8 BOM is accepted **only** at the start of the stream.
//!
//! Nothing here mutates state and nothing reaches the network.

use std::collections::BTreeMap;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde_json::{Number, Value};

use crate::diagnostics::{Code, Diagnostic};

/// A UTF-8 byte-order mark.
const BOM: &[u8] = &[0xEF, 0xBB, 0xBF];

/// Sentinel embedded in the `serde` error for a duplicate JSON key. It is only
/// ever emitted by [`NodeVisitor`], so matching on it is unambiguous; the
/// alternative (a bespoke error type threaded through `serde_json`) buys no
/// extra safety.
const DUPLICATE_KEY_MARKER: &str = "thisismyharness::parse::duplicate_key";

/// Input encoding of a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Yaml,
}

impl Format {
    /// Pick a format from a filename, defaulting to YAML (the manifest default).
    #[must_use]
    pub fn from_filename(name: &str) -> Self {
        if name.to_ascii_lowercase().ends_with(".json") {
            Format::Json
        } else {
            Format::Yaml
        }
    }
}

/// Parse `bytes` according to `format`.
///
/// # Errors
///
/// Returns a `parse.*` diagnostic: `parse.invalid` for malformed or non-UTF-8
/// input, `parse.duplicate_key` for a duplicate mapping key, and
/// `parse.multiple_documents` for a multi-document YAML stream.
pub fn parse(bytes: &[u8], format: Format) -> Result<Value, Diagnostic> {
    match format {
        Format::Json => parse_json(bytes),
        Format::Yaml => parse_yaml(bytes),
    }
}

fn strip_bom(bytes: &[u8]) -> &[u8] {
    match bytes.strip_prefix(BOM) {
        Some(rest) => rest,
        None => bytes,
    }
}

fn decode(bytes: &[u8]) -> Result<&str, Diagnostic> {
    std::str::from_utf8(bytes)
        .map_err(|_| Diagnostic::error("", Code::parse_invalid(), "input is not valid UTF-8"))
}

/// Parse a JSON document; duplicate object keys fail closed.
///
/// # Errors
///
/// Returns a `parse.*` diagnostic.
pub fn parse_json(bytes: &[u8]) -> Result<Value, Diagnostic> {
    let text = decode(strip_bom(bytes))?;
    let mut deserializer = serde_json::Deserializer::from_str(text);
    let node = Node::deserialize(&mut deserializer).map_err(json_error)?;
    deserializer.end().map_err(json_error)?;
    Ok(node.into_value())
}

fn json_error(error: serde_json::Error) -> Diagnostic {
    let message = error.to_string();
    if message.contains(DUPLICATE_KEY_MARKER) {
        Diagnostic::error(
            "",
            Code::parse_duplicate_key(),
            "duplicate object key in JSON input",
        )
    } else {
        Diagnostic::error(
            "",
            Code::parse_invalid(),
            format!("invalid JSON: {message}"),
        )
    }
}

/// Parse a single YAML 1.2 document.
///
/// # Errors
///
/// Returns a `parse.*` diagnostic.
pub fn parse_yaml(bytes: &[u8]) -> Result<Value, Diagnostic> {
    let text = decode(strip_bom(bytes))?;
    let mut options = serde_saphyr::Options::default();
    // YAML 1.2 core schema: only `true`/`false` are booleans (`yes`/`no` are
    // strings in 1.2). Fail closed on duplicates (the default, set explicitly).
    options.strict_booleans = true;
    options.duplicate_keys = serde_saphyr::DuplicateKeyPolicy::Error;
    // Keep the diagnostic message compact (no rustc-like code frame); the
    // machine contract is `{path, code}`, and `message` is non-normative.
    options.with_snippet = false;
    serde_saphyr::from_str_with_options::<Value>(text, options).map_err(yaml_error)
}

fn yaml_error(error: serde_saphyr::Error) -> Diagnostic {
    // Variant matching must look through the `WithSnippet` wrapper; the rendered
    // message is still taken from the original error.
    match error.without_snippet() {
        serde_saphyr::Error::DuplicateMappingKey { .. } => Diagnostic::error(
            "",
            Code::parse_duplicate_key(),
            "duplicate mapping key in YAML input",
        ),
        serde_saphyr::Error::MultipleDocuments { .. } => Diagnostic::error(
            "",
            Code::parse_multiple_documents(),
            "multiple YAML documents are not supported",
        ),
        _ => Diagnostic::error("", Code::parse_invalid(), format!("invalid YAML: {error}")),
    }
}

/// A recursive JSON value that detects duplicate object keys while deserializing.
enum Node {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Node>),
    Object(BTreeMap<String, Node>),
}

impl Node {
    fn into_value(self) -> Value {
        match self {
            Node::Null => Value::Null,
            Node::Bool(value) => Value::Bool(value),
            Node::Number(value) => Value::Number(value),
            Node::String(value) => Value::String(value),
            Node::Array(items) => Value::Array(items.into_iter().map(Node::into_value).collect()),
            Node::Object(entries) => Value::Object(
                entries
                    .into_iter()
                    .map(|(key, value)| (key, value.into_value()))
                    .collect(),
            ),
        }
    }
}

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NodeVisitor)
    }
}

struct NodeVisitor;

impl<'de> Visitor<'de> for NodeVisitor {
    type Value = Node;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any JSON value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Node, E> {
        Ok(Node::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Node, E> {
        Ok(Node::Number(Number::from(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Node, E> {
        Ok(Node::Number(Number::from(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Node, E>
    where
        E: de::Error,
    {
        Number::from_f64(value)
            .map(Node::Number)
            .ok_or_else(|| E::custom("JSON numbers must be finite"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Node, E> {
        Ok(Node::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Node, E> {
        Ok(Node::String(value))
    }

    fn visit_unit<E>(self) -> Result<Node, E> {
        Ok(Node::Null)
    }

    fn visit_none<E>(self) -> Result<Node, E> {
        Ok(Node::Null)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Node, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(item) = sequence.next_element::<Node>()? {
            items.push(item);
        }
        Ok(Node::Array(items))
    }

    fn visit_map<A>(self, mut mapping: A) -> Result<Node, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut entries = BTreeMap::new();
        while let Some(key) = mapping.next_key::<String>()? {
            if entries.contains_key(&key) {
                return Err(de::Error::custom(DUPLICATE_KEY_MARKER));
            }
            let value = mapping.next_value::<Node>()?;
            entries.insert(key, value);
        }
        Ok(Node::Object(entries))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_parses_a_simple_document() {
        let value = parse_json(br#"{"a": 1, "b": [true, null, "x"]}"#).expect("valid JSON");
        assert_eq!(value["a"], serde_json::json!(1));
        assert_eq!(value["b"][0], serde_json::json!(true));
    }

    #[test]
    fn json_duplicate_key_fails_closed() {
        let diagnostic = parse_json(br#"{"a": 1, "a": 2}"#).expect_err("duplicate must fail");
        assert_eq!(diagnostic.code.as_str(), "parse.duplicate_key");
    }

    #[test]
    fn json_duplicate_key_is_detected_in_nested_objects() {
        let diagnostic =
            parse_json(br#"{"outer": {"a": 1, "a": 2}}"#).expect_err("duplicate must fail");
        assert_eq!(diagnostic.code.as_str(), "parse.duplicate_key");
    }

    #[test]
    fn json_rejects_trailing_content() {
        let diagnostic = parse_json(br#"{"a": 1} {"b": 2}"#).expect_err("trailing must fail");
        assert_eq!(diagnostic.code.as_str(), "parse.invalid");
    }

    #[test]
    fn yaml_duplicate_key_fails_closed() {
        let diagnostic = parse_yaml(b"a: 1\na: 2\n").expect_err("duplicate must fail");
        assert_eq!(diagnostic.code.as_str(), "parse.duplicate_key");
    }

    #[test]
    fn yaml_multiple_documents_are_rejected() {
        let diagnostic = parse_yaml(b"---\na: 1\n---\nb: 2\n").expect_err("multi-doc must fail");
        assert_eq!(diagnostic.code.as_str(), "parse.multiple_documents");
    }

    #[test]
    fn yaml_implicit_typing_is_preserved_not_coerced() {
        // YAML 1.2: `1.0` is a number. A schema expecting a string must see a
        // number and reject it — the parser must not coerce it to "1.0".
        let value = parse_yaml(b"version: 1.0\n").expect("valid YAML");
        assert!(value["version"].is_number(), "got {value:?}");
    }

    #[test]
    fn yaml_strict_booleans_treats_yes_as_a_string() {
        let value = parse_yaml(b"flag: yes\n").expect("valid YAML");
        assert_eq!(value["flag"], serde_json::json!("yes"));
    }

    #[test]
    fn bom_is_accepted_at_stream_start_only() {
        let mut bytes = BOM.to_vec();
        bytes.extend_from_slice(b"{}");
        assert!(parse_json(&bytes).is_ok());

        // A BOM after other bytes is not a stream-start BOM and must fail.
        let mid = b"{}\xEF\xBB\xBF";
        assert!(parse_json(mid).is_err());
    }
}
