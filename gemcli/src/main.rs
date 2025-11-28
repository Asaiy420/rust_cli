use clap::Parser;
use dotenvy::dotenv;
use reqwest::Client;
use serde_json::Value;
use std::env;

#[derive(Parser, Debug)]
#[command(name = "gemcli")]
#[command(about = "Fast AI using Gemini from your terminal")]
struct Args {
    prompt: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();

    let prompt = args.prompt.join(" ");

    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY is not set in system environment or .env"); // Free Gemini model
    let model = "gemini-2.5-flash-lite";

    // Correct endpoint for free Gemini models
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let system_prompt = "You are a fast terminal AI helper. Respond in concise, clean, Unix-style output: \
- Keep answers short and focused (5–10 lines max). \
- Use clear headings, bullet points, and code blocks. \
- Never add unnecessary stories or analogies. \
- Prefer examples over explanations. \
- Avoid emojis. \
- Output plain text only.";
    // JSON structure Gemini expects
    let body = serde_json::json!({
        "contents": [
            {
                "parts": [
                    { "text": system_prompt.to_string() + "\nUser: " + &prompt}
                ]
            }
        ]
    });

    let client = Client::new();

    let res = client.post(&url).json(&body).send().await?;

    let json: Value = res.json().await?;

    if json.get("error").is_some() {
        eprintln!("❌ Gemini API Error:");
        eprintln!("{}", json);
        return Ok(());
    }

    // Extract text safely
    if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        println!("{}", text);
    } else {
        println!("⚠️ No text returned. Full response:\n{}", json);
    }

    Ok(())
}
