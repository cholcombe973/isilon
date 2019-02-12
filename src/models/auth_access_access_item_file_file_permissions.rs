#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAccessAccessItemFileFilePermissions {
    /// Returns a status message if the Null ACL is set.
    #[serde(rename = "dacl")]
    pub dacl: Option<String>,
    /// Returns a status message if the parent directoryhas the delete_child property set for the user.If the delete_child property is set for a user,that user is able to delete the file.the delete_child for the user.
    #[serde(rename = "delete_child")]
    pub delete_child: Option<String>,
    /// Specifies the Access Control Entry (ACE) for the user.
    #[serde(rename = "expected")]
    pub expected: Option<String>,
    /// Specifies the mode bits on the file.
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    /// Returns a status message if the user owns the file.
    #[serde(rename = "ownership")]
    pub ownership: Option<String>,
    /// Specifies a list of the relevant Access Control Entrieswith respect to the user in the share.
    #[serde(rename = "relevant_aces")]
    pub relevant_aces:
        Option<Vec <crate::models::AuthAccessAccessItemShareSharePermissionsShareRelevantAce>>,
    /// Specifies the mode bits that are related to the user.
    #[serde(rename = "relevant_mode")]
    pub relevant_mode: Option<String>,
    /// Returns a status message if the user owns the file.
    #[serde(rename = "sticky")]
    pub sticky: Option<String>,
}
