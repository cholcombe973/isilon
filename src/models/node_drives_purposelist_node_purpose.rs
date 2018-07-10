#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDrivesPurposelistNodePurpose {
    /// String representation of this purpose for API use.
    #[serde(rename = "purpose")]
    pub purpose: Option<String>,
    /// A description of this purpose.
    #[serde(rename = "purpose_description")]
    pub purpose_description: Option<String>,
}
