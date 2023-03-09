use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, generic_array::{GenericArray}};
use aes_gcm::aead::rand_core::RngCore;
use base64;
use base64::{Engine};
use base64::engine::general_purpose;


pub struct AesEncryptor;

impl AesEncryptor {
    pub fn encrypt(key_base64: &str, message: String) -> String {
        let key_decoded = general_purpose::STANDARD.decode(key_base64).unwrap();
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_decoded[..32]);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
        let mut iv = [0u8; 12];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut iv);
        let iv = GenericArray::from_slice(&iv);
        let ciphertext = cipher.encrypt(iv, message.as_bytes())
            .expect("encryption failed");
        let mut result = Vec::new();
        result.extend_from_slice(&iv);
        result.extend_from_slice(&ciphertext);
        general_purpose::STANDARD.encode(&result)
    }

/*
    pub fn decrypt(key_base64: &str, ciphertext: &[u8]) -> Result<String, String> {

        // Decode the Base64-encoded key into a byte array.
        let key_decoded = general_purpose::STANDARD.decode(key_base64).unwrap();

        // Define a mutable array of 32 bytes and set all its elements to 0.
        let mut key = [0u8; 32];

        // Copy the first 32 bytes of the decoded key into the mutable array.
        key.copy_from_slice(&key_decoded[..32]);

        // Create a new AES-256-GCM cipher instance with the key.
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

        // Extract the first 12 bytes of the ciphertext and convert them into a generic array.
        // Extract the first 12 bytes of the concatenated ciphertext array as the IV and the remaining bytes as the ciphertext.
        let iv = GenericArray::from_slice(&ciphertext[..12]);
        println!("iv : {:?}", iv);
        let ciphertext = &ciphertext[0..];

        println!("ciphertext : {:?}", ciphertext);

        // Decrypt the ciphertext using the AES-256-GCM cipher and the IV.
        match cipher.decrypt(iv, ciphertext) { // dereference ciphertext with *
            // If decryption is successful, convert the decrypted bytes into a UTF-8 string.
            Ok(bytes) => match String::from_utf8(bytes) {
                // If the string is valid UTF-8, return it.
                Ok(s) => Ok(format!("Success: {}", s)),
                // If the string is not valid UTF-8, return an error message.
                Err(_) => Err("Decryption error: Invalid UTF-8 string".to_owned()),
            },
            // If decryption fails, return an error message.
            Err(_) => Err("Decryption error: Incorrect key or message has been tampered with".to_owned()),
        }
    }

     */

    pub fn decrypt(key_base64: &str, ciphertext: &Vec<u8>) -> Result<String, String> {


        let key_decoded = general_purpose::STANDARD.decode(key_base64).unwrap();
      //  println!("key_decoded : {:?}", key_decoded);

        let mut key = [0u8; 32];
        key.copy_from_slice(&key_decoded[..32]);
       // println!("key.copy_from_slice : {:?}", key);

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

        let iv = GenericArray::from_slice(&ciphertext[..12]);
        //println!("iv : {:?}", iv);

        let ciphertext = &ciphertext[12..];
       // println!("ciphertext : {:?}", ciphertext);

        match cipher.decrypt(iv, &*ciphertext) { // dereference ciphertext with *
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => Ok(s),
                Err(_) => Err("Decryption error: Invalid UTF-8 string".to_owned()),
            },
            Err(_) => Err("Decryption error: Incorrect key or message has been tampered with".to_owned()),
        }
    }


}
