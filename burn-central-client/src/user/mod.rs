pub mod response;

use reqwest::header::SET_COOKIE;

use crate::{
    BurnCentralCredentials, Client, ClientError,
    client::ResponseExt,
    user::response::{GetUserOrganizationsResponse, UserResponseSchema},
};

impl Client {
    /// Log in to the Burn Central server with the given credentials.
    pub fn login(&self, credentials: &BurnCentralCredentials) -> Result<String, ClientError> {
        let url = self.join("login/api-key");

        let res = self
            .http_client
            .post(url)
            .form::<BurnCentralCredentials>(credentials)
            .send()?
            .map_to_burn_central_err()?;

        let cookie_header = res.headers().get(SET_COOKIE);
        if let Some(cookie) = cookie_header {
            let cookie_str = cookie
                .to_str()
                .expect("Session cookie should be able to convert to str");
            Ok(cookie_str.to_string())
        } else {
            Err(ClientError::BadSessionId)
        }
    }

    pub fn get_current_user(&self) -> Result<UserResponseSchema, ClientError> {
        let url = self.join("user");

        self.get_json::<UserResponseSchema>(url)
    }

    pub fn get_user_organizations(&self) -> Result<GetUserOrganizationsResponse, ClientError> {
        let url = self.join("user/organizations");

        self.get_json(url)
    }
}
