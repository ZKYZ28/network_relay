use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use crate::aes_encryptor::AesEncryptor;
use crate::protocol::Protocol;
use crate::server_config_manager::ServerConfigManager;

pub struct ServerRunnable {
    handle: Option<thread::JoinHandle<()>>,
    connected_server: Arc<Mutex<HashMap<String, TcpStream>>>,
    current_server: String,
    server_config_manager: ServerConfigManager,
}

impl ServerRunnable {
    pub(crate) fn new(connected_server: Arc<Mutex<HashMap<String, TcpStream>>>, current_server: String, server_config_manager: ServerConfigManager) -> ServerRunnable {
        ServerRunnable {
            handle: None,
            connected_server,
            current_server,
            server_config_manager
        }
    }


    pub (crate) fn handle_client(&self)  {
        let my_connected_server = self.connected_server.lock().unwrap();
        let stream = my_connected_server.get(&*self.current_server).unwrap();

        let mut reader = BufReader::new(stream);

        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer) {
                Ok(0) => break, // Connexion fermée
                Ok(_) => {
                    // CAS : la ligne est correctement lue

                    let key = self.server_config_manager.get_server_config(&*self.current_server).map(|sc| sc.get_base64_key_aes()).unwrap_or("");
                    println!("SEND reçu CHIFFRE LOULOU : {:?}", buffer.as_bytes());
                    let decrypted_message = AesEncryptor::decrypt(key, buffer.as_bytes());
                    println!("SEND reçu : {:?}", decrypted_message);


                  // self::analyse_message(decrypted_message)

                }
                Err(e) => {
                    // Erreur de lecture
                    println!("Erreur de lecture");
                    break;
                }
            }
        }
    }

    fn analyse_message(msg: Result<String, String>) {
        //reçois le msg
        //Fais le traitement


        //unwrap pour convertir la result en string
        /*Ok(groupes) = Protocol::decomposer(&msg.unwrap(), "send");
        let mut server_destinataire;
        if groupes.len() < 11 {
            //CAS d'une TREND
            server_destinataire = &groupes[8];
        }else{
            //CAS D'UN MSGS
            server_destinataire = &groupes[9]
        }*/

        //Vérifier le domaine expéditeur
       /* if connected_server.contains_key(server_destinataire){
            let key = server_config_manager.get_server_config(server_destinataire).map(|sc| sc.get_base64_key_aes()).unwrap_or("");
            let aes_encryptor = AesEncryptor::new(key);

            //unwrap pour convertir la result en string
            let msg_crypted = aes_encryptor.encrypt(msg.unwrap());

            let mut socket = connected_server.get(server_destinataire).unwrap();


            socket.try_write(msg_crypted.as_slice())?;*/

        //A la fin utilise la méthode send_message en donnant le message et le stream à utiliser pour l'envoie
    }

    fn send_message(msg: Result<String, String>, stream : TcpStream) {

    }
}
