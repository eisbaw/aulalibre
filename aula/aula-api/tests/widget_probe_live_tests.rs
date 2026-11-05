//! Live probe: which third-party school platforms does this account's school
//! actually use?
//!
//! Aula issues a widget SSO token for almost any widget id, so token issuance
//! does not tell us which providers are in use. This probe goes one level
//! deeper: it calls each provider's API with the token and reports the HTTP
//! status plus the shape of the response (keys/counts only — never values).
//!
//! ```text
//! cargo test -p aula-api --test widget_probe_live_tests -- --ignored --nocapture
//! ```

use aula_api::services::widget;
use aula_api::session;
use aula_api::Environment;
use serde_json::Value;

/// Everything the provider APIs need, gathered from the Aula session.
struct Ctx {
    guardian_user_id: String,
    guardian_profile_id: String,
    child_profile_ids: Vec<i64>,
    child_user_ids: Vec<String>,
    institution_codes: Vec<String>,
    week: String,
}

async fn gather(session: &mut aula_api::Session) -> Ctx {
    let context = session
        .get::<Value>("?method=profiles.getProfileContext&portalrole=guardian&deviceId=aula-cli")
        .await
        .expect("getProfileContext");
    let guardian_user_id = context
        .get("userId")
        .map(|v| match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        })
        .unwrap_or_default();

    session
        .ensure_context_initialized()
        .await
        .expect("context initialised");

    let mut child_profile_ids = Vec::new();
    let mut child_user_ids = Vec::new();
    let mut guardian_profile_id = String::new();
    if let Some(profile_data) = session.profile_data() {
        for profile in &profile_data.profiles {
            if let Some(profile_id) = profile.profile_id {
                guardian_profile_id = profile_id.to_string();
            }
            for child in profile.children.as_deref().unwrap_or_default() {
                if let Some(id) = child.profile_id {
                    child_profile_ids.push(id);
                }
                if let Some(user_id) = child.user_id.clone() {
                    child_user_ids.push(user_id);
                }
            }
        }
    }

    Ctx {
        guardian_user_id,
        guardian_profile_id,
        child_profile_ids,
        child_user_ids,
        institution_codes: session.children_institution_codes(),
        week: chrono::Local::now().format("%V").to_string(),
    }
}

/// Report status and response shape without echoing any content.
async fn report(label: &str, request: reqwest::RequestBuilder) {
    match request.send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            let shape = match serde_json::from_str::<Value>(&body) {
                Ok(Value::Object(map)) => {
                    // Error envelopes (responseStatus / exceptionMessage) are
                    // API metadata, not user data, so they are safe to surface
                    // and are exactly what tells us which parameter is wrong.
                    let detail = ["responseStatus", "exceptionMessage", "message", "error"]
                        .iter()
                        .filter_map(|key| map.get(*key).map(|v| format!("{key}={v}")))
                        .collect::<Vec<_>>()
                        .join(" ");
                    let mut keys: Vec<String> = map.keys().cloned().collect();
                    keys.sort();
                    if detail.is_empty() {
                        format!("object keys={keys:?}")
                    } else {
                        format!("object keys={keys:?} | {}", truncate(&detail, 220))
                    }
                }
                Ok(Value::Array(items)) => format!("array len={}", items.len()),
                Ok(other) => format!("scalar {other}"),
                Err(_) => {
                    let lower = body.to_ascii_lowercase();
                    let kind = if lower.contains("<!doctype") || lower.contains("<html") {
                        "HTML"
                    } else {
                        "non-JSON"
                    };
                    format!("{kind}, {} chars", body.len())
                }
            };
            println!("  {label:<28} HTTP {status} | {shape}");
        }
        Err(e) => println!("  {label:<28} request failed ({e})"),
    }
}

fn truncate(value: &str, max: usize) -> String {
    if value.chars().count() <= max {
        value.to_string()
    } else {
        value.chars().take(max).collect::<String>() + "…"
    }
}

