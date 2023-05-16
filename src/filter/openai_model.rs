use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OpenaiChatCompletionRequest<'a> {
  pub model: &'a str,
  pub messages: Vec<OpenaiChatCompletionMessage<'a>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenaiChatCompletionMessage<'a> {
  #[serde(borrow)]
  pub role: Cow<'a, str>,
  #[serde(borrow)]
  pub content: Cow<'a, str>,
}

#[derive(Deserialize)]
pub struct OpenAiChatCompletionResponse<'a> {
  #[serde(borrow)]
  pub id: Cow<'a, str>,
  #[serde(borrow)]
  pub object: Cow<'a, str>,
  pub created: u64,
  #[serde(borrow)]
  pub choices: Vec<OpenaiChatCompletionResponseChoice<'a>>,
}

#[derive(Deserialize)]
pub struct OpenaiChatCompletionResponseChoice<'a> {
  pub index: usize,
  #[serde(borrow)]
  pub message: OpenaiChatCompletionMessage<'a>,
  #[serde(borrow)]
  pub finish_reason: Cow<'a, str>,
}

#[derive(Deserialize)]
pub struct OpenAiChatCompletionResponseUsage {
  pub prompt_tokens: u64,
  pub completion_tokens: u64,
  pub total_tokens: u64,
}
