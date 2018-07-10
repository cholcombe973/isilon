#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryClientClientItem {
    /// The class of the operation.
    #[serde(rename = "class")]
    pub class: String,
    /// Rate of input (in bytes/second) for an operation since the last time isi statistics collected the data.
    #[serde(rename = "in")]
    pub _in: f32,
    /// Average input (received) bytes for an operation, in bytes.
    #[serde(rename = "in_avg")]
    pub in_avg: f32,
    /// Maximum input (received) bytes for an operation, in bytes.
    #[serde(rename = "in_max")]
    pub in_max: f32,
    /// Minimum input (received) bytes for an operation, in bytes.
    #[serde(rename = "in_min")]
    pub in_min: f32,
    /// The IP address (in dotted-quad form) of the host receiving the operation request.
    #[serde(rename = "local_addr")]
    pub local_addr: String,
    /// The resolved text name of the LocalAddr, if resolution can be performed.
    #[serde(rename = "local_name")]
    pub local_name: String,
    /// The node on which the operation was performed.
    #[serde(rename = "node")]
    pub node: Option<i32>,
    /// The number of times an operation has been performed.
    #[serde(rename = "num_operations")]
    pub num_operations: i32,
    /// The rate (in ops/second) at which an operation has been performed.
    #[serde(rename = "operation_rate")]
    pub operation_rate: f32,
    /// Rate of output (in bytes/second) for an operation since the last time isi statistics collected the data.
    #[serde(rename = "out")]
    pub out: f32,
    /// Average output (sent) bytes for an operation, in bytes.
    #[serde(rename = "out_avg")]
    pub out_avg: f32,
    /// Maximum output (sent) bytes for an operation, in bytes.
    #[serde(rename = "out_max")]
    pub out_max: f32,
    /// Minimum output (sent) bytes for an operation, in bytes.
    #[serde(rename = "out_min")]
    pub out_min: f32,
    /// The protocol of the operation.
    #[serde(rename = "protocol")]
    pub protocol: String,
    /// The IP address (in dotted-quad form) of the host sending the operation request.
    #[serde(rename = "remote_addr")]
    pub remote_addr: String,
    /// The resolved text name of the RemoteAddr, if resolution can be performed.
    #[serde(rename = "remote_name")]
    pub remote_name: String,
    /// Unix Epoch time in seconds of the request.
    #[serde(rename = "time")]
    pub time: i32,
    /// The average elapsed time (in microseconds) taken to complete an operation.
    #[serde(rename = "time_avg")]
    pub time_avg: f32,
    /// The maximum elapsed time (in microseconds) taken to complete an operation.
    #[serde(rename = "time_max")]
    pub time_max: f32,
    /// The minimum elapsed time (in microseconds) taken to complete an operation.
    #[serde(rename = "time_min")]
    pub time_min: f32,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "user")]
    pub user: Option<::models::AuthAccessAccessItemFileGroup>,
}
