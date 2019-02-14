#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateServerCertificate {
    /// Boolean identifying if a certificate is the default certificate.The default certificate is used as the fallback when no other certificates match a TLS enabled service's particular criteria. There must always be a configured default certificate.
    #[serde(rename = "default")]
    pub default: bool,
    /// Description field associated with a certificate provided for administrative convenience.
    #[serde(rename = "description")]
    pub description: String,
    /// A list of DNS names/patterns for which this certificate is valid. This list is extracted from the certificates CN (Common Name) and subjectAtlName extension fields.
    #[serde(rename = "dnsnames")]
    pub dnsnames: Vec<String>,
    /// True if the certificate has expired and is no longer valid.
    #[serde(rename = "expired")]
    pub expired: bool,
    /// A list of zero or more certificate fingerprints which can be used for certificate identification.
    #[serde(rename = "fingerprints")]
    pub fingerprints: Vec <crate::models::CertificateServerCertificateFingerprint>,
    /// Unique server certificate identifier.
    #[serde(rename = "id")]
    pub id: String,
    /// Certificate issuer field extracted from the certificate.
    #[serde(rename = "issuer")]
    pub issuer: String,
    /// Subject common name extracted from the certificate.
    #[serde(rename = "name")]
    pub name: String,
    /// Certificate notAfter field extracted from the certificate encoded as a UNIX epoch timestamp.  The certificate is not valid after this timestamp.
    #[serde(rename = "not_after")]
    pub not_after: i32,
    /// Certificate notBefore field extracted from the certificate encoded as a UNIX epoch timestamp.  The certificate is not valid before this timestamp.
    #[serde(rename = "not_before")]
    pub not_before: i32,
    /// Certificate subject field extracted from the certificate.
    #[serde(rename = "subject")]
    pub subject: String,
    /// True if the certificate is valid (ie: not_before <= now <= not_after).
    #[serde(rename = "valid")]
    pub valid: bool,
}
