//! Projections that turn Aula's nested DTOs into small, LLM-friendly JSON.
//!
//! Aula's raw DTOs are large (`MessageThreadSubscription` alone has 28 fields,
//! most of them null or UI bookkeeping). Feeding those to a model burns context
//! on noise, so every tool projects through these helpers: plain text instead
//! of HTML, truncated bodies, and only the fields a question can be answered
//! from.

use serde::Serialize;
use serde_json::{Map, Value};

/// Collapse to a single line, strip HTML, and truncate to `max` characters.
pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let cut: String = s.chars().take(max).collect();
    format!("{}…", cut.trim_end())
}

/// Plain text from an HTML field (Aula sends rich text as `RichTextWrapperDto`).
pub fn text(html: Option<&str>, max: usize) -> Option<String> {
    let raw = aula_api::util::strip_html_tags(html?);
    let collapsed = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        None
    } else {
        Some(truncate(&collapsed, max))
    }
}

/// Serialize any DTO into a JSON value.
pub fn to_value<T: Serialize>(v: &T) -> Value {
    serde_json::to_value(v).unwrap_or(Value::Null)
}

/// First non-empty string among `keys`.
///
/// Used where a DTO's exact field name differs per domain (`displayName` vs
/// `fullName` vs `name`) and mirroring each struct would be pure boilerplate.
pub fn str_field(v: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| {
        v.get(*k)
            .and_then(|x| x.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
    })
}

/// Best-effort human name for a profile-ish DTO.
pub fn name_of<T: Serialize>(v: &T) -> Option<String> {
    str_field(
        &to_value(v),
        &["displayName", "fullName", "name", "shortName", "title"],
    )
}

/// Keep only the given keys, dropping nulls.
pub fn pick(v: &Value, keys: &[&str]) -> Value {
    let mut out = Map::new();
    if let Some(obj) = v.as_object() {
        for k in keys {
            if let Some(val) = obj.get(*k) {
                if !val.is_null() {
                    out.insert((*k).to_string(), val.clone());
                }
            }
        }
    }
    Value::Object(out)
}

/// Recursively drop null object members.
///
/// Aula fills most DTO fields with null, and a model reading the output learns
/// nothing from `"location": null`. Applied once, centrally, in the tool
/// dispatcher.
pub fn non_null(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k, non_null(v)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(non_null).collect()),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn text_strips_html_and_collapses_whitespace() {
        assert_eq!(
            text(Some("<p>Hej   med\n dig</p>"), 100),
            Some("Hej med dig".to_string())
        );
    }

    #[test]
    fn text_is_none_for_empty_markup() {
        assert_eq!(text(Some("<p></p>"), 100), None);
        assert_eq!(text(None, 100), None);
    }

    #[test]
    fn truncate_marks_the_cut() {
        assert_eq!(truncate("abcdef", 3), "abc…");
        assert_eq!(truncate("abc", 3), "abc");
    }

    #[test]
    fn name_of_tries_common_spellings() {
        assert_eq!(
            name_of(&json!({"fullName": "Ada Testesen"})),
            Some("Ada Testesen".into())
        );
        assert_eq!(
            name_of(&json!({"displayName": "Testskolen"})),
            Some("Testskolen".into())
        );
        assert_eq!(name_of(&json!({"nope": 1})), None);
    }

    #[test]
    fn pick_drops_missing_and_null_keys() {
        let v = json!({"id": 1, "title": null, "extra": "x"});
        assert_eq!(pick(&v, &["id", "title", "nope"]), json!({"id": 1}));
    }

    #[test]
    fn non_null_strips_nulls_at_every_depth() {
        let v = json!({"a": null, "b": {"c": null, "d": 2}, "e": [{"f": null}, 3]});
        assert_eq!(non_null(v), json!({"b": {"d": 2}, "e": [{}, 3]}));
    }
}
