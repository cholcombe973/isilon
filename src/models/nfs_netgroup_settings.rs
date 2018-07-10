#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsNetgroupSettings {
    /// To-disk backup interval for the netgroup cache.
    #[serde(rename = "bgwrite")]
    pub bgwrite: Option<i32>,
    /// Time between updates of netgroups in the cache.
    #[serde(rename = "expiration")]
    pub expiration: Option<i32>,
    /// Length of time an un-accessed netgroup remains in the cache.
    #[serde(rename = "lifetime")]
    pub lifetime: Option<i32>,
    /// Retry interval for netgroup updates if the remote provider is unresponsive.
    #[serde(rename = "retry")]
    pub retry: Option<i32>,
}
