//! Runtime configuration loaded from environment variables.

use std::env;

use crate::helpers::DEFAULT_COLLECTION;
use piim_common::config::CommonConfig;

/// Runtime configuration loaded from environment
/// variables, with fallback defaults.
pub struct Config {
    /// URL of the Qdrant server (`QDRANT_URL`).
    pub qdrant_url: String,
    /// API key for the Qdrant server (`QDRANT_API_KEY`, optional).
    pub qdrant_api_key: Option<String>,
    /// Name of the Qdrant collection (`QDRANT_COLLECTION`).
    pub collection_name: String,
    /// Embedding model name (`EMBEDDINGS_MODEL`).
    pub model: String,
    /// Default project name used to scope records (`PROJECT_NAME`).
    ///
    /// Multiple projects can share one Qdrant collection:
    /// every point carries a `project` payload field and
    /// all reads/writes are filtered by it. The value
    /// can be overridden per tool call via the optional
    /// `project` argument.
    pub project: String,
}

impl Config {
    /// Loads configuration from environment variables,
    /// falling back to defaults when unset or invalid.
    pub fn get_from_env() -> Config {
        // `qdrant-client` talks to the server over gRPC, which Qdrant
        // serves on 6334 (6333 is the REST/dashboard port).
        let qdrant_url =
            env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".to_string());

        let qdrant_api_key = env::var("QDRANT_API_KEY").ok();

        let collection_name =
            env::var("QDRANT_COLLECTION").unwrap_or_else(|_| DEFAULT_COLLECTION.to_string());

        let common = CommonConfig::from_env();

        Config {
            qdrant_url,
            qdrant_api_key,
            collection_name,
            model: common.model,
            project: common.project,
        }
    }
}
