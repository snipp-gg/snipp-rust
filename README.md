# snipp-rust

Rust API wrapper for [Snipp](https://api.snipp.gg).

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
snipp-rust = { path = "../snipp-rust" }
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use snipp_rust::{SnippClient, GetUserOptions, Privacy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SnippClient::new("snp_your_api_key");

    // Get the authenticated user
    let me = client.get_user("@me", None).await?;
    println!("{}", me.user.username.unwrap_or_default());

    // Get a user with posts included
    let opts = GetUserOptions {
        include_posts: Some(true),
        posts_limit: Some(10),
    };
    let user = client.get_user("some-user-id", Some(opts)).await?;

    // Upload a file
    let upload = client.upload("./screenshot.png", Some(Privacy::Unlisted)).await?;
    println!("Uploaded: {}", upload.url.unwrap_or_default());

    // List recent uploads
    let uploads = client.list_uploads().await?;

    // Delete an upload
    client.delete_upload("a3f7b2c91d4e8f0612ab34cd56ef7890.png").await?;

    // Browse public uploads
    let discover = client.discover().await?;

    Ok(())
}
```

## API Reference

All methods are async and return `Result<T, SnippError>`.

| Method | Description |
|---|---|
| `get_user(id, options)` | Get a user by ID (`"@me"` for self). Options: `include_posts`, `posts_limit` (1-50). |
| `upload(path, privacy)` | Upload a file. Privacy: `Public`, `Unlisted`, or `Private`. |
| `list_uploads()` | List the authenticated user's recent uploads. |
| `delete_upload(filename)` | Delete an upload by filename. |
| `discover()` | Browse publicly shared uploads. |

### Authentication

All requests require an API key passed to `SnippClient::new()`. Keys start with `snp_` and are sent via the `api-key` header.

### Error Handling

`SnippError` covers HTTP errors, API errors (non-2xx responses), deserialization failures, and IO errors during file uploads.

## License

MIT
