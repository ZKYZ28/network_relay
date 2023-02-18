use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, generic_array::{GenericArray}};
use base64::{encode, decode, Engine};
use base64::engine::general_purpose;

pub struct AesEncryptor {
    key: [u8; 32],
}

impl AesEncryptor {
    pub fn new(key_base64: &str) -> AesEncryptor {
        let key_decoded = general_purpose::STANDARD.decode(key_base64).unwrap();
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_decoded[..32]);
        AesEncryptor { key }
    }

    pub fn encrypt(&self, message: String) -> Vec<u8> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.key));
        let nonce = GenericArray::from_slice(b"unique nonce");
        cipher.encrypt(nonce, message.as_bytes())
            .expect("encryption failed")
    }


    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<String, String> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.key));
        let nonce = GenericArray::from_slice(b"unique nonce");
        match cipher.decrypt(nonce, ciphertext) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => Ok(s),
                Err(_) => Err("Decryption error: Invalid UTF-8 string".to_owned()),
            },
            Err(_) => Err("Decryption error: Incorrect key or message has been tampered with".to_owned()),
        }
    }

}

