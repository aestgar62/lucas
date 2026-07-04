//

use crate::{Config, message::{Message, Role}};

use tools::ToolDefinition;

use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::Value;

/// An API provider for the OpenAI API.
pub struct OpenAiApi {
    /// The HTTP client used to send requests to the OpenAI API.
    client: Client,
    /// The configuration for the OpenAI API provider.
    config: Config,
}

impl OpenAiApi {
    /// Creates a new instance of the OpenAiApi provider with the given configuration.
    /// 
    /// # Arguments
    ///
    /// * `config` - A reference to the configuration for the OpenAI API provider.
    /// 
    /// # Returns
    /// 
    /// A new instance of the OpenAiApi provider.
    /// 
    pub fn new(config: &Config) -> Self {
        let client = Client::new();
        Self { client, config: config.clone() }
    }
}

#[derive(Serialize)]
struct OpenAiRequest<'a> {
    model: &'a str,
    max_tokens: u32,
    messages: Vec<OpenAiMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<OpenAiTool>,
}

#[derive(Serialize)]
struct OpenAiMessage {
    role: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tool_calls: Vec<OpenAiToolCall>,
}

impl From<&Message> for OpenAiMessage {
    fn from(msg: &Message) -> Self {
        let content = String::new();
        match msg.role {
            Role::User => {}                
            Role::Assistant => {},
        }
        todo!()
    }
}

#[derive(Serialize)]
struct OpenAiToolCall {
    id: String,
    #[serde(rename = "type")]
    kind: &'static str,
    function: OpenAiToolCallFn,
}

#[derive(Serialize)]
struct OpenAiToolCallFn {
    name: String,
    arguments: String,
}

#[derive(Serialize)]
struct OpenAiTool {
    #[serde(rename = "type")]
    kind: String,
    function: OpenAiToolFn,
}

#[derive(Serialize)]
struct OpenAiToolFn {
    name: String,
    description: String,
    parameters: Value,
}

impl From<&ToolDefinition> for OpenAiTool {
    fn from(tool: &ToolDefinition) -> Self {
        OpenAiTool {
            kind: tool.kind.clone().unwrap_or_else(|| "function".to_string()),
            function: OpenAiToolFn {
                name: tool.name.clone(),
                description: tool.description.clone(),
                parameters: tool.input_schema.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_openai_api() {}
}