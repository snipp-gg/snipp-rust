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
snipp = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use snipp::{SnippClient, GetUserOptions, Privacy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SnippClient::new("YOUR_API_KEY");

    // Get the authenticated user
    let me = client.get_user("@me", None).await?;
    println!("{}", me.user.username.unwrap_or_default());

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

## API

All methods are async and return `Result<T, SnippError>`.

### `SnippClient::new(api_key)`

Create a client. The key is sent via the `api-key` header on every request.

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

### `upload(path, privacy)`

Upload a file from a path. Privacy: `Public`, `Unlisted`, or `Private`.

```rust
let result = client.upload("./image.png", Some(Privacy::Unlisted)).await?;
```

### `list_uploads()`

List the authenticated user's recent uploads. Each upload entry includes the file URL, size metadata, optional post `code`, and `is_album` when the upload belongs to an album post.

### `delete_upload(filename)`

Delete an upload by its filename.

### `discover()`

Browse public uploads.

## Error Handling

`SnippError` covers HTTP errors, API errors (non-2xx responses), deserialization failures, and IO errors during file uploads.

## Contributing

We welcome suggestions and improvements:

- Open an issue
- Submit a pull request that adheres to our [Terms of Service](https://snipp.gg/terms) and [Privacy Policy](https://snipp.gg/privacy)

## License

MIT License © 2026 Snipp. See [LICENSE](LICENSE) for full details.
