#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelistLinsCtime {
    /// Nanoseconds component of timespec.
    #[serde(rename = "nsec")]
    pub nsec: Option<i32>,
    /// Seconds component of timespec.
    #[serde(rename = "sec")]
    pub sec: i32,
}
