//! Shared serde helper functions.
//!
//! The Aula API (backed by .NET / Newtonsoft.Json) is inconsistent about
//! whether ID fields are serialised as JSON strings or numbers.  C# clients
//! silently coerce either form, but Rust `serde` is strict.  The helpers
//! below let us accept both representations.

use serde::Deserialize;

/// Deserialize an `Option<String>` that may arrive as a JSON string **or**
/// number (integer / float).
///
/// Apply via:
/// ```ignore
/// #[serde(default, deserialize_with = "crate::serde_helpers::deserialize_optional_string_from_any")]
/// pub some_id: Option<String>,
/// ```
pub fn deserialize_optional_string_from_any<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match v {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::String(s)) => Ok(Some(s)),
        Some(serde_json::Value::Number(n)) => Ok(Some(n.to_string())),
        Some(other) => Ok(Some(other.to_string())),
    }
}
