use reqwest::blocking::Client;

use crate::data::{structs, LLM_PROMPT};

pub fn generate_text(prompt: &String, model: &String, base_url: &String, token: &String) -> Result<String, String> {
    let client = Client::new();

    let url = {
        if base_url.ends_with('/') {
            format!("{}chat/completions", base_url)
        } else {
            format!("{}/chat/completions", base_url)
        }
    };

    let res = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": LLM_PROMPT,
                },
                {
                    "role": "user",
                    "content": prompt,
                },
            ]
        }))
        .send()
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let response: structs::OpenAIResponse = res.json::<structs::OpenAIResponse>().map_err(|e| e.to_string())?;
        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No choices in the AI response—maybe it's on a coffee break?".to_string())
        }
    } else {
        Err(format!("HTTP error: {}", res.status()))
    }
}
