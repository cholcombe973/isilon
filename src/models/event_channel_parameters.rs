#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventChannelParameters {
    /// Email addresses to send to.
    #[serde(rename = "address")]
    pub address: Option<Vec<String>>,
    /// Batching criterion.
    #[serde(rename = "batch")]
    pub batch: Option<String>,
    /// Period over which batching is to be performed.
    #[serde(rename = "batch_period")]
    pub batch_period: Option<i32>,
    /// Path to custom notification template.
    #[serde(rename = "custom_template")]
    pub custom_template: Option<String>,
    /// Email address to use as from.
    #[serde(rename = "send_as")]
    pub send_as: Option<String>,
    /// SMTP relay host.
    #[serde(rename = "smtp_host")]
    pub smtp_host: Option<String>,
    /// Password for SMTP authentication - only if smtp_use_auth true.
    #[serde(rename = "smtp_password")]
    pub smtp_password: Option<String>,
    /// SMTP relay port - optional defaults to 25.
    #[serde(rename = "smtp_port")]
    pub smtp_port: Option<i32>,
    /// Encryption protocol to use for SMTP.
    #[serde(rename = "smtp_security")]
    pub smtp_security: Option<String>,
    /// Use SMTP authentication - optional defaulst to false.
    #[serde(rename = "smtp_use_auth")]
    pub smtp_use_auth: Option<bool>,
    /// Username for SMTP authentication - only if smtp_use_auth true.
    #[serde(rename = "smtp_username")]
    pub smtp_username: Option<String>,
    /// Subject for emails.
    #[serde(rename = "subject")]
    pub subject: Option<String>,
}
