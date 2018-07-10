

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatusNodeBatterystatus {
  /// The last battery test time for battery 1.
  #[serde(rename = "last_test_time1")]
  last_test_time1: Option<String>,
  /// The last battery test time for battery 2.
  #[serde(rename = "last_test_time2")]
  last_test_time2: Option<String>,
  /// The next checkup for battery 1.
  #[serde(rename = "next_test_time1")]
  next_test_time1: Option<String>,
  /// The next checkup for battery 2.
  #[serde(rename = "next_test_time2")]
  next_test_time2: Option<String>,
  /// Node has battery status.
  #[serde(rename = "present")]
  present: Option<bool>,
  /// The result of the last battery test for battery 1.
  #[serde(rename = "result1")]
  result1: Option<String>,
  /// The result of the last battery test for battery 2.
  #[serde(rename = "result2")]
  result2: Option<String>,
  /// The status of battery 1.
  #[serde(rename = "status1")]
  status1: Option<String>,
  /// The status of battery 2.
  #[serde(rename = "status2")]
  status2: Option<String>,
  /// Node supports battery status.
  #[serde(rename = "supported")]
  supported: Option<bool>
}

