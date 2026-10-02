use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::ParsePrivacyError;

/// Post privacy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Privacy {
    Public,
    Unlisted,
    Private,
}

impl fmt::Display for Privacy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Privacy::Public => "public",
            Privacy::Unlisted => "unlisted",
            Privacy::Private => "private",
        };
        f.write_str(s)
    }
}

impl FromStr for Privacy {
    type Err = ParsePrivacyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "public" => Ok(Privacy::Public),
            "unlisted" => Ok(Privacy::Unlisted),
            "private" => Ok(Privacy::Private),
            other => Err(ParsePrivacyError(other.to_string())),
        }
    }
}

/// A user's plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Plan {
    Free,
    Plus,
    Ultra,
}

impl fmt::Display for Plan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Plan::Free => "free",
            Plan::Plus => "plus",
            Plan::Ultra => "ultra",
        };
        f.write_str(s)
    }
}

/// A user embedded in another resource, such as a post's `author`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRef {
    pub id: String,
    /// `None` when the user has no username.
    pub username: Option<String>,
    pub nickname: Option<String>,
    /// Avatar URL. Users without one get the default avatar URL.
    pub avatar: String,
    pub verified: bool,
    pub plan: Plan,
}

/// Usage within one limit window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageWindow {
    /// Amount used in the current window: bytes for `usage`, minutes for
    /// `priority_minutes`.
    pub used: u64,
    /// Allowance for the window, in the same unit as `used`.
    pub limit: u64,
    /// `used` as a percentage of `limit`.
    pub used_percent: f64,
    /// ISO 8601 time the window resets, or `None` when no window is open.
    pub resets_at: Option<String>,
}

/// Your plan limits and usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limits {
    /// Largest single file you can upload, in bytes.
    pub max_file_size: u64,
    /// Weekly upload quota.
    pub usage: UsageWindow,
    /// Priority (adaptive) streaming minutes.
    pub priority_minutes: UsageWindow,
}

/// A user profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    /// `None` when the user has no username.
    pub username: Option<String>,
    pub nickname: Option<String>,
    /// Avatar URL. Users without one get the default avatar URL.
    pub avatar: String,
    /// Banner URL, or `None` when unset.
    pub banner: Option<String>,
    pub bio: Option<String>,
    pub socials: Option<serde_json::Value>,
    pub plan: Plan,
    pub verified: bool,
    pub staff: bool,
    pub partner: bool,
    pub translator: bool,
    pub bug_hunter_tier: u32,
    pub suspended: bool,
    /// ISO 8601 timestamp, or `None` when unknown.
    pub created_at: Option<String>,
    pub custom_embed: Option<serde_json::Value>,
    pub follower_count: u64,
    pub following_count: u64,
    /// Whether you follow this user. `false` on yourself.
    pub following: bool,
    /// Whether you block this user. `false` on yourself.
    pub blocking: bool,
    /// Your API key. Only present on yourself.
    pub api_key: Option<String>,
    /// Whether your API key can manage uploads. Only present on yourself.
    pub key_has_uploads_access: Option<bool>,
    /// Your total uploads. Only present on yourself.
    pub upload_count: Option<u64>,
    /// Your plan limits and usage. Only present on yourself.
    pub limits: Option<Limits>,
}

/// Response of [`get_user`](crate::SnippClient::get_user).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

/// One file in a post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostFile {
    /// Stored filename (`<32 hex>.<ext>`), used by
    /// [`delete_file`](crate::SnippClient::delete_file).
    pub name: String,
    /// Direct file URL. Signed with a 24-hour expiry when the post is private.
    pub url: String,
    /// Size in bytes, or `None` when unknown.
    pub size: Option<u64>,
    /// MIME type, or `None` when unknown.
    pub mime_type: Option<String>,
    /// Pixel width, or `None` when unknown.
    pub width: Option<u32>,
    /// Pixel height, or `None` when unknown.
    pub height: Option<u32>,
    /// Video thumbnail URL, or `None` when there is none.
    pub thumbnail_url: Option<String>,
}

