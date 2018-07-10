#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdActiveActiveItem {
    /// The node class of an ssd compatibility
    #[serde(rename = "class_1")]
    pub class_1: String,
    /// Is this SSD Compatibility Count Compatible.
    #[serde(rename = "count")]
    pub count: bool,
    /// The id of this ssd compatibility
    #[serde(rename = "id")]
    pub id: i32,
}
