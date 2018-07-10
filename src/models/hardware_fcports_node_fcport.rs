#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareFcportsNodeFcport {
    /// The unique display id
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Port ID
    #[serde(rename = "port")]
    pub port: Option<String>,
    #[serde(rename = "rate")]
    pub rate: Option<String>,
    /// State of the port
    #[serde(rename = "state")]
    pub state: Option<String>,
    #[serde(rename = "topology")]
    pub topology: Option<String>,
    /// World wide node name of the port
    #[serde(rename = "wwnn")]
    pub wwnn: Option<String>,
    /// World wide port name of the port
    #[serde(rename = "wwpn")]
    pub wwpn: Option<String>,
}
