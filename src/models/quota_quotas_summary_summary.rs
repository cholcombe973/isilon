#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaQuotasSummarySummary {
    /// Total number of quotas.
    #[serde(rename = "count")]
    pub count: i32,
    /// Total number of default-group quotas.
    #[serde(rename = "default_group_quotas_count")]
    pub default_group_quotas_count: i32,
    /// Total number of default-user quotas.
    #[serde(rename = "default_user_quotas_count")]
    pub default_user_quotas_count: i32,
    /// Total number of directory quotas.
    #[serde(rename = "directory_quotas_count")]
    pub directory_quotas_count: i32,
    /// Total number of -group quotas.
    #[serde(rename = "group_quotas_count")]
    pub group_quotas_count: i32,
    /// Total number of user and group totals that are linked.
    #[serde(rename = "linked_quotas_count")]
    pub linked_quotas_count: i32,
    /// Total number of user quotas.
    #[serde(rename = "user_quotas_count")]
    pub user_quotas_count: i32,
}
