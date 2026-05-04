use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::credentials::BurnCentralCredentials;
use crate::error::{ApiErrorBody, ApiErrorCode, ClientError};
use crate::transport::{ApiTransport, Auth};

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        match error.status() {
            Some(status) => ClientError::ApiError {
                status,
                body: ApiErrorBody {
                    code: ApiErrorCode::Unknown,
                    message: error.to_string(),
                },
            },
            None => ClientError::UnknownError(error.to_string()),
        }
    }
}

/// A client for making HTTP requests to the Burn Central API.
///
/// The client can be used to interact with the Burn Central server, such as creating and starting experiments, saving and loading checkpoints, and uploading logs.
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) transport: ApiTransport,
    pub(crate) env: Env,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Env {
    Production,
    Staging(u8),
    Development,
}

impl Env {
    pub fn get_url(&self) -> Url {
        match self {
            Env::Production => Url::parse("https://central.burn.dev/api/").unwrap(),
            Env::Staging(version) => {
                Url::parse(&format!("https://s{}-central.burn.dev/api/", version)).unwrap()
            }
            Env::Development => Url::parse("http://localhost:9001/").unwrap(),
        }
    }
}

impl Client {
    /// Create a new HttpClient with the given base URL and API key.
    pub fn new(env: Env, credentials: &BurnCentralCredentials) -> Result<Self, ClientError> {
        let mut client = Client {
            transport: ApiTransport::new(env.get_url()),
            env: env.clone(),
        };

        let cookie = client.login(credentials)?;
        client.transport.set_auth(Auth::SessionCookie(cookie));
        Ok(client)
    }

    #[deprecated]
    /// Please use environment based constructor
    pub fn from_url(url: Url, credentials: &BurnCentralCredentials) -> Result<Self, ClientError> {
        let mut client = Client {
            transport: ApiTransport::new(url),
            env: Env::Production,
        };

        let cookie = client.login(credentials)?;
        client.transport.set_auth(Auth::SessionCookie(cookie));
        Ok(client)
    }

    #[deprecated]
    /// Please use environment instead of url
    pub fn get_endpoint(&self) -> &Url {
        self.transport.base_url()
    }

    pub fn get_env(&self) -> &Env {
        &self.env
    }

    /// Generic method to upload bytes to the given URL.
    pub fn upload_bytes_to_url(&self, url: &str, bytes: Vec<u8>) -> Result<(), ClientError> {
        self.transport.upload_bytes_to_url(url, bytes)
    }

    /// Generic method to download bytes from the given URL.
    pub fn download_bytes_from_url(&self, url: &str) -> Result<Vec<u8>, ClientError> {
        self.transport.download_bytes_from_url(url)
    }
}
