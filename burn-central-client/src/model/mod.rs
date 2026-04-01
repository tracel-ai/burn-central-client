pub mod request;
pub mod response;

use crate::{
    Client, ClientError,
    model::{
        request::{
            CompleteModelVersionUploadRequest, CreateModelRequest,
            CreateModelVersionUploadRequest,
        },
        response::{
            CheckModelNameResponse, ModelDownloadResponse, ModelListResponse, ModelResponse,
            ModelVersionResponse, ModelVersionUploadResponse,
        },
    },
};

impl Client {
    /// Create a model under the given project.
    ///
    /// The client must be logged in before calling this method.
    pub fn create_model(
        &self,
        namespace: &str,
        project_name: &str,
        req: CreateModelRequest,
    ) -> Result<ModelResponse, ClientError> {
        let url = self.join(&format!("projects/{namespace}/{project_name}/models"));

        self.post_json::<CreateModelRequest, ModelResponse>(url, Some(req))
    }

    /// List models for the given project.
    ///
    /// The client must be logged in before calling this method.
    pub fn list_models(
        &self,
        namespace: &str,
        project_name: &str,
    ) -> Result<ModelListResponse, ClientError> {
        let url = self.join(&format!("projects/{namespace}/{project_name}/models"));

        self.get_json::<ModelListResponse>(url)
    }

    /// Check whether a model name is available under the given project.
    ///
    /// The client must be logged in before calling this method.
    pub fn check_model_name(
        &self,
        namespace: &str,
        project_name: &str,
        model_name: &str,
    ) -> Result<CheckModelNameResponse, ClientError> {
        let mut url = self.join(&format!("projects/{namespace}/{project_name}/models/check-name"));
        url.query_pairs_mut().append_pair("name", model_name);

        self.get_json::<CheckModelNameResponse>(url)
    }

    /// Start a direct model-version upload by requesting presigned upload URLs.
    ///
    /// The client must be logged in before calling this method.
    pub fn create_model_version_upload(
        &self,
        namespace: &str,
        project_name: &str,
        model_name: &str,
        req: CreateModelVersionUploadRequest,
    ) -> Result<ModelVersionUploadResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{namespace}/{project_name}/models/{model_name}/versions"
        ));

        self.post_json::<CreateModelVersionUploadRequest, ModelVersionUploadResponse>(
            url,
            Some(req),
        )
    }

    /// Complete a model-version upload after all presigned uploads have finished.
    ///
    /// If `file_names` is None, all files in the model version are marked as complete.
    pub fn complete_model_version_upload(
        &self,
        namespace: &str,
        project_name: &str,
        model_name: &str,
        version: u32,
        file_names: Option<Vec<String>>,
    ) -> Result<(), ClientError> {
        let url = self.join(&format!(
            "projects/{namespace}/{project_name}/models/{model_name}/versions/{version}/complete"
        ));

        self.post(
            url,
            Some(CompleteModelVersionUploadRequest { file_names }),
        )
    }

    /// Get details about a specific model.
    ///
    /// The client must be logged in before calling this method.
    pub fn get_model(
        &self,
        namespace: &str,
        project_name: &str,
        model_name: &str,
    ) -> Result<ModelResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{namespace}/{project_name}/models/{model_name}"
        ));

        self.get_json::<ModelResponse>(url)
    }

    /// Get details about a specific model version.
    ///
    /// The client must be logged in before calling this method.
    pub fn get_model_version(
        &self,
        namespace: &str,
        project_name: &str,
        model_name: &str,
        version: u32,
    ) -> Result<ModelVersionResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{namespace}/{project_name}/models/{model_name}/versions/{version}"
        ));

        self.get_json::<ModelVersionResponse>(url)
    }

    /// Generate presigned URLs for downloading model version files.
    ///
    /// The client must be logged in before calling this method.
    pub fn presign_model_download(
        &self,
        namespace: &str,
        project_name: &str,
        model_name: &str,
        version: u32,
    ) -> Result<ModelDownloadResponse, ClientError> {
        let url = self.join(&format!(
            "projects/{namespace}/{project_name}/models/{model_name}/versions/{version}/download"
        ));

        self.get_json::<ModelDownloadResponse>(url)
    }
}
