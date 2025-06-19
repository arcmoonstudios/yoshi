//! # MCP Server Implementation for Yoshi MCP
//!
//! This module implements the Model Context Protocol server for VS Code integration
//! with complete Yoshi error handling and context management.

use crate::{error::YoshiMcpError, inference::InferenceEngine, Hatch, YoshiMcpConfig};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use yoshi_std::{Yoshi, YoshiKind};

/// MCP server capabilities
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct McpCapabilities {
    /// Available tools
    pub tools: Vec<McpTool>,
    /// Available resources
    pub resources: Vec<McpResource>,
    /// Available prompts
    pub prompts: Vec<McpPrompt>,
}

/// MCP tool definition
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct McpTool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema
    pub input_schema: Value,
}

/// MCP resource definition
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct McpResource {
    /// Resource URI
    pub uri: String,
    /// Resource name
    pub name: String,
    /// Resource description
    pub description: String,
}

/// MCP prompt definition
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct McpPrompt {
    /// Prompt name
    pub name: String,
    /// Prompt description
    pub description: String,
    /// Prompt arguments
    pub arguments: Vec<McpPromptArgument>,
}

/// MCP prompt argument
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct McpPromptArgument {
    /// Argument name
    pub name: String,
    /// Argument description
    pub description: String,
    /// Whether argument is required
    pub required: bool,
}

/// MCP server for handling VS Code integration
#[derive(Debug)]
pub struct McpServer {
    /// Server configuration
    config: YoshiMcpConfig,
    /// Inference engine reference
    inference_engine: Option<Arc<RwLock<InferenceEngine>>>,
    /// Server capabilities
    capabilities: McpCapabilities,
    /// Server running state
    running: Arc<RwLock<bool>>,
}

impl McpServer {
    /// Create a new MCP server
    pub async fn new(config: YoshiMcpConfig) -> Hatch<Self> {
        let capabilities = Self::create_capabilities();
        let running = Arc::new(RwLock::new(false));

        Ok(Self {
            config,
            inference_engine: None,
            capabilities,
            running,
        })
    }

