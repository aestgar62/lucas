//

use crate::error::Error;
use serde::Deserialize;


pub struct Agent {
    pub header: AgentHeader,
    pub body: String,
}

impl Agent {

    pub fn deserialize(doc: &str) -> Result<Agent, Error> {
        let (header, body) = markdown_frontmatter::parse::<AgentHeader>(doc)?;
        Ok(Agent { header, body: body.to_owned() })
    }
}

#[derive(Clone, Deserialize)]
pub struct AgentHeader {
    pub name: String,
    pub description: String,
    pub version: String,
    pub mode: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub steps: Option<u32>,   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_desrialize() {
        let agent_doc = r#"---
name: "Test Agent"
description: "This is a test agent"
version: "1.0"
mode: "test"
---
            # Agent
            ## Instructions
            - Instriction 1
            - Instriction 2
            ## Examples
            - Example 1
            - Example 2
        "#;

        let agent = Agent::deserialize(agent_doc).unwrap();
        assert_eq!(agent.header.name, "Test Agent");
        assert_eq!(agent.header.description, "This is a test agent");
        assert_eq!(agent.header.version, "1.0");
        assert_eq!(agent.header.mode, "test");
        //assert_eq!(agent.header.model.as_deref(), Some("gpt-4"));
        //assert_eq!(agent.header.temperature, Some(0.7));
        //assert_eq!(agent.header.steps, Some(10));
    }
}