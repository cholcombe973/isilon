#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotaCreateParams {
    /// If true, SMB shares using the quota directory see the quota thresholds as share size.
    #[serde(rename = "container")]
    pub container: Option<bool>,
    /// True if the quota provides enforcement, otherwise a accounting quota.
    #[serde(rename = "enforced")]
    pub enforced: bool,
    /// Force creation of quotas on the root of /ifs.
    #[serde(rename = "force")]
    pub force: Option<bool>,
    /// If true, quota governs snapshot data as well as head data.
    #[serde(rename = "include_snapshots")]
    pub include_snapshots: bool,
    /// The /ifs path governed.
    #[serde(rename = "path")]
    pub path: String,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "persona")]
    pub persona: Option <crate::models::AuthAccessAccessItemFileGroup>,
    ///
    #[serde(rename = "thresholds")]
    pub thresholds: Option <crate::models::QuotaQuotaThresholds>,
    /// If true, thresholds apply to data plus filesystem overhead required to store the data (i.e. 'physical' usage).
    #[serde(rename = "thresholds_include_overhead")]
    pub thresholds_include_overhead: bool,
    /// The type of quota.
    #[serde(rename = "type")]
    pub _type: String,
}
