// src/conversation.rs

use std::io::{self, Write};
use crate::models::AIModel;
use crate::text_formatter::{print_formatted_response, format_error, color_text};

pub async fn continue_conversation(
    mut model: Box<dyn AIModel>,
    is_command_mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let you = color_text("You: ", "yellow");
        print!("\n{}", you);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() == "quit" {
            println!("Conversation ended.");
            break;
        }
        match model.generate_response(&input).await {
            Ok(response) => {
                print_formatted_response(&response, is_command_mode);
            }
            Err(e) => eprintln!("{}", format_error(&e.to_string())),
        }
    }
    Ok(())
}