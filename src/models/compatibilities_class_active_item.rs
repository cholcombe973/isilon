#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesClassActiveItem {
    /// Do not create compatibility, only assess if creation is possible.
    #[serde(rename = "assess")]
    pub assess: Option<bool>,
    /// The first class in the desired compatibility
    #[serde(rename = "class_1")]
    pub class_1: String,
    /// The second class in the desired compatibility
    #[serde(rename = "class_2")]
    pub class_2: String,
}
