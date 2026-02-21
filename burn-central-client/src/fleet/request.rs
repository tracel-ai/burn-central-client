use serde::{Deserialize, Serialize};

/// Request to sync device state with the fleet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDeviceRequest {
    /// Fleet registration token provided by the fleet administrator.
    pub registration_token: String,
    /// Unique identity key for the device (stable across runs).
    pub identity_key: String,
    /// Optional updated metadata about the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Request to download model files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadModelRequest {
    /// Fleet registration token provided by the fleet administrator.
    pub registration_token: String,
    /// Unique identity key for the device (stable across runs).
    pub identity_key: String,
}
