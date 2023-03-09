use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream};
use std::sync::{Arc, Mutex};
use crate::aes_encryptor::AesEncryptor;
use crate::protocol::Protocol;
use base64::engine::general_purpose;
use base64;
use base64::Engine;

pub struct ServerRunnable {
    servers_map: Arc<Mutex<HashMap<String, TcpStream>>>,
    aes_key: String,
}

impl ServerRunnable {
    pub(crate) fn new(servers_map: Arc<Mutex<HashMap<String, TcpStream>>>, aes_key: String) -> ServerRunnable {
        ServerRunnable {
            servers_map,
            aes_key,
        }
    }


    pub(crate) fn handle_client(&self, stream: &TcpStream) {
        let mut reader = BufReader::new(stream);
        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    match general_purpose::STANDARD.decode(&buffer.trim_end()) {
                        Ok(bytes) => {
                            let decrypted_message = AesEncryptor::decrypt(&self.aes_key, &bytes);
                            println!("Décrypté : {:?}", decrypted_message);

                            Self::analyse_message(&self, decrypted_message.unwrap())
                        }
                        Err(e) => {
                            println!("Erreur de décodage Base64 : {:?}", e);
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("Erreur de lecture");
                    break;
                }
            }
        }
    }


    fn analyse_message(&self, msg: String) {
        let dest_domain = Protocol::get_receiving_domain(&msg).unwrap();

        if let Some(lock_guard) = self.servers_map.try_lock().ok() {
            if lock_guard.contains_key(&dest_domain) {
                drop(lock_guard); // Release the lock
                self.send_message(&dest_domain, msg);
            } else {
                println!("Message perdu car le serveur {} n'était pas en ligne ou n'existe pas.", dest_domain);
            }
        } else {
            println!("Failed to acquire lock on servers_map.");
        }
    }

    /**
     * Méthode qui sert à envoyer un message à un des serveurs connecté.
     */
    fn send_message(&self, domain: &str, msg: String) {
        let encrypted_msg = AesEncryptor::encrypt(&self.aes_key, msg) + "\n";                          //Encryption du message
        let mut tcp_socket = self.servers_map.lock().unwrap().get(domain).unwrap().try_clone().unwrap();     //Récupération du socket du serveur destinataire
        tcp_socket.write_all(&encrypted_msg.as_bytes()).unwrap();                                                               //Envoi
    }
}
