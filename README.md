# snipp-rust

A Rust wrapper for the [Snipp API](https://api.snipp.gg).

## Features

- Async/await with Tokio runtime
- Built on `reqwest` with full type safety
- Rich error handling with `SnippError`

## Requirements

- Rust (latest stable)
- A valid API key from the [Snipp Console](https://snipp.gg/settings/console)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
snipp = "3"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use snipp::{Privacy, SnippClient, UploadOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SnippClient::new("YOUR_API_KEY");

    // Get the authenticated user
    let me = client.get_user("@me").await?;
    println!("{}", me.user.username.unwrap_or_default());

    // Upload a file
    let opts = UploadOptions { privacy: Some(Privacy::Unlisted), ..Default::default() };
    let upload = client.upload("./screenshot.png", Some(opts)).await?;
    println!("Uploaded: {} ({})", upload.url, upload.post.url);

    // List your posts
    let page = client.list_posts(None).await?;
    println!("{} posts", page.posts.len());

    // Delete a post
    client.delete_post(&upload.post.code).await?;

    Ok(())
}
```

## API

All methods are async and return `Result<T, SnippError>`. Responses deserialize into the structs in `snipp::models` (`User`, `Post`, `PostFile`, `PostList`, and the rest), all re-exported from the crate root.

### `SnippClient::new(api_key)`

Create a client. The key is sent via the `api-key` header on every request. Requests go to `api.snipp.gg`.

### `SnippClient::with_region(api_key, region)`

Create a client pinned to a regional endpoint, `"eu-west-1"` or `"us-west-1"`. Uploads run at the same speed either way; this controls which region stores your files. Returns `SnippError::Validation` if the region is not recognized.

```rust
let client = SnippClient::with_region("YOUR_API_KEY", "eu-west-1")?;
```

### `get_user(id)`

Get a user by ID. Pass `"@me"` for the authenticated user. `api_key`, `key_has_uploads_access`, `upload_count`, and `limits` are only `Some` on yourself.

```rust
let user = client.get_user("@me").await?.user;
if let Some(limits) = &user.limits {
    println!("{} plan, {}% of weekly quota used", user.plan, limits.usage.used_percent);
}
```

### `get_user_posts(id, options)`

List a user's public posts, newest first. Team, private, unlisted, moderated, and restricted posts are never included. Pass `"@me"` for your own public posts. `ListOptions` takes `limit` (1-100, default 30) and `cursor` (the `next_cursor` from the previous page).

```rust
use snipp::ListOptions;

let opts = ListOptions { limit: Some(10), ..Default::default() };
let page = client.get_user_posts("987654321098765432", Some(opts)).await?;
println!("{} more: {}", page.posts.len(), page.has_more);
```

### `list_posts(options)`

List your own posts of every privacy, newest first. Team posts are not included. Takes the same `ListOptions` as `get_user_posts`. Follow `next_cursor` until it is `None` to walk every page:

```rust
use snipp::ListOptions;

let mut cursor = None;
loop {
    let opts = ListOptions { limit: Some(100), cursor };
    let page = client.list_posts(Some(opts)).await?;
    for post in &page.posts {
        println!("{} {}", post.code, post.privacy);
    }
    cursor = page.next_cursor;
    if cursor.is_none() {
        break;
    }
}
```

### `get_post(code)`

Get a post by its share code. Team posts are only readable by members of that team, and have `like_count` and `liked` set to `None`.

```rust
let post = client.get_post("AbC123").await?.post;
println!("{} {}", post.url, post.files[0].url);
```

### `upload(path, options)`

Upload a file from a path as a new post. `UploadOptions` takes `privacy` (`Public`, `Unlisted`, or `Private`; defaults to `private` when omitted), `title` (max 30 chars), `description` (max 200 chars), and `post_type` (`Album` or `Individual`, sent as the `post-type` header; it has no effect through `upload()`, which sends a single file, so use `add_files` to build an album), `include_metadata` (keep the file's metadata such as EXIF and location, which the server strips when `None`), and `priority` (priority adaptive streaming for videos; `None` lets the server enable it when your plan is eligible). Returns `url` (the direct file URL) and `post`.

```rust
use snipp::{Privacy, UploadOptions};

let opts = UploadOptions { privacy: Some(Privacy::Unlisted), ..Default::default() };
let result = client.upload("./image.png", Some(opts)).await?;
println!("{} {}", result.url, result.post.url);
```

### `update_post(code, options)`

Update a post's title, description, or privacy. Only the `UpdatePostOptions` fields that are `Some` are changed; empty strings clear the title or description. A team post's privacy cannot be changed. Returns `post`.

```rust
use snipp::{Privacy, UpdatePostOptions};

let opts = UpdatePostOptions {
    title: Some("New title".into()),
    privacy: Some(Privacy::Public),
    ..Default::default()
};
client.update_post("AbC123", opts).await?;
```

### `add_files(code, file_paths, options)`

Add 1 or more files to an existing post, turning it into an album. Posts cap at 50 files total. New files inherit the post's privacy. `AddFilesOptions` takes `include_metadata` and `priority`, which work as on `upload()`. Returns `post`, plus `failed` when some files were rejected, each with its `index` and `error`.

```rust
use snipp::AddFilesOptions;

let opts = AddFilesOptions { include_metadata: Some(true), ..Default::default() };
let result = client.add_files("AbC123", &["./extra.png"], Some(opts)).await?;
println!("{} files", result.post.file_count);
for failed in result.failed.unwrap_or_default() {
    eprintln!("file {} failed: {}", failed.index, failed.error.message);
}
```

### `delete_file(code, name)`

Delete one file from a post, by the `name` it has in `post.files`. Deleting a post's only file deletes the post.

```rust
let post = client.get_post("AbC123").await?.post;
client.delete_file("AbC123", &post.files[1].name).await?;
```

### `delete_post(code)`

Delete a post and every file in it.

```rust
client.delete_post("AbC123").await?;
```

### `report_post(code, reason)`

Report a post, with an optional reason (max 200 chars).

```rust
client.report_post("AbC123", Some("Spam")).await?;
```

### `report_user(id, reason)`

Report a user, with an optional reason (max 200 chars).

```rust
client.report_user("987654321098765432", Some("Impersonation")).await?;
```

## Error Handling

`SnippError` covers HTTP errors, API errors (non-2xx responses), local input validation, deserialization failures, and IO errors during file uploads.

`SnippError::Api` carries:

| Field | Type | Description |
|---|---|---|
| `status` | `u16` | HTTP status code. |
| `kind` | `Option<String>` | The error `type`, such as `not_found`, `rate_limited`, or `quota_exceeded`. `None` when the response did not carry one. |
| `message` | `String` | Human-readable message, or the HTTP status text when the response did not carry one. |
| `body` | `Option<serde_json::Value>` | Parsed JSON response, or `None` when it was not JSON. Context fields live under `body["error"]`, such as `resets_at` on `quota_exceeded`. |

```rust
use snipp::SnippError;

match client.upload("./image.png", None).await {
    Ok(result) => println!("{}", result.url),
    Err(SnippError::Api { kind: Some(kind), body, .. }) if kind == "quota_exceeded" => {
        let resets_at = body
            .as_ref()
            .and_then(|b| b["error"]["resets_at"].as_str())
            .unwrap_or("soon");
        eprintln!("Weekly quota used up, resets at {resets_at}");
    }
    Err(err) => eprintln!("{err}"),
}
```

## Migrating from 2.x

3.0 follows the reorganized Snipp API. Methods:

| 2.x | 3.0 |
|---|---|
| `list_uploads(limit)` | `list_posts(options)`, cursor-paginated posts via `ListOptions` |
| `edit_upload(code, EditUploadOptions)` | `update_post(code, UpdatePostOptions)` |
| `append_upload(code, file_paths)` | `add_files(code, file_paths, options)` |
| `delete_upload(filename)` | `delete_file(code, name)`, which now needs the post's share code |
| `get_user(id, Option<GetUserOptions>)` | `get_user(id)` plus `get_user_posts(id, Option<ListOptions>)` |
| `report_post(code, "")` | `report_post(code, None)`; the reason is now `Option<&str>` |
| | New: `delete_post(code)`, `report_user(id, reason)` |

Responses:

- Response structs now match the API: fields the API always sends are plain values instead of `Option`, and `Option` is kept only where a value can be null or absent.
- `GetPostResponse`, `UploadsResponse`, `EditUploadResponse`, and `AppendUploadResponse` are replaced by `PostResponse`, `PostList`, and `AddFilesResponse`, all built on one `Post` struct: `privacy` (was `post_privacy`), `created_at` (was `created`), `view_count` (was `views`), `files[].name` (was `file_name`), and `file_count` replaces `is_album`.
- `UploadResponse` is `{ url, post }`; `message`, `file`, and `processing_time` are gone.
- `User` carries `plan` (`Plan::Free`, `Plus`, or `Ultra`) in place of `plus`, `ultra`, and `badges`, `created_at` in place of `created`, and `blocking` in place of `blocked_by_you`.

Errors:

- The API now sends every error as an `error` object with `type`, `message`, and any context fields. `SnippError::Api` gains `kind`, `message` falls back to the HTTP status text instead of the raw response body, and context fields moved from the top of `body` into `body["error"]`. A check on `body["suspended"]` becomes `kind.as_deref() == Some("suspended")`.

## Contributing

We welcome suggestions and improvements:

- Open an issue
- Submit a pull request that adheres to our [Terms of Service](https://snipp.gg/terms) and [Privacy Policy](https://snipp.gg/privacy)

## License

MIT License © 2026 Snipp. See [LICENSE](LICENSE) for full details.
