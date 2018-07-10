#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersAds {
    #[serde(rename = "ads")]
    pub ads: Option<Vec<::models::ProvidersAdsAdsItem>>,
}
