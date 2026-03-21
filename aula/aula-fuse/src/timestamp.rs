//! Aula datetime string to [`SystemTime`] conversion.
//!
//! Aula API timestamps are ISO 8601-ish strings like `"2024-03-15T08:00:00"`
//! without timezone info. They are implicitly Europe/Copenhagen (Denmark-only
//! platform).
//!
//! Fallback: if parsing fails, returns [`UNIX_EPOCH`] to make the problem
//! visible rather than silently using the current time.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::NaiveDateTime;
use chrono_tz::Europe::Copenhagen;

/// Parse an Aula datetime string into a [`SystemTime`].
///
/// Tries multiple formats since the API is inconsistent:
/// - `%Y-%m-%dT%H:%M:%S` (most common)
/// - `%Y-%m-%dT%H:%M:%S%.f` (with fractional seconds)
/// - `%Y-%m-%d` (date only, used in some calendar fields)
///
/// Returns [`UNIX_EPOCH`] on parse failure.
pub fn parse_aula_datetime(s: &str) -> SystemTime {
    parse_aula_datetime_opt(s).unwrap_or(UNIX_EPOCH)
}

/// Parse an Aula datetime string, returning `None` on failure.
pub fn parse_aula_datetime_opt(s: &str) -> Option<SystemTime> {
    let naive = try_parse_naive(s)?;

    // Assume Europe/Copenhagen timezone.
    use chrono::TimeZone;
    let local = Copenhagen.from_local_datetime(&naive);

    // Handle ambiguous times (DST transitions) by picking the earliest.
    let dt = match local {
        chrono::LocalResult::Single(dt) => dt,
        chrono::LocalResult::Ambiguous(earliest, _) => earliest,
        chrono::LocalResult::None => {
            // This can happen during the spring-forward gap.
            // Fall back to UTC interpretation.
            let utc = chrono::Utc.from_utc_datetime(&naive);
            utc.with_timezone(&Copenhagen)
        }
    };

    let ts = dt.timestamp();
    if ts >= 0 {
        Some(UNIX_EPOCH + Duration::from_secs(ts as u64))
    } else {
        None
    }
}

fn try_parse_naive(s: &str) -> Option<NaiveDateTime> {
    // Try formats in order of likelihood.
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Some(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f") {
        return Some(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M") {
        return Some(dt);
    }
    // Date-only: treat as midnight.
    if let Ok(d) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return d.and_hms_opt(0, 0, 0);
    }
    None
}

/// Pick the more recent of two optional timestamps, or fall back to [`UNIX_EPOCH`].
pub fn mtime_from(primary: Option<&str>, fallback: Option<&str>) -> SystemTime {
    if let Some(p) = primary {
        let t = parse_aula_datetime(p);
        if t != UNIX_EPOCH {
            return t;
        }
    }
    if let Some(f) = fallback {
        return parse_aula_datetime(f);
    }
    UNIX_EPOCH
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn parse_standard_format() {
        let t = parse_aula_datetime("2024-03-15T08:00:00");
        assert_ne!(t, UNIX_EPOCH);
    }

    #[test]
    fn parse_with_fractional_seconds() {
        let t = parse_aula_datetime("2024-03-15T08:00:00.123");
        assert_ne!(t, UNIX_EPOCH);
    }

    #[test]
    fn parse_date_only() {
        let t = parse_aula_datetime("2024-03-15");
        assert_ne!(t, UNIX_EPOCH);
    }

    #[test]
    fn parse_without_seconds() {
        let t = parse_aula_datetime("2024-03-15T08:00");
        assert_ne!(t, UNIX_EPOCH);
    }

    #[test]
    fn invalid_returns_epoch() {
        assert_eq!(parse_aula_datetime("not-a-date"), UNIX_EPOCH);
        assert_eq!(parse_aula_datetime(""), UNIX_EPOCH);
    }

    #[test]
    fn mtime_from_prefers_primary() {
        let t = mtime_from(Some("2024-06-01T12:00:00"), Some("2024-01-01T00:00:00"));
        let fallback_t = parse_aula_datetime("2024-06-01T12:00:00");
        assert_eq!(t, fallback_t);
    }

    #[test]
    fn mtime_from_falls_back() {
        let t = mtime_from(None, Some("2024-01-01T00:00:00"));
        assert_ne!(t, UNIX_EPOCH);
    }

    #[test]
    fn mtime_from_none_is_epoch() {
        assert_eq!(mtime_from(None, None), UNIX_EPOCH);
    }

    #[test]
    fn copenhagen_timezone_applied() {
        // 2024-03-15 is CET (UTC+1), so 08:00 local = 07:00 UTC
        let t = parse_aula_datetime("2024-03-15T08:00:00");
        let epoch_secs = t.duration_since(UNIX_EPOCH).unwrap().as_secs();
        // Expected: 2024-03-15 07:00:00 UTC
        // Let's verify it's different from naive UTC interpretation
        let naive_utc =
            chrono::NaiveDateTime::parse_from_str("2024-03-15T08:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap();
        let utc_secs = naive_utc.and_utc().timestamp() as u64;
        // CET is UTC+1, so our time should be 1 hour earlier = 3600 seconds less
        assert_eq!(epoch_secs, utc_secs - 3600);
    }

    #[test]
    fn summer_time_applied() {
        // 2024-06-15 is CEST (UTC+2), so 08:00 local = 06:00 UTC
        let t = parse_aula_datetime("2024-06-15T08:00:00");
        let epoch_secs = t.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let naive_utc =
            chrono::NaiveDateTime::parse_from_str("2024-06-15T08:00:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap();
        let utc_secs = naive_utc.and_utc().timestamp() as u64;
        // CEST is UTC+2
        assert_eq!(epoch_secs, utc_secs - 7200);
    }
}
