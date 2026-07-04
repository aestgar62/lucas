//

use crate::Error;

use async_trait::async_trait;

/// The `Embedder` trait defines the interface for embedding text and documents into vector 
/// representations.    
#[async_trait]
pub trait Embedder: Send + Sync {

    /// Embeds a single string and returns its embedding as a vector of f64 values.
    /// 
    /// # Arguments
    /// 
    /// text: The input string to be embedded.
    /// 
    /// # Returns
    /// 
    /// A Result containing a vector of f64 values representing the embedding, or an Error if the 
    /// embedding process fails.
    async fn embed_str(&mut self, text: &str) -> Result<Vec<f64>, Error>;

    /// Embeds multiple documents and returns their embeddings as a vector of vectors of f64 values.
    /// 
    /// # Arguments
    /// 
    /// docs: A slice of strings representing the documents to be embedded.
    /// 
    /// # Returns
    /// 
    /// A Result containing a vector of vectors of f64 values representing the embeddings, or an 
    /// Error if the embedding process fails.
    /// 
    async fn embed_docs(&mut self, docs: &[String]) -> Result<Vec<Vec<f64>>, Error>;
}