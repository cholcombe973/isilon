#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotaThresholdsExtended {
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
    /// True if the advisory threshold has been hit.
    #[serde(rename = "advisory_exceeded")]
    pub advisory_exceeded: Option<bool>,
    /// Time at which advisory threshold was hit.
    #[serde(rename = "advisory_last_exceeded")]
    pub advisory_last_exceeded: Option<i32>,
    /// True if the hard threshold has been hit.
    #[serde(rename = "hard_exceeded")]
    pub hard_exceeded: Option<bool>,
    /// Time at which hard threshold was hit.
    #[serde(rename = "hard_last_exceeded")]
    pub hard_last_exceeded: Option<i32>,
    /// True if the soft threshold has been hit.
    #[serde(rename = "soft_exceeded")]
    pub soft_exceeded: Option<bool>,
    /// Time at which soft threshold was hit
    #[serde(rename = "soft_last_exceeded")]
    pub soft_last_exceeded: Option<i32>,
}
