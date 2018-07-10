

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkExternalSettings {
  /// Default client-side DNS settings for non-multitenancy aware programs
  #[serde(rename = "default_groupnet")]
  default_groupnet: String,
  /// Enable or disable Source Based Routing (Defaults to false)
  #[serde(rename = "sbr")]
  sbr: bool,
  /// Delay in seconds for IP rebalance.
  #[serde(rename = "sc_rebalance_delay")]
  sc_rebalance_delay: i32,
  /// List of client TCP ports.
  #[serde(rename = "tcp_ports")]
  tcp_ports: Vec<i32>
}

