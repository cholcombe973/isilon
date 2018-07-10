

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventAlertConditions {
  #[serde(rename = "alert-conditions")]
  alert_conditions: Option<Vec<::models::EventAlertConditionsAlertCondition>>
}

