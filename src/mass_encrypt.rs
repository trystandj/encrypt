use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use std::fs;
use std::path::Path;

pub fn run(folder_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load the file key
    let key_bytes = fs::read("filekey.key")?;
    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes for AES-256-GCM".into());
    }

    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Go through each file in the folder
    for file in fs::read_dir(folder_path)? {
        let file = file?;
        let path = file.path();

        if path.is_file() {

            // Read file contents
            let data = fs::read(&path)?;

            // Generate a random 12-byte IV (nonce)
            let mut iv = [0u8; 12];
            OsRng.fill_bytes(&mut iv);
            let nonce = Nonce::from_slice(&iv);

            // Encrypt the file contents
            let encrypted_data = cipher
                .encrypt(nonce, data.as_ref())
                .expect("encryption failure!");

            // Combine IV + ciphertext
            let mut output = Vec::new();
            output.extend_from_slice(&iv);
            output.extend_from_slice(&encrypted_data);

            // Overwrite the original file
            fs::write(&path, &output)?;

            println!("Encrypted successfully: {}", path.display());
        } else if path.is_dir() {
            println!("Skipping directory: {:?}", path.display());
        }
    }

    Ok(())
}
