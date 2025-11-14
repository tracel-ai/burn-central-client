pub mod request;

use crate::{Client, ClientError, job::request::ComputeProviderQueueJobRequest};

impl Client {
    pub fn start_remote_job(
        &self,
        compute_provider_group_name: &str,
        owner_name: &str,
        project_name: &str,
        digest: &str,
        command: &str,
    ) -> Result<(), ClientError> {
        let url = self.join(&format!("projects/{owner_name}/{project_name}/jobs/queue"));

        let body = ComputeProviderQueueJobRequest {
            compute_provider_group_name: compute_provider_group_name.to_string(),
            digest: digest.to_string(),
            command: command.to_string(),
        };

        self.post(url, Some(body))
    }
}

