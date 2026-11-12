use std::collections::BTreeMap;

use piim_common::mcp;
use rust_mcp_sdk::mcp_server::{McpServerOptions, server_runtime};
use rust_mcp_sdk::{McpServer, ToMcpServerHandler};
use rust_mcp_sdk::{
    StdioTransport, TransportOptions,
    error::SdkResult,
    schema::{ServerCapabilities, ServerCapabilitiesTools},
};

mod config;

/// Main entry point for the `piim` MCP server.
#[tokio::main]
async fn main() -> SdkResult<()> {
    let config = crate::config::Config::get_from_env();

    // Create the appropriate embedding actor based on the selected backend.
    #[allow(unused)]
    let embed_actor = match &config.embedding_backend {
        config::EmbeddingBackend::Http => {
            Box::new(piim_common::actors::embedding::http::EmbeddingActor::new(
                config.common.embeddings_url.clone(),
                config.common.model.clone(),
                None,
                config.common.vector_dimension,
            )) as Box<dyn piim_common::Embedder>
        }
        #[cfg(feature = "fastembed")]
        config::EmbeddingBackend::Fastembed(model) => {
            let actor =
                piim_fastembed::EmbeddingActor::new_fastembed(model.clone()).map_err(|e| {
                    rust_mcp_sdk::error::McpSdkError::Internal {
                        description: format!("Failed to initialize local embedding model: {e}"),
                    }
                })?;

            Box::new(actor) as Box<dyn piim_common::Embedder>
        }
    };

    let memory_actor_ref = cfg_select! {
            feature = "qdrant_store" => {
                piim_qdrant::init_project_actor(embed_actor).await?
            }
            feature = "lance_store" => {
                piim_lance::init_project_actor(embed_actor).await?
            }
            _ => {
                return Err(rust_mcp_sdk::error::McpSdkError::Internal {
                    description: "No memory backend selected. Enable either `qdrant_store` or `lance_store` feature.".into(),
                })
            }
    };
    let server_details = server_details();

    let transport = StdioTransport::new(TransportOptions::default())?;
    let handler = mcp::MemoryToolHandler::new(memory_actor_ref, env!("CARGO_PKG_VERSION"))
        .to_mcp_server_handler();
    let server = server_runtime::create_server(McpServerOptions {
        transport,
        handler,
        server_details,
        message_observer: None,
    });
    server.start().await
}

pub fn server_details() -> rust_mcp_sdk::ServerDetails {
    rust_mcp_sdk::ServerDetails {
        server_info: rust_mcp_sdk::schema::Implementation {
            name: "piim".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            title: Some("Project Info in MCP".into()),
            description: Some("Persistent project memory with vector search".into()),
            icons: vec![],
            website_url: None,
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools {
                list_changed: Some(true),
            }),
            experimental: Some({
                let mut exp = BTreeMap::new();
                exp.insert(
                    "customRequests".to_string(),
                    rust_mcp_sdk::schema::JsonObject(BTreeMap::new()),
                );
                exp
            }),
            ..Default::default()
        },
        instructions: None,
        meta: None,
    }
}
