

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateServerCertificateFingerprint {
  /// Fingerprint hash algorithm
  #[serde(rename = "type")]
  _type: Option<String>,
  /// Fingerprint value
  #[serde(rename = "value")]
  value: Option<String>
}

