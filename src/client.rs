use reqwest::{multipart, Client};
use std::path::Path;

use crate::error::SnippError;
use crate::models::*;

const BASE_URL: &str = "https://api.snipp.gg";
const REGIONS: [&str; 2] = ["eu-west-1", "us-west-1"];

#[derive(Debug, Clone)]
pub struct SnippClient {
    api_key: String,
    base_url: String,
    http: Client,
}

impl SnippClient {
    /// Create a new client with a Snipp API key.
    ///
    /// Requests go to `api.snipp.gg`, which routes to the nearest region.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: BASE_URL.to_string(),
            http: Client::new(),
        }
    }

    /// Create a client pinned to a regional endpoint, one of `eu-west-1` or
    /// `us-west-1`.
    ///
    /// Uploads run at the same speed either way; this controls which region
    /// stores your files.
    ///
    /// Returns [`SnippError::Validation`] if the region is not recognized.
    pub fn with_region(api_key: impl Into<String>, region: &str) -> Result<Self, SnippError> {
        if !REGIONS.contains(&region) {
            return Err(SnippError::Validation(format!(
                "Unknown region {region:?}. Expected one of: {}",
                REGIONS.join(", ")
            )));
        }
        Ok(Self {
            api_key: api_key.into(),
            base_url: format!("https://{region}.api.snipp.gg"),
            http: Client::new(),
        })
    }

    /// Get a user by ID. Use `@me` for the authenticated user.
    pub async fn get_user(
        &self,
        id: &str,
        options: Option<GetUserOptions>,
    ) -> Result<UserResponse, SnippError> {
        let url = format!("{}/users/{id}", self.base_url);
        let mut req = self.http.get(&url).header("api-key", &self.api_key);

        if let Some(opts) = options {
            let mut params: Vec<(&str, String)> = Vec::new();
            if let Some(include) = opts.include_posts {
                params.push(("include_posts", include.to_string()));
            }
            if let Some(limit) = opts.posts_limit {
                params.push(("posts_limit", limit.to_string()));
            }
            if !params.is_empty() {
                req = req.query(&params);
            }
        }

        let resp = req.send().await?;
        Self::handle_response(resp).await
    }

    /// Get a post by its share code. Team posts are only readable by members
    /// of that team, and carry no `like_count`.
    pub async fn get_post(&self, code: &str) -> Result<GetPostResponse, SnippError> {
        let resp = self
            .http
            .get(format!("{}/posts/{code}", self.base_url))
            .header("api-key", &self.api_key)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    /// Upload a file from disk. Privacy defaults to the server default
    /// (`private`) when omitted. Titles cap at 30 chars, descriptions at 200.
    pub async fn upload(
        &self,
        file_path: impl AsRef<Path>,
        options: Option<UploadOptions>,
    ) -> Result<UploadResponse, SnippError> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let bytes = tokio::fs::read(path).await?;
        let part = multipart::Part::bytes(bytes).file_name(file_name);
        let mut form = multipart::Form::new().part("file", part);

        let mut req = self
            .http
            .post(format!("{}/upload", self.base_url))
            .header("api-key", &self.api_key);

        if let Some(opts) = options {
            if let Some(p) = opts.privacy {
                req = req.header("post-privacy", p.to_string());
            }
            if let Some(t) = &opts.title {
                form = form.text("title", t.clone());
            }
            if let Some(d) = &opts.description {
                form = form.text("description", d.clone());
            }
            if let Some(pt) = opts.post_type {
                req = req.header("post-type", pt.to_string());
            }
        }

        let req = req.multipart(form);

        let resp = req.send().await?;
        Self::handle_response(resp).await
    }

    /// List the authenticated user's recent uploads (limit 1-1000).
    pub async fn list_uploads(&self, limit: Option<u32>) -> Result<UploadsResponse, SnippError> {
        let mut req = self
            .http
            .get(format!("{}/uploads", self.base_url))
            .header("api-key", &self.api_key);
        if let Some(n) = limit {
            req = req.query(&[("limit", n.to_string())]);
        }
        let resp = req.send().await?;
        Self::handle_response(resp).await
    }

    /// Edit an existing upload's title, description, or privacy.
    /// Empty strings clear the title or description.
    pub async fn edit_upload(
        &self,
        code: &str,
        options: EditUploadOptions,
    ) -> Result<EditUploadResponse, SnippError> {
        let mut req = self
            .http
            .patch(format!("{}/editUpload", self.base_url))
            .header("api-key", &self.api_key)
            .header("code", code);

        let mut form = multipart::Form::new();
        if let Some(title) = &options.title {
            form = form.text("title", title.clone());
        }
        if let Some(description) = &options.description {
            form = form.text("description", description.clone());
        }
        if let Some(privacy) = &options.privacy {
            req = req.header("post-privacy", privacy.to_string());
        }

        let resp = req.multipart(form).send().await?;
        Self::handle_response(resp).await
    }

    /// Append 1 or more files to an existing album post. The post's share
    /// code, privacy, title, and description are preserved. Albums cap at 50
    /// files total; requests that would exceed the cap are rejected. New
    /// files inherit the post's privacy; returned URLs are signed with a
    /// 24-hour expiry for private posts.
    pub async fn append_upload<P: AsRef<Path>>(
        &self,
        code: &str,
        file_paths: &[P],
    ) -> Result<AppendUploadResponse, SnippError> {
        if code.is_empty() {
            return Err(SnippError::Validation("code is required".to_string()));
        }
        if file_paths.is_empty() {
            return Err(SnippError::Validation("file_paths must not be empty".to_string()));
        }

        let mut form = multipart::Form::new();
        for path in file_paths {
            let path = path.as_ref();
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let bytes = tokio::fs::read(path).await?;
            let part = multipart::Part::bytes(bytes).file_name(file_name);
            form = form.part("file", part);
        }

        let resp = self
            .http
            .post(format!("{}/appendUpload", self.base_url))
            .header("api-key", &self.api_key)
            .header("post-code", code)
            .multipart(form)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    /// Delete an upload by filename.
    pub async fn delete_upload(&self, filename: &str) -> Result<serde_json::Value, SnippError> {
        let resp = self
            .http
            .delete(format!("{}/deleteUpload", self.base_url))
            .header("api-key", &self.api_key)
            .header("file", filename)
            .send()
            .await?;

        Self::handle_response(resp).await
    }

    /// Report a post. The reason is optional (max 200 chars); pass an empty
    /// string to omit it.
    pub async fn report_post(&self, code: &str, reason: &str) -> Result<ReportResponse, SnippError> {
        let body = ReportRequest {
            code: code.to_string(),
            reason: reason.to_string(),
        };

        let resp = self
            .http
            .post(format!("{}/report-post", self.base_url))
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
            let raw = resp.text().await.unwrap_or_default();
            let body = serde_json::from_str::<serde_json::Value>(&raw).ok();
            let message = body
                .as_ref()
                .and_then(|v| {
                    v.get("error")
                        .or_else(|| v.get("message"))
                        .and_then(|s| s.as_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or(raw);
            return Err(SnippError::Api {
                status: status.as_u16(),
                message,
                body,
            });
        }
        let body = resp.text().await?;
        let parsed = serde_json::from_str(&body)?;
        Ok(parsed)
    }
}
