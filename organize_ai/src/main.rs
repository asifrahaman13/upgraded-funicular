use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, stdin};
use std::process::{Command, Output};

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    stream: bool,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: ChatMessageContent,
}

#[derive(Deserialize)]
struct ChatMessageContent {
    content: String,
}

fn run_command(command: &str) -> io::Result<Output> {
    Command::new("sh").arg("-c").arg(command).output()
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let mut input_lines = String::new();
    println!("Please enter the directory or file path (end with an empty line):");

    // Read single-line input (directory or file path)
    stdin().read_line(&mut input_lines)?;

    // Trim newline characters from input_lines
    let input_lines = input_lines.trim();

    // Construct the command with the directory or file path
    let initial_command = format!("tree {}", input_lines);

    let initial_structur = match run_command(&initial_command) {
        Ok(output) => {
            println!("Command output:\n{}", String::from_utf8_lossy(&output.stdout));
            output.stdout
        }
        Err(e) => {
            eprintln!("Error executing command: {}", e);
            return Err(e.into());
        }
    };

    let overall_prompt = format!(
        "Hello, your job is to give me a set of bash commands. I have a folder and file structure. The file structure is as follows:\n{}\nYour job is to give only the bash commands to organize those files. Remember only give the bash commands and no other texts",
        String::from_utf8_lossy(&initial_structur)
    );

    // Prepare the chat message
    let chat_request = ChatRequest {
        model: "llama3.1:8b".to_string(),
        stream: false,
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: overall_prompt,
        }],
    };

    // Send the POST request
    let response = client.post("http://localhost:11434/api/chat").json(&chat_request).send()?;

    // Deserialize and print the response
    let chat_response: ChatResponse = response.json()?;
    println!("Response: {}", chat_response.message.content);

   let command = &chat_response.message.content;

    let cleaned_command = command.trim().trim_matches('`').to_string();

    // Concatenate `input_lines` with the command obtained from the response
    let final_command = format!("{} {}", input_lines.trim(), cleaned_command);

    println!("Executing command: {}", final_command);

    match run_command(&final_command) {
        Ok(output) => {
            if output.status.success() {
                println!("Command executed successfully:");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("Command failed with status: {}", output.status);
                eprintln!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }

    Ok(())
}
