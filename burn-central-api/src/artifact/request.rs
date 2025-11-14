use serde::Serialize;

#[derive(Serialize)]
pub struct ArtifactFileSpecRequest {
    pub rel_path: String,
    pub size_bytes: u64,
    pub checksum: String,
}

#[derive(Serialize)]
pub struct CreateArtifactRequest {
    pub name: String,
    pub kind: String,
    pub files: Vec<ArtifactFileSpecRequest>,
}

#[derive(Serialize)]
pub struct AddFilesToArtifactRequest {
    pub files: Vec<ArtifactFileSpecRequest>,
}

#[derive(Serialize)]
pub struct CompleteUploadRequest {
    pub file_names: Option<Vec<String>>,
}
