pub mod http;

use async_trait::async_trait;

#[async_trait]
pub trait Embedder: Send + Sync + 'static {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, String>;

    /// Return the vector dimension for this backend.
    fn dimension(&self) -> usize;
}
