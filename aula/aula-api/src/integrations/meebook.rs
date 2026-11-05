//! Meebook week plans (`ugeplan`), reached through Aula widget `0004`.
//!
//! ```text
//! GET https://app.meebook.com/aulaapi/relatedweekplan/all
//!     ?currentWeekNumber=yyyy-Www&userProfile=guardian
//!     &childFilter[]=<child unilogin>&institutionFilter[]=<institution code>
//! Headers: authorization: Bearer <widget token>, sessionuuid: <guardian user id>,
//!          x-version: 1.0, origin/referer: https://www.aula.dk
//! ```
//!
//! Response: one entry per child, each with a `weekPlan` array of days, each day
//! carrying `tasks`. Verified live: HTTP 200 with one entry per child.
//!
//! Note `currentWeekNumber` must be `yyyy-Www`; a bare week number is rejected.

use serde::{Deserialize, Serialize};

use crate::services::widget;
use crate::session::Session;

/// Aula widget id that carries the Meebook SSO token.
const WIDGET_ID: &str = "0004";

/// Provider endpoint.
const ENDPOINT: &str = "https://app.meebook.com/aulaapi/relatedweekplan/all";

/// Everything the provider call needs, gathered from the Aula session.
#[derive(Debug, Clone)]
pub struct MeebookContext {
    /// Guardian user id from `profiles.getProfileContext`, sent as `sessionuuid`.
    pub guardian_user_id: String,
    /// Children to query, as their unilogin user ids.
    pub child_user_ids: Vec<String>,
    /// Institution codes, e.g. `G12345`.
    pub institution_codes: Vec<String>,
    /// ISO week in `yyyy-Www` form.
    pub week: String,
}

/// One child's week plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeebookChild {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub unilogin: Option<String>,
    #[serde(default, rename = "weekPlan")]
    pub week_plan: Vec<MeebookDay>,
}

/// One day of the week plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeebookDay {
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub tasks: Vec<MeebookTask>,
}

/// A single task/note on a day.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeebookTask {
    #[serde(default)]
    pub id: Option<i64>,
    #[serde(default)]
    pub content: Option<String>,
    /// Subject or class the task belongs to.
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub pill: Option<String>,
    #[serde(default, rename = "type")]
    pub kind: Option<String>,
}

/// Fetch a week plan using an already-issued widget token.
///
/// Split from [`week_plan`] so tests can drive it against a mock server.
pub async fn fetch(
    http: &reqwest::Client,
    base_url: &str,
    token: &str,
    ctx: &MeebookContext,
) -> crate::Result<Vec<MeebookChild>> {
    let mut url = format!(
        "{base_url}?currentWeekNumber={}&userProfile=guardian",
        ctx.week
    );
    for child in &ctx.child_user_ids {
        url.push_str(&format!("&childFilter[]={child}"));
    }
    for code in &ctx.institution_codes {
        url.push_str(&format!("&institutionFilter[]={code}"));
    }

    let response = http
        .get(url)
        .header("authorization", format!("Bearer {token}"))
        .header("sessionuuid", &ctx.guardian_user_id)
        .header("x-version", "1.0")
        .header("accept", "application/json")
        .header("origin", "https://www.aula.dk")
        .header("referer", "https://www.aula.dk/")
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        return Err(crate::error::AulaError::Api {
            message: format!(
                "meebook weekplan failed (HTTP {status}): {}",
                body.chars().take(200).collect::<String>()
            ),
            status: None,
        });
    }

    serde_json::from_str(&body).map_err(crate::error::AulaError::from)
}

