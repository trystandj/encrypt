use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use std::fs;
use std::io;

mod choose_file;
mod encrypt;
mod decrypt;
mod key;
mod mass_encrypt;
mod mass_decrypt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple command-line interface
    loop {
        println!("\nChoose an option:");
        println!("1. Encrypt a file");
        println!("2. Encrypt all files in a folder");
        println!("3. Decrypt a file");
        println!("4. Decrypt all files in a folder");
        println!("5. Generate Key");
        println!("6. Exit");

        // Read user choice
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: &str = choice.trim();
        
        // Handle user choice
        match choice {
            "1" => {
                // Choose file to encrypt
                let file_path = choose_file::run()?;
                println!("Selected file: {}", file_path);
                println!("Encrypting...");
                encrypt::run(&file_path)?; 
            }
            "2" => {
                // Choose folder to encrypt
                println!("Enter the path of the folder to encrypt:");
                let mut folder_path = String::new();
                io::stdin().read_line(&mut folder_path)?;
                let folder_path = folder_path.trim().trim_matches('"').to_string();
                mass_encrypt::run(&folder_path)?;
            }
            "3" => {
                // Choose file to decrypt
                let file_path = choose_file::run()?;
                println!("Selected file: {}", file_path);
                println!("Decrypting...");
                decrypt::run(&file_path)?;
            }
            "4" => {
                // Choose folder to decrypt
                println!("Enter the path of the folder to decrypt:");
                let mut folder_path = String::new();
                io::stdin().read_line(&mut folder_path)?;
                let folder_path = folder_path.trim().trim_matches('"').to_string();
                mass_decrypt::run(&folder_path)?;
            }
            "5" => {
                // Generate encryption key
                println!("Generating key...");
                key::run()?;
            }
            "6" => {
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