/// Field names of the first element of a named array, so Rust structs can be
/// designed from real data without printing any values.
async fn report_fields(label: &str, request: reqwest::RequestBuilder, array_key: &str) {
    match request.send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            let shape = match serde_json::from_str::<Value>(&body) {
                Ok(value) => {
                    let items = value.get(array_key).and_then(Value::as_array);
                    match items {
                        Some(items) if !items.is_empty() => {
                            let mut keys: Vec<String> = items[0]
                                .as_object()
                                .map(|m| m.keys().cloned().collect())
                                .unwrap_or_default();
                            keys.sort();
                            format!(
                                "HTTP {status} | {array_key}[{}] first item keys={keys:?}",
                                items.len()
                            )
                        }
                        Some(_) => format!("HTTP {status} | {array_key} empty"),
                        None => {
                            let mut keys: Vec<String> = value
                                .as_object()
                                .map(|m| m.keys().cloned().collect())
                                .unwrap_or_default();
                            keys.sort();
                            format!("HTTP {status} | top-level keys={keys:?}")
                        }
                    }
                }
                Err(_) => format!("HTTP {status} | non-JSON {} chars", body.len()),
            };
            println!("  {label:<28} {shape}");
        }
        Err(e) => println!("  {label:<28} request failed ({e})"),
    }
}

