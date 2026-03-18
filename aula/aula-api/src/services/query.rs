//! Query-string building helpers for service modules.
//!
//! All query parameter values MUST be percent-encoded to avoid breaking URLs
//! when values contain `&`, `=`, `+`, spaces, or other reserved characters.

use url::form_urlencoded;

/// Percent-encode a single query parameter value for safe inclusion in a URL.
///
/// Uses `application/x-www-form-urlencoded` encoding (spaces become `+`,
/// reserved characters are `%`-escaped).
///
/// # Examples
///
/// ```
/// # use aula_api::services::query::encode_value;
/// assert_eq!(encode_value("hello world"), "hello+world");
/// assert_eq!(encode_value("a&b=c"), "a%26b%3Dc");
/// assert_eq!(encode_value("42"), "42"); // safe chars pass through
/// ```
pub fn encode_value(value: &str) -> String {
    form_urlencoded::byte_serialize(value.as_bytes()).collect()
}

/// Format a single `key=encoded_value` query parameter pair.
pub fn param(key: &str, value: &str) -> String {
    format!("{key}={}", encode_value(value))
}

/// Format a single `key=value` query parameter pair for numeric values
/// (no encoding needed).
pub fn param_num<N: std::fmt::Display>(key: &str, value: N) -> String {
    format!("{key}={value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_value_passthrough_safe_chars() {
        assert_eq!(encode_value("2024-03-01"), "2024-03-01");
        assert_eq!(encode_value("42"), "42");
    }

    #[test]
    fn encode_value_encodes_special_chars() {
        assert_eq!(encode_value("a&b"), "a%26b");
        assert_eq!(encode_value("hello world"), "hello+world");
        assert_eq!(encode_value("key=val"), "key%3Dval");
    }

    #[test]
    fn param_encodes_value() {
        assert_eq!(param("q", "foo bar"), "q=foo+bar");
    }

    #[test]
    fn param_num_no_encoding() {
        assert_eq!(param_num("id", 42), "id=42");
    }
}
