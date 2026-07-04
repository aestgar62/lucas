//

use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<Block>,
}

#[derive(Debug, Clone, Serialize)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize)]
pub enum Block {
    Text { text: String },
    ToolCall { 
        id: String, 
        name: String, 
        input: Value 
    },
    ToolResult { tool_use_id: String, content: String },
}
