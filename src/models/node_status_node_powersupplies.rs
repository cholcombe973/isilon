#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodePowersupplies {
    /// Count of how many power supplies are supported.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// Count of how many power supplies have failed.
    #[serde(rename = "failures")]
    pub failures: Option<i32>,
    /// Does this node have a CFF power supply.
    #[serde(rename = "has_cff")]
    pub has_cff: Option<bool>,
    /// A descriptive status string for this node's power supplies.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// List of this node's power supplies.
    #[serde(rename = "supplies")]
    pub supplies: Option<Vec<::models::NodeStatusNodePowersuppliesSupply>>,
    /// Does this node support CFF power supplies.
    #[serde(rename = "supports_cff")]
    pub supports_cff: Option<bool>,
}
