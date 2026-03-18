//! Shared output formatting for aula-cli.
//!
//! Provides three output modes:
//!   1. **JSON** -- raw `serde_json::to_string_pretty` for piping/scripting.
//!   2. **Table** -- columnar output with headers, alignment, truncation.
//!   3. **Human-readable** -- prose-like detail views for `show`/`read` commands.
//!
//! Terminal colors respect the `NO_COLOR` environment variable (https://no-color.org/).

use serde::Serialize;
use std::io::IsTerminal;
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Color support
// ---------------------------------------------------------------------------

/// Cached result of the color-support check.
///
/// The answer cannot change during a process lifetime, so we compute it once.
static COLORS_ENABLED: OnceLock<bool> = OnceLock::new();

/// Returns `true` when the terminal supports color output.
///
/// Color is disabled when:
///   - `NO_COLOR` env var is set (any value), per <https://no-color.org/>
///   - stdout is not a terminal (piped)
///
/// The result is cached after the first call.
pub fn colors_enabled() -> bool {
    *COLORS_ENABLED.get_or_init(|| {
        if std::env::var_os("NO_COLOR").is_some() {
            return false;
        }
        std::io::stdout().is_terminal()
    })
}

// ANSI escape helpers -- all return empty strings when color is off.

pub fn bold(s: &str) -> String {
    if colors_enabled() {
        format!("\x1b[1m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}

pub fn red(s: &str) -> String {
    if colors_enabled() {
        format!("\x1b[31m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}

pub fn green(s: &str) -> String {
    if colors_enabled() {
        format!("\x1b[32m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}

pub fn yellow(s: &str) -> String {
    if colors_enabled() {
        format!("\x1b[33m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}

pub fn dim(s: &str) -> String {
    if colors_enabled() {
        format!("\x1b[2m{s}\x1b[0m")
    } else {
        s.to_string()
    }
}

// ---------------------------------------------------------------------------
// JSON output
// ---------------------------------------------------------------------------

/// Print any serializable value as pretty-printed JSON to stdout.
///
/// On serialization failure, prints a JSON error object instead.
pub fn print_json<T: Serialize>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(s) => println!("{s}"),
        Err(e) => println!("{{\"error\": \"serialization failed: {e}\"}}"),
    }
}

// ---------------------------------------------------------------------------
// Text helpers
// ---------------------------------------------------------------------------

/// Truncate a string to at most `max` characters, appending "..." if needed.
pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max.saturating_sub(3)).collect();
        format!("{truncated}...")
    }
}

/// Extract just the date+time portion from an ISO datetime string.
///
/// "2024-01-15T08:30:00" -> "2024-01-15 08:30"
pub fn format_datetime(s: &str) -> String {
    if s.len() >= 16 {
        s[..16].replace('T', " ")
    } else {
        s.to_string()
    }
}

/// Extract date and time portions from an ISO datetime string.
///
/// Returns (date, time) e.g. ("2024-01-15", "08:30").
pub fn split_datetime(datetime: Option<&str>) -> (String, String) {
    match datetime {
        Some(dt) if dt.len() >= 16 => {
            let date = &dt[..10];
            let time = &dt[11..16];
            (date.to_string(), time.to_string())
        }
        Some(dt) if dt.len() >= 10 => (dt[..10].to_string(), String::new()),
        Some(dt) => (dt.to_string(), String::new()),
        None => (String::new(), String::new()),
    }
}

/// Extract HH:MM from datetime strings like "2024-01-15T08:30:00".
pub fn extract_time(s: &str) -> String {
    if let Some(t_pos) = s.find('T') {
        let time_part = &s[t_pos + 1..];
        if time_part.len() >= 5 {
            return time_part[..5].to_string();
        }
    }
    s.to_string()
}

/// Very basic HTML tag stripping for terminal display.
///
/// Also decodes common HTML entities (&amp;, &lt;, etc.).
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

// ---------------------------------------------------------------------------
// Status coloring
// ---------------------------------------------------------------------------

/// Apply color to a presence status string.
pub fn color_presence_status(status: &str) -> String {
    let lower = status.to_lowercase();
    if lower.contains("sick") {
        red(status)
    } else if lower.contains("present") && !lower.contains("not") {
        green(status)
    } else if lower.contains("absence") || lower.contains("notpresent") || lower.contains("not") {
        yellow(status)
    } else {
        status.to_string()
    }
}

/// Format an unread marker -- bold asterisk when unread.
pub fn unread_marker(is_read: bool) -> String {
    if is_read {
        " ".to_string()
    } else {
        bold("*")
    }
}

// ---------------------------------------------------------------------------
// Table builder
// ---------------------------------------------------------------------------

/// A simple column-aligned table printer.
///
/// Usage:
/// ```ignore
/// let mut t = Table::new(vec![
///     Column::new("ID", 8),
///     Column::new("NAME", 30),
/// ]);
/// t.print_header();
/// t.print_row(&["42", "Alice"]);
/// ```
pub struct Table {
    columns: Vec<Column>,
}

pub struct Column {
    pub header: String,
    pub width: usize,
}

impl Column {
    pub fn new(header: &str, width: usize) -> Self {
        Self {
            header: header.to_string(),
            width,
        }
    }
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self { columns }
    }

    /// Print the header row and separator line.
    pub fn print_header(&self) {
        let header: String = self
            .columns
            .iter()
            .map(|c| format!("{:<width$}", c.header, width = c.width))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", bold(&header));

        let total_width: usize =
            self.columns.iter().map(|c| c.width).sum::<usize>() + self.columns.len() - 1;
        println!("{}", dim(&"-".repeat(total_width)));
    }

    /// Print a data row. Values are truncated to column width.
    pub fn print_row(&self, values: &[&str]) {
        let line: String = self
            .columns
            .iter()
            .enumerate()
            .map(|(i, col)| {
                let val = values.get(i).copied().unwrap_or("");
                // If the value contains ANSI escapes, we can't just pad by len --
                // but for simplicity we truncate the raw text first, then pad.
                let display = truncate(val, col.width);
                format!("{:<width$}", display, width = col.width)
            })
            .collect::<Vec<_>>()
            .join(" ");
        println!("{line}");
    }

    /// Print a row where values may already contain ANSI color codes.
    /// The `raw_values` are used for width calculation, `display_values` for output.
    pub fn print_colored_row(&self, raw_values: &[&str], display_values: &[&str]) {
        let mut parts = Vec::new();
        for (i, col) in self.columns.iter().enumerate() {
            let raw = raw_values.get(i).copied().unwrap_or("");
            let display = display_values.get(i).copied().unwrap_or("");
            let raw_truncated = truncate(raw, col.width);
            let raw_len = raw_truncated.chars().count();
            let padding = col.width.saturating_sub(raw_len);
            // Use the display (colored) version but compute padding from raw
            let display_truncated = if raw.chars().count() > col.width {
                // Need to truncate the colored version too -- just use the raw truncation
                truncate(raw, col.width)
            } else {
                display.to_string()
            };
            parts.push(format!("{}{}", display_truncated, " ".repeat(padding)));
        }
        println!("{}", parts.join(" "));
    }
}

// ---------------------------------------------------------------------------
// Pagination hint
// ---------------------------------------------------------------------------

/// Print a pagination hint to stderr.
pub fn print_pagination_hint(current_page: Option<i32>, has_more: bool, flag: &str) {
    if has_more {
        let next = current_page.unwrap_or(0) + 1;
        eprintln!("\n(more available -- use {flag} {next})");
    }
}

// ---------------------------------------------------------------------------
// Trait: CliDisplay
// ---------------------------------------------------------------------------

/// Trait for types that can render themselves in table or detail mode.
///
/// Implementations live alongside the command handlers that know how to
/// extract fields from domain models.
pub trait CliDisplay {
    /// Column definitions for table output.
    fn table_columns() -> Vec<Column>;

    /// Values for one table row (order must match `table_columns`).
    fn table_row(&self) -> Vec<String>;

    /// Human-readable detail view, printed to stdout.
    fn print_detail(&self);
}
