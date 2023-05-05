extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use tiktoken_rs::cl100k_base;
use colored::*;

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
    if args.len() < 2 {
        println!("{}", "Usage: cargo run <text_file_path> [--raw]".color("purple"));
        return;
    }
    let engine = if args.len() > 2 {
        match args[2].as_str() {
            "g4" => "gpt-4",
            "g4-32" => "gpt-4-32k",
            "g3" => "gpt-3.5-turbo",
            _ => {
                println!("{}", "No engine specified. Defaulting to GPT-4.".color("yellow"));
                "gpt-4"
            }
        }
    } else {
        "gpt-4"
    };

    let text_file_path = &args[1];
    let text = fs::read_to_string(text_file_path).expect("Failed to read the text");
    let bpe = cl100k_base().unwrap();
    let token_count = bpe.encode_with_special_tokens(&text).len();
    let parent_prompt = "Summarise the following in 300 tokens or less. Give your best attempt.";
    let parent_prompt_token_count = bpe.encode_with_special_tokens(&parent_prompt).len();
    let total_tokens = token_count + parent_prompt_token_count;
    println!("Number of tokens in the text file: {}", token_count);
    println!("Number of tokens in parent prompt: {}", parent_prompt_token_count);
    println!("Total number of tokens: {}", total_tokens);
    let max_tokens = match engine {
        "gpt-4" => 8192,
        "gpt-4-32k" => 32768,
        "gpt-3.5-turbo" => 4096,
        _ => {
            println!("Invalid engine specified. Exiting.");
            return;
        }
    };

    if total_tokens > max_tokens {
        println!(
            "The total number of tokens ({}) exceeds the maximum allowed tokens for the model ({}). Please reduce the input size.",
            total_tokens, max_tokens
        );
        return;
    }




    match env::var("OPENAI_API_KEY") {
        Ok(api_key) => {
            let system_message = Message {
                role: "system".to_string(),
                content: parent_prompt.to_string(),
            };

            let user_message = Message {
                role: "user".to_string(),
                content: text,
            };

            let chat_completion_request_body = ChatCompletionRequestBody {
                model: engine.to_string(),
                messages: vec![system_message, user_message],
            };
            let raw_response_flag = args.contains(&"--raw".to_string());
            let response = query_chat_completion_api(api_key, chat_completion_request_body, raw_response_flag);

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
            println!("{}",
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
                 ═══════════════════════════════════════════════════════════════".color("red")
            );
        }
    }
}

fn query_chat_completion_api(api_key: String, chat_completion_request_body: ChatCompletionRequestBody, raw_response_flag: bool) -> Result<ChatCompletionResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(300)).build()?;

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&chat_completion_request_body)
        .send()?;

    let raw_response = response.text()?;
    if raw_response_flag{
        println!("Raw API response: {}", raw_response);
    }
    let chat_completion_response: ChatCompletionResponse = serde_json::from_str(&raw_response)?; 

    Ok(chat_completion_response)
}

