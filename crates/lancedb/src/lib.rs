use kameo::actor::{ActorRef, Spawn};
use piim_common::actors::project_info::ProjectInfoActor;
use rust_mcp_sdk::error::SdkResult;

use crate::config::Config;

mod config;
mod helpers;
mod http;
mod store;

pub async fn init_project_actor(
    embed_actor: Box<dyn piim_common::Embedder>,
) -> SdkResult<ActorRef<ProjectInfoActor>> {
    let config = Config::get_from_env();
    let store = store::LanceStore::new(config.db_dir.clone(), config.vector_dimension)
        .await
        .map_err(|description| rust_mcp_sdk::error::McpSdkError::Internal { description })?;

    let memory_actor_ref = ProjectInfoActor::spawn(ProjectInfoActor::new(
        Box::new(store),
        embed_actor,
        config.project.clone(),
    ));

    // Qdrant-style HTTP API + dashboard on :HTTP_PORT (runs alongside MCP stdio).
    if config.http_port != 0 {
        let state = http::AppState {
            actor: memory_actor_ref.clone(),
            db_dir: config.db_dir.clone(),
            snapshot_dir: config.snapshot_dir.clone(),
            default_project: config.project.clone(),
            model: config.model.clone(),
            vector_dimension: config.vector_dimension,
            started_unix: chrono::Utc::now().timestamp(),
        };
        let _ = http::snapshots::ensure_dir(&config.snapshot_dir);
        let app = http::router(state);
        let port = config.http_port;
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await;
            match listener {
                Ok(l) => {
                    eprintln!("piim-lance dashboard: http://localhost:{port}/dashboard");
                    if let Err(e) = axum::serve(l, app).await {
                        eprintln!("HTTP server error: {e}");
                    }
                }
                Err(e) => eprintln!("HTTP server bind error on port {port}: {e}"),
            }
        });
    }

    Ok(memory_actor_ref)
}
