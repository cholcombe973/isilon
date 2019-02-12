#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsPreferredIpsPreference {
    #[serde(rename = "data_subnets")]
    pub data_subnets: Option<Vec <crate::models::NdmpSettingsPreferredIpDataSubnet>>,
    /// The unique display id, same as scope
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Either cluster or a network subnet defined in OneFS.
    #[serde(rename = "scope")]
    pub scope: Option<String>,
}
