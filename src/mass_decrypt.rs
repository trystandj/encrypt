use aes_gcm::{
    aead::{Aead, KeyInit, generic_array::GenericArray},
    Aes256Gcm, Nonce,
};
use std::fs;

pub fn run(folder_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load AES key
    let key_bytes = fs::read("filekey.key")?;
    if key_bytes.len() != 32 {
        return Err("Key must be 32 bytes for AES-256-GCM".into());
    }

    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Loop through all files in the folder
    for file in fs::read_dir(folder_path)? {
        let file = file?;
        let path = file.path();

        if path.is_file() {

            // Read encrypted data
            let data = fs::read(&path)?;
            if data.len() < 12 {
                println!("Skipping (invalid size): {}", path.display());
                continue;
            }

            // Split IV and ciphertext
            let (iv, ciphertext) = data.split_at(12);
            let nonce = Nonce::from_slice(iv);

            // Decrypt the ciphertext
            let decrypted = match cipher.decrypt(nonce, ciphertext) {
                Ok(plain) => plain,
                Err(_) => {
                    println!("Decryption failed for: {}", path.display());
                    continue;
                }
            };

            // Overwrite the original file with decrypted data
            fs::write(&path, &decrypted)?;

            println!("Decrypted successfully: {}", path.display());
        } else if path.is_dir() {
            println!("Skipping directory: {:?}", path.display());
        }
    }

    Ok(())
}
