// main.rs

mod models;
mod conversation;
mod response_types;
mod syntax_highlighter;
mod text_formatter;

use conversation::continue_conversation;
use dotenv::dotenv;
use models::{claude::Claude, gpt::GPT, AIModel};
use response_types::{command::Command, short::Short, ResponseModifier};
use std::env;
use std::io::{self};
use text_formatter::{color_text, format_error, print_formatted_response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get command line arguments, should have at least 2
    let args: Vec<String> = env::args().collect();
    
    // Check if the user has provided the required arguments, -cl or -gpt is required, -s or -c is optional, and the prompt is required
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <-cl|-gpt> [-s|-c] \"your question in quotes\"",
            args[0]
        );
        std::process::exit(1);
    }
    
    // Create a new instance of the model based on the user's choice, -cl for Claude, -gpt for ChatGPT
    let mut model: Box<dyn AIModel> = match args[1].as_str() {
        "-cl" => Box::new(Claude),
        "-gpt" => Box::new(GPT::new()),
        _ => {
            eprintln!("Invalid model specified. Use -cl for Claude or -gpt for GPT.");
            std::process::exit(1);
        }
    };

    // Check if the user has provided a response modifier, -s for short response, -c for command response
    let (response_modifier, prompt_index) = if args.len() >= 4 {
        match args[2].as_str() {
            "-s" => (Some(Box::new(Short) as Box<dyn ResponseModifier>), 3),
            "-c" => (Some(Box::new(Command) as Box<dyn ResponseModifier>), 3),
            _ => (None, 2),
        }
    // If the user has not provided a response modifier, then the default API response will be used
    } else {
        (None, 2)
    };

    let original_prompt = &args[prompt_index];

    // Modify the prompt if a response modifier is provided
    let modified_prompt = if let Some(modifier) = response_modifier {
        modifier.modify_prompt(original_prompt)
    } else {
        original_prompt.to_string()
    };

    let is_command_mode = args.len() >= 4 && args[2] == "-c";

    match model.generate_response(&modified_prompt).await {
        Ok(response) => {
            // Print the response to the console, formatted with headers, colors, and sectioning
            print_formatted_response(&response, is_command_mode);

            // Ask the user if they would like to continue the conversation
            let yes_no = color_text("(y/n)", "yellow");
            println!("\nWould you like to continue the conversation? {}", yes_no);

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                println!("\nAt anytime, enter quit to exit the conversation.");
                continue_conversation(model, is_command_mode).await?;
            } else if input.trim().to_lowercase() != "n" {
                eprintln!("Invalid input. Conversation ended.");
            }
        }
        Err(e) => eprintln!("{}", format_error(&e.to_string())),
    }
    Ok(())
}
