#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventgroupDefinitionsExtended {
    #[serde(rename = "eventgroup-definitions")]
    pub eventgroup_definitions:
        Option<Vec <crate::models::EventEventgroupDefinitionsEventgroupDefinition>>,
    /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
    #[serde(rename = "resume")]
    pub resume: Option<String>,
    /// Total number of items available.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
