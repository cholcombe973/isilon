#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsPreferredIpCreateParams {
    #[serde(rename = "data_subnets")]
    pub data_subnets: Vec<::models::NdmpSettingsPreferredIpDataSubnet>,
    /// Either cluster or a network subnet defined in OneFS.
    #[serde(rename = "scope")]
    pub scope: String,
}
