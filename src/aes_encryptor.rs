use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, generic_array::{GenericArray}};
use aes_gcm::aead::rand_core::RngCore;
use base64::{decode, encode, Engine};
use base64::engine::general_purpose;

pub struct AesEncryptor;

impl AesEncryptor {
    pub fn encrypt(key_base64: &str, message: String) -> Vec<u8> {
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
        result
    }

    pub fn decrypt(key_base64: &str, ciphertext: &[u8]) -> Result<String, String> {
        println!("CLE RECUE : {}", key_base64);
        let key_decoded = general_purpose::STANDARD.decode(key_base64).unwrap();
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_decoded[..32]);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
        let iv = GenericArray::from_slice(&ciphertext[..12]);
        let ciphertext = &ciphertext[12..];
        match cipher.decrypt(iv, &*ciphertext) { // dereference ciphertext with *
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => Ok(s),
                Err(_) => Err("Decryption error: Invalid UTF-8 string".to_owned()),
            },
            Err(_) => Err("Decryption error: Incorrect key or message has been tampered with".to_owned()),
        }
    }


}
