

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsReportingEulaItem {
  /// Indicates whether the telemetry collection warning has been acknowledged
  #[serde(rename = "accepted")]
  accepted: Option<bool>,
  /// The body of the telemetry collection warning
  #[serde(rename = "body")]
  body: Option<String>
}

