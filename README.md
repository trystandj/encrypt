# Overview
A encryption tool created with Rust. 

This tool allows you to generate your own AES key. Which you can then be used to decrypt a single or multiple documetns in a folder. Then you can decrypt a single or multiple files. This file utilizes the AES-GCM meathod which is one of the leading encryption standars. 

The purpose of writing this tool was to gain more familiarity with how encryption and decryption works. Additionally, to gain more knowledge on today's encryption standards and best practices. This tool was also created to help encyprt files to keep them secured so they cannot be accessed by anyone else. 


{Provide a link to your YouTube demonstration. It should be a 4-5 minute demo of the software running and a walkthrough of the code. Focus should be on sharing what you learned about the language syntax.}

[Software Demo Video](https://youtu.be/lOvX6GxyU2g)

# Development Environment

Rust Language
Visual Studio Code
Rust plugin extension 
Microsoft Visual Studio Code and Tools

I used the Rust language to create this tool. Then I used the AES-GCM library to provide AES-GCM authentication. I use this to create 256 bit keys and a 12 byte nonces. Rand is also used to generate random bytes for nonces. This helps keep the encryption secure and ensures randomness. STD::FS the standard library for file reading and writing. Then STD::Path for file path management. 

# Useful Websites

* [Let's Get Rusty](https://www.youtube.com/watch?v=ZhedgZtd8gw)
* [Rust](https://rust-lang.org/tools/install)
* [AESGCM](https://medium.com/@pravallikayakkala123/understanding-aes-encryption-and-aes-gcm-mode-an-in-depth-exploration-using-java-e03be85a3faa)
* [Encrypt with Rust](https://medium.com/coderhack-com/coderhack-cryptography-libraries-and-uses-in-rust-31957242299f)
* [Encrypt with Rust Dev.to](https://dev.to/vapourisation/file-encryption-in-rust-3kid)

# Future Work

- Create a simple UI system to allow a more friendly user experience. 
- Include a more secure meathod of storing the key for the user rather then keeping it locally. 
