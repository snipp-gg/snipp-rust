use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::error::ParsePrivacyError;

// ---------------------------------------------------------------------------
// Privacy
// ---------------------------------------------------------------------------

/// Upload privacy level.
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

// ---------------------------------------------------------------------------
// User
// ---------------------------------------------------------------------------

/// A badge displayed on a user's profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

/// A Snipp user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub bio: Option<String>,
    pub verified: Option<bool>,
    pub created: Option<String>,
    pub badges: Option<Vec<Badge>>,
    pub posts: Option<Vec<Post>>,
}

/// Response wrapper for user endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

// ---------------------------------------------------------------------------
// Posts / Uploads
// ---------------------------------------------------------------------------

/// A post (code snippet) on Snipp.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub code: Option<String>,
    pub url: Option<String>,
    pub post_privacy: Option<String>,
}

/// Response returned after uploading a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub message: Option<String>,
    pub url: Option<String>,
    pub post: Option<Post>,
}

/// A single upload entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upload {
    pub url: Option<String>,
    pub size: Option<u64>,
    pub uploaded: Option<String>,
}

/// Response wrapper for listing uploads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadsResponse {
    pub uploads: Vec<Upload>,
}

// ---------------------------------------------------------------------------
// Discover
// ---------------------------------------------------------------------------

/// A publicly discoverable upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverUpload {
    pub url: Option<String>,
    pub code: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: Option<String>,
    pub author: Option<String>,
    pub author_avatar: Option<String>,
    pub author_id: Option<String>,
    pub size: Option<u64>,
    pub mime_type: Option<String>,
}

/// Response wrapper for the discover endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverResponse {
    pub uploads: Vec<DiscoverUpload>,
}

// ---------------------------------------------------------------------------
// Request helpers
// ---------------------------------------------------------------------------

/// Options for the get-user endpoint.
#[derive(Debug, Clone, Default)]
pub struct GetUserOptions {
    /// Include the user's posts in the response.
    pub include_posts: Option<bool>,
    /// Maximum number of posts to return (1-50).
    pub posts_limit: Option<u32>,
}
