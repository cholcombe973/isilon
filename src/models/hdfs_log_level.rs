#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HdfsLogLevel {
    /// Setup the HDFS service log level for this node
    #[serde(rename = "level")]
    pub level: Option<String>,
}
