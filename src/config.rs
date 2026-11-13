//! Runtime configuration loaded from environment variables.

use std::env;

use piim_common::config::CommonConfig;

/// Embedding backend selection.
#[derive(Debug, Clone, PartialEq)]
pub enum EmbeddingBackend {
    /// Use an external OpenAI-compatible HTTP endpoint.
    Http,
    /// Use local inference via fastembed.
    #[cfg(feature = "fastembed")]
    Fastembed(piim_fastembed::FastembedModel),
}

impl EmbeddingBackend {
    /// Parse backend from `EMBEDDINGS_BACKEND` env var.
    ///
    /// Supports: `http` (default), `fastembed`.
    /// When `fastembed`, the model is resolved from `EMBEDDINGS_MODEL`.
    pub fn from_env() -> Self {
        match env::var("EMBEDDINGS_BACKEND").ok().as_deref() {
            #[cfg(feature = "fastembed")]
            Some("fastembed") => {
                if env::var("FASTEMBED_CACHE_DIR").is_err() {
                    if let Some(home) = dirs::home_dir() {
                        let cache_dir = home.join(".cache").join("fastembed");
                        unsafe {
                            env::set_var("FASTEMBED_CACHE_DIR", cache_dir);
                        }
                    }
                }

                let model_name =
                    env::var("EMBEDDINGS_MODEL").unwrap_or_else(|_| "qwen3-embed".to_string());
                Self::Fastembed(piim_fastembed::FastembedModel::from_http_name(&model_name))
            }
            #[cfg(feature = "fastembed")]
            Some(_) | None => Self::Http,
            #[cfg(not(feature = "fastembed"))]
            _ => Self::Http,
        }
    }
}

/// Runtime configuration loaded from environment
/// variables, with fallback defaults.
pub struct Config {
    /// Embedding backend selection.
    pub embedding_backend: EmbeddingBackend,

    pub common: CommonConfig,
}

impl Config {
    /// Loads configuration from environment variables,
    /// falling back to defaults when unset or invalid.
    pub fn get_from_env() -> Config {
        let backend = EmbeddingBackend::from_env();

        let common = CommonConfig::from_env();

        Config {
            embedding_backend: backend,
            common,
        }
    }
}
