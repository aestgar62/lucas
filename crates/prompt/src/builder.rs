//

use crate::prompt::Prompt;
use serde::{Deserialize, Serialize};

/// A builder for constructing prompts with various components such as identity, instructions, 
/// output, examples, and context.
/// 
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PromptBuilder {
    /// The identity of the prompt, which can be used to identify the source or purpose of the 
    /// prompt.
    identity: Option<String>,
    /// A list of instructions for the prompt, which can guide the user on how to interact with it.
    instructions: Vec<String>,
    /// The expected output of the prompt, which can be used to validate the user's response.
    output: Option<String>,
    /// A list of examples for the prompt, which can help the user understand the expected input 
    /// and output.
    examples: Vec<(String, String)>,
    /// Additional context information for the prompt, which can provide more details or 
    /// background.
    context: Vec<String>,
}

impl PromptBuilder {

    /// Creates a new instance of `PromptBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the identity of the prompt.
    /// 
    /// # Arguments
    /// 
    /// * `identity` - A string representing the identity of the prompt.
    /// 
    /// # Returns
    /// 
    /// A `PromptBuilder` instance with the updated identity.
    /// 
    pub fn with_identity(mut self, identity: impl Into<String>) -> Self {
        self.identity = Some(identity.into());
        self
    }

    /// Adds an instruction to the prompt.
    /// 
    /// # Arguments
    /// 
    /// * `instruction` - A string representing the instruction to be added.
    /// 
    /// # Returns
    /// 
    /// A `PromptBuilder` instance with the updated instructions.
    /// 
    pub fn with_instruction(mut self, instruction: impl Into<String>) -> Self {
        self.instructions.push(instruction.into());
        self
    }

    /// Sets the expected output of the prompt.
    /// 
    /// # Arguments
    /// 
    /// * `output` - A string representing the expected output of the prompt.
    /// 
    /// # Returns
    /// 
    /// A `PromptBuilder` instance with the updated output.
    /// 
    pub fn with_output(mut self, output: impl Into<String>) -> Self {
        self.output = Some(output.into());
        self
    }

    /// Adds an example to the prompt.
    /// 
    /// # Arguments
    /// 
    /// * `input` - A string representing the input example.
    /// * `output` - A string representing the output example.
    /// 
    /// # Returns
    /// 
    /// A `PromptBuilder` instance with the updated examples.
    /// 
    pub fn with_example(mut self, input: impl Into<String>, output: impl Into<String>) -> Self {
        self.examples.push((input.into(), output.into()));
        self
    }

    /// Adds additional context information to the prompt.
    /// 
    /// # Arguments
    /// 
    /// * `context` - A string representing the additional context information.
    /// 
    /// # Returns
    /// 
    /// A `PromptBuilder` instance with the updated context.
    /// 
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }

    /// Builds the `Prompt` instance.
    /// 
    /// # Returns
    /// 
    /// A `Prompt` instance with the configured properties.
    /// 
    pub fn build(self) -> Prompt {
        // Static layers (cacheable prefix).
        let mut static_layers: Vec<String> = Vec::new();

        if let Some(id) = self.identity {
            static_layers.push(format!("## Identity\n\n{id}"));
        }
        if !self.instructions.is_empty() {
            let body = self
                .instructions
                .iter()
                .map(|i| format!("- {i}"))
                .collect::<Vec<_>>()
                .join("\n");
            static_layers.push(format!("## Instructions\n\n{body}"));
        }
        if let Some(out) = self.output {
            static_layers.push(format!("## Output\n\n{out}"));
        }
        if !self.examples.is_empty() {
            let body = self
                .examples
                .iter()
                .enumerate()
                .map(|(i, (u, a))| {
                    format!("Example {}:\nUser: {u}\nAssistant: {a}", i + 1)
                })
                .collect::<Vec<_>>()
                .join("\n\n");
            static_layers.push(format!("## Examples\n\n{body}"));
        }

        // Dynamic layers (per-turn suffix).
        let mut dynamic_layers: Vec<String> = Vec::new();
        if !self.context.is_empty() {
            dynamic_layers.push(format!("## Context\n\n{}", self.context.join("\n\n")));
        }

        Prompt {
            cacheable_prefix: static_layers.join("\n\n"),
            dynamic_suffix: dynamic_layers.join("\n\n"),
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_prompt_builder() {
        let prompt = PromptBuilder::new()
            .with_identity("MyApp")
            .with_instruction("Please provide your input.")
            .with_output("Your output will be generated.")
            .with_example("Input example", "Output example")
            .with_context("Additional context information.");

        assert_eq!(prompt.identity, Some("MyApp".to_string()));
        assert_eq!(prompt.instructions.len(), 1);
        assert_eq!(prompt.output, Some("Your output will be generated.".to_string()));
        assert_eq!(prompt.examples.len(), 1);
        assert_eq!(prompt.context.len(), 1);
    }
}   