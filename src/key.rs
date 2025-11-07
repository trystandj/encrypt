use rand::RngCore;
use std::fs::File;
use std::io::Write;

pub fn run() -> std::io::Result<()> {

    // Generate a random 32-byte key for AES-256-GCM
    let mut key: [u8; 32] = [0u8; 32]; 
    rand::thread_rng().fill_bytes(&mut key);

    // Save the key to a key file
    let mut file: File = File::create("filekey.key")?;
    file.write_all(&key)?;
    println!("Generated key and saved to filekey.key");
    Ok(())
}