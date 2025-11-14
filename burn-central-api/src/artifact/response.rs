use serde::Deserialize;

#[derive(Deserialize)]
pub struct MultipartUploadResponse {
    pub id: String,
    pub parts: Vec<PresignedUploadUrlResponse>,
}
#[derive(Deserialize)]
pub struct PresignedUploadUrlResponse {
    pub part: u32,
    pub url: String,
    pub size_bytes: u64,
}

#[derive(Deserialize)]
pub struct PresignedArtifactFileUploadUrlsResponse {
    pub rel_path: String,
    pub urls: MultipartUploadResponse,
}

#[derive(Deserialize)]
pub struct ArtifactCreationResponse {
    pub id: String,
    pub files: Vec<PresignedArtifactFileUploadUrlsResponse>,
}

#[derive(Deserialize)]
pub struct ArtifactAddFileResponse {
    pub files: Vec<PresignedArtifactFileUploadUrlsResponse>,
}

#[derive(Deserialize)]
pub struct PresignedArtifactFileUrlResponse {
    pub rel_path: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct ArtifactDownloadResponse {
    pub files: Vec<PresignedArtifactFileUrlResponse>,
}

#[derive(Deserialize)]
pub struct ArtifactResponse {
    pub id: String,
    pub created_at: String,
    pub name: String,
    pub kind: String,
    pub bucket_id: String,
    pub experiment: ArtifactSourceResponse,
    pub manifest: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ArtifactListResponse {
    pub items: Vec<ArtifactResponse>,
    pub total: usize,
}

#[derive(Deserialize)]
pub struct ArtifactSourceResponse {
    pub id: i32,
    pub experiment_num: i32,
}
