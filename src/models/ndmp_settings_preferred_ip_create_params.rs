

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsPreferredIpCreateParams {
  #[serde(rename = "data_subnets")]
  data_subnets: Vec<::models::NdmpSettingsPreferredIpDataSubnet>,
  /// Either cluster or a network subnet defined in OneFS.
  #[serde(rename = "scope")]
  scope: String
}

