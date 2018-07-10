
/// ClusterConfig : General cluster information.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterConfig {
  /// Customer configurable description.
  #[serde(rename = "description")]
  description: String,
  #[serde(rename = "devices")]
  devices: Vec<::models::ClusterConfigDevice>,
  /// Default encoding.
  #[serde(rename = "encoding")]
  encoding: String,
  /// Cluster GUID.
  #[serde(rename = "guid")]
  guid: String,
  /// If true, the local node is in a group with quorum: It is communicating, storing, and protecting data normally with other nodes in its group.  Under normal circumstances, every node in the cluster is part of one group.
  #[serde(rename = "has_quorum")]
  has_quorum: bool,
  /// If true, the cluster is in compliance mode.  Compliance mode clusters do not allow root access and have stricter WORM (Write Once Read Many) features for retaining data in compliance with U.S. Securities and Exchange Commission rule 17a-4.
  #[serde(rename = "is_compliance")]
  is_compliance: bool,
  /// true if the cluster is deployed on a desktop VMWorkstation
  #[serde(rename = "is_virtual")]
  is_virtual: bool,
  /// true if this is a vOneFS cluster on an ESXi
  #[serde(rename = "is_vonefs")]
  is_vonefs: bool,
  /// Node join mode: 'manual' or 'secure'.
  #[serde(rename = "join_mode")]
  join_mode: String,
  /// Device ID of the queried node.
  #[serde(rename = "local_devid")]
  local_devid: i32,
  /// Device logical node number of the queried node.
  #[serde(rename = "local_lnn")]
  local_lnn: i32,
  /// Device serial number of the queried node.
  #[serde(rename = "local_serial")]
  local_serial: String,
  /// Cluster name.
  #[serde(rename = "name")]
  name: String,
  /// 
  #[serde(rename = "onefs_version")]
  onefs_version: Option<::models::ClusterConfigOnefsVersion>,
  /// The cluster timezone settings.
  #[serde(rename = "timezone")]
  timezone: Option<::models::ClusterConfigTimezone>,
  #[serde(rename = "upgrade_type")]
  upgrade_type: Option<String>
}

