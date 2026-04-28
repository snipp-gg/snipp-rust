use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::ParsePrivacyError;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub bio: Option<String>,
    pub socials: Option<serde_json::Value>,
    pub key: Option<String>,
    pub key_has_uploads_access: Option<bool>,
    pub plus: Option<bool>,
    pub enterprise: Option<bool>,
    pub verified: Option<bool>,
    pub staff: Option<bool>,
    pub partner: Option<bool>,
    pub translator: Option<bool>,
    pub bug_hunter_tier: Option<u32>,
    pub suspended: Option<bool>,
    pub created: Option<String>,
    pub custom_embed: Option<serde_json::Value>,
    pub badges: Option<Vec<Badge>>,
    pub uploads: Option<u32>,
    pub public_uploads: Option<Vec<PublicUpload>>,
    pub blocked_by_you: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUpload {
    pub code: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: Option<String>,
    pub is_album: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub code: Option<String>,
    pub url: Option<String>,
    pub post_privacy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPostDetail {
    pub code: Option<String>,
    pub url: Option<String>,
    pub urls: Option<Vec<String>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_album: Option<bool>,
    pub post_privacy: Option<String>,
    pub created: Option<String>,
    pub file: Option<FileInfo>,
    pub moderated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPostResponse {
    pub post: GetPostDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub size: Option<u64>,
    pub size_formatted: Option<String>,
    pub mime_type: Option<String>,
    pub dimensions: Option<Dimensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub message: Option<String>,
    pub url: Option<String>,
    pub file: Option<FileInfo>,
    pub processing_time: Option<u64>,
    pub post: Option<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upload {
    pub code: Option<String>,
    #[serde(rename = "isAlbum")]
    pub is_album: Option<bool>,
    pub url: Option<String>,
    pub size: Option<u64>,
    pub size_formatted: Option<String>,
    pub uploaded: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadsResponse {
    pub uploads: Vec<Upload>,
}

#[derive(Debug, Clone, Default)]
pub struct GetUserOptions {
    pub include_posts: Option<bool>,
    pub posts_limit: Option<u32>,
}

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

#[derive(Debug, Clone, Default)]
pub struct UploadOptions {
    pub privacy: Option<Privacy>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub post_type: Option<PostType>,
}

#[derive(Debug, Clone, Default)]
pub struct EditUploadOptions {
    pub title: Option<String>,
    pub description: Option<String>,
    pub privacy: Option<Privacy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditUploadResponse {
    pub message: Option<String>,
    pub post: Option<EditedPost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditedPost {
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub post_privacy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendedFile {
    pub index: Option<u32>,
    pub file_name: Option<String>,
    pub url: Option<String>,
    pub size: Option<u64>,
    pub size_formatted: Option<String>,
    pub mime_type: Option<String>,
    pub status: Option<String>,
    pub dimensions: Option<Dimensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailedFile {
    pub index: Option<u32>,
    pub error: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendedPost {
    pub code: Option<String>,
    pub url: Option<String>,
    pub post_privacy: Option<String>,
    pub file_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppendUploadResponse {
    pub message: Option<String>,
    pub post: Option<AppendedPost>,
    pub files: Option<Vec<AppendedFile>>,
    pub failed: Option<Vec<FailedFile>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReportRequest {
    pub code: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResponse {
    pub success: bool,
}