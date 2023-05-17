use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OpenaiChatCompletionRequest {
  pub model: String,
  pub messages: Vec<OpenaiChatCompletionMessage>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenaiChatCompletionMessage {
  pub role: String,
  pub content: String,
}

#[derive(Deserialize)]
pub struct OpenAiChatCompletionResponse {
  pub id: String,
  pub object: String,
  pub created: u64,
  pub choices: Vec<OpenaiChatCompletionResponseChoice>,
}

#[derive(Deserialize)]
pub struct OpenaiChatCompletionResponseChoice {
  pub index: usize,
  pub message: OpenaiChatCompletionMessage,
  pub finish_reason: String,
}

#[derive(Deserialize)]
pub struct OpenAiChatCompletionResponseUsage {
  pub prompt_tokens: u64,
  pub completion_tokens: u64,
  pub total_tokens: u64,
}
