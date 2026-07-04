//

use std::str::FromStr;

use crate::{Embedder, Error};

use fastembed::{TextEmbedding, TextInitOptions, EmbeddingModel};
use serde::Deserialize;
use async_trait::async_trait;

/// The `FastEmbed` struct provides an implementation of the `Embedder` trait using the FastEmbed 
/// library for text embedding.
/// 
pub struct FastEmbed {
    /// The underlying FastEmbed model used for generating embeddings.
    model: TextEmbedding,
    /// Optional batch size for processing multiple documents at once.
    batch_size: Option<usize>,
}

impl FastEmbed {

    /// Creates a new instance of `FastEmbed` with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A `Config` struct containing the model name and optional batch size.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `FastEmbed` instance or an `Error`.
    /// 
    pub fn new(config: Config) -> Result<Self, Error> {
        let model = EmbeddingModel::from_str(&config.model)
            .map_err(|e| Error::InvalidModelName(format!("Invalid model name: {}. Error: {}", config.model, e)))?;
        let options = TextInitOptions::new(model);
        let model = TextEmbedding::try_new(options)?;
        Ok(Self { model, batch_size: config.batch_size })
    }

    /// Creates a new instance of `FastEmbed` with default settings.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `FastEmbed` instance or an `Error`.
    /// 
    pub fn try_new() -> Result<Self, Error> {
        let model = TextEmbedding::try_new(Default::default())?;
        Ok(Self { model, batch_size: None })
    }
}

#[async_trait]
impl Embedder for FastEmbed {

    async fn embed_str(&mut self, text: &str) -> Result<Vec<f64>, Error> {
        let embedding = self.model.embed(vec![text], self.batch_size)?;
        Ok(embedding[0].iter().map(|x| *x as f64).collect())
    }

    async fn embed_docs(&mut self, docs: &[String]) -> Result<Vec<Vec<f64>>, Error> {
        let embeddings = self
            .model
            .embed(docs.to_vec(), self.batch_size)?;
        Ok(embeddings
            .into_iter()
            .map(|inner_vec| {
                inner_vec
                    .into_iter()
                    .map(|x| x as f64)
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>())
    }
}

/// The `Config` struct holds the configuration for the `FastEmbed` embedder, including the model
/// name and an optional batch size for processing multiple documents.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// The name of the model to use for embedding.
    pub model: String,
    /// Optional batch size for processing multiple documents at once.
    pub batch_size: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embed_str() {
        let mut fast_embed = FastEmbed::try_new().unwrap();
        let text = "This is a test string.";
        let embedding = fast_embed.embed_str(text).await.unwrap();
        assert!(!embedding.is_empty());
    }

    #[tokio::test]
    async fn test_embed_docs() {
        let mut fast_embed = FastEmbed::try_new().unwrap();
        let docs = vec![
            "This is the first document.".to_string(),
            "This is the second document.".to_string(),
        ];
        let embeddings = fast_embed.embed_docs(&docs).await.unwrap();
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].len(), embeddings[1].len());
   }
}