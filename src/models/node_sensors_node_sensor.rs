

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSensorsNodeSensor {
  /// The count of values in this sensor group.
  #[serde(rename = "count")]
  count: Option<i32>,
  /// The name of this sensor group.
  #[serde(rename = "name")]
  name: Option<String>,
  /// The list of specific sensor value info in this sensor group.
  #[serde(rename = "values")]
  values: Option<Vec<::models::NodeSensorsNodeSensorValue>>
}

