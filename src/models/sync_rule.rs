/// SyncRule : A rule limiting the bandwidth, file operations, cpu usage, or worker count for all sync jobs on this cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRule {
    /// User-entered description of this performance rule.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether this performance rule is currently in effect during its specified intervals.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Amount the specified system resource type is limited by this rule.  Units are kb/s for bandwidth, files/s for file-count, processing percentage used for cpu, or percentage of maximum available workers.
    #[serde(rename = "limit")]
    pub limit: Option<i32>,
    /// A schedule defining when during a week this performance rule is in effect.  If unspecified or null, the schedule will always be in effect.
    #[serde(rename = "schedule")]
    pub schedule: Option<::models::SyncRuleSchedule>,
}
