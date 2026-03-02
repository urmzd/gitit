pub mod claude;
pub mod gemini;

use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct AiRequest {
    pub system_prompt: String,
    pub user_prompt: String,
    pub json_schema: Option<String>,
    pub working_dir: String,
}

#[derive(Debug, Clone)]
pub struct AiResponse {
    pub text: String,
}

#[async_trait]
pub trait AiBackend: Send + Sync {
    fn name(&self) -> &str;
    async fn is_available(&self) -> bool;
    async fn request(&self, req: &AiRequest) -> Result<AiResponse>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Backend {
    Claude,
    Gemini,
}

pub struct BackendConfig {
    pub backend: Option<Backend>,
    pub model: Option<String>,
    pub budget: f64,
    pub debug: bool,
}

pub async fn resolve_backend(config: &BackendConfig) -> Result<Box<dyn AiBackend>> {
    let preferred = config.backend;

    let claude = claude::ClaudeBackend::new(config.model.clone(), config.budget, config.debug);
    let gemini = gemini::GeminiBackend::new(config.model.clone(), config.debug);

    match preferred {
        Some(Backend::Claude) => {
            if claude.is_available().await {
                return Ok(Box::new(claude));
            }
            eprintln!("Warning: claude CLI not found, falling back to gemini");
            if gemini.is_available().await {
                return Ok(Box::new(gemini));
            }
        }
        Some(Backend::Gemini) => {
            if gemini.is_available().await {
                return Ok(Box::new(gemini));
            }
            eprintln!("Warning: gemini CLI not found, falling back to claude");
            if claude.is_available().await {
                return Ok(Box::new(claude));
            }
        }
        None => {
            if claude.is_available().await {
                return Ok(Box::new(claude));
            }
            if gemini.is_available().await {
                return Ok(Box::new(gemini));
            }
        }
    }

    anyhow::bail!(crate::error::GitAiError::NoBackendAvailable)
}
