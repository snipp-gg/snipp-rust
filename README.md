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
snipp = "2"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use snipp::{SnippClient, Privacy, UploadOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SnippClient::new("YOUR_API_KEY");

    // Get the authenticated user
    let me = client.get_user("@me", None).await?;
    println!("{}", me.user.username.unwrap_or_default());

    // Upload a file
    let opts = UploadOptions { privacy: Some(Privacy::Unlisted), ..Default::default() };
    let upload = client.upload("./screenshot.png", Some(opts)).await?;
    println!("Uploaded: {}", upload.url.unwrap_or_default());

    // List recent uploads
    let uploads = client.list_uploads(None).await?;

    // Delete an upload
    client.delete_upload("a3f7b2c91d4e8f0612ab34cd56ef7890.png").await?;

    Ok(())
}
```

## API

All methods are async and return `Result<T, SnippError>`.

### `SnippClient::new(api_key)`

Create a client. The key is sent via the `api-key` header on every request. Requests go to `api.snipp.gg`.

### `SnippClient::with_region(api_key, region)`

Create a client pinned to a regional endpoint, `"eu-west-1"` or `"us-west-1"`. Uploads run at the same speed either way; this controls which region stores your files. Returns `SnippError::Validation` if the region is not recognized.

```rust
let client = SnippClient::with_region("YOUR_API_KEY", "eu-west-1")?;
```

### `get_user(id, options)`

Get a user by ID. Pass `"@me"` for the authenticated user.

| Option | Type | Description |
|---|---|---|
| `include_posts` | `Option<bool>` | Include the user's public uploads. |
| `posts_limit` | `Option<u32>` | Number of posts to return (1-50). |

```rust
let opts = GetUserOptions {
    include_posts: Some(true),
    posts_limit: Some(10),
};
let user = client.get_user("some-user-id", Some(opts)).await?;
```

### `get_post(code)`

Get a post by its share code. Team posts are only readable by members of that team, and carry no `like_count`.

```rust
let post = client.get_post("AbC123").await?.post;
```

### `upload(path, options)`

Upload a file from a path. `UploadOptions` takes `privacy` (`Public`, `Unlisted`, or `Private`; defaults to `private` when omitted), `title` (max 30 chars), `description` (max 200 chars), and `post_type` (`Album` or `Individual`, sent as the `post-type` header; it has no effect through `upload()`, which sends a single file, so use `append_upload` to build an album).

```rust
let opts = UploadOptions { privacy: Some(Privacy::Unlisted), ..Default::default() };
let result = client.upload("./image.png", Some(opts)).await?;
```

### `list_uploads(limit)`

List the authenticated user's recent uploads (`limit` 1-1000). Each upload entry includes the file URL, size metadata, optional post `code`, and `is_album` when the upload belongs to an album post.

```rust
let uploads = client.list_uploads(Some(100)).await?;
```

### `edit_upload(code, options)`

Edit an existing upload's title, description, or privacy. Empty strings clear the title or description.

```rust
let opts = EditUploadOptions { title: Some("New title".into()), ..Default::default() };
client.edit_upload("AbC123", opts).await?;
```

### `append_upload(code, file_paths)`

Append 1 or more files to an existing album post. Albums cap at 50 files total.

```rust
client.append_upload("AbC123", &["./extra.png"]).await?;
```

### `delete_upload(filename)`

Delete an upload by its filename.

```rust
client.delete_upload("a3f7b2c91d4e8f0612ab34cd56ef7890.png").await?;
```

### `report_post(code, reason)`

Report a post. Pass an empty string to omit the reason (max 200 chars).

```rust
client.report_post("AbC123", "Spam").await?;
```

## Error Handling

`SnippError` covers HTTP errors, API errors (non-2xx responses), local input validation, deserialization failures, and IO errors during file uploads.

`SnippError::Api` carries `status`, `message`, and `body`. `body` is the parsed JSON error response, or `None` when the response was not JSON. It carries the fields the API sends alongside `error`, such as `suspended` on a suspended user or `moderated` on a moderated post.

```rust
use snipp::SnippError;

match client.get_user("987654321098765432", None).await {
    Ok(res) => println!("{:?}", res.user.username),
    Err(SnippError::Api { status, message, body }) => {
        let suspended = body
            .as_ref()
            .and_then(|b| b.get("suspended")?.as_bool())
            .unwrap_or(false);
        if suspended {
            eprintln!("user is suspended");
        } else {
            eprintln!("{status}: {message}");
        }
    }
    Err(err) => eprintln!("{err}"),
}
```

## Contributing

We welcome suggestions and improvements:

- Open an issue
- Submit a pull request that adheres to our [Terms of Service](https://snipp.gg/terms) and [Privacy Policy](https://snipp.gg/privacy)

## License

MIT License © 2026 Snipp. See [LICENSE](LICENSE) for full details.
