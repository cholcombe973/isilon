

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkExternalExtended {
  /// Enable or disable Source Based Routing (Defaults to false)
  #[serde(rename = "sbr")]
  sbr: Option<bool>,
  /// Delay in seconds for IP rebalance.
  #[serde(rename = "sc_rebalance_delay")]
  sc_rebalance_delay: Option<i32>,
  /// List of client TCP ports.
  #[serde(rename = "tcp_ports")]
  tcp_ports: Option<Vec<i32>>
}

