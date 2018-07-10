#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkDnscacheSettings {
    /// DNS cache entry limit
    #[serde(rename = "cache_entry_limit")]
    pub cache_entry_limit: i32,
    /// Timeout value for calls made to other nodes in the cluster
    #[serde(rename = "cluster_timeout")]
    pub cluster_timeout: i32,
    /// Timeout value for calls made to the dns resolvers
    #[serde(rename = "dns_timeout")]
    pub dns_timeout: i32,
    /// Lead time to refresh cache entries nearing expiration
    #[serde(rename = "eager_refresh")]
    pub eager_refresh: i32,
    /// Deltas for checking cbind cluster health
    #[serde(rename = "testping_delta")]
    pub testping_delta: i32,
    /// Upper bound on ttl for cache hits
    #[serde(rename = "ttl_max_noerror")]
    pub ttl_max_noerror: i32,
    /// Upper bound on ttl for nxdomain
    #[serde(rename = "ttl_max_nxdomain")]
    pub ttl_max_nxdomain: i32,
    /// Upper bound on ttl for non-nxdomain failures
    #[serde(rename = "ttl_max_other")]
    pub ttl_max_other: i32,
    /// Upper bound on ttl for server failures
    #[serde(rename = "ttl_max_servfail")]
    pub ttl_max_servfail: i32,
    /// Lower bound on ttl for cache hits
    #[serde(rename = "ttl_min_noerror")]
    pub ttl_min_noerror: i32,
    /// Lower bound on ttl for nxdomain
    #[serde(rename = "ttl_min_nxdomain")]
    pub ttl_min_nxdomain: i32,
    /// Lower bound on ttl for non-nxdomain failures
    #[serde(rename = "ttl_min_other")]
    pub ttl_min_other: i32,
    /// Lower bound on ttl for server failures
    #[serde(rename = "ttl_min_servfail")]
    pub ttl_min_servfail: i32,
}
