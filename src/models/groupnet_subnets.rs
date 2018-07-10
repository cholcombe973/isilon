#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupnetSubnets {
    #[serde(rename = "subnets")]
    pub subnets: Option<Vec<::models::GroupnetSubnetExtended>>,
}
