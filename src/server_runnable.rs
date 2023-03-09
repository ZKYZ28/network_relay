use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::aes_encryptor::AesEncryptor;
use crate::protocol::Protocol;
use base64;

pub struct ServerRunnable {
    handle: Option<thread::JoinHandle<()>>,
    servers_map: Arc<Mutex<HashMap<String, TcpStream>>>,
    domain: String,
    aes_key: String,
}

impl ServerRunnable {
    pub(crate) fn new(
        servers_map: Arc<Mutex<HashMap<String, TcpStream>>>,
        domain: String,
        aes_key: String,
    ) -> ServerRunnable {
        ServerRunnable {
            handle: None,
            servers_map,
            domain,
            aes_key,
        }
    }

    pub(crate) fn handle_client(&self) {
        let arc = self.servers_map.clone();
        let stream = {
            let binding = arc.try_lock().unwrap();
            binding.get(&self.domain).map(|tcp_stream| tcp_stream.try_clone().unwrap())
        };

        if let Some(stream) = stream {
            let mut reader = BufReader::new(stream);
            loop {
                let mut buffer = String::new();

                match reader.read_line(&mut buffer) {
                    Ok(0) => break,
                    Ok(_) => {
                        let decrypted_message = AesEncryptor::decrypt(&self.aes_key, &base64::decode(&buffer.trim_end()).unwrap()).unwrap();
                        println!("Décrypté : {:?}", decrypted_message);

                        self.analyse_message(decrypted_message)
                    }
                    Err(_) => {
                        println!("Erreur de lecture");
                        break;
                    }
                }
            }
        } else {
            println!("Socket introuvable pour le nom de domaine {}", self.domain);
        }
    }



    fn analyse_message(&self, msg: String) {
        let dest_domain = Protocol::get_receiving_domain(&msg).unwrap();

        if let Some(lock_guard) = self.servers_map.try_lock().ok() {
            if lock_guard.contains_key(&dest_domain) {
                self.send_message(&dest_domain, msg);
            } else {
                println!("Message perdu car le serveur {} n'était pas en ligne ou n'existe pas.", dest_domain);
            }
            drop(lock_guard); // Release the lock
        } else {
            println!("Failed to acquire lock on servers_map.");
        }
    }

    /**
     * Méthode qui sert à envoyé un message à un des serveurs connecté.
     */
    fn send_message(&self, domain: &str, msg: String) {
        let encrypted_msg = AesEncryptor::encrypt(&self.aes_key, msg);
        let mut tcp_socket = match self.servers_map.try_lock() {
            Ok(lock_guard) => lock_guard.get(domain).unwrap().try_clone().unwrap(),
            Err(_) => {
                println!("Failed to acquire lock on servers_map.");
                return;
            }
        };
        tcp_socket.write_all(&encrypted_msg).unwrap();
        println!("Message transféré au serveur {}.", domain);
        drop(tcp_socket); // Release the lock
    }
}
