

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersAds {
  #[serde(rename = "ads")]
  ads: Option<Vec<::models::ProvidersAdsAdsItem>>
}

