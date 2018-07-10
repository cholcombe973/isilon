#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportSubreports {
    #[serde(rename = "subreports")]
    pub subreports: Option<Vec<::models::ReportSubreport>>,
}
