#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotaThresholds {
    /// Usage bytes at which notifications will be sent but writes will not be denied.
    #[serde(rename = "advisory")]
    pub advisory: Option<i32>,
    /// Usage bytes at which further writes will be denied.
    #[serde(rename = "hard")]
    pub hard: Option<i32>,
    /// Usage bytes at which notifications will be sent and soft grace time will be started.
    #[serde(rename = "soft")]
    pub soft: Option<i32>,
    /// Time in seconds after which the soft threshold has been hit before writes will be denied.
    #[serde(rename = "soft_grace")]
    pub soft_grace: Option<i32>,
}
