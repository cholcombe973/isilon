#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsSettingsGlobalSettings {
    /// True if NFSv3 is enabled.
    #[serde(rename = "nfsv3_enabled")]
    pub nfsv3_enabled: Option<bool>,
    /// True if NFSv4 is enabled.
    #[serde(rename = "nfsv4_enabled")]
    pub nfsv4_enabled: Option<bool>,
    /// Specifies the maximum number of threads in the nfsd thread pool.
    #[serde(rename = "rpc_maxthreads")]
    pub rpc_maxthreads: Option<i32>,
    /// Specifies the minimum number of threads in the nfsd thread pool.
    #[serde(rename = "rpc_minthreads")]
    pub rpc_minthreads: Option<i32>,
    /// True if the NFS service is enabled. When set to false, the NFS service is disabled.
    #[serde(rename = "service")]
    pub service: Option<bool>,
}
