mod models;
mod response_types;

use dotenv::dotenv;
use models::{claude::Claude, gpt::GPT, AIModel};
use response_types::{command::Command, short::Short, ResponseModifier};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <-cl|-gpt> <-s|-c> \"your question in quotes\"",
            args[0]
        );
        std::process::exit(1);
    }

    let model: Box<dyn AIModel> = match args[1].as_str() {
        "-cl" => Box::new(Claude),
        "-gpt" => Box::new(GPT),
        _ => {
            eprintln!("Invalid model specified. Use -cl for Claude or -gpt for GPT.");
            std::process::exit(1);
        }
    };

    let response_modifier: Box<dyn ResponseModifier> = match args[2].as_str() {
        "-s" => Box::new(Short),
        "-c" => Box::new(Command),
        _ => {
            eprintln!("Invalid response type specified. Use -s for short or -c for command.");
            std::process::exit(1);
        }
    };

    let original_prompt = &args[3];
    let modified_prompt = response_modifier.modify_prompt(original_prompt);

    match model.generate_response(&modified_prompt).await {
        Ok(response) => println!("AI response:\n{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
