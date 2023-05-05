extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatCompletionRequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

fn main() {
        let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <text_file_path>");
        return;
    }

    let text_file_path = &args[1];
    let text = fs::read_to_string(text_file_path).expect("Failed to read the text file");

    match env::var("OPENAI_API_KEY") {
        Ok(api_key) => {
            let system_message = Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            };

            let user_message = Message {
                role: "user".to_string(),
                content: text,
            };

            let chat_completion_request_body = ChatCompletionRequestBody {
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![system_message, user_message],
            };

            let response = query_chat_completion_api(api_key, chat_completion_request_body);

            match response {
                Ok(chat_completion_response) => {
                    let assistant_response = &chat_completion_response.choices[0].message.content;
                    println!("{}", assistant_response);
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                }
            }
        }
        Err(_) => {
            println!(
                "═══════════════════════════════════════════════════════════════\n\
                 ⚠️  OPENAI_API_KEY environment variable not set                ⚠️\n\
                 ───────────────────────────────────────────────────────────────\n\
                 Please set the OPENAI_API_KEY environment variable with your   \n\
                 API key before running this program.                          \n\
                 For example:                                                  \n\
                 $ export OPENAI_API_KEY=your_api_key_here                     \n\
                 $ cargo run <text_file_path>                                  \n\
                 ───────────────────────────────────────────────────────────────\n\
                 If you don't have an API key, sign up at:                     \n\
                 https://beta.openai.com/signup/                               \n\
                 ───────────────────────────────────────────────────────────────\n\
                 For more information, visit:                                  \n\
                 https://beta.openai.com/docs/                                 \n\
                 ═══════════════════════════════════════════════════════════════"
            );
        }
    }
}

fn query_chat_completion_api(api_key: String, chat_completion_request_body: ChatCompletionRequestBody) -> Result<ChatCompletionResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&chat_completion_request_body)
        .send()?;

    let chat_completion_response: ChatCompletionResponse = response.json()?;

    Ok(chat_completion_response)
}

