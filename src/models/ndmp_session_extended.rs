#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSessionExtended {
    /// Bytes transferred to/from the filesystem
    #[serde(rename = "data_bytes_transferred")]
    pub data_bytes_transferred: u64,
    /// State of the NDMP Data Service
    #[serde(rename = "data_state")]
    pub data_state: String,
    /// The path being recovered to
    #[serde(rename = "dest_path")]
    pub dest_path: String,
    /// IP address of the DMA
    #[serde(rename = "dma_ip_addr")]
    pub dma_ip_addr: String,
    /// Number of seconds elapsed since the backup was started
    #[serde(rename = "elapsed_time")]
    pub elapsed_time: i32,
    /// Unique display ID.
    #[serde(rename = "id")]
    pub id: String,
    /// Bytes transferred to/from tape or remote writer
    #[serde(rename = "mover_bytes_transferred")]
    pub mover_bytes_transferred: u64,
    /// State of the NDMP Mover Service
    #[serde(rename = "mover_state")]
    pub mover_state: String,
    /// The type of backup session
    #[serde(rename = "operation")]
    pub operation: String,
    /// IP address of the remote NDMP participant
    #[serde(rename = "remote_ip_addr")]
    pub remote_ip_addr: String,
    /// Name of the media changer device used if any
    #[serde(rename = "scsi_device")]
    pub scsi_device: String,
    /// Session ID in form <lnn>.<pid>.
    #[serde(rename = "session")]
    pub session: String,
    /// The path being backed up
    #[serde(rename = "source_path")]
    pub source_path: String,
    /// Time backup was started in seconds since epoch
    #[serde(rename = "start_time")]
    pub start_time: i32,
    /// Name of the tape device used if any
    #[serde(rename = "tape_device")]
    pub tape_device: String,
    /// Describes the mode in which the tape is opened
    #[serde(rename = "tape_open_mode")]
    pub tape_open_mode: String,
    /// The throughput in MB/s
    #[serde(rename = "throughput")]
    pub throughput: i32,
}
