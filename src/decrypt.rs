use aes_gcm::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};
use std::fs;

pub fn run(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load key from file
    let key_bytes = fs::read("filekey.key")?;
    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes for AES-256-GCM".into());
    }
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    //  Read encrypted file
    let data = fs::read(file_path)?;
    if data.len() < 12 {
        return Err("Invalid encrypted file: too short".into());
    }

    // Split IV and ciphertext
    let (iv, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(iv);

    // Decrypt data
    let decrypted = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed!")?;

    
    // Prepare output path
    let output_path = format!("{}", file_path);
    

    // Write decrypted file (overwrites original or restores it)
    fs::write(&output_path, &decrypted)?;

    println!("File decrypted successfully: {}", output_path);

    // Return success
    Ok(())
}
