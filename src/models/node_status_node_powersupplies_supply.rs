#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodePowersuppliesSupply {
    /// Which node chassis is this power supply in.
    #[serde(rename = "chassis")]
    pub chassis: Option<i32>,
    /// The current firmware revision of this power supply.
    #[serde(rename = "firmware")]
    pub firmware: Option<String>,
    /// Is this power supply in a failure state.
    #[serde(rename = "good")]
    pub good: Option<String>,
    /// Identifying index for this power supply.
    #[serde(rename = "id")]
    pub id: i32,
    /// Complete identifying string for this power supply.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A descriptive status string for this power supply.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The type of this power supply.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
