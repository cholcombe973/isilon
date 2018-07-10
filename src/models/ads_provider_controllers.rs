#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdsProviderControllers {
    #[serde(rename = "controllers")]
    pub controllers: Option<Vec<::models::AdsProviderControllersController>>,
}
