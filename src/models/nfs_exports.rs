#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsExports {
    #[serde(rename = "exports")]
    pub exports: Option<Vec <crate::models::NfsExportExtended>>,
}
