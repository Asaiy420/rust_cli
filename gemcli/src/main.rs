use clap::Parser;
use colored::*;
use dotenvy::dotenv;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(name = "gemcli")]
#[command(about = "Fast AI using Gemini from your terminal")]
struct Args {
    /// Your prompt text
    prompt: Vec<String>,

    /// Optional model override
    #[arg(short, long, default_value = "gemini-2.5-flash-lite")]
    model: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // Load .env if present
    let args = Args::parse();

    // Detect piped input
    let prompt = if atty::isnt(atty::Stream::Stdin) {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf.trim().to_string()
    } else {
        args.prompt.join(" ")
    };

    if prompt.trim().is_empty() {
        eprintln!("Usage: gemcli <prompt>");
        std::process::exit(1);
    }

    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY is not set in system environment or .env");

    let system_prompt = r#"You are a fast terminal AI helper. Respond in concise, clean, Unix-style output:
- Keep answers short and focused.
- Use headings, bullet points, code blocks.
- No emojis.
- Plain text only.
"#;

    let model = args.model;
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?key={}&alt=sse",
        model, api_key
    );

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": format!("{} \nUser: {}", system_prompt, prompt)
            }]
        }]
    });

    let client = Client::new();

    let res = client.post(url).json(&body).send().await?;
    let mut stream = res.bytes_stream();

    print!("{}", "GemCLI: ".green().bold());

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(&chunk);

        for line in text.lines() {
            if !line.starts_with("data: ") {
                continue;
            }

            let json_str = line.trim_start_matches("data: ").trim();

            if json_str == "[DONE]" {
                break;
            }

            let Ok(json) = serde_json::from_str::<Value>(json_str) else {
                continue;
            };

            if let Some(t) = json["candidates"]
                .get(0)
                .and_then(|c| c["content"]["parts"].get(0))
                .and_then(|p| p["text"].as_str())
            {
                print!("{}", t);
            }
        }
    }

    println!();
    Ok(())
}
