/// NdmpDumpdates : Get list of dumpdates entries.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpDumpdates {
    #[serde(rename = "dumpdates")]
    pub dumpdates: Option<Vec<::models::NdmpDumpdate>>,
    /// Resume string returned by previous query.
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// The number of dumpdates entries.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
