#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateServer {
    #[serde(rename = "certificates")]
    pub certificates: Option<Vec <crate::models::CertificateServerCertificate>>,
    /// Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options).
    #[serde(rename = "resume")]
    pub resume: Option<String>,
}
