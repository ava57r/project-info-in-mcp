use kameo::actor::{ActorRef, Spawn};
use piim_common::actors::project_info::ProjectInfoActor;
use qdrant_client::Qdrant;
use rust_mcp_sdk::error::SdkResult;

use crate::config::Config;

mod config;
mod helpers;
mod store;

pub async fn init_project_actor(
    embed_actor: Box<dyn piim_common::Embedder>,
) -> SdkResult<ActorRef<ProjectInfoActor>> {
    let config = Config::get_from_env();

    // Initialize Qdrant client
    let mut client_builder = Qdrant::from_url(&config.qdrant_url);
    if let Some(ref api_key) = config.qdrant_api_key {
        client_builder = client_builder.api_key(api_key.clone());
    }
    let client =
        client_builder
            .build()
            .map_err(|e| rust_mcp_sdk::error::McpSdkError::Internal {
                description: e.to_string(),
            })?;

    // Ensure the collection exists with proper vector config
    helpers::ensure_collection(
        &client,
        &config.collection_name,
        &config.model,
        embed_actor.dimension(),
    )
    .await
    .map_err(|e| rust_mcp_sdk::error::McpSdkError::Internal {
        description: e.to_string(),
    })?;

    let store = store::QdrantStore::new(
        client,
        config.collection_name,
        config.model.clone(),
        embed_actor.dimension(),
    );

    Ok(ProjectInfoActor::spawn(ProjectInfoActor::new(
        Box::new(store),
        embed_actor,
        config.project,
    )))
}
