//! Small text helpers shared by the CLI and the MCP server.

/// Strip HTML tags and decode the handful of entities Aula emits.
///
/// Aula message bodies and post content arrive as HTML (`RichTextWrapperDto`).
/// Both the CLI table output and the MCP server want plain text, so the
/// stripper lives here rather than in either binary.
pub fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_tags_and_decodes_entities() {
        assert_eq!(
            strip_html_tags("<p>Hej &amp; velkommen</p>"),
            "Hej & velkommen"
        );
    }

    #[test]
    fn leaves_plain_text_alone() {
        assert_eq!(strip_html_tags("no tags"), "no tags");
    }
}
