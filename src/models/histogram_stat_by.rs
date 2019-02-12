#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistogramStatBy {
    /// Unix Epoch time of start of results collection job.
    #[serde(rename = "begin_time")]
    pub begin_time: i32,
    /// Histogram breakout data according to breakout parameter.
    #[serde(rename = "breakouts")]
    pub breakouts: Vec <crate::models::HistogramStatByBreakout>,
    /// Total length of the result list.
    #[serde(rename = "result_length")]
    pub result_length: i32,
}
