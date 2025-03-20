// Telly - A simple Telegram Bot API client
// Copyright (c) 2025 Andrea Raponi <andrearaponi@outlook.com>
// Licensed under the MIT License

// External crate imports
use clap::Parser;                   // Command line argument parsing
use configparser::ini::Ini;         // INI file configuration parsing
use std::error::Error;              // Error trait for error handling
use urlencoding::encode;            // URL encoding for message text
use curl::easy::{Easy, Form};       // HTTP requests and multipart form handling
use std::io::{stderr, Write};       // Standard error output
use std::path::PathBuf;             // Cross-platform file path handling

/// Command-line interface for Telly
/// Defines the arguments that can be passed to the application
#[derive(Parser, Debug)]
#[command(author, version, about = "A simple Telegram bot message sender", long_about = None)]
struct Cli {
    /// Path to the configuration file (.ini)
    #[arg(value_parser = clap::value_parser!(std::path::PathBuf))]
    config: PathBuf,
    
    /// Message to send to the recipient
    message: String,
    
    /// Optional file to attach to the message
    #[arg(short, long)]
    file: Option<PathBuf>
}

/// Main entry point for the Telly application
/// Handles command line parsing, configuration loading, and message sending
fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Cli::parse();
    
    // Load configuration from the specified .ini file
    let mut config = Ini::new();
    let _ = config.load(&args.config)?;
    
    // Extract required configuration values
    let basic: String = config.get("DEFAULT", "basic")
        .ok_or("Missing 'basic' configuration value")?;
    let api_key = config.get("DEFAULT", "api_key")
        .ok_or("Missing 'api_key' configuration value")?;
    let recipient = config.get("DEFAULT", "recipient")
        .ok_or("Missing 'recipient' configuration value")?;
    
    // Initialize curl for HTTP requests and response data buffer
    let mut easy = Easy::new();
    let mut response_data = Vec::new();
    
    // Determine if we're sending a file or just a text message
    match &args.file {
        Some(file_path) => {
            // If a file is provided, use sendDocument API
            
            // Build the URL for sending a document
            let url = format!(
                "{}{}/sendDocument?chat_id={}",
                basic,
                api_key,
                recipient
            );
            
            // Create a multipart form for the file upload
            let mut form = Form::new();
            
            // Add the file to the form
            form.part("document")
                .file(file_path)
                .add()
                .map_err(|e| format!("Error attaching file: {}", e))?;
            
            // Add the caption (message) to the form if not empty
            if !args.message.is_empty() {
                form.part("caption")
                    .contents(args.message.as_bytes())
                    .add()
                    .map_err(|e| format!("Error adding caption: {}", e))?;
            }
            
            // Setup curl with the form and URL
            easy.url(&url)?;
            easy.httppost(form)?;
            
            // Send the request and capture response
            {
                let mut transfer = easy.transfer();
                transfer.write_function(|data| {
                    response_data.extend_from_slice(data);
                    Ok(data.len())
                })?;
                transfer.perform()?;
            }
            
            // Verify the response was successful
            let response_code = easy.response_code()?;
            if response_code != 200 {
                writeln!(
                    stderr(),
                    "Error: Telegram API returned status code {} with message: {}",
                    response_code,
                    String::from_utf8_lossy(&response_data)
                )?;
                return Err("Failed to send document".into());
            }
            
            println!("Message with file sent successfully!");
        },
        None => {
            // If no file is provided, use sendMessage API
            
            // Build the URL for sending a text message
            let url = format!(
                "{}{}/sendMessage?chat_id={}&text={}",
                basic,
                api_key,
                recipient,
                encode(&args.message)
            );
            
            // Setup curl and send the request
            easy.url(&url)?;
            
            // Capture the response
            {
                let mut transfer = easy.transfer();
                transfer.write_function(|data| {
                    response_data.extend_from_slice(data);
                    Ok(data.len())
                })?;
                transfer.perform()?;
            }
            
            // Verify the response was successful
            let response_code = easy.response_code()?;
            if response_code != 200 {
                writeln!(
                    stderr(),
                    "Error: Telegram API returned status code {} with message: {}",
                    response_code,
                    String::from_utf8_lossy(&response_data)
                )?;
                return Err("Failed to send message".into());
            }
            
            println!("Message sent successfully!");
        }
    }
    
    Ok(())
}

/// Test module for the Telly application
/// Contains unit tests for core functionality
#[cfg(test)]
mod tests {
    use configparser::ini::Ini;
    use urlencoding::encode;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    /// Test proper loading and parsing of the configuration file
    #[test]
    fn test_load_config() {
        // Create a temporary configuration file
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("test_config.ini");
        
        // Write test configuration data
        {
            let mut file = File::create(&config_path).unwrap();
            file.write_all(b"[DEFAULT]\napi_key = test_key\nbasic = https://api.test.org/bot\nrecipient = 123456").unwrap();
        }
        
        // Load the configuration
        let mut config = Ini::new();
        let result = config.load(&config_path);
        assert!(result.is_ok());
        
        // Verify the configuration values were loaded correctly
        assert_eq!(config.get("DEFAULT", "api_key"), Some("test_key".to_string()));
        assert_eq!(config.get("DEFAULT", "basic"), Some("https://api.test.org/bot".to_string()));
        assert_eq!(config.get("DEFAULT", "recipient"), Some("123456".to_string()));
    }
    
    /// Test proper URL encoding of message text
    #[test]
    fn test_url_encoding() {
        // Test basic encoding with special characters
        let original = "Hello World! Special chars: <>&?";
        let encoded = encode(original);
        assert_eq!(encoded, "Hello%20World%21%20Special%20chars%3A%20%3C%3E%26%3F");
        
        // Test encoding with emojis and special characters
        let message_with_specials = "Test message with emoji 😊 and special chars: <>&?";
        let encoded_message = encode(message_with_specials);
        assert!(!encoded_message.contains("?"));
        assert!(!encoded_message.contains("<"));
        assert!(!encoded_message.contains(">"));
    }
}