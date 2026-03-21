//! Filename sanitization for filesystem safety.
//!
//! Ensures names are safe for ext4/FUSE (no illegal chars, length limits,
//! human-readable). Pure functions, no state.

/// Maximum byte length for sanitized names (well under 255-byte ext4 limit).
const MAX_NAME_BYTES: usize = 200;

/// Sanitize a string for use as a filesystem name.
///
/// Strategy:
/// 1. Replace `/` with `-`
/// 2. Remove NUL bytes
/// 3. Remove control characters (0x00-0x1F, 0x7F)
/// 4. Replace `<>:"|?*\` with `_`
/// 5. Collapse consecutive whitespace to single space
/// 6. Trim leading/trailing whitespace and dots
/// 7. If empty after sanitization, use `_unnamed`
/// 8. Truncate to [`MAX_NAME_BYTES`] on a UTF-8 char boundary
pub fn sanitize_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len());

    for ch in name.chars() {
        match ch {
            '/' => result.push('-'),
            '\0' => {}
            // Remove control chars but preserve whitespace (tab, newline, etc.)
            c if c.is_control() && !c.is_whitespace() => {}
            '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\\' => result.push('_'),
            _ => result.push(ch),
        }
    }

    // Collapse consecutive whitespace to single space.
    let collapsed = collapse_whitespace(&result);

    // Trim leading/trailing whitespace and dots.
    let trimmed = collapsed.trim().trim_matches('.').trim();

    let trimmed = if trimmed.is_empty() {
        "_unnamed".to_string()
    } else {
        trimmed.to_string()
    };

    // Truncate to MAX_NAME_BYTES on a UTF-8 char boundary.
    truncate_utf8(&trimmed, MAX_NAME_BYTES)
}

/// Build a directory name from an ID and a title.
///
/// Format: `{id}-{sanitized_title}`.
/// The ID prefix guarantees uniqueness within a directory.
pub fn dir_name(id: i64, title: &str) -> String {
    let sanitized = sanitize_name(title);
    format!("{}-{}", id, sanitized)
}

/// Build a directory name from a string ID and a title.
#[allow(dead_code)]
pub fn dir_name_str(id: &str, title: &str) -> String {
    let sanitized = sanitize_name(title);
    let safe_id = sanitize_name(id);
    format!("{}-{}", safe_id, sanitized)
}

fn collapse_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_was_space = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
            }
            prev_was_space = true;
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }
    result
}

fn truncate_utf8(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_string();
    }
    // Find the last char boundary at or before max_bytes.
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_sanitization() {
        assert_eq!(sanitize_name("hello world"), "hello world");
    }

    #[test]
    fn slashes_become_dashes() {
        assert_eq!(sanitize_name("path/to/file"), "path-to-file");
    }

    #[test]
    fn nul_removed() {
        assert_eq!(sanitize_name("hello\0world"), "helloworld");
    }

    #[test]
    fn control_chars_removed() {
        assert_eq!(sanitize_name("hello\x01\x1fworld"), "helloworld");
    }

    #[test]
    fn special_chars_replaced() {
        assert_eq!(sanitize_name("a<b>c:d\"e|f?g*h\\i"), "a_b_c_d_e_f_g_h_i");
    }

    #[test]
    fn whitespace_collapsed() {
        assert_eq!(sanitize_name("hello   world\t\nfoo"), "hello world foo");
    }

    #[test]
    fn leading_trailing_trimmed() {
        assert_eq!(sanitize_name("  hello  "), "hello");
        assert_eq!(sanitize_name("...hello..."), "hello");
        assert_eq!(sanitize_name("  ...hello...  "), "hello");
    }

    #[test]
    fn empty_becomes_unnamed() {
        assert_eq!(sanitize_name(""), "_unnamed");
        assert_eq!(sanitize_name("   "), "_unnamed");
        assert_eq!(sanitize_name("..."), "_unnamed");
        assert_eq!(sanitize_name("\0\x01"), "_unnamed");
    }

    #[test]
    fn truncation_respects_utf8() {
        // 201 ASCII chars should be truncated to 200.
        let long = "a".repeat(201);
        let result = sanitize_name(&long);
        assert_eq!(result.len(), 200);

        // Multi-byte chars: each is 2 bytes. 101 chars = 202 bytes.
        // Should truncate to 100 chars = 200 bytes.
        let long_mb: String = std::iter::repeat('\u{00E6}').take(101).collect(); // 'ae' ligature
        let result = sanitize_name(&long_mb);
        assert!(result.len() <= MAX_NAME_BYTES);
        assert!(result.is_char_boundary(result.len()));
    }

    #[test]
    fn dir_name_format() {
        assert_eq!(dir_name(42, "Tur til Tivoli"), "42-Tur til Tivoli");
    }

    #[test]
    fn dir_name_sanitizes_title() {
        assert_eq!(dir_name(1, "Bad/Name"), "1-Bad-Name");
    }

    #[test]
    fn dir_name_str_format() {
        assert_eq!(dir_name_str("abc", "title"), "abc-title");
    }

    #[test]
    fn danish_chars_preserved() {
        assert_eq!(
            sanitize_name("Foraeldremoedet i 3.A"),
            "Foraeldremoedet i 3.A"
        );
        assert_eq!(sanitize_name("aeoeaa"), "aeoeaa");
    }

    #[test]
    fn real_world_post_title() {
        assert_eq!(
            sanitize_name("Vi skal paa tur! (3.A)"),
            "Vi skal paa tur! (3.A)"
        );
    }
}
