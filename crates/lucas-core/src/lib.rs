//

//! Core types and traits for the Lucas framework.
//! This crate provides the foundational building blocks for the Lucas framework, including core 
//! data structures, traits, and utilities that are used across other crates in the workspace. 
//! It is designed to be lightweight and efficient, providing essential functionality without 
//! unnecessary overhead.
//! 

use serde::{Serialize, Deserialize};
use serde_json::Value;

/// The role in a conversation, indicating whether the message is from the user, assistant, 
/// system, or a tool.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// The user role, representing messages sent by the user.
    User,
    /// The assistant role, representing messages sent by the AI assistant.
    Assistant,
    /// The system role, representing messages sent by the system.
    System,
    /// The tool role, representing messages sent by a tool.
    Tool,
}

/// A block of content in a message, which can be text, a tool call, or a tool result.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Block {
    /// A text block, representing plain text content.
    Text { text: String },
    /// A tool call block, representing a call to a tool with input parameters.
    ToolCall { 
        id: String, 
        name: String, 
        input: Value 
    },
    /// A tool result block, representing the result of a tool call.
    ToolResult { tool_use_id: String, content: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Message {
    role: Role,
    content: Vec<Block>,
}

impl Message {
    /// Creates a new message with the specified role and empty content.
    /// 
    /// # Arguments
    ///
    /// * `role` - The role of the message (User, Assistant, System, or Tool).
    /// 
    /// # Returns
    /// 
    /// A new `Message` instance with the specified role and empty content.
    /// 
    pub fn new(role: Role) -> Self {
        Self { role, content: Vec::new() }
    }

    /// Returns the role of the message.
    /// 
    /// # Returns
    /// 
    /// A reference to the `Role` of the message.
    /// 
    pub fn role(&self) -> &Role {
        &self.role
    }

    /// Returns the content of the message.
    /// 
    /// # Returns
    /// 
    /// A reference to the vector of `Block`s representing the content of the message.
    /// 
    pub fn content(&self) -> &Vec<Block> {
        &self.content
    }

    /// Adds a text block of content to the message.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The text content to add to the message.
    /// 
    pub fn add_text(&mut self, text: impl Into<String>) {
        self.content.push(Block::Text { text: text.into() });
    }

    /// Adds a tool call block of content to the message.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The unique identifier for the tool call.
    /// * `name` - The name of the tool being called.
    /// * `input` - The input parameters for the tool call, represented as a 
    ///   `serde_json::Value`.
    ///
    pub fn add_tool_call(
        &mut self, 
        id: impl Into<String>, 
        name: impl Into<String>, 
        input: Value
    ) {
        self.content.push(Block::ToolCall { 
            id: id.into(), 
            name: name.into(), 
            input 
        });
    }

    /// Adds a tool result block of content to the message.
    /// 
    /// # Arguments
    /// 
    /// * `tool_use_id` - The unique identifier for the tool use that produced this result.
    /// * `content` - The content of the tool result.
    /// 
    pub fn add_tool_result(
        &mut self, 
        tool_use_id: impl Into<String>, 
        content: impl Into<String>
    ) {
        self.content.push(Block::ToolResult { 
            tool_use_id: tool_use_id.into(), 
            content: content.into() 
        }); 
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Session {
    id: String,
    messages: Vec<Message>,
}

impl Session {
    /// Creates a new session with the specified ID and empty messages.
    /// 
    /// # Arguments
    /// 
    /// * `id` - The unique identifier for the session.
    /// 
    /// # Returns
    /// 
    /// A new `Session` instance with the specified ID and empty messages.
    /// 
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), messages: Vec::new() }
    }

    /// Returns the ID of the session.
    /// 
    /// # Returns
    /// 
    /// A reference to the session's ID.
    /// 
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Returns the messages in the session.
    /// 
    /// # Returns
    /// 
    /// A reference to the vector of `Message`s in the session.
    /// 
    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    /// Adds a message to the session.
    /// 
    /// # Arguments
    /// 
    /// * `message` - The message to add to the session.
    /// 
    pub fn add_message(&mut self, message: &Message) {
        self.messages.push(message.clone());
    }
}    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let mut message = Message::new(Role::User);
        message.add_text("Hello, Assistant!");
        message.add_tool_call("tool_1", "Calculator", serde_json::json!({"operation": "add", "operands": [1, 2]}));
        message.add_tool_result("tool_1", "Result: 3"); 

        assert_eq!(message.role(), &Role::User);
        assert_eq!(message.content().len(), 3); 
        let block = &message.content()[0];
        assert!(matches!(block, &Block::Text { .. }));
        let block = &message.content()[1];
        assert!(matches!(block, &Block::ToolCall { .. }));
        let block = &message.content()[2];
        assert!(matches!(block, &Block::ToolResult { .. }));
    }

    #[test]
    fn test_session_creation() {
        let mut session = Session::new("session_1");
        let mut message = Message::new(Role::User);
        message.add_text("Hello, Assistant!");
        session.add_message(&message);
        assert_eq!(session.id(), "session_1");
        assert_eq!(session.messages().len(), 1);
    }
}