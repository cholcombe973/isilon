

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryDriveDriveItem {
  /// The average operation latency.
  #[serde(rename = "access_latency")]
  access_latency: f32,
  /// The rate of slow (long-latency) operations.
  #[serde(rename = "access_slow")]
  access_slow: f32,
  /// The percentage of time the drive was busy.
  #[serde(rename = "busy")]
  busy: f32,
  /// The rate of bytes written.
  #[serde(rename = "bytes_in")]
  bytes_in: f32,
  /// The rate of bytes read.
  #[serde(rename = "bytes_out")]
  bytes_out: f32,
  /// Drive ID LNN:bay.
  #[serde(rename = "drive_id")]
  drive_id: String,
  /// The average time spent in the I/O scheduler.
  #[serde(rename = "iosched_latency")]
  iosched_latency: f32,
  /// The average length of the I/O scheduler queue.
  #[serde(rename = "iosched_queue")]
  iosched_queue: f32,
  /// Unix Epoch time in seconds of the request.
  #[serde(rename = "time")]
  time: i32,
  /// The type of the drive.
  #[serde(rename = "type")]
  _type: String,
  /// The percent of blocks used on the drive.
  #[serde(rename = "used_bytes_percent")]
  used_bytes_percent: f32,
  /// The number of inodes allocated on the drive.
  #[serde(rename = "used_inodes")]
  used_inodes: f32,
  /// The average size of write operations.
  #[serde(rename = "xfer_size_in")]
  xfer_size_in: f32,
  /// The average size of read operations.
  #[serde(rename = "xfer_size_out")]
  xfer_size_out: f32,
  /// The rate of write operations.
  #[serde(rename = "xfers_in")]
  xfers_in: f32,
  /// The rate of read operations.
  #[serde(rename = "xfers_out")]
  xfers_out: f32
}