/// Fetch the week plan for the current (or given) ISO week.
pub async fn week_plan(
    session: &mut Session,
    week: Option<&str>,
) -> crate::Result<Vec<MeebookChild>> {
    session.ensure_context_initialized().await?;

    let week = week.map(str::to_string).unwrap_or_else(current_iso_week);
    let guardian_user_id = session.guardian_user_id().unwrap_or_default().to_string();
    let child_user_ids = session.children_user_ids();
    if child_user_ids.is_empty() {
        return Err(crate::error::AulaError::Api {
            message: "meebook: no child user ids on this login".to_string(),
            status: None,
        });
    }
    let ctx = MeebookContext {
        guardian_user_id,
        child_user_ids,
        institution_codes: session.children_institution_codes(),
        week,
    };

    let token = widget::get_aula_token(session, WIDGET_ID).await?;
    let token = match token {
        serde_json::Value::String(token) => token,
        other => other
            .get("token")
            .and_then(|t| t.as_str())
            .map(str::to_string)
            .ok_or_else(|| crate::error::AulaError::Api {
                message: "meebook: widget token response had no token".to_string(),
                status: None,
            })?,
    };

    fetch(session.client().http(), ENDPOINT, &token, &ctx).await
}

/// Current ISO week as `yyyy-Www`, the only format the provider accepts.
fn current_iso_week() -> String {
    chrono::Local::now().format("%G-W%V").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn context() -> MeebookContext {
        MeebookContext {
            guardian_user_id: "guardian-1".to_string(),
            child_user_ids: vec!["child-a".to_string(), "child-b".to_string()],
            institution_codes: vec!["G12345".to_string()],
            week: "2026-W45".to_string(),
        }
    }

    const RESPONSE: &str = r#"[
      {
        "id": 1,
        "name": "Child A",
        "unilogin": "child-a",
        "weekPlan": [
          {
            "date": "2026-11-03",
            "tasks": [
              {"id": 10, "content": "Read chapter 3", "group": "Danish", "pill": "homework", "type": "task"}
            ]
          }
        ]
      }
    ]"#;

    #[tokio::test]
    async fn parses_a_week_plan() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/aulaapi/relatedweekplan/all"))
            .and(header("authorization", "Bearer widget-token"))
            .and(header("sessionuuid", "guardian-1"))
            .and(header("x-version", "1.0"))
            .respond_with(ResponseTemplate::new(200).set_body_string(RESPONSE))
            .mount(&server)
            .await;

        let base = format!("{}/aulaapi/relatedweekplan/all", server.uri());
        let children = fetch(&reqwest::Client::new(), &base, "widget-token", &context())
            .await
            .expect("week plan");

        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name.as_deref(), Some("Child A"));
        assert_eq!(children[0].week_plan.len(), 1);
        let day = &children[0].week_plan[0];
        assert_eq!(day.date.as_deref(), Some("2026-11-03"));
        assert_eq!(day.tasks[0].content.as_deref(), Some("Read chapter 3"));
        assert_eq!(day.tasks[0].group.as_deref(), Some("Danish"));
    }

    #[tokio::test]
    async fn sends_the_iso_week_and_child_filters() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/aulaapi/relatedweekplan/all"))
            .and(wiremock::matchers::query_param(
                "currentWeekNumber",
                "2026-W45",
            ))
            .and(wiremock::matchers::query_param("userProfile", "guardian"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let base = format!("{}/aulaapi/relatedweekplan/all", server.uri());
        let children = fetch(&reqwest::Client::new(), &base, "token", &context())
            .await
            .expect("week plan");
        assert!(children.is_empty());

        let requests = server.received_requests().await.expect("requests");
        let url = requests[0].url.to_string();
        assert!(url.contains("childFilter[]=child-a"), "url was: {url}");
        assert!(url.contains("institutionFilter[]=G12345"), "url was: {url}");
    }

    #[tokio::test]
    async fn http_errors_surface_as_api_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(400).set_body_string(
                r#"{"exceptionMessage":"Uge nummer er ikke angivet i det korrekte format"}"#,
            ))
            .mount(&server)
            .await;

        let base = format!("{}/aulaapi/relatedweekplan/all", server.uri());
        let err = fetch(&reqwest::Client::new(), &base, "token", &context())
            .await
            .expect_err("400 must be an error");
        assert!(err.to_string().contains("400"), "error was: {err}");
    }

    #[test]
    fn current_week_is_iso_formatted() {
        let week = current_iso_week();
        assert!(week.len() == 8, "expected yyyy-Www, got {week}");
        assert!(week.contains("-W"), "expected yyyy-Www, got {week}");
    }
}
