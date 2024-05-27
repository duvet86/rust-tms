use serde::{Deserialize, Serialize};
use validator::Validate;

// The user data we'll get back from Microsoft Graph.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VendorDto {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub address: String,
    pub contact_number: Option<String>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateVendorRequest {
    pub name: String,
    #[validate(length(min = 1, max = 1000))]
    pub email: String,
    pub address: String,
    pub contact_number: Option<String>,
}
