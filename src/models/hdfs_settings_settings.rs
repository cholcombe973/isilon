

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsSettingsSettings {
  /// Ambari metrics collector
  #[serde(rename = "ambari_metrics_collector")]
  ambari_metrics_collector: Option<String>,
  /// NameNode of Ambari server
  #[serde(rename = "ambari_namenode")]
  ambari_namenode: Option<String>,
  /// Ambari server
  #[serde(rename = "ambari_server")]
  ambari_server: Option<String>,
  /// Type of authentication for HDFS protocol.
  #[serde(rename = "authentication_mode")]
  authentication_mode: Option<String>,
  /// Encryption algorithm to use for data transfer (if any)
  #[serde(rename = "data_transfer_cipher")]
  data_transfer_cipher: Option<String>,
  #[serde(rename = "default_block_size")]
  default_block_size: Option<i32>,
  /// Checksum type reported by HDFS server.
  #[serde(rename = "default_checksum_type")]
  default_checksum_type: Option<String>,
  /// ODP stack repository version number
  #[serde(rename = "odp_version")]
  odp_version: Option<String>,
  /// HDFS root directory
  #[serde(rename = "root_directory")]
  root_directory: Option<String>,
  /// Enable or disable the HDFS service.
  #[serde(rename = "service")]
  service: Option<bool>,
  /// Enable or disable WebHDFS
  #[serde(rename = "webhdfs_enabled")]
  webhdfs_enabled: Option<bool>
}

