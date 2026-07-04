//

mod error;
mod message;
mod providers;

pub use error::Error;

use lucas_core::{Message, Block, Role};
use tools::ToolDefinition;
use serde::Deserialize;
use async_trait::async_trait;

/// Generative AI provider trait. This trait defines the interface for a provider API that can be 
/// used to interact with a generative AI model.
#[async_trait]
pub trait Provider: Send + Sync {

    /// Completes a request using the provider.
    /// 
    /// # Arguments
    /// 
    /// * `req` - A reference to a `CompletionRequest` containing the details of the request.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `CompletionResponse` if successful, or an `Error` if an 
    /// error occurs.
    /// 
    async fn complete(
        &self, 
        req: &CompletionRequest
    ) -> Result<CompletionResponse, Error>;
}


/// Represents a completion request to a generative AI provider. This struct contains the 
/// necessary information for the provider to generate a response.
/// 
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    /// The system prompt that guides the behavior of the AI model.
    system: String,
    /// A list of tool definitions available to the AI model.
    tools: Vec<ToolDefinition>,
    /// A list of messages that form the conversation history.
    messages: Vec<Message>,
    /// The maximum number of tokens to generate in the response.
    max_tokens: u32,
}

/// Represents a completion response from a generative AI provider. This struct contains the
/// generated content and the reason why the completion stopped.
/// 
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    /// The content of the completion response.
    content: Vec<Block>,
    /// The reason why the completion stopped.
    stop_reason: StopReason,
}

/// The reason why a completion stopped. This enum represents the possible reasons for stopping
/// a completion, such as reaching the end of the turn, using a tool, or other.
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StopReason {
    /// The completion stopped because the end of the turn was reached.
    EndTurn,
    /// The completion stopped because a tool was used.
    ToolUse,
    /// The completion stopped for another reason.
    Other,
}

/// Configuration for the API provider. This struct contains the necessary information to
/// configure and initialize an API provider, including the name, URL, model, API key,
/// and provider type.
/// 
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// The name of the provider.
    pub name: String,
    /// The URL of the provider's API endpoint.
    pub url: String,
    /// The model used by the provider.
    pub model: String,
    /// The API key for the provider, if required.
    pub api_key: Option<String>,
    /// The type of the provider.
    pub provider_type: ProviderType,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    #[cfg(test)]
    Mock,
}

#[cfg(test)]
mod tests {
    use super::*;
    


    pub struct MockProvider;

    #[async_trait]
    impl Provider for MockProvider {
        async fn complete(
            &self, 
            req: &CompletionRequest
        ) -> Result<CompletionResponse, Error> {
            let block = Block::Text{ text: format!("Mock response from system: {}", req.system) };
            Ok(CompletionResponse {           
                content: vec![block],
                stop_reason: StopReason::EndTurn,
            })
        }
    }

    #[tokio::test]
    async fn test_mock_provider() {
        let api_provider = MockProvider;
        let mut message = Message::new(Role::User);
        message.add_text("Text block");
        
        let request = CompletionRequest {
            system: "Test system prompt".to_string(),
            tools: vec![],
            messages: vec![message],
            max_tokens: 50,
        };

        let response = api_provider.complete(&request).await.unwrap();
        let text = match &response.content[0] {
            Block::Text { text } => text.clone(),
            _ => panic!("Expected a text block"),
        };
        assert_eq!(text, "Mock response from system: Test system prompt");
        assert_eq!(response.stop_reason, StopReason::EndTurn);
    }
}