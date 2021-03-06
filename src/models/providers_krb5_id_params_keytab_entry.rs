#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersKrb5IdParamsKeytabEntry {
    /// Specifies the encryption types.
    #[serde(rename = "encryption")]
    pub encryption: Option<Vec<String>>,
    /// Specifies the version number of the SPN key.
    #[serde(rename = "kvno")]
    pub kvno: Option<i32>,
    /// Specifies the Service Principal Name (SPN).
    #[serde(rename = "spn")]
    pub spn: Option<String>,
    /// Specifies the time the SPN key was created.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i32>,
}