    /// Create server capabilities
    fn create_capabilities() -> McpCapabilities {
        McpCapabilities {
            tools: vec![
                McpTool {
                    name: "generate_code".to_string(),
                    description: "Generate code based on natural language description".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "description": {
                                "type": "string",
                                "description": "Natural language description of the code to generate"
                            },
                            "language": {
                                "type": "string",
                                "description": "Programming language (optional)",
                                "default": "rust"
                            },
                            "max_tokens": {
                                "type": "number",
                                "description": "Maximum tokens to generate",
                                "default": 200
                            }
                        },
                        "required": ["description"]
                    }),
                },
                McpTool {
                    name: "explain_code".to_string(),
                    description: "Explain existing code functionality".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "code": {
                                "type": "string",
                                "description": "Code to explain"
                            },
                            "language": {
                                "type": "string",
                                "description": "Programming language",
                                "default": "rust"
                            }
                        },
                        "required": ["code"]
                    }),
                },
                McpTool {
                    name: "fix_code".to_string(),
                    description: "Fix code errors and improve quality".to_string(),
                    input_schema: json!({
                        "type": "object",
                        "properties": {
                            "code": {
                                "type": "string",
                                "description": "Code to fix"
                            },
                            "error_message": {
                                "type": "string",
                                "description": "Error message or description of the issue"
                            },
                            "language": {
                                "type": "string",
                                "description": "Programming language",
                                "default": "rust"
                            }
                        },
                        "required": ["code"]
                    }),
                },
            ],
            resources: vec![
                McpResource {
                    uri: "local://workspace/files".to_string(),
                    name: "Workspace Files".to_string(),
                    description: "Access to workspace file contents for context".to_string(),
                },
                McpResource {
                    uri: "local://model/status".to_string(),
                    name: "Model Status".to_string(),
                    description: "Current model loading and inference status".to_string(),
                },
            ],
            prompts: vec![McpPrompt {
                name: "code_review".to_string(),
                description: "Perform a comprehensive code review".to_string(),
                arguments: vec![
                    McpPromptArgument {
                        name: "code".to_string(),
                        description: "Code to review".to_string(),
                        required: true,
                    },
                    McpPromptArgument {
                        name: "focus".to_string(),
                        description: "Specific areas to focus on (security, performance, etc.)"
                            .to_string(),
                        required: false,
                    },
                ],
            }],
        }
    }

    /// Set the inference engine
    pub fn set_inference_engine(&mut self, engine: Arc<RwLock<InferenceEngine>>) {
        self.inference_engine = Some(engine);
    }

    /// Start the MCP server
    pub async fn start(&self) -> Hatch<()> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(()); // Already running
        }

        // Initialize server components
        self.initialize_server().await?;

        *running = true;
        Ok(())
    }

    /// Stop the MCP server
    pub async fn stop(&self) -> Hatch<()> {
        let mut running = self.running.write().await;
        *running = false;
        Ok(())
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Initialize server components
    async fn initialize_server(&self) -> Hatch<()> {
        // Server initialization logic would go here
        // For now, just validate configuration
        self.validate_configuration()?;
        Ok(())
    }

    /// Validate server configuration
    fn validate_configuration(&self) -> Hatch<()> {
        if self.config.max_tokens == 0 {
            return Err(YoshiMcpError::config_invalid(
                "max_tokens",
                "positive integer",
                "0",
            ));
        }

        if self.config.temperature < 0.0 || self.config.temperature > 2.0 {
            return Err(YoshiMcpError::config_invalid(
                "temperature",
                "0.0 to 2.0",
                &self.config.temperature.to_string(),
            ));
        }

        Ok(())
    }

    /// Handle MCP tool call
    pub async fn handle_tool_call(&self, tool_name: &str, arguments: Value) -> Hatch<Value> {
        match tool_name {
            "generate_code" => self.handle_generate_code(arguments).await,
            "explain_code" => self.handle_explain_code(arguments).await,
            "fix_code" => self.handle_fix_code(arguments).await,
            _ => Err(YoshiMcpError::config_invalid(
                "tool_name",
                "valid tool name",
                tool_name,
            )),
        }
    }

    /// Handle code generation tool
    async fn handle_generate_code(&self, arguments: Value) -> Hatch<Value> {
        let description = arguments["description"]
            .as_str()
            .ok_or_else(|| YoshiMcpError::config_invalid("description", "string", "missing"))?;

        let language = arguments["language"].as_str().unwrap_or("rust");
        let max_tokens = arguments["max_tokens"].as_u64().unwrap_or(200) as usize;

        let prompt = format!(
            "Generate {} code for the following description:\n\n{}\n\nCode:",
            language, description
        );

        let result = self.generate_with_inference(&prompt, max_tokens).await?;

        Ok(json!({
            "content": [{
                "type": "text",
                "text": result
            }]
        }))
    }

    /// Handle code explanation tool
    async fn handle_explain_code(&self, arguments: Value) -> Hatch<Value> {
        let code = arguments["code"]
            .as_str()
            .ok_or_else(|| YoshiMcpError::config_invalid("code", "string", "missing"))?;

        let language = arguments["language"].as_str().unwrap_or("rust");

        let prompt = format!(
            "Explain the following {} code:\n\n```{}\n{}\n```\n\nExplanation:",
            language, language, code
        );

        let result = self.generate_with_inference(&prompt, 300).await?;

        Ok(json!({
            "content": [{
                "type": "text",
                "text": result
            }]
        }))
    }

    /// Handle code fixing tool
    async fn handle_fix_code(&self, arguments: Value) -> Hatch<Value> {
        let code = arguments["code"]
            .as_str()
            .ok_or_else(|| YoshiMcpError::config_invalid("code", "string", "missing"))?;

        let error_message = arguments["error_message"].as_str().unwrap_or("");
        let language = arguments["language"].as_str().unwrap_or("rust");

        let prompt = if error_message.is_empty() {
            format!(
                "Fix and improve the following {} code:\n\n```{}\n{}\n```\n\nFixed code:",
                language, language, code
            )
        } else {
            format!(
                "Fix the following {} code that has this error: {}\n\n```{}\n{}\n```\n\nFixed code:",
                language, error_message, language, code
            )
        };

        let result = self.generate_with_inference(&prompt, 400).await?;

        Ok(json!({
            "content": [{
                "type": "text",
                "text": result
            }]
        }))
    }

    /// Generate text using the inference engine
    async fn generate_with_inference(&self, prompt: &str, max_tokens: usize) -> Hatch<String> {
        let engine = self.inference_engine.as_ref().ok_or_else(|| {
            YoshiMcpError::inference_failed(prompt, "Inference engine not available")
        })?;

        let engine_guard = engine.read().await;
        engine_guard.generate_text(prompt, max_tokens).await
    }

    /// Get server capabilities
    pub fn get_capabilities(&self) -> &McpCapabilities {
        &self.capabilities
    }

    /// Handle resource request
    pub async fn handle_resource_request(&self, uri: &str) -> Hatch<Value> {
        match uri {
            "local://model/status" => {
                let status = if let Some(engine) = &self.inference_engine {
                    let engine_guard = engine.read().await;
                    json!({
                        "model_loaded": engine_guard.is_model_loaded(),
                        "metrics": engine_guard.get_metrics()
                    })
                } else {
                    json!({
                        "model_loaded": false,
                        "metrics": {}
                    })
                };

                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": status.to_string()
                    }]
                }))
            }
            _ => Err(YoshiMcpError::config_invalid(
                "resource_uri",
                "valid resource URI",
                uri,
            )),
        }
    }
}
