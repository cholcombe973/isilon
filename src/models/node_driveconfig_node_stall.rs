#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDriveconfigNodeStall {
    /// The amount of time in seconds with no stalls before ignoring previous stalls.
    #[serde(rename = "clear_time")]
    pub clear_time: Option<i32>,
    /// Number of stripes to read during a diskscrub.
    #[serde(rename = "diskscrub_stripes")]
    pub diskscrub_stripes: Option<i32>,
    /// The number of errors during stalled drive disk exercises to cause the drive to be softfailed.
    #[serde(rename = "max_error_frequency")]
    pub max_error_frequency: Option<i32>,
    /// The number of slow accesses during stalled drive disk exercises to cause the drive to be softfailed.
    #[serde(rename = "max_slow_access")]
    pub max_slow_access: Option<i32>,
    /// The number of slow frequency triggers during stalled drive disk exercises to cause the drive to be softfailed.
    #[serde(rename = "max_slow_frequency")]
    pub max_slow_frequency: Option<i32>,
    /// The maximum amount of time, in seconds, to remain stalled before softfailing the drive.
    #[serde(rename = "max_total_stall_time")]
    pub max_total_stall_time: Option<i32>,
    /// Maximum delay in seconds after an ECC correction during a scan.
    #[serde(rename = "scan_max_ecc_delay")]
    pub scan_max_ecc_delay: Option<i32>,
    /// Total bytes of error-free reads to complete a scan.
    #[serde(rename = "scan_size")]
    pub scan_size: Option<i32>,
    /// Delay in seconds between evaluations.
    #[serde(rename = "sleep")]
    pub sleep: Option<i32>,
}
