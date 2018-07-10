#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateServerCertificateFingerprint {
    /// Fingerprint hash algorithm
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// Fingerprint value
    #[serde(rename = "value")]
    pub value: Option<String>,
}
