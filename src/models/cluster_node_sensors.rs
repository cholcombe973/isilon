

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterNodeSensors {
  /// This node's sensor information.
  #[serde(rename = "sensors")]
  sensors: Option<Vec<::models::NodeSensorsNodeSensor>>
}

