use crate::aes_encryptor::AesEncryptor;
use crate::protocol::Protocol;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream};
use std::sync::{Arc, Mutex};
use base64::engine::general_purpose;
use base64;
use base64::Engine;

pub struct ServerRunnable {
    servers_map: Arc<Mutex<HashMap<String, TcpStream>>>,
    server_aes: Arc<Mutex<HashMap<String, String>>>,
    aes_key: String,
    is_connected: bool
}

impl ServerRunnable {


    /// Cette fonction crée une nouvelle instance de la structure ServerRunnable en prenant en entrée une carte partagée des serveurs et une clé AES.
    /// La fonction retourne l'instance créée avec les valeurs des paramètres passés.
    ///
    pub(crate) fn new(server_aes: Arc<Mutex<HashMap<String, String>>>, servers_map: Arc<Mutex<HashMap<String, TcpStream>>>, aes_key: String) -> ServerRunnable {
        ServerRunnable {
            server_aes,
            servers_map,
            aes_key,
            is_connected: true
        }
    }


    /// La méthode handle_client est utilisée pour gérer la communication entre un client TCP et un serveur.
    /// Elle prend en entrée un flux TCP stream et utilise un BufReader pour lire les données entrantes.
    ///
    pub(crate) fn handle_client(&mut self, stream: &TcpStream, domain:&str) {
        let mut reader = BufReader::new(stream);                                                                  // Création d'un BufReader pour lire les donnée reçue depuis le stream TCP
        while self.is_connected {
            let mut message = String::new();
            match reader.read_line(&mut message) {                                                                                     // Lecture des données du flux et les stock dans la variable message. La méthode read_line est bloquante
                Ok(0) => break,
                Ok(_) => {
                    match general_purpose::STANDARD.decode(&message.trim_end()) {                                                     // Retourne soit une valeur Ok contenant les octets décodés, soit une erreur Err en cas de décodage invalide.
                        Ok(bytes) => {
                            let decrypted_message = AesEncryptor::decrypt(&self.aes_key, &bytes);       // Si le décodage se passe bien, on décrypte le message grâce à la clef AES du serveur

                            println!("Message reçu ! Décrypté : {:?}", decrypted_message);

                            Self::treat_message(&self, decrypted_message.unwrap())                                                     // On traite le message pour le rediriger vers le bon serveur.
                        }
                        Err(e) => {
                            println!("Erreur de décodage : {:?}", e);
                            continue;
                        }
                    }
                }
                Err(_) => {
                    self.is_connected = false;
                    let stream = self.servers_map.try_lock().unwrap().remove(domain).unwrap();
                    stream.shutdown(std::net::Shutdown::Both).expect("Could not shutdown stream");
                    println!("Serveur {} déconnecté...", domain);
                    break;
                }
            }
        }
        println!("Le thread du serveur {} a été tué...", domain);
    }



    /// La méthode treat_message sert à, en fonction du destinataire, envoyer/transférer le message vers le bon serveur.
    ///
    fn treat_message(&self, msg: String) {
        let dest_domain = Protocol::get_receiving_domain(&msg).unwrap();                                            // On récupère, grâce à la méthode get_receiving_domain de la classe Protocol, le nom du domaine de destination

        if let Some(lock_guard) = self.servers_map.try_lock().ok() {                                // Vérification que la map est bien accessible (pas en deadlock)
            if lock_guard.contains_key(&dest_domain) {                                                                  // Si oui, on récupère le stream TCP du serveur via son domaine
                drop(lock_guard);                                                                                      // Retrait du verrou pour libérer la map
                self.send_message(&dest_domain, msg);                                                                      // Envoi du message au Stream TCP du serveur connecté
            } else {
                println!("Message perdu car le serveur {} n'existe pas, ou n'est pas en ligne.", dest_domain);
            }
        } else {
            println!("Impossible de transférer le message car la serveur_map est en deadlock.");
        }
    }



    /// La méthode send_message sert à envoyé un message à un stream TCP en le cryptant avec AES256
    ///
    fn send_message(&self, domain: &str, msg: String) {
        let encrypted_msg = AesEncryptor::encrypt(&self.server_aes.lock().unwrap().get(domain).unwrap(), msg) + "\n";                          // Encryption du message avec AES256
        let mut tcp_socket = self.servers_map.lock().unwrap().get(domain).unwrap().try_clone().unwrap();                                                     // Récupération du socket du serveur destinataire
        tcp_socket.write_all(&encrypted_msg.as_bytes()).unwrap();                                                                                                    // Envoi
    }
}
