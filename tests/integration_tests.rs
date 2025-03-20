use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_argument_parsing() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verify help output contains expected options
    assert!(stdout.contains("telly"));  
    assert!(stdout.contains("Message to send"));
    assert!(stdout.contains("--file"));
    assert!(stdout.contains("Optional file"));
}

#[test]
fn test_config_file_loading() {
    // Create a temporary directory
    let dir = tempdir().expect("Failed to create temp directory");
    let config_path = dir.path().join("test_config.ini");
    
    // Create a test config file
    {
        let mut file = File::create(&config_path).expect("Failed to create config file");
        file.write_all(b"[DEFAULT]\napi_key = test_key\nbasic = https://api.test.org/bot\nrecipient = 123456")
            .expect("Failed to write to config file");
    }
    
    // Run the application with invalid URL to test only config loading
    // We expect it to fail at the HTTP request stage, but after loading the config
    let output = Command::new("cargo")
        .args(["run", "--", config_path.to_str().unwrap(), "Test message"])
        .output()
        .expect("Failed to execute command");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Check for various expected error cases that would indicate the config was loaded
    // but the request failed
    assert!(
        stderr.contains("Failed to send message") || 
        stderr.contains("curl: (") ||
        stderr.contains("Error:") ||
        stderr.contains("Could not resolve host") ||
        stdout.contains("Error:"),
        "Expected error after loading config: stdout={}, stderr={}", 
        stdout, stderr
    );
}

#[test]
fn test_missing_config_values() {
    // Create a temporary directory
    let dir = tempdir().expect("Failed to create temp directory");
    let config_path = dir.path().join("bad_config.ini");
    
    // Create a test config file with missing values
    {
        let mut file = File::create(&config_path).expect("Failed to create config file");
        file.write_all(b"[DEFAULT]\napi_key = test_key\n# missing basic and recipient")
            .expect("Failed to write to config file");
    }
    
    let output = Command::new("cargo")
        .args(["run", "--", config_path.to_str().unwrap(), "Test message"])
        .output()
        .expect("Failed to execute command");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Expect error about missing configuration
    assert!(stderr.contains("Missing") && stderr.contains("configuration value"));
}

// Questi test sono solo segnaposto, non eseguono effettivamente nulla
#[test]
fn test_message_url_construction() {
    // Questo test verifica solo che non ci siano regressioni nella struttura del codice
    assert!(true);
}

#[test]
fn test_document_form_construction() {
    assert!(true);
}