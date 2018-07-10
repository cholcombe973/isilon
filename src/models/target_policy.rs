#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetPolicy {
    /// The condition of this policy with respect to sync failover/failback.
    #[serde(rename = "failover_failback_state")]
    pub failover_failback_state: String,
    /// The system ID given to this sync policy.
    #[serde(rename = "id")]
    pub id: String,
    /// The state of the last job run for this policy.
    #[serde(rename = "last_job_state")]
    pub last_job_state: String,
    /// The IP address from which a SyncIQ coordinator daemon most recently connected to this cluster to update it about the progress of a job for this policy.
    #[serde(rename = "last_source_coordinator_ip")]
    pub last_source_coordinator_ip: String,
    /// The last time this cluster was updated with sync information from the source cluster for this policy, in unix epoch seconds.  Null if no such update has occurred yet.
    #[serde(rename = "last_update_from_source")]
    pub last_update_from_source: Option<i32>,
    /// Was this policy defined by a OneFS version earlier than 6.0? (Pre-6.0 policies did not have the target policy concept and canceling from the target side will not be available.)
    #[serde(rename = "legacy_policy")]
    pub legacy_policy: bool,
    /// User-assigned name of this sync policy.
    #[serde(rename = "name")]
    pub name: String,
    /// Unique identifier for the source cluster.
    #[serde(rename = "source_cluster_guid")]
    pub source_cluster_guid: String,
    /// Hostname or IP address of sync source cluster.
    #[serde(rename = "source_host")]
    pub source_host: String,
    /// Absolute filesystem path on the target cluster for the sync destination.
    #[serde(rename = "target_path")]
    pub target_path: String,
}
