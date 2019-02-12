/// NdmpContextsBre : View a NDMP restartable backup Context

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpContextsBre {
    /// Backup type
    #[serde(rename = "backup type")]
    pub backup_type: Option<String>,
    /// Backup Context ID
    #[serde(rename = "bkp_context_id")]
    pub bkp_context_id: Option<String>,
    /// Unique ID of NDMP BRE context
    #[serde(rename = "bre_context_id")]
    pub bre_context_id: Option<String>,
    /// Context creation time
    #[serde(rename = "create_time")]
    pub create_time: Option<i32>,
    /// List of environment variables for restartable backup
    #[serde(rename = "env_variables")]
    pub env_variables: Option<Vec <crate::models::NdmpContextsBreEnvVariable>>,
    /// Unique display id.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Backup result
    #[serde(rename = "results")]
    pub results: Option<i32>,
    /// Snapshot name of backup
    #[serde(rename = "snap_name")]
    pub snap_name: Option<String>,
    /// Context status bits
    #[serde(rename = "status")]
    pub status: Option<i32>,
    /// Backup Stream ID
    #[serde(rename = "stream_id")]
    pub stream_id: Option<i32>,
}
