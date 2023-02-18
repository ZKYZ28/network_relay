mod aes_encryptor;

use std::str;
use base64::{decode, encode};
use crate::aes_encryptor::AesEncryptor;

fn main() {
    // Clé secrète pour l'AES
    let key_base64 = "z01JW7/j8Acb5PYfrl+P15O/axfLZ1DvJpE+lyxjNtQ=";

    // Créer un encrypteur AES
    let aes_encryptor = AesEncryptor::new(key_base64);

    // Message à encrypter
    let message = "Hello, world!".to_string();

    // Encrypter le message
    let ciphertext = aes_encryptor.encrypt(message);

    // Afficher le message encrypté en base64
    let ciphertext_to_string = String::from_utf8_lossy(&ciphertext);
    println!("Ciphertext : {:?}", ciphertext_to_string);

    // Decrypter le message
    let decrypted_message = aes_encryptor.decrypt(&ciphertext);
    match decrypted_message {
        Ok(msg) => println!("Decrypted message: {}", msg),
        Err(e) => println!("Error: {}", e),
    }
}
