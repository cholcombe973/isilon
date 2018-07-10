#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompatibilitiesClassActiveItemResponseMerge {
    /// The nodepool ids that will be merged
    #[serde(rename = "ids")]
    pub ids: Vec<i32>,
    /// The nodepool names that will be merged, in the sameorder as the ids
    #[serde(rename = "names")]
    pub names: Vec<String>,
    /// The name of the nodepool all others will merge into
    #[serde(rename = "result_name")]
    pub result_name: String,
}
