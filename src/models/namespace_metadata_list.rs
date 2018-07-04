/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceMetadataList {
  #[serde(rename = "attrs")]
  attrs: Option<Vec<::models::NamespaceMetadataListAttrs>>
}

impl NamespaceMetadataList {
  pub fn new() -> NamespaceMetadataList {
    NamespaceMetadataList {
      attrs: None
    }
  }

  pub fn set_attrs(&mut self, attrs: Vec<::models::NamespaceMetadataListAttrs>) {
    self.attrs = Some(attrs);
  }

  pub fn with_attrs(mut self, attrs: Vec<::models::NamespaceMetadataListAttrs>) -> NamespaceMetadataList {
    self.attrs = Some(attrs);
    self
  }

  pub fn attrs(&self) -> Option<&Vec<::models::NamespaceMetadataListAttrs>> {
    self.attrs.as_ref()
  }

  pub fn reset_attrs(&mut self) {
    self.attrs = None;
  }

}


