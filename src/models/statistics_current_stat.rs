#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsCurrentStat {
    /// Devid of node of statistic or 0 for cluster scoped statistics.
    #[serde(rename = "devid")]
    pub devid: i32,
    /// Key specific error string, if applicable.
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// Key specific error number, if applicable.
    #[serde(rename = "error_code")]
    pub error_code: Option<i32>,
    /// Key name of statistic.
    #[serde(rename = "key")]
    pub key: String,
    /// Unix Epoch time in seconds that statistic was collected.
    #[serde(rename = "time")]
    pub time: i32,
    /// Key dependent value.
    #[serde(rename = "value")]
    pub value: Option<String>,
}
