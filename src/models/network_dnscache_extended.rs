

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkDnscacheExtended {
  /// DNS cache entry limit
  #[serde(rename = "cache_entry_limit")]
  cache_entry_limit: Option<i32>,
  /// Timeout value for calls made to other nodes in the cluster
  #[serde(rename = "cluster_timeout")]
  cluster_timeout: Option<i32>,
  /// Timeout value for calls made to the dns resolvers
  #[serde(rename = "dns_timeout")]
  dns_timeout: Option<i32>,
  /// Lead time to refresh cache entries nearing expiration
  #[serde(rename = "eager_refresh")]
  eager_refresh: Option<i32>,
  /// Deltas for checking cbind cluster health
  #[serde(rename = "testping_delta")]
  testping_delta: Option<i32>,
  /// Upper bound on ttl for cache hits
  #[serde(rename = "ttl_max_noerror")]
  ttl_max_noerror: Option<i32>,
  /// Upper bound on ttl for nxdomain
  #[serde(rename = "ttl_max_nxdomain")]
  ttl_max_nxdomain: Option<i32>,
  /// Upper bound on ttl for non-nxdomain failures
  #[serde(rename = "ttl_max_other")]
  ttl_max_other: Option<i32>,
  /// Upper bound on ttl for server failures
  #[serde(rename = "ttl_max_servfail")]
  ttl_max_servfail: Option<i32>,
  /// Lower bound on ttl for cache hits
  #[serde(rename = "ttl_min_noerror")]
  ttl_min_noerror: Option<i32>,
  /// Lower bound on ttl for nxdomain
  #[serde(rename = "ttl_min_nxdomain")]
  ttl_min_nxdomain: Option<i32>,
  /// Lower bound on ttl for non-nxdomain failures
  #[serde(rename = "ttl_min_other")]
  ttl_min_other: Option<i32>,
  /// Lower bound on ttl for server failures
  #[serde(rename = "ttl_min_servfail")]
  ttl_min_servfail: Option<i32>
}

