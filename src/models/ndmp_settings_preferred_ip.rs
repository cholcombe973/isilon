#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSettingsPreferredIp {
    #[serde(rename = "data_subnets")]
    pub data_subnets: Vec<::models::NdmpSettingsPreferredIpDataSubnet>,
}
