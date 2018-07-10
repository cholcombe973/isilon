#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummarySystemSystemItem {
    /// The percentage CPU utilization.
    #[serde(rename = "cpu")]
    pub cpu: f32,
    /// Traffic to disk (in bytes/sec).
    #[serde(rename = "disk_in")]
    pub disk_in: f32,
    /// Traffic from disk (in bytes/sec).
    #[serde(rename = "disk_out")]
    pub disk_out: f32,
    /// The total throughput (in bytes/sec) for FTP operations.
    #[serde(rename = "ftp")]
    pub ftp: f32,
    /// The total throughput (in bytes/second) for HDFS operations.
    #[serde(rename = "hdfs")]
    pub hdfs: f32,
    /// The total throughput (in bytes/sec) for HTTP operations.
    #[serde(rename = "http")]
    pub http: f32,
    /// Incoming network traffic (in bytes/sec) for all operations.
    #[serde(rename = "net_in")]
    pub net_in: f32,
    /// Outgoing network traffic (in bytes/sec) for all operations.
    #[serde(rename = "net_out")]
    pub net_out: f32,
    /// The total throughput (in bytes/sec) for NFS (NFS3 & NFS4) operations.
    #[serde(rename = "nfs")]
    pub nfs: f32,
    /// Node ID/LNN, 'All' for cluster.
    #[serde(rename = "node")]
    pub node: String,
    /// The total throughput (in bytes/sec) for SMB (SMB1 & SMB2) operations.
    #[serde(rename = "smb")]
    pub smb: f32,
    /// Unix Epoch time in seconds of the request.
    #[serde(rename = "time")]
    pub time: i32,
    /// The total throughput (in bytes/sec) for all protocols listed.
    #[serde(rename = "total")]
    pub total: f32,
}
