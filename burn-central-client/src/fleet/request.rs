use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Supported metric payload variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
pub enum MetricData {
    Gauge {
        value: f64,
    },
    Counter {
        value: f64,
    },
    Histogram {
        count: u64,
        sum: f64,
        buckets: Vec<(f64, u64)>,
    },
}

/// A telemetry metric sample to ingest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricIngestionEvent {
    pub name: String,
    pub timestamp_ms: i64,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    #[serde(flatten)]
    pub data: MetricData,
}

/// Optional metadata/descriptor attached to a metric name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDescriptorIngestionEvent {
    pub name: String,
    pub kind: String,
    pub unit: Option<String>,
    pub description: Option<String>,
}

/// A log line to ingest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogIngestionEvent {
    pub timestamp_ms: i64,
    pub level: String,
    pub message: String,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

/// Envelope for fleet telemetry ingestion.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryIngestionEvents {
    #[serde(default)]
    pub metrics: Vec<MetricIngestionEvent>,
    #[serde(default)]
    pub metric_descriptors: Vec<MetricDescriptorIngestionEvent>,
    #[serde(default)]
    pub logs: Vec<LogIngestionEvent>,
}

/// Request to ingest telemetry for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestTelemetryRequest {
    /// Fleet registration token provided by the fleet administrator.
    pub registration_token: String,
    /// Unique identity key for the device (stable across runs).
    pub identity_key: String,
    /// Telemetry events to ingest.
    pub events: TelemetryIngestionEvents,
}
