/// EventEventgroupOccurrence : Ignore flag for eventgroup.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventgroupOccurrence {
    /// True if eventgroup is to be ignored
    #[serde(rename = "ignore")]
    pub ignore: Option<bool>,
    /// True if eventgroup is to be resolved
    #[serde(rename = "resolved")]
    pub resolved: Option<bool>,
}
