#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncSettingsSettings {
    /// The per-worker burst socket memory constraint, in bytes.
    #[serde(rename = "burst_memory_constraint")]
    pub burst_memory_constraint: Option<i32>,
    /// The per-worker burst socket buffer coalesced data, in bytes.
    #[serde(rename = "burst_socket_buffer_size")]
    pub burst_socket_buffer_size: Option<i32>,
    /// NOTE: This field should not be changed without the help of Isilon support.  Default for the \"force_interface\" property that will be applied to each new sync policy unless otherwise specified at the time of policy creation.  Determines whether data is sent only through the subnet and pool specified in the \"source_network\" field. This option can be useful if there are multiple interfaces for the given source subnet.
    #[serde(rename = "force_interface")]
    pub force_interface: Option<bool>,
    /// The max concurrent jobs that SyncIQ can support. This number is based on the size of the current cluster and the current SyncIQ worker throttle rule.
    #[serde(rename = "max_concurrent_jobs")]
    pub max_concurrent_jobs: Option<i32>,
    /// Email sync reports to these addresses.
    #[serde(rename = "report_email")]
    pub report_email: Option<Vec<String>>,
    /// The default length of time (in seconds) a policy report will be stored.
    #[serde(rename = "report_max_age")]
    pub report_max_age: Option<i32>,
    /// The default maximum number of reports to retain for a policy.
    #[serde(rename = "report_max_count")]
    pub report_max_count: Option<i32>,
    /// Default for the \"restrict_target_network\" property that will be applied to each new sync policy unless otherwise specified at the time of policy creation.  If you specify true, and you specify a SmartConnect zone in the \"target_host\" field, replication policies will connect only to nodes in the specified SmartConnect zone.  If you specify false, replication policies are not restricted to specific nodes on the target cluster.
    #[serde(rename = "restrict_target_network")]
    pub restrict_target_network: Option<bool>,
    /// If disabled, no RPO alerts will be generated.
    #[serde(rename = "rpo_alerts")]
    pub rpo_alerts: Option<bool>,
    /// Specifies if the SyncIQ service currently on, paused, or off.  If paused, all sync jobs will be paused.  If turned off, all jobs will be canceled.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// Restricts replication policies on the local cluster to running on the specified subnet and pool.
    #[serde(rename = "source_network")]
    pub source_network: Option<::models::SyncPolicySourceNetwork>,
    /// The interval (in seconds) in which treewalk syncs are forced to checkpoint.
    #[serde(rename = "tw_chkpt_interval")]
    pub tw_chkpt_interval: Option<i32>,
}
