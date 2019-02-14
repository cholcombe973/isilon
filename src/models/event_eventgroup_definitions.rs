#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventgroupDefinitions {
    #[serde(rename = "eventgroup-definitions")]
    pub eventgroup_definitions:
        Option<Vec <crate::models::EventEventgroupDefinitionsEventgroupDefinition>>,
}
