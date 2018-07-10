#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateServerIdParams {
    /// Boolean identifying if a certificate is the default certificate.The default certificate is used as the fallback when no other certificates match a TLS enabled service's particular criteria. There must always be a configured default certificate.
    #[serde(rename = "default")]
    pub default: Option<bool>,
    /// Description field associated with a certificate provided for administrative convenience.
    #[serde(rename = "description")]
    pub description: Option<String>,
}
