#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuota {
    /// If true, SMB shares using the quota directory see the quota thresholds as share size.
    #[serde(rename = "container")]
    pub container: Option<bool>,
    /// True if the quota provides enforcement, otherwise a accounting quota.
    #[serde(rename = "enforced")]
    pub enforced: Option<bool>,
    /// If false and the quota is linked, attempt to unlink.
    #[serde(rename = "linked")]
    pub linked: Option<bool>,
    ///
    #[serde(rename = "thresholds")]
    pub thresholds: Option<::models::QuotaQuotaThresholds>,
    /// If true, thresholds apply to data plus filesystem overhead required to store the data (i.e. 'physical' usage).
    #[serde(rename = "thresholds_include_overhead")]
    pub thresholds_include_overhead: Option<bool>,
}
