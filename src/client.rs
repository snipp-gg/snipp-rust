use reqwest::{multipart, Client, Method, RequestBuilder, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::path::Path;

use crate::error::SnippError;
use crate::models::*;

const BASE_URL: &str = "https://api.snipp.gg";
const REGIONS: [&str; 2] = ["eu-west-1", "us-west-1"];

#[derive(Serialize)]
struct ReportBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<&'a str>,
}

/// Async client for the Snipp API.
#[derive(Debug, Clone)]
pub struct SnippClient {
    api_key: String,
    base_url: Url,
    http: Client,
}

impl SnippClient {
    /// Create a new client with a Snipp API key.
    ///
    /// Requests go to `api.snipp.gg`. Use [`with_region`](Self::with_region)
    /// to pin a regional endpoint.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: Url::parse(BASE_URL).expect("valid base URL"),
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
            base_url: Url::parse(&format!("https://{region}.api.snipp.gg")).expect("valid base URL"),
            http: Client::new(),
        })
    }

    /// Get a user by ID. Use `@me` for the authenticated user.
    ///
    /// `api_key`, `key_has_uploads_access`, `upload_count` and `limits` are
    /// only present on yourself.
    pub async fn get_user(&self, id: &str) -> Result<UserResponse, SnippError> {
        Self::send(self.request(Method::GET, &["users", id])).await
    }

    /// List a user's public posts, newest first. Use `@me` for the
    /// authenticated user. Team, private, unlisted, moderated, and restricted
    /// posts are never included.
    pub async fn get_user_posts(
        &self,
        id: &str,
        options: Option<ListOptions>,
    ) -> Result<PostList, SnippError> {
        let req = self.request(Method::GET, &["users", id, "posts"]);
        Self::send(Self::page(req, options)).await
    }

    /// List the authenticated user's own posts of every privacy, newest
    /// first. Team posts are not included.
    pub async fn list_posts(&self, options: Option<ListOptions>) -> Result<PostList, SnippError> {
        let req = self.request(Method::GET, &["posts"]);
        Self::send(Self::page(req, options)).await
    }

    /// Get a post by its share code. Team posts are only readable by members
    /// of that team.
    pub async fn get_post(&self, code: &str) -> Result<PostResponse, SnippError> {
        Self::send(self.request(Method::GET, &["posts", code])).await
    }

    /// Upload a file from disk as a new post. Privacy defaults to the server
    /// default (`private`) when omitted. Titles cap at 30 chars, descriptions
    /// at 200.
    pub async fn upload(
        &self,
        file_path: impl AsRef<Path>,
        options: Option<UploadOptions>,
    ) -> Result<UploadResponse, SnippError> {
        let mut form = multipart::Form::new().part("file", Self::file_part(file_path.as_ref()).await?);
        let mut req = self.request(Method::POST, &["upload"]);

        if let Some(opts) = options {
            req = Self::upload_headers(req, opts.include_metadata, opts.priority);
            if let Some(privacy) = opts.privacy {
                req = req.header("post-privacy", privacy.to_string());
            }
            if let Some(post_type) = opts.post_type {
                req = req.header("post-type", post_type.to_string());
            }
            if let Some(title) = opts.title {
                form = form.text("title", title);
            }
            if let Some(description) = opts.description {
                form = form.text("description", description);
            }
        }

        Self::send(req.multipart(form)).await
    }

    /// Update a post's title, description, or privacy. Only the fields that
    /// are `Some` are changed; empty strings clear the title or description.
    /// A team post's privacy cannot be changed.
    pub async fn update_post(
        &self,
        code: &str,
        options: UpdatePostOptions,
    ) -> Result<PostResponse, SnippError> {
        Self::send(self.request(Method::PATCH, &["posts", code]).json(&options)).await
    }

    /// Add 1 or more files to an existing post, turning it into an album. The
    /// post's share code, privacy, title, and description are preserved.
    /// Posts cap at 50 files total; requests that would exceed the cap are
    /// rejected. New files inherit the post's privacy.
    pub async fn add_files<P: AsRef<Path>>(
        &self,
        code: &str,
        file_paths: &[P],
        options: Option<AddFilesOptions>,
    ) -> Result<AddFilesResponse, SnippError> {
        if code.is_empty() {
            return Err(SnippError::Validation("code is required".to_string()));
        }
        if file_paths.is_empty() {
            return Err(SnippError::Validation("file_paths must not be empty".to_string()));
        }

        let mut form = multipart::Form::new();
        for path in file_paths {
            form = form.part("file", Self::file_part(path.as_ref()).await?);
        }

        let mut req = self.request(Method::POST, &["posts", code, "files"]);
        if let Some(opts) = options {
            req = Self::upload_headers(req, opts.include_metadata, opts.priority);
        }
        Self::send(req.multipart(form)).await
    }

    /// Delete one file from a post by its stored filename, as in
    /// [`PostFile::name`]. Deleting a post's only file deletes the post.
    pub async fn delete_file(&self, code: &str, name: &str) -> Result<DeletedFile, SnippError> {
        Self::send(self.request(Method::DELETE, &["posts", code, "files", name])).await
    }

    /// Delete a post and every file in it.
    pub async fn delete_post(&self, code: &str) -> Result<DeletedPost, SnippError> {
        Self::send(self.request(Method::DELETE, &["posts", code])).await
    }

    /// Report a post to Snipp moderation, with an optional reason (max 200
    /// chars).
    pub async fn report_post(
        &self,
        code: &str,
        reason: Option<&str>,
    ) -> Result<ReportResponse, SnippError> {
        let req = self.request(Method::POST, &["posts", code, "report"]);
        Self::send(req.json(&ReportBody { reason })).await
    }

    /// Report a user to Snipp moderation, with an optional reason (max 200
    /// chars).
    pub async fn report_user(
        &self,
        id: &str,
        reason: Option<&str>,
    ) -> Result<ReportResponse, SnippError> {
        let req = self.request(Method::POST, &["users", id, "report"]);
        Self::send(req.json(&ReportBody { reason })).await
    }

    fn request(&self, method: Method, segments: &[&str]) -> RequestBuilder {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .expect("valid base URL")
            .pop_if_empty()
            .extend(segments);
        self.http.request(method, url).header("api-key", &self.api_key)
    }

    fn upload_headers(
        mut req: RequestBuilder,
        include_metadata: Option<bool>,
        priority: Option<bool>,
    ) -> RequestBuilder {
        if let Some(include_metadata) = include_metadata {
            req = req.header("include-metadata", include_metadata.to_string());
        }
        if let Some(priority) = priority {
            req = req.header("priority", priority.to_string());
        }
        req
    }

    fn page(req: RequestBuilder, options: Option<ListOptions>) -> RequestBuilder {
        let Some(opts) = options else { return req };
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(limit) = opts.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(cursor) = opts.cursor {
            params.push(("cursor", cursor));
        }
        req.query(&params)
    }

    async fn file_part(path: &Path) -> Result<multipart::Part, SnippError> {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let bytes = tokio::fs::read(path).await?;
        Ok(multipart::Part::bytes(bytes).file_name(file_name))
    }

    async fn send<T: DeserializeOwned>(req: RequestBuilder) -> Result<T, SnippError> {
        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let raw = resp.text().await.unwrap_or_default();
            let body = serde_json::from_str::<Value>(&raw).ok();
            let error = body.as_ref().and_then(|b| b.get("error"));
            let field = |key: &str| {
                error
                    .and_then(|e| e.get(key))
                    .and_then(Value::as_str)
                    .map(str::to_string)
            };
            let kind = field("type");
            let message = field("message")
                .unwrap_or_else(|| status.canonical_reason().unwrap_or_default().to_string());
            return Err(SnippError::Api {
                status: status.as_u16(),
                kind,
                message,
                body,
            });
        }
        Ok(serde_json::from_str(&resp.text().await?)?)
    }
}
