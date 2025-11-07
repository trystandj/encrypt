use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use std::fs;

pub fn run(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load key from the key file
    let key_bytes = fs::read("filekey.key")?;
    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes for AES-256-GCM".into());
    }

    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate random 12-byte IV (nonce) for AES-GCM
    let mut iv = [0u8; 12];
    OsRng.fill_bytes(&mut iv);
    let nonce = Nonce::from_slice(&iv);

    // Read file contents
    let data = fs::read(file_path)?;

    // Encrypt data within the file
    let encrypted_data = cipher.encrypt(nonce, data.as_ref())
        .expect("encryption failure!");

    // Combine IV + ciphertext for storage 
    let mut output = Vec::new();
    output.extend_from_slice(&iv);
    output.extend_from_slice(&encrypted_data);

    //  Write to a new encrypted file with the encrypted data
    let encrypted_path = format!("{}", file_path);
    fs::write(&encrypted_path, &output)?;

    println!("File encrypted successfully: {}", encrypted_path);

    // Return success
    Ok(())
}
