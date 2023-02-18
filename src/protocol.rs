use regex::Regex;
use super::*;
use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, generic_array::{ArrayLength, GenericArray}};
use base64::{encode, decode, Engine};
use crate::aes_encryptor::AesEncryptor;


const RX_LETTER: &str = "[a-zA-Z]";
const RX_DIGIT: &str = "[0-9]";
const TAG: &str = "#[a-zA-Z0-9]{5,20}";
const RX_LETTER_DIGIT: &'static str = "[a-zA-Z0-9]";
const RX_USERNAME: &'static str = "(([a-zA-Z0-9]){5,20})";
const RX_DOMAIN: &'static str = "(([a-zA-Z0-9]|\\.){5,200})";
const RX_USER_DOMAIN: &'static str = "({{login:{},domain:{}}})";
const RX_SEND: &'static str = "SEND (?P<id_domaine>[0-9]{1,5}) {{login:([a-zA-Z0-9]){5,20},domain:([a-zA-Z0-9]|\\.){5,200}}} (?P<tag_domaine>[a-zA-Z0-9]+) (?P<data>.{1,500})\r\n";
const RX_ECHO: &'static str = "ECHO (?P<port>[0-9]{1,5}) ([a-zA-Z0-9]|\\.){5,200}\r\n";



pub struct Protocol {
    send_regex: Regex,
    echo_regex: Regex,
    pub(crate) aes_encryptor: AesEncryptor,
}

impl Protocol {
    pub fn new(aes_key_base64: &str) -> Protocol {
        Protocol {
            send_regex: Regex::new(RX_SEND).unwrap(),
            echo_regex: Regex::new(RX_ECHO).unwrap(),
            aes_encryptor: AesEncryptor::new(aes_key_base64),
        }
    }

    pub fn decrypt_and_process_message(&self, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
        let plaintext = match self.aes_encryptor.decrypt(ciphertext) {
            Ok(p) => p,
            Err(e) => return Err(format!("Decryption failed: {}", e)),
        };

        let plaintext_str = match String::from_utf8(Vec::from(plaintext)) {
            Ok(s) => s,
            Err(_) => return Err("Invalid UTF-8 string".to_owned()),
        };

        if let Some(captures) = self.send_regex.captures(&plaintext_str) {
            let id_domaine = captures.name("id_domaine").unwrap().as_str();
            let login = captures.name("login").unwrap().as_str();
            let domain = captures.name("domain").unwrap().as_str();
            let tag_domaine = captures.name("tag_domaine").unwrap().as_str();
            let data = captures.name("data").unwrap().as_str();

            // Process SEND message
            let response = format!("Processed SEND message: {}\r\n", data);
            let encrypted_response = self.aes_encryptor.encrypt(response);
            Ok(encrypted_response)
        } else if let Some(captures) = self.echo_regex.captures(&plaintext_str) {
            let port = captures.name("port").unwrap().as_str();
            let domain = captures.name("domain").unwrap().as_str();

            // Process ECHO message
            let response = format!("Processed ECHO message on port {}\r\n", port);
            let encrypted_response = self.aes_encryptor.encrypt(response);
            Ok(encrypted_response)
        } else {
            Err("Unrecognized message format".to_owned())
        }
    }

}

