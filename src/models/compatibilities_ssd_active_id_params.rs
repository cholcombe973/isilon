#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilitiesSsdActiveIdParams {
    /// Do not delete ssd compatibility, only assess if deletion is possible.
    #[serde(rename = "assess")]
    pub assess: Option<bool>,
    /// Are we enabling or disabling count
    #[serde(rename = "count")]
    pub count: bool,
    /// The optional id of the second ssd compatibility.
    #[serde(rename = "id_2")]
    pub id_2: Option<i32>,
}
