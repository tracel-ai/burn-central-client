pub mod request;
pub mod response;

use crate::{
    Client, ClientError,
    artifact::{
        request::{
            AddFilesToArtifactRequest, ArtifactFileSpecRequest, CompleteUploadRequest,
            CreateArtifactRequest,
        },
        response::{
            ArtifactAddFileResponse, ArtifactCreationResponse, ArtifactDownloadResponse,
            ArtifactListResponse, ArtifactResponse,
        },
    },
};

impl Client {
    /// Creates an artifact entry on the Burn Central server with the given files.
    ///
    /// The client must be logged in before calling this method.
    pub fn create_artifact(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
        req: CreateArtifactRequest,
    ) -> Result<ArtifactCreationResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/artifacts"
        ));

        self.post_json::<CreateArtifactRequest, ArtifactCreationResponse>(url, Some(req))
    }

    /// Add files to an existing artifact.
    ///
    /// The client must be logged in before calling this method.
    pub fn add_files_to_artifact(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
        artifact_id: &str,
        files: Vec<ArtifactFileSpecRequest>,
    ) -> Result<ArtifactAddFileResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/artifacts/{artifact_id}/files"
        ));

        self.post_json::<AddFilesToArtifactRequest, ArtifactAddFileResponse>(
            url,
            Some(AddFilesToArtifactRequest { files }),
        )
    }

    /// Complete an artifact upload.
    ///
    /// The client must be logged in before calling this method.
    ///
    /// If `file_names` is None, all files in the artifact will be marked as complete.
    /// If `file_names` is Some, only the specified files will be marked as complete.
    pub fn complete_artifact_upload(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
        artifact_id: &str,
        file_names: Option<Vec<String>>,
    ) -> Result<(), ClientError> {
        let url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/artifacts/{artifact_id}/complete"
        ));

        let body = Some(CompleteUploadRequest { file_names });
        self.post(url, body)
    }

    /// List artifacts for the given experiment.
    ///
    /// The client must be logged in before calling this method.
    pub fn list_artifacts(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
    ) -> Result<ArtifactListResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/artifacts"
        ));

        self.get_json::<ArtifactListResponse>(url)
    }

    /// Query artifacts by name for the given experiment.
    ///
    /// The client must be logged in before calling this method.
    pub fn list_artifacts_by_name(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
        name: &str,
    ) -> Result<ArtifactListResponse, ClientError> {
        let mut url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/artifacts"
        ));
        url.query_pairs_mut().append_pair("name", name);

        self.get_json::<ArtifactListResponse>(url)
    }

    /// Get details about a specific artifact by its ID.
    ///
    /// The client must be logged in before calling this method.
    pub fn get_artifact(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
        artifact_id: &str,
    ) -> Result<ArtifactResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/artifacts/{artifact_id}"
        ));

        self.get_json::<ArtifactResponse>(url)
    }

    /// Request presigned URLs to download an artifact's files from the Burn Central server.
    ///
    /// The client must be logged in before calling this method.
    pub fn presign_artifact_download(
        &self,
        owner_name: &str,
        project_name: &str,
        exp_num: i32,
        artifact_id: &str,
    ) -> Result<ArtifactDownloadResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{owner_name}/{project_name}/experiments/{exp_num}/artifacts/{artifact_id}/download"
        ));

        self.get_json::<ArtifactDownloadResponse>(url)
    }
}
