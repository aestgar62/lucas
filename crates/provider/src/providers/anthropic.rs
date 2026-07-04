//
use crate::{
    Provider, Config, CompletionRequest, CompletionResponse, StopReason, Error,
    message::{Block, Message, Role},
};

use tools::ToolDefinition;

use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use async_trait::async_trait;

/// An API provider for the Anthropic API.
pub struct AnthropicApi {
    client: Client,
    config: Config,
}

impl AnthropicApi {
    pub fn new(config: &Config) -> Self {
        let client = Client::new();
        Self { client, config: config.clone() }
    }

}

#[async_trait]
impl Provider for AnthropicApi {

    async fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, Error> {
        // Convert the request messages to the format expected by the Anthropic API
        let messages = request.messages.iter().map(|msg| {
            AnthropichMessage::from(msg)
        }).collect();
        let body = AnthropicRequest {
            model: &self.config.model,
            max_tokens: request.max_tokens,
            messages,
            tools: request.tools.clone(),
        };

        // Send the request to the Anthropic API
        let response = self.client.post(&self.config.url)
            .header("x-api-key", self.config.api_key.clone().unwrap_or_default())
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        // Parse the response
        let status = response.status().as_u16();
        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(Error::HttpResponse(status, text));
        }
        let resp_body: AnthropicResponse = response.json().await?;
        let content = resp_body.content.into_iter().map(|block| Block::from(block)).collect();
        let stop_reason = match resp_body.stop_reason.as_deref() {
            Some("tool_use") => StopReason::ToolUse,
            Some("end_turn") => StopReason::EndTurn,
            _ => StopReason::Other,
        };
        Ok(CompletionResponse { content, stop_reason })
    }
}

#[derive(Serialize)]
struct AnthropicRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    messages: Vec<AnthropichMessage>,
    tools: Vec<ToolDefinition>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum AnthropicBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: Value },
    ToolResult { tool_use_id: String, content: String },
}

impl From<&Block> for AnthropicBlock {
    fn from(block: &Block) -> Self {
        match block {
            Block::Text { text } => AnthropicBlock::Text { text: text.clone() },
            Block::ToolCall { id, name, input } => AnthropicBlock::ToolUse { id: id.clone(), name: name.clone(), input: input.clone() },
            Block::ToolResult { tool_use_id, content } => AnthropicBlock::ToolResult { tool_use_id: tool_use_id.clone(), content: content.clone() },
        }
    }
}

impl From<AnthropicBlock> for Block {
    fn from(block: AnthropicBlock) -> Self {
        match block {
            AnthropicBlock::Text { text } => Block::Text { text },
            AnthropicBlock::ToolUse { id, name, input } => Block::ToolCall { id, name, input },
            AnthropicBlock::ToolResult { tool_use_id, content } => Block::ToolResult { tool_use_id, content },
        }
    }
}

#[derive(Serialize)]
struct AnthropichMessage {
    role: &'static str,
    content: Vec<AnthropicBlock>,
}

impl From<&Message> for AnthropichMessage {
    fn from(msg: &Message) -> Self {
        Self {
            role: match msg.role {
                Role::User => "user",
                Role::Assistant => "assistant",
            },
            content: msg.content.iter().map(|block| block.into()).collect(),
        }
    }
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicBlock>,
    stop_reason: Option<String>,
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_anthropic_block_serialization() {}
}