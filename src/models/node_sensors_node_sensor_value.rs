#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSensorsNodeSensorValue {
    /// The descriptive name of this sensor.
    #[serde(rename = "desc")]
    pub desc: Option<String>,
    /// The identifier name of this sensor.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The units of this sensor.
    #[serde(rename = "units")]
    pub units: Option<String>,
    /// The value of this sensor.
    #[serde(rename = "value")]
    pub value: Option<String>,
}
