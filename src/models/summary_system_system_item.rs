

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummarySystemSystemItem {
  /// The percentage CPU utilization.
  #[serde(rename = "cpu")]
  cpu: f32,
  /// Traffic to disk (in bytes/sec).
  #[serde(rename = "disk_in")]
  disk_in: f32,
  /// Traffic from disk (in bytes/sec).
  #[serde(rename = "disk_out")]
  disk_out: f32,
  /// The total throughput (in bytes/sec) for FTP operations.
  #[serde(rename = "ftp")]
  ftp: f32,
  /// The total throughput (in bytes/second) for HDFS operations.
  #[serde(rename = "hdfs")]
  hdfs: f32,
  /// The total throughput (in bytes/sec) for HTTP operations.
  #[serde(rename = "http")]
  http: f32,
  /// Incoming network traffic (in bytes/sec) for all operations.
  #[serde(rename = "net_in")]
  net_in: f32,
  /// Outgoing network traffic (in bytes/sec) for all operations.
  #[serde(rename = "net_out")]
  net_out: f32,
  /// The total throughput (in bytes/sec) for NFS (NFS3 & NFS4) operations.
  #[serde(rename = "nfs")]
  nfs: f32,
  /// Node ID/LNN, 'All' for cluster.
  #[serde(rename = "node")]
  node: String,
  /// The total throughput (in bytes/sec) for SMB (SMB1 & SMB2) operations.
  #[serde(rename = "smb")]
  smb: f32,
  /// Unix Epoch time in seconds of the request.
  #[serde(rename = "time")]
  time: i32,
  /// The total throughput (in bytes/sec) for all protocols listed.
  #[serde(rename = "total")]
  total: f32
}

