mod models;
mod response_types;
mod text_formatter;

use dotenv::dotenv;
use models::{claude::Claude, gpt::GPT, AIModel};
use response_types::{command::Command, short::Short, ResponseModifier};
use text_formatter::{print_formatted_response, format_error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <-cl|-gpt> [-s|-c] \"your question in quotes\"",
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

    let (response_modifier, prompt_index) = if args.len() >= 4 {
        match args[2].as_str() {
            "-s" => (Some(Box::new(Short) as Box<dyn ResponseModifier>), 3),
            "-c" => (Some(Box::new(Command) as Box<dyn ResponseModifier>), 3),
            _ => (None, 2),
        }
    } else {
        (None, 2)
    };

    let original_prompt = &args[prompt_index];
    let modified_prompt = if let Some(modifier) = response_modifier {
        modifier.modify_prompt(original_prompt)
    } else {
        original_prompt.to_string()
    };

    let is_command_mode = args.len() >= 4 && args[2] == "-c";
    
    match model.generate_response(&modified_prompt).await {
        Ok(response) => {
            print_formatted_response(&response, is_command_mode);
        },
        Err(e) => eprintln!("{}", format_error(&e.to_string())),
    }

    Ok(())
}