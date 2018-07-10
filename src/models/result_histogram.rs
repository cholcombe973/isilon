#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultHistogram {
    /// Access time enabled.
    #[serde(rename = "atime_enabled")]
    pub atime_enabled: bool,
    /// User attribute count.
    #[serde(rename = "attribute_count")]
    pub attribute_count: i32,
    /// Unix Epoch time of start of results collection job.
    #[serde(rename = "begin_time")]
    pub begin_time: i32,
    /// Histogram data of specified file count parameter.
    #[serde(rename = "histogram")]
    pub histogram: Vec<::models::ResultHistogramHistogramItem>,
}
