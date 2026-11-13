# piim-qdrant

Qdrant storage backend library for PIIM (Project Info In MCP). Provides `QdrantStore` and `init_project_actor()` for use with the `piim` binary or any downstream project via the `qdrant_store` Cargo feature.

## Prerequisites

- **Qdrant** — vector database server (local or remote)
- **Embeddings** — either an external API endpoint or local inference via [fastembed-rs](https://github.com/anush008/fastembed-rs) (see [Embedding backends](../../README.md#embedding-backends) in the workspace README)

### Running Qdrant

Locally via Docker:

```bash
docker run -p 6333:6333 -p 6334:6334 \
  -v $(pwd)/qdrant_storage:/qdrant/storage:z \
  qdrant/qdrant
```

Or connect to an existing remote server.

## Environment Variables

| Variable | Required | Default | Description |
|---|---|---|---|
| `QDRANT_URL` | Yes | `http://localhost:6334` | Qdrant server URL. Must be the **gRPC** port (6334) — `qdrant-client` talks gRPC; 6333 is REST/dashboard. |
| `QDRANT_API_KEY` | No | — | API key for authentication. |
| `QDRANT_COLLECTION` | No | `project_memory` | Name of the Qdrant collection. |
| `PROJECT_NAME` | No | `default` | Default project scope. Overridable per tool call. |

> Embedding settings (`EMBEDDINGS_BACKEND`, `EMBEDDINGS_URL`, `EMBEDDINGS_MODEL`, `VECTOR_DIMENSION`, `FASTEMBED_CACHE_DIR`) are shared by both backends and documented in the workspace README: [Configuration](../../README.md#configuration) and [Embedding backends](../../README.md#embedding-backends). Qdrant-specific note: `EMBEDDINGS_MODEL` also names the collection vector, so several models can coexist in one collection.

## Usage

Add `piim-qdrant` as a dependency and call `init_project_actor()` to get a `ProjectInfoActor` backed by Qdrant:

```rust
use piim_qdrant::init_project_actor;

let actor = init_project_actor().await?;
// Use the actor with MCP server setup...
```

## Client setup (opencode)

Then add to `opencode.json`:

```json
{
  "mcp": {
    "piim": {
      "type": "local",
      "command": ["/full/path/to/target/release/piim"],
      "environment": {
        "QDRANT_URL": "http://localhost:6334",
        "EMBEDDINGS_BACKEND": "fastembed",
        "EMBEDDINGS_MODEL": "bge-small-en",
        "PROJECT_NAME": "my-project"
      },
      "enabled": true
    }
  }
}
```

For HTTP backend (external embeddings API):

```json
{
  "mcp": {
    "piim": {
      "type": "local",
      "command": ["/full/path/to/target/release/piim"],
      "environment": {
        "QDRANT_URL": "http://localhost:6334",
        "EMBEDDINGS_BACKEND": "http",
        "EMBEDDINGS_URL": "http://localhost:8002/v1/embeddings",
        "EMBEDDINGS_MODEL": "qwen3-embed",
        "VECTOR_DIMENSION": "1024",
        "PROJECT_NAME": "my-project"
      },
      "enabled": true
    }
  }
}
```

> Use an absolute `command` path. Ensure Qdrant is running at `QDRANT_URL` and `VECTOR_DIMENSION` matches your embedding model. Optional: set `QDRANT_API_KEY` and `QDRANT_COLLECTION` if using authentication or a custom collection name.

## Client setup (Zed)

Then add to `~/.config/zed/settings.json`:

```json
{
  "mcp": {
    "piim": {
      "command": "/full/path/to/target/release/piim",
      "args": [],
      "env": {
        "QDRANT_URL": "http://localhost:6334",
        "EMBEDDINGS_BACKEND": "fastembed",
        "EMBEDDINGS_MODEL": "bge-small-en",
        "PROJECT_NAME": "my-project"
      }
    }
  }
}
```

Or via `~/.config/zed/mcp.json`:

```json
{
  "mcp": {
    "piim": {
      "command": "/full/path/to/target/release/piim",
      "args": [],
      "env": {
        "QDRANT_URL": "http://localhost:6334",
        "EMBEDDINGS_BACKEND": "fastembed",
        "EMBEDDINGS_MODEL": "bge-small-en",
        "PROJECT_NAME": "my-project"
      }
    }
  }
}
```

> Use an absolute `command` path. Ensure Qdrant is running at `QDRANT_URL` and `VECTOR_DIMENSION` matches your embedding model. Optional: set `QDRANT_API_KEY` and `QDRANT_COLLECTION` if using authentication or a custom collection name.

## Structure

```
crates/qdrant/
├── src/
│   ├── lib.rs           # Library entry, init_project_actor()
│   ├── config.rs        # Configuration from environment variables
│   ├── helpers.rs       # Helper functions (collection creation)
│   ├── store.rs         # Qdrant storage backend implementation
├── Cargo.toml
└── README.md
```

The embedding actor and MCP handlers are in the shared `piim-common` crate.

## License

MIT
