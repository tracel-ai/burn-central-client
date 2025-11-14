use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RegisteredFunctionRequest {
    pub mod_path: String,
    pub fn_name: String,
    pub proc_type: String,
    pub code: String,
    pub routine: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct BurnCentralCodeMetadataRequest {
    pub functions: Vec<RegisteredFunctionRequest>,
}

#[derive(Debug, Serialize)]
pub struct CodeUploadRequest {
    pub target_package_name: String,
    pub burn_central_metadata: BurnCentralCodeMetadataRequest,
    pub crates: Vec<CrateVersionMetadataRequest>,
    pub digest: String,
}

#[derive(Debug, Serialize)]
pub struct CrateVersionMetadataRequest {
    pub checksum: String,
    pub metadata: serde_json::Value,
    pub size: u64,
}
