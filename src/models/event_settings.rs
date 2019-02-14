#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSettings {
    /// Settings for CELOG system
    #[serde(rename = "settings")]
    pub settings: Option <crate::models::EventSettingsSettings>,
}
