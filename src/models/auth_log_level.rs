#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLogLevel {
    ///
    #[serde(rename = "level")]
    pub level: Option<::models::AuthLogLevelLevel>,
}
