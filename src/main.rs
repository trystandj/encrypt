use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use std::fs;
use std::io;

mod chooseFile;
mod encrypt;
mod decrypt;
mod key;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple command-line interface
    loop {
        println!("\nChoose an option:");
        println!("1. Encrypt a file");
        println!("2. Decrypt a file");
        println!("3. Generate Key");
        println!("4. Exit");

        // Read user choice
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: &str = choice.trim();
        
        // Handle user choice
        match choice {
            "1" => {
                // Choose file to encrypt
                let file_path = chooseFile::run()?;
                println!("Selected file: {}", file_path);
                println!("Encrypting...");
                encrypt::run(&file_path)?; 
            }
            "2" => {
                // Choose file to decrypt
                let file_path = chooseFile::run()?;
                println!("Selected file: {}", file_path);
                println!("Decrypting...");
                decrypt::run(&file_path)?;
            }
            "3" => {
                // Generate encryption key
                println!("Generating key...");
                key::run()?;
            }
            "4" => {
                // Exit the program
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
    // Return success and user options completed
    Ok(())
}
