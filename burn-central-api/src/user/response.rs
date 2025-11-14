use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserResponseSchema {
    #[serde(rename = "id")]
    pub _id: i32,
    pub username: String,
    pub email: String,
    pub namespace: String,
}

#[derive(Deserialize)]
pub struct GetUserOrganizationsResponse {
    pub organizations: Vec<OrganizationResponse>,
}

#[derive(Deserialize)]
pub struct OrganizationResponse {
    pub name: String,
    pub namespace: String,
}
