#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodeCpu {
    /// Manufacturer model description of this CPU.
    #[serde(rename = "model")]
    pub model: Option<String>,
    /// CPU overtemp state.
    #[serde(rename = "overtemp")]
    pub overtemp: Option<String>,
    /// Type of processor and core of this CPU.
    #[serde(rename = "proc")]
    pub _proc: Option<String>,
    /// CPU throttling (expressed as a percentage).
    #[serde(rename = "speed_limit")]
    pub speed_limit: Option<String>,
}
