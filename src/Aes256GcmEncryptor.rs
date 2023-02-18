use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};

pub struct Aes256GcmEncryptor {
    key: [u8; 32],
}

impl Aes256GcmEncryptor {
    pub fn new(key: [u8; 32]) -> Aes256GcmEncryptor {
        Aes256GcmEncryptor {
            key,
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let key = GenericArray::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);
        let nonce = rand::random::<[u8; 12]>();
        let nonce_str = base64::encode_config(&nonce, base64::URL_SAFE_NO_PAD);

        let ciphertext = cipher.encrypt(&nonce.into(), plaintext.as_bytes())
            .expect("encryption failure");
        let ciphertext_str = base64::encode_config(&ciphertext, base64::URL_SAFE_NO_PAD);

        format!("{}:{}", nonce_str, ciphertext_str)
    }

    pub fn decrypt(&self, ciphertext_str: &str) -> String {
        let key = GenericArray::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);

        let parts: Vec<&str> = ciphertext_str.split(':').collect();
        let nonce = base64::decode_config(parts[0], base64::URL_SAFE_NO_PAD)
            .expect("nonce decoding failure");
        let ciphertext = base64::decode_config(parts[1], base64::URL_SAFE_NO_PAD)
            .expect("ciphertext decoding failure");

        let plaintext = cipher.decrypt(&nonce.into(), &ciphertext)
            .expect("decryption failure");
        String::from_utf8_lossy(&plaintext).to_string()
    }
}