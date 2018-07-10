

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodePowersupplies {
  /// Count of how many power supplies are supported.
  #[serde(rename = "count")]
  count: Option<i32>,
  /// Count of how many power supplies have failed.
  #[serde(rename = "failures")]
  failures: Option<i32>,
  /// Does this node have a CFF power supply.
  #[serde(rename = "has_cff")]
  has_cff: Option<bool>,
  /// A descriptive status string for this node's power supplies.
  #[serde(rename = "status")]
  status: Option<String>,
  /// List of this node's power supplies.
  #[serde(rename = "supplies")]
  supplies: Option<Vec<::models::NodeStatusNodePowersuppliesSupply>>,
  /// Does this node support CFF power supplies.
  #[serde(rename = "supports_cff")]
  supports_cff: Option<bool>
}

