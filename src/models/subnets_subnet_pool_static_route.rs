

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubnetsSubnetPoolStaticRoute {
  /// Address of the gateway in the format: yyy.yyy.yyy.yyy
  #[serde(rename = "gateway")]
  gateway: String,
  /// Prefix length in the format: nn.
  #[serde(rename = "prefixlen")]
  prefixlen: i32,
  /// Network address in the format: xxx.xxx.xxx.xxx
  #[serde(rename = "subnet")]
  subnet: String
}

