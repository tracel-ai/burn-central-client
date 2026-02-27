//! Fleet management client for Burn Central.

pub mod request;
pub mod response;

use crate::ClientError;
use crate::Env;
use crate::client::ResponseExt;
use request::{
    DownloadModelRequest, IngestTelemetryRequest, SyncDeviceRequest, TelemetryIngestionEvents,
};
use reqwest::Url;
use response::{FleetModelDownloadResponse, FleetSyncSnapshotResponse};

/// A client for interacting with the Burn Central Fleet API.
#[derive(Debug, Clone)]
pub struct FleetClient {
    http_client: reqwest::blocking::Client,
    base_url: Url,
}

impl FleetClient {
    /// Create a new FleetClient for the given environment.
    pub fn new(env: Env) -> Self {
        FleetClient {
            http_client: reqwest::blocking::Client::new(),
            base_url: env.get_url(),
        }
    }

    /// Create a FleetClient with a custom base URL.
    pub fn from_url(url: Url) -> Self {
        FleetClient {
            http_client: reqwest::blocking::Client::new(),
            base_url: url,
        }
    }

    /// Sync device state with the fleet.
    pub fn sync(
        &self,
        registration_token: impl Into<String>,
        identity_key: impl Into<String>,
        metadata: Option<serde_json::Value>,
    ) -> Result<FleetSyncSnapshotResponse, ClientError> {
        let request = SyncDeviceRequest {
            registration_token: registration_token.into(),
            identity_key: identity_key.into(),
            metadata,
        };

        self.post_json("fleets/device/sync", Some(request))
    }

    /// Get the model download information for the device's assigned fleet.
    pub fn model_download(
        &self,
        registration_token: impl Into<String>,
        identity_key: impl Into<String>,
    ) -> Result<FleetModelDownloadResponse, ClientError> {
        let request = DownloadModelRequest {
            registration_token: registration_token.into(),
            identity_key: identity_key.into(),
        };

        self.post_json("fleets/device/model/download", Some(request))
    }

    /// Ingest telemetry events for a fleet device.
    pub fn ingest_telemetry(
        &self,
        registration_token: impl Into<String>,
        identity_key: impl Into<String>,
        events: TelemetryIngestionEvents,
    ) -> Result<(), ClientError> {
        let request = IngestTelemetryRequest {
            registration_token: registration_token.into(),
            identity_key: identity_key.into(),
            events,
        };

        self.post("fleets/device/telemetry", Some(request))
    }

    fn post<T>(&self, path: impl AsRef<str>, body: Option<T>) -> Result<(), ClientError>
    where
        T: serde::Serialize,
    {
        self.req(reqwest::Method::POST, path, body).map(|_| ())
    }

    fn post_json<T, R>(&self, path: impl AsRef<str>, body: Option<T>) -> Result<R, ClientError>
    where
        T: serde::Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let response = self.req(reqwest::Method::POST, path, body)?;
        let bytes = response.bytes()?;
        let json = serde_json::from_slice::<R>(&bytes)?;
        Ok(json)
    }

    fn req<T: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: impl AsRef<str>,
        body: Option<T>,
    ) -> Result<reqwest::blocking::Response, ClientError> {
        let url = self.join(path.as_ref());
        let request_builder = self.http_client.request(method, url);

        let request_builder = if let Some(body) = body {
            request_builder
                .body(serde_json::to_vec(&body)?)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
        } else {
            request_builder
        };

        let request_builder = request_builder.header("X-SDK-Version", env!("CARGO_PKG_VERSION"));

        tracing::debug!("Sending fleet request: {:?}", request_builder);

        let response = request_builder.send()?.map_to_burn_central_err()?;

        tracing::debug!("Received fleet response: {:?}", response);

        Ok(response)
    }

    fn join(&self, path: &str) -> Url {
        self.join_versioned(path, 1)
    }

    fn join_versioned(&self, path: &str, version: u8) -> Url {
        self.base_url
            .join(&format!("v{version}/"))
            .unwrap()
            .join(path)
            .expect("Should be able to join url")
    }
}
