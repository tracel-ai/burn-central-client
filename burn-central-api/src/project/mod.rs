pub mod request;
pub mod response;

use reqwest::Url;

use crate::{
    Client, ClientError,
    project::{
        request::{
            BurnCentralCodeMetadataRequest, CodeUploadRequest, CrateVersionMetadataRequest,
            CreateProjectRequest,
        },
        response::{CodeUploadUrlsResponse, ProjectResponse},
    },
};

impl Client {
    fn create_project(
        &self,
        project_name: &str,
        project_description: Option<&str>,
        url: Url,
    ) -> Result<ProjectResponse, ClientError> {
        let project_data = CreateProjectRequest {
            name: project_name.to_string(),
            description: project_description.map(|desc| desc.to_string()),
        };

        self.post_json::<CreateProjectRequest, ProjectResponse>(url, Some(project_data))
    }

    pub fn create_user_project(
        &self,
        project_name: &str,
        project_description: Option<&str>,
    ) -> Result<ProjectResponse, ClientError> {
        let url = self.join("user/projects");
        self.create_project(project_name, project_description, url)
    }

    pub fn get_project(
        &self,
        owner_name: &str,
        project_name: &str,
    ) -> Result<ProjectResponse, ClientError> {
        let url = self.join(&format!("projects/{owner_name}/{project_name}"));

        self.get_json::<ProjectResponse>(url)
    }

    pub fn create_organization_project(
        &self,
        owner_name: &str,
        project_name: &str,
        project_description: Option<&str>,
    ) -> Result<ProjectResponse, ClientError> {
        let url = self.join(&format!("organizations/{owner_name}/projects"));
        self.create_project(project_name, project_description, url)
    }

    pub fn publish_project_version_urls(
        &self,
        owner_name: &str,
        project_name: &str,
        target_package_name: &str,
        code_metadata: BurnCentralCodeMetadataRequest,
        crates_metadata: Vec<CrateVersionMetadataRequest>,
        digest: &str,
    ) -> Result<CodeUploadUrlsResponse, ClientError> {
        let url = self.join(&format!("projects/{owner_name}/{project_name}/code/upload"));

        self.post_json(
            url,
            Some(CodeUploadRequest {
                target_package_name: target_package_name.to_string(),
                burn_central_metadata: code_metadata,
                crates: crates_metadata,
                digest: digest.to_string(),
            }),
        )
    }

    pub fn complete_project_version_upload(
        &self,
        owner_name: &str,
        project_name: &str,
        code_version_id: &str,
    ) -> Result<(), ClientError> {
        let url = self.join(&format!(
            "projects/{owner_name}/{project_name}/code/{code_version_id}/complete"
        ));

        self.post(url, None::<()>)
    }
}
