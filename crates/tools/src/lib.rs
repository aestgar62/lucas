//

mod edit;

use serde::Serialize;
use serde_json::Value;

/// Defines a tool that can be used by the AI model.
#[derive(Debug, Clone, Serialize)]
pub struct ToolDefinition {
    /// The type of the tool, which is optional and can be used to specify the kind of tool.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The name of the tool, which is a required field.
    pub name: String,
    /// A description of the tool, which is a required field.
    pub description: String,
    /// The input schema for the tool, which is a required field and defines the expected input 
    /// parameters.
    pub input_schema: Value,
}