#[tokio::test]
#[ignore = "live: needs a logged-in token store"]
async fn probe_provider_apis() {
    let mut session = session::from_store(Environment::Production).expect("session from store");
    let ctx = gather(&mut session).await;
    println!(
        "context: {} child profile id(s), {} child user id(s), {} institution code(s), week {}",
        ctx.child_profile_ids.len(),
        ctx.child_user_ids.len(),
        ctx.institution_codes.len(),
        ctx.week
    );
    println!();

    let http = reqwest::Client::new();
    let codes = ctx.institution_codes.join(",");

    // --- Meebook (widget 0004) -------------------------------------------
    if let Ok(token) = widget::get_aula_token(&mut session, "0004").await {
        let token = token.as_str().unwrap_or_default().to_string();
        let mut url = format!(
            "https://app.meebook.com/aulaapi/relatedweekplan/all?currentWeekNumber=2026-W{}&userProfile=guardian",
            ctx.week
        );
        for id in &ctx.child_user_ids {
            url.push_str(&format!("&childFilter[]={id}"));
        }
        for code in &ctx.institution_codes {
            url.push_str(&format!("&institutionFilter[]={code}"));
        }
        report(
            "Meebook weekplan",
            http.get(&url)
                .header("authorization", format!("Bearer {token}"))
                .header("sessionuuid", &ctx.guardian_user_id)
                .header("x-version", "1.0")
                .header("origin", "https://www.aula.dk")
                .header("referer", "https://www.aula.dk/"),
        )
        .await;

        // Structure dump for designing the Rust structs: keys and counts only.
        if let Ok(response) = http
            .get(&url)
            .header("authorization", format!("Bearer {token}"))
            .header("sessionuuid", &ctx.guardian_user_id)
            .header("x-version", "1.0")
            .header("origin", "https://www.aula.dk")
            .header("referer", "https://www.aula.dk/")
            .send()
            .await
        {
            let body = response.text().await.unwrap_or_default();
            if let Ok(serde_json::Value::Array(items)) = serde_json::from_str(&body) {
                println!("  Meebook shape: array[{}]", items.len());
                if let Some(first) = items.first().and_then(serde_json::Value::as_object) {
                    let mut keys: Vec<&String> = first.keys().collect();
                    keys.sort();
                    println!("    item keys: {keys:?}");
                    for (key, value) in first {
                        if let Some(array) = value.as_array() {
                            let nested = array
                                .first()
                                .and_then(serde_json::Value::as_object)
                                .map(|m| {
                                    let mut k: Vec<&String> = m.keys().collect();
                                    k.sort();
                                    format!("{k:?}")
                                })
                                .unwrap_or_else(|| "-".to_string());
                            println!("    {key}: array[{}] of {nested}", array.len());
                            if key == "weekPlan" {
                                if let Some(tasks) = array
                                    .first()
                                    .and_then(|day| day.get("tasks"))
                                    .and_then(serde_json::Value::as_array)
                                {
                                    println!("      tasks: array[{}]", tasks.len());
                                    if let Some(task) =
                                        tasks.first().and_then(serde_json::Value::as_object)
                                    {
                                        let mut k: Vec<&String> = task.keys().collect();
                                        k.sort();
                                        println!("      task keys: {k:?}");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // --- Min Uddannelse (widgets 0030 opgaver, 0029 ugebrev) -------------
    // Exact parameter set from the reference implementation: the endpoint
    // wants assuranceLevel/isMobileApp/placement/sessionUUID, not sessionId.
    for (widget_id, label, path) in [
        ("0030", "MinU opgaveliste", "opgaveliste"),
        ("0029", "MinU ugebrev", "ugebrev"),
    ] {
        let Ok(token) = widget::get_aula_token(&mut session, widget_id).await else {
            println!("  {label:<28} no widget token");
            continue;
        };
        let token = token.as_str().unwrap_or_default().to_string();

        // The reference is ambiguous about whether sessionUUID is the guardian
        // user id or the numeric profile id, so try both.
        let mut variants: Vec<(&str, String)> = vec![
            ("userId", ctx.guardian_user_id.clone()),
            ("profileId", ctx.guardian_profile_id.clone()),
        ];
        variants.retain(|(_, value)| !value.is_empty());

        for (kind, session_id) in variants {
            // The reference passes its "isoWeek" (2026-W45 style) into
            // currentWeekNumber; try the bare week too, and no week at all.
            for (week_kind, week_value) in [
                ("week=45", format!("&currentWeekNumber={}", ctx.week)),
                ("week=ISO", format!("&currentWeekNumber=2026-W{}", ctx.week)),
                ("no-week", String::new()),
            ] {
                let url = format!(
                    "https://api.minuddannelse.net/aula/{path}?assuranceLevel=2&childFilter={}&isMobileApp=false&placement=narrow&sessionUUID={}&userProfile=guardian{week_value}",
                    ctx.child_user_ids.join(","),
                    session_id
                );
                report(
                    &format!("{label} [{kind}/{week_kind}]"),
                    http.get(&url)
                        .header("authorization", format!("Bearer {token}"))
                        .header("accept", "application/json"),
                )
                .await;
            }
        }
    }

    // --- Systematic MOMO reminders (widget 0062) -------------------------
    if let Ok(token) = widget::get_aula_token(&mut session, "0062").await {
        let token = token.as_str().unwrap_or_default().to_string();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let later = (chrono::Local::now() + chrono::Duration::days(30))
            .format("%Y-%m-%d")
            .to_string();
        let url = format!(
            "https://systematic-momo.dk/api/aula/reminders/v1?children={}&institutions={}&from={today}&dueNoLaterThan={later}&widgetVersion=1.10&userProfile=guardian&sessionId={}",
            ctx.child_profile_ids.iter().map(i64::to_string).collect::<Vec<_>>().join(","),
            codes,
            ctx.guardian_user_id
        );
        report(
            "Systematic reminders",
            http.get(&url)
                .header("aula-authorization", format!("Bearer {token}"))
                .header("accept", "application/json, text/plain, */*")
                .header("origin", "https://www.aula.dk")
                .header("referer", "https://www.aula.dk/")
                .header("zone", "Europe/Copenhagen"),
        )
        .await;
    }

    // --- Field shapes for the integrations that answer --------------------
    println!();
    println!("field names (for the Rust structs):");
    for (widget_id, label, path, array_key) in [
        ("0030", "MinU opgaver fields", "opgaveliste", "opgaver"),
        ("0029", "MinU ugebrev fields", "ugebrev", "personer"),
    ] {
        if let Ok(token) = widget::get_aula_token(&mut session, widget_id).await {
            let token = token.as_str().unwrap_or_default().to_string();
            let url = format!(
                "https://api.minuddannelse.net/aula/{path}?assuranceLevel=2&childFilter={}&currentWeekNumber=2026-W{}&isMobileApp=false&placement=narrow&sessionUUID={}&userProfile=guardian",
                ctx.child_user_ids.join(","),
                ctx.week,
                ctx.guardian_user_id
            );
            report_fields(
                label,
                http.get(&url)
                    .header("authorization", format!("Bearer {token}"))
                    .header("accept", "application/json"),
                array_key,
            )
            .await;
        }
    }
}
