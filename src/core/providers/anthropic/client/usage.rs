use serde_json::Value;

use crate::core::types::responses::{PromptTokensDetails, Usage};

pub(crate) fn build_usage(usage_data: &Value) -> Usage {
    let read = |key: &str| usage_data.get(key).and_then(|v| v.as_u64()).unwrap_or(0);
    // Saturate instead of `as u32`, which silently wraps on overflow.
    let to_u32 = |v: u64| u32::try_from(v).unwrap_or(u32::MAX);
    let cache_creation = usage_data
        .get("cache_creation_input_tokens")
        .and_then(Value::as_u64);
    let cache_read = usage_data
        .get("cache_read_input_tokens")
        .and_then(Value::as_u64);
    let input_tokens = read("input_tokens");
    let prompt_tokens = input_tokens
        .saturating_add(cache_creation.unwrap_or(0))
        .saturating_add(cache_read.unwrap_or(0));
    let completion_tokens = read("output_tokens");

    Usage {
        prompt_tokens: to_u32(prompt_tokens),
        completion_tokens: to_u32(completion_tokens),
        total_tokens: to_u32(prompt_tokens.saturating_add(completion_tokens)),
        completion_tokens_details: None,
        prompt_tokens_details: if cache_creation.is_some() || cache_read.is_some() {
            Some(PromptTokensDetails {
                cached_tokens: cache_read.map(to_u32),
                cache_creation_tokens: cache_creation.map(to_u32),
                cache_read_tokens: cache_read.map(to_u32),
                audio_tokens: None,
            })
        } else {
            None
        },
        thinking_usage: None,
    }
}
