#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStateServicelight {
    /// A list of errors encountered by the individual nodes involved in this request, or an empty list if there were no errors.
    #[serde(rename = "errors")]
    pub errors: Option<Vec <crate::models::NodeDrivesPurposelistError>>,
    /// The responses from the individual nodes involved in this request.
    #[serde(rename = "nodes")]
    pub nodes: Option<Vec <crate::models::NodeStateServicelightNode>>,
    /// The total number of nodes responding.
    #[serde(rename = "total")]
    pub total: Option<i32>,
}
