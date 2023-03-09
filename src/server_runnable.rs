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
            let mut reader = BufReader::new(stream);    //Création de l'input stream du socket pour écouté les messages entrants

            loop {
                let mut buffer = String::new();                       //Déclaration/Initialisation de la variable repésentant la ligne entrante
                match reader.read_line(&mut buffer) {                   //Lecture du premier message reçu
                    Ok(0) => break,     // Buffer vide
                    Ok(_) => {          // Pas de problème

                        println!("Ligne récu du serveur {} : {}, {}", self.domain, buffer, buffer.trim_end().len());
                        println!("Ligne AsByte {} : {:?}", self.domain, &base64::decode(&buffer.trim_end()).unwrap());




                        use base64;
                        let decrypted_message = AesEncryptor::decrypt(&self.aes_key, &base64::decode(&buffer.trim_end()).unwrap()); //TODO decrypt() ne marche pas
                        println!("Décrypté : {:?}", decrypted_message);

                        //self::analyse_message(decrypted_message)
                    }
                    Err(_) => {         // Erreur de lecture
                        println!("Erreur de lecture");
                        break;
                    }
                }
            }
        } else {
            println!("Socket introuvable pour le nom de domaine {}", self.domain);
        }
    }


    fn analyse_message(msg: Result<String, String>) {
        //let send_map = Protocol::get_send_map(&msg.unwrap());
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
