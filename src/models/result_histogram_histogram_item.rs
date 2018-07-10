

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultHistogramHistogramItem {
  /// Bucket for holding file counts within a range.
  #[serde(rename = "bucket")]
  bucket: i32,
  /// Number of files within the bucket.
  #[serde(rename = "value")]
  value: i32
}

