use reqwest::{multipart, Client};
use std::path::Path;

use crate::error::SnippError;
use crate::models::*;

const BASE_URL: &str = "https://api.snipp.gg";

#[derive(Debug, Clone)]
pub struct SnippClient {
    api_key: String,
    http: Client,
}

impl SnippClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            http: Client::new(),
        }
    }

    pub async fn get_user(
        &self,
        id: &str,
        options: Option<GetUserOptions>,
    ) -> Result<UserResponse, SnippError> {
        let url = format!("{BASE_URL}/users/{id}");
        let mut req = self.http.get(&url).header("api-key", &self.api_key);

        if let Some(opts) = options {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(include) = opts.include_posts {
                params.push(("includePosts", include.to_string()));
            }
            if let Some(limit) = opts.posts_limit {
                params.push(("postsLimit", limit.to_string()));
            }
            if !params.is_empty() {
                req = req.query(&params);
            }
        }

        let resp = req.send().await?;
        Self::handle_response(resp).await
    }

    pub async fn get_post(&self, code: &str) -> Result<GetPostResponse, SnippError> {
        let resp = self
            .http
            .get(format!("{BASE_URL}/posts/{code}"))
            .header("api-key", &self.api_key)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    pub async fn upload(
        &self,
        file_path: impl AsRef<Path>,
        privacy: Option<Privacy>,
    ) -> Result<UploadResponse, SnippError> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let bytes = tokio::fs::read(path).await?;
        let part = multipart::Part::bytes(bytes).file_name(file_name);
        let form = multipart::Form::new().part("file", part);

        let mut req = self
            .http
            .post(format!("{BASE_URL}/upload"))
            .header("api-key", &self.api_key)
            .multipart(form);

        if let Some(p) = privacy {
            req = req.header("postprivacy", p.to_string());
        }

        let resp = req.send().await?;
        Self::handle_response(resp).await
    }

    pub async fn list_uploads(&self) -> Result<UploadsResponse, SnippError> {
        let resp = self
            .http
            .get(format!("{BASE_URL}/uploads"))
            .header("api-key", &self.api_key)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    pub async fn edit_upload(
        &self,
        code: &str,
        options: EditUploadOptions,
    ) -> Result<EditUploadResponse, SnippError> {
        let mut req = self
            .http
            .patch(format!("{BASE_URL}/editUpload"))
            .header("api-key", &self.api_key)
            .header("code", code);

        if let Some(title) = &options.title {
            req = req.header("title", title.as_str());
        }
        if let Some(description) = &options.description {
            req = req.header("description", description.as_str());
        }
        if let Some(privacy) = &options.privacy {
            req = req.header("postprivacy", privacy.to_string());
        }

        let resp = req.send().await?;
        Self::handle_response(resp).await
    }

    pub async fn delete_upload(&self, filename: &str) -> Result<serde_json::Value, SnippError> {
        let resp = self
            .http
            .delete(format!("{BASE_URL}/deleteUpload"))
            .header("api-key", &self.api_key)
            .header("file", filename)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    pub async fn discover(&self) -> Result<DiscoverResponse, SnippError> {
        let resp = self
            .http
            .get(format!("{BASE_URL}/discover"))
            .header("api-key", &self.api_key)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    /// List blocked users.
    pub async fn list_blocks(&self) -> Result<BlocksResponse, SnippError> {
        let resp = self
            .http
            .get(format!("{BASE_URL}/blocks"))
            .header("api-key", &self.api_key)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    /// Block a user by their ID.
    pub async fn block_user(&self, target_id: &str) -> Result<BlockResponse, SnippError> {
        let body = BlockRequest {
            target_id: target_id.to_string(),
        };

        let resp = self
            .http
            .post(format!("{BASE_URL}/block"))
            .header("api-key", &self.api_key)
            .json(&body)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    /// Unblock a user by their ID.
    pub async fn unblock_user(&self, target_id: &str) -> Result<BlockResponse, SnippError> {
        let body = BlockRequest {
            target_id: target_id.to_string(),
        };

        let resp = self
            .http
            .post(format!("{BASE_URL}/unblock"))
            .header("api-key", &self.api_key)
            .json(&body)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    pub async fn report_post(&self, code: &str, reason: &str) -> Result<ReportResponse, SnippError> {
        let body = ReportRequest {
            code: code.to_string(),
            reason: reason.to_string(),
        };

        let resp = self
            .http
            .post(format!("{BASE_URL}/reports"))
            .header("api-key", &self.api_key)
            .json(&body)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        resp: reqwest::Response,
    ) -> Result<T, SnippError> {
        let status = resp.status();
        if !status.is_success() {
            let message = resp.text().await.unwrap_or_default();
            return Err(SnippError::Api {
                status: status.as_u16(),
                message,
            });
        }
        let body = resp.text().await?;
        let parsed = serde_json::from_str(&body)?;
        Ok(parsed)
    }
}
