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
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub bio: Option<String>,
    pub verified: Option<bool>,
    pub created: Option<String>,
    pub badges: Option<Vec<Badge>>,
    pub posts: Option<Vec<Post>>,
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
pub struct UploadResponse {
    pub message: Option<String>,
    pub url: Option<String>,
    pub post: Option<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upload {
    pub url: Option<String>,
    pub size: Option<u64>,
    pub uploaded: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadsResponse {
    pub uploads: Vec<Upload>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoverResponse {
    pub uploads: Vec<DiscoverUpload>,
}

#[derive(Debug, Clone, Default)]
pub struct GetUserOptions {
    pub include_posts: Option<bool>,
    pub posts_limit: Option<u32>,
}
