use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExperimentRequest {
    pub description: Option<String>,
    pub routine_run: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListExperimentsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSummaryQuery {
    pub metric: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregatedQuery {
    pub metric: String,
    pub max_points: i64,
    pub downsampling_factor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactFileSpecRequest {
    pub rel_path: String,
    pub size_bytes: u64,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArtifactRequest {
    pub name: String,
    pub kind: String,
    pub files: Vec<ArtifactFileSpecRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFilesRequest {
    pub files: Vec<ArtifactFileSpecRequest>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompleteUploadRequest {
    pub file_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListArtifactsQuery {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogUrlsQuery {
    pub start: Option<u64>,
}
