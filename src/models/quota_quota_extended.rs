

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotaExtended {
  /// If true, SMB shares using the quota directory see the quota thresholds as share size.
  #[serde(rename = "container")]
  container: bool,
  /// True if the quota provides enforcement, otherwise a accounting quota.
  #[serde(rename = "enforced")]
  enforced: bool,
  /// The system ID given to the quota.
  #[serde(rename = "id")]
  id: String,
  /// If true, quota governs snapshot data as well as head data.
  #[serde(rename = "include_snapshots")]
  include_snapshots: bool,
  #[serde(rename = "linked")]
  linked: Option<bool>,
  /// Summary of notifications: 'custom' indicates one or more notification rules available from the notifications sub-resource; 'default' indicates system default rules are used; 'disabled' indicates that no notifications will be used for this quota.
  #[serde(rename = "notifications")]
  notifications: String,
  /// The /ifs path governed.
  #[serde(rename = "path")]
  path: String,
  /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
  #[serde(rename = "persona")]
  persona: Option<::models::AuthAccessAccessItemFileGroup>,
  /// True if the accounting is accurate on the quota.  If false, this quota is waiting on completion of a QuotaScan job.
  #[serde(rename = "ready")]
  ready: bool,
  /// 
  #[serde(rename = "thresholds")]
  thresholds: ::models::QuotaQuotaThresholdsExtended,
  /// If true, thresholds apply to data plus filesystem overhead required to store the data (i.e. 'physical' usage).
  #[serde(rename = "thresholds_include_overhead")]
  thresholds_include_overhead: bool,
  /// The type of quota.
  #[serde(rename = "type")]
  _type: String,
  /// 
  #[serde(rename = "usage")]
  usage: ::models::QuotaQuotaUsage
}