/// A post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    /// Share code.
    pub code: String,
    /// Share page URL (`https://snipp.gg/p/<code>`).
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub privacy: Privacy,
    /// ISO 8601 timestamp.
    pub created_at: String,
    pub view_count: u64,
    /// `None` on team posts, which cannot be liked.
    pub like_count: Option<u64>,
    pub comment_count: u64,
    /// Whether the post uses priority (adaptive) streaming.
    pub priority: bool,
    pub file_count: u32,
    /// Every file in the post, in order.
    pub files: Vec<PostFile>,
    /// Owning team, or `None` on personal posts.
    pub team_id: Option<String>,
    /// `None` when the author could not be resolved.
    pub author: Option<UserRef>,
    /// Whether you liked the post. `None` on team posts.
    pub liked: Option<bool>,
    /// Only present for the owner.
    pub moderated: Option<bool>,
    /// Only present for the owner.
    pub restricted: Option<bool>,
}

/// Response of [`get_post`](crate::SnippClient::get_post) and
/// [`update_post`](crate::SnippClient::update_post).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostResponse {
    pub post: Post,
}

/// One page of posts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostList {
    pub posts: Vec<Post>,
    /// Whether another page exists.
    pub has_more: bool,
    /// Pass as [`ListOptions::cursor`] to fetch the next page. `None` on the
    /// last page.
    pub next_cursor: Option<String>,
}

/// The `error` object of a failed request or a rejected file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// Error type, such as `not_found` or `quota_exceeded`.
    #[serde(rename = "type")]
    pub kind: String,
    pub message: String,
    /// Context fields sent alongside `type` and `message`, such as `quota`,
    /// `used`, and `resets_at` on `quota_exceeded`.
    #[serde(flatten)]
    pub context: serde_json::Map<String, serde_json::Value>,
}

/// A file that was rejected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedFile {
    /// Position of the file in the request.
    pub index: u32,
    pub error: ErrorDetail,
}

/// Response of [`upload`](crate::SnippClient::upload).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    /// Direct URL of the uploaded file.
    pub url: String,
    /// The created post.
    pub post: Post,
}

/// Response of [`add_files`](crate::SnippClient::add_files).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFilesResponse {
    /// The post after the files were added.
    pub post: Post,
    /// Files that were rejected. Present only when some files failed.
    pub failed: Option<Vec<FailedFile>>,
}

/// Response of [`delete_file`](crate::SnippClient::delete_file).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedFile {
    pub name: String,
    pub deleted: bool,
}

/// Response of [`delete_post`](crate::SnippClient::delete_post).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedPost {
    pub code: String,
    pub deleted: bool,
}

/// Response of [`report_post`](crate::SnippClient::report_post) and
/// [`report_user`](crate::SnippClient::report_user).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResponse {
    pub reported: bool,
}

/// Pagination options for list methods.
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    /// Posts per page (1-100, default 30).
    pub limit: Option<u32>,
    /// `next_cursor` from the previous page.
    pub cursor: Option<String>,
}

/// Value of the `post-type` upload header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostType {
    Album,
    Individual,
}

impl fmt::Display for PostType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PostType::Album => "album",
            PostType::Individual => "individual",
        };
        f.write_str(s)
    }
}

/// Options for [`upload`](crate::SnippClient::upload).
#[derive(Debug, Clone, Default)]
pub struct UploadOptions {
    /// Post privacy. `None` lets the server upload as `private`.
    pub privacy: Option<Privacy>,
    /// Post title (max 30 chars).
    pub title: Option<String>,
    /// Post description (max 200 chars).
    pub description: Option<String>,
    /// Sent as the `post-type` header. Has no effect through `upload()`, which
    /// sends a single file; use `add_files` to build an album.
    pub post_type: Option<PostType>,
    /// Keep the file's metadata (EXIF, location, and the like). `None` lets
    /// the server strip it.
    pub include_metadata: Option<bool>,
    /// Use priority (adaptive) streaming for videos. `None` lets the server
    /// enable it when your plan is eligible.
    pub priority: Option<bool>,
}

/// Options for [`add_files`](crate::SnippClient::add_files).
#[derive(Debug, Clone, Default)]
pub struct AddFilesOptions {
    /// Keep the files' metadata (EXIF, location, and the like). `None` lets
    /// the server strip it.
    pub include_metadata: Option<bool>,
    /// Use priority (adaptive) streaming for videos. `None` lets the server
    /// enable it when your plan is eligible.
    pub priority: Option<bool>,
}

/// Fields for [`update_post`](crate::SnippClient::update_post). Only the
/// fields that are `Some` are changed.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdatePostOptions {
    /// New title (max 30 chars). Empty string to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New description (max 200 chars). Empty string to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New privacy. A team post's privacy cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Privacy>,
}
