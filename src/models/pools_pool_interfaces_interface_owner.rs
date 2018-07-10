#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolsPoolInterfacesInterfaceOwner {
    #[serde(rename = "groupnet")]
    pub groupnet: Option<String>,
    #[serde(rename = "pool")]
    pub pool: Option<String>,
    #[serde(rename = "subnet")]
    pub subnet: Option<String>,
}
