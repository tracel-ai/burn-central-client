use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct CreateModelRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ModelVersionFileSpecRequest {
    pub rel_path: String,
    pub size_bytes: u64,
    pub checksum: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct CreateModelVersionUploadRequest {
    pub files: Vec<ModelVersionFileSpecRequest>,
}

#[derive(Serialize, Clone, Debug)]
pub(crate) struct CompleteModelVersionUploadRequest {
    pub file_names: Option<Vec<String>>,
}
