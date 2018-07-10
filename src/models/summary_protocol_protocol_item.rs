

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryProtocolProtocolItem {
  /// The class of the operation.
  #[serde(rename = "class")]
  class: String,
  /// Rate of input (in bytes/second) for an operation since the last time isi statistics collected the data.
  #[serde(rename = "in")]
  _in: f32,
  /// Average input (received) bytes for an operation, in bytes.
  #[serde(rename = "in_avg")]
  in_avg: f32,
  /// Maximum input (received) bytes for an operation, in bytes.
  #[serde(rename = "in_max")]
  in_max: f32,
  /// Minimum input (received) bytes for an operation, in bytes.
  #[serde(rename = "in_min")]
  in_min: f32,
  /// Standard deviation for input (received) bytes for an operation, in bytes.
  #[serde(rename = "in_standard_dev")]
  in_standard_dev: f32,
  /// The node on which the operation was performed.
  #[serde(rename = "node")]
  node: Option<i32>,
  /// The operation performed.
  #[serde(rename = "operation")]
  operation: String,
  /// The number of times an operation has been performed.
  #[serde(rename = "operation_count")]
  operation_count: i32,
  /// The rate (in ops/second) at which an operation has been performed.
  #[serde(rename = "operation_rate")]
  operation_rate: f32,
  /// Rate of output (in bytes/second) for an operation since the last time isi statistics collected the data.
  #[serde(rename = "out")]
  out: f32,
  /// Average output (sent) bytes for an operation, in bytes.
  #[serde(rename = "out_avg")]
  out_avg: f32,
  /// Maximum output (sent) bytes for an operation, in bytes.
  #[serde(rename = "out_max")]
  out_max: f32,
  /// Minimum output (sent) bytes for an operation, in bytes.
  #[serde(rename = "out_min")]
  out_min: f32,
  /// Standard deviation for output (received) bytes for an operation, in bytes.
  #[serde(rename = "out_standard_dev")]
  out_standard_dev: f32,
  /// The protocol of the operation.
  #[serde(rename = "protocol")]
  protocol: String,
  /// Unix Epoch time in seconds of the request.
  #[serde(rename = "time")]
  time: i32,
  /// The average elapsed time (in microseconds) taken to complete an operation.
  #[serde(rename = "time_avg")]
  time_avg: f32,
  /// The maximum elapsed time (in microseconds) taken to complete an operation.
  #[serde(rename = "time_max")]
  time_max: f32,
  /// The minimum elapsed time (in microseconds) taken to complete an operation.
  #[serde(rename = "time_min")]
  time_min: f32,
  /// The standard deviation time (in microseconds) taken to complete an operation.
  #[serde(rename = "time_standard_dev")]
  time_standard_dev: f32
}

