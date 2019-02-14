#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportAbout {
    #[serde(rename = "reports")]
    pub reports: Option<Vec <crate::models::ReportAboutReport>>,
}
