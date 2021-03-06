#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpDiagnostics {
    ///
    #[serde(rename = "diagnostics")]
    pub diagnostics: Option <crate::models::NdmpDiagnosticsDiagnostics>,
}
