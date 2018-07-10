#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdActiveItem {
    /// Do not create ssd compatibility, only assess if creation is possible.
    #[serde(rename = "assess")]
    pub assess: Option<bool>,
    /// The node class of the desired ssd compatibility
    #[serde(rename = "class_1")]
    pub class_1: String,
    /// The optional second node class to turn on ssd compatibility
    #[serde(rename = "class_2")]
    pub class_2: Option<String>,
    /// Is this SSD Compatibility Count Compatible.
    #[serde(rename = "count")]
    pub count: Option<bool>,
}
