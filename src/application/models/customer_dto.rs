use serde::{Deserialize, Serialize};

// The user data we'll get back from Microsoft Graph.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerDto {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    pub surname: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    pub id: String,
}
