use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};

use rand::RngCore;
use std::fs;
use std::io;

// Function to choose a file by entering its path
pub fn run() -> Result<String, Box<dyn std::error::Error>> {
    println!("Enter the path of the file to process:");
    // Read user input
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    // Trim whitespace and quotes
    let choice = choice.trim().trim_matches('"').to_string();
    // Check if the file exists
    if !std::path::Path::new(&choice).exists() {
        return Err(format!("File not found: {}", choice).into());
    }
    // Return the file path
    Ok(choice)
}
