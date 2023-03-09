use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::aes_encryptor::AesEncryptor;
use crate::protocol::Protocol;

pub struct ServerRunnable {
    handle: Option<thread::JoinHandle<()>>,
    servers_map: Arc<Mutex<HashMap<String, TcpStream>>>,
    domain: String,
    aes_key: String,
}

impl ServerRunnable {
    pub(crate) fn new(servers_map: Arc<Mutex<HashMap<String, TcpStream>>>, domain: String, aes_key: String) -> ServerRunnable {
        ServerRunnable {
            handle: None,
            servers_map,
            domain,
            aes_key,
        }
    }



    pub(crate) fn handle_client(&self) {
        let binding = self.servers_map.clone();
        let binding = binding.lock().unwrap();
        let stream = binding.get(&self.domain);

        if let Some(stream) = stream {
            let mut reader = BufReader::new(stream);

            loop {
                let mut buffer = String::new();
                match reader.read_line(&mut buffer) {
                    Ok(0) => break,
                    Ok(_) => {
                        match base64::decode(&buffer.trim_end()) {
                            Ok(bytes) => {
                                let decrypted_message = AesEncryptor::decrypt(&self.aes_key, &bytes);
                                println!("Décrypté : {:?}", decrypted_message);

                                Self::analyse_message(&self, decrypted_message)
                            },
                            Err(e) => {
                                println!("Erreur de décodage Base64 : {:?}", e);
                                continue;
                            }
                        }
                    },
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




    fn analyse_message(&self, msg: Result<String, String>) {
        // Vérifier si le message contient une erreur
        let msg = match msg {
            Ok(value) => value,
            Err(error) => {
                println!("Erreur lors de l'analyse du message : {}", error);
                return;
            }
        };
        println!("{}", msg);
        // Décomposer le message
        let groupes = match Protocol::decomposer(&msg, "send") {
            Ok(value) => value,
            Err(error) => {
                println!("Erreur lors de la décomposition du message : {}", error);
                return;
            }
        };

        // Déterminer le serveur destinataire
        let server_destinataire = &groupes[8];
        println!("{}", server_destinataire);
        // Envoyer le message
      //  println!("SERVEUR DESTINATAIRE CONNECTE");
        Self::send_message(&self, server_destinataire, msg);
    }



    /**
     * Méthode qui sert à envoyé un message à un des serveurs connecté.
     */
    fn send_message(&self, domain: &str, msg: String) {
        let encrypted_msg = AesEncryptor::encrypt(&self.aes_key, msg);                          //Encryption du message
        let mut tcp_socket = self.servers_map.lock().unwrap().get(domain).unwrap().try_clone().unwrap();     //Récupération du socket du serveur destinataire
        tcp_socket.write_all(&encrypted_msg).unwrap();                                                               //Envoi
    }
}
