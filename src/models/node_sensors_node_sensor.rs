#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSensorsNodeSensor {
    /// The count of values in this sensor group.
    #[serde(rename = "count")]
    pub count: Option<i32>,
    /// The name of this sensor group.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The list of specific sensor value info in this sensor group.
    #[serde(rename = "values")]
    pub values: Option<Vec<::models::NodeSensorsNodeSensorValue>>,
}
