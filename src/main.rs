mod server_config;
mod config_reader;
mod aes_encryptor;
mod protocol;
mod server_config_manager;
mod server_runnable;

use std::collections::HashMap;
use std::str;
use base64::{decode, encode};
use crate::aes_encryptor::AesEncryptor;

use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::net::TcpStream;
use crate::protocol::Protocol;
use crate::server_config::ServerConfig;
use crate::server_config_manager::ServerConfigManager;
use crate::server_runnable::ServerRunnable;

// TODO : POUR LA CLE NE PAS LA METTRE DANS LE CONSTRUCTEUR MAIS LA METTRE DANS LA METHODE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

fn main() -> std::io::Result<()> {
    // Set the multicast address and port to listen on
    let multicast_addr = "224.1.1.255";
    let port = 23106;

    // Create a UDP socket bound to the multicast address and port
    let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))?;
    socket.join_multicast_v4(&Ipv4Addr::from_str(multicast_addr).unwrap(), &Ipv4Addr::new(0, 0, 0, 0))?;


    println!("{}", "---------------------ECHO encrypté ----------------------");

    // Clé secrète pour l'AES
    let key_base64 = "z01JW7/j8Acb5PYfrl+P15O/axfLZ1DvJpE+lyxjNtQ=";

    // Message à encrypter
    let message = "Hello, world!".to_string();

    // Encrypter le message
    let ciphertext = AesEncryptor::encrypt(key_base64, message);

    // Afficher le message encrypté en base64
    let ciphertext_to_string = String::from_utf8_lossy(&ciphertext);
    println!("Ciphertext : {:?}", ciphertext_to_string);


// Decrypter le message
    match AesEncryptor::decrypt(key_base64, &ciphertext) {
        Ok(msg) => println!("Decrypted message: {}", msg),
        Err(e) => println!("Error: {}", e),
    }


    // Créer un encrypteur AES
   // let aes_encryptor = AesEncryptor::new(key_base64);

    // lire la configuration du fichier
    let map_server_config = config_reader::read_config("src/ressources/relayConfig.json").unwrap();
    // créer une instance de ServerConfigManager
    let mut server_config_manager = ServerConfigManager::new(map_server_config);
    let mut connected_server: HashMap<String, TcpStream> = HashMap::new();



    // Listen for multicast packets
    let mut buf = [0; 1024];
    loop {
        let (size, mut src) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, src);
        println!("{}", String::from_utf8_lossy(&buf[..size]));

        let message_to_encrypt = String::from_utf8_lossy(&buf[..size]).to_string();

        // Encrypter le message
        let ciphertext = AesEncryptor::encrypt(key_base64, message_to_encrypt);

        // Afficher le message encrypté en base64
        let ciphertext_to_string = String::from_utf8_lossy(&ciphertext);
        println!("Ciphertext : {:?}", ciphertext_to_string);


        // Decrypter le message
        let decrypted_message = AesEncryptor::decrypt(key_base64, &ciphertext);
        //Si le chiffrement s'est bien déroule, le message déchiffré est stocké dans msg si tout se passe bien
        // sinon message par défaut //TODO peut être mettre chaine vide afin de déterminer si ça passe dans from_message ou non
        println!("Decyphertext : {:?}", decrypted_message);
        let msg = match decrypted_message {
            Ok(msg) => msg,
            Err(e) => {
                // Mettre un message par défaut si le déchiffrement n'a pas fonctionné
                "Impossible de décrypter le message".to_owned()
            }
        };

        println!("Msg : {:?}", msg);

        //Uniquement SEND ou ECHO
        if let Some(message_type) = Protocol::from_message(&msg) {
            let type_message = message_type.to_lowercase();

            //Récupération des groupes puis traitement
            if let Ok(groupes) = Protocol::decomposer(&msg, &type_message) {

                if type_message == "echo" {
                    let domaine_groupement = &groupes[1];

                    //Vérification de la connexion
                    let domaine_groupement_echo = server_config_manager.get_server_config(domaine_groupement).map(|sc| sc.is_connected()).unwrap_or(false);

                    //Si serveur non connecté, vérification server_is_valid
                    if !domaine_groupement_echo {

                        //Connecter le serveur au relai si les conditions sont respectées
                        if server_config_manager.server_is_valid(domaine_groupement){
                            //Ajouter à la map des serveurs connectées ce nom de domaine + le socket avec les inforamtion
                            src.set_port((&groupes[0]).parse().unwrap());
                            println!("IP + PORT  : {}", src);

                            let stream = TcpStream::connect(&src).unwrap_or_else(|err| {
                                eprintln!("Erreur lors de la connexion au serveur : {}", err);
                                std::process::exit(1);
                            });

                            connected_server.insert(domaine_groupement.to_string(), stream);

                            let map_server_config_two = config_reader::read_config("src/ressources/relayConfig.json").unwrap();
                            let mut server_config_manager_two = ServerConfigManager::new(map_server_config_two);
                            let mut server_runnable = ServerRunnable::new(&mut connected_server,  domaine_groupement.to_string(), server_config_manager_two);
                            server_runnable.start();
                            server_runnable.join();

                        }
                    } else {
                        println!("{}", "Serveur déjà connecté");
                    }
                } else if type_message == "send" {

                    println!("{}", "SEND TYPE");

                    let mut server_destinataire;
                    if groupes.len() < 11 {
                        //CAS d'une TREND
                        println!("{}", "TREND ENVOYE SEND");
                        server_destinataire = &groupes[8];
                    }else{
                        //CAS D'UN MSGS
                        println!("{}", "TREND ENVOYE MSGS");
                        server_destinataire = &groupes[9]
                    }
                    //Vérifier le domaine expéditeur
                    if connected_server.contains_key(server_destinataire){
                        let key = server_config_manager.get_server_config(server_destinataire).map(|sc| sc.get_base64_key_aes()).unwrap_or("");
                       // let aes_encryptor = AesEncryptor::new(key);
                        //let msg_crypted = aes_encryptor.encrypt(msg);

                      // println!(" MESSAGE CRYPTE SEND PRET A ENVOYER{:?}",  String::from_utf8_lossy(&msg_crypted));

                        let mut socket = connected_server.get(server_destinataire).unwrap();

                        //TODO : GERER ERREUR D'E/S
                        //
                        println!(" ECRIT AVANT SOCKET");
                       // socket.try_write(msg_crypted.as_slice())?;
                        println!(" ECRIT APRES SOCKET");

                    }else{
                        println!("Server destinataire non connecté");
                    }

                }
            } else {
                println!("La décomposition du message est invalide.");
            }
        } else {
            println!("Le message reçu est invalide.");
        }
    }



    //Corbeille :
    // //TEST ECHO
    // if let Some(message_type) = Protocol::from_message(message2) {
    //     println!("Message type: {}", message_type);
    //
    //     //Récupération des différentes parties de echo
    //     let groupes = Protocol::decomposer(message2, "echo").unwrap();
    //     let port = &groupes[0];
    //     let domaine = &groupes[1];
    //
    //
    //     //Vérification du domaine quand on reçoit le echo
    //     if mapServerConfig.contains_key(domaine){
    //         let server_config = mapServerConfig.get(domaine);
    //         let key = server_config.unwrap().get_base64_key_aes();
    //
    //         if !key.is_empty(){
    //             println!("OK");
    //         }else {
    //             println!("CLE VIDE");
    //         }
    //
    //     }else {
    //         println!("PAS DEDANS")
    //     }
    //
    // } else {
    //     println!("Invalid message");
    // }

    //TEST ERREUR
    /*if let Some(message_type) = Protocol::from_message(message3) {
        println!("Message type: {}", message_type);
    } else {
        println!("Invalid message");
    }*/


    // let mapServerConfig = config_reader::read_config("src/ressources/relayConfig.json").unwrap();
    // let server_congi_manager = ServerConfigManager(mapServerConfig);

    /*println!("{}", test.get_server_name());

    // Clé secrète pour l'AES
    let key_base64 = "z01JW7/j8Acb5PYfrl+P15O/axfLZ1DvJpE+lyxjNtQ=";

    // Créer un encrypteur AES
    let aes_encryptor = AesEncryptor::new(key_base64);

    // Message à encrypter
    let message = "Hello, world!".to_string();

    // Encrypter le message
    let ciphertext = aes_encryptor.encrypt(message);

    // Afficher le message encrypté en base64
    let ciphertext_to_string = String::from_utf8_lossy(&ciphertext);
    println!("Ciphertext : {:?}", ciphertext_to_string);

    // Decrypter le message
    let decrypted_message = aes_encryptor.decrypt(&ciphertext);
    match decrypted_message {
        Ok(msg) => println!("Decrypted message: {}", msg),
        Err(e) => println!("Error: {}", e),
    }*/

    // //TEST PROTOCOL
    // let message1 = "SEND 12345@mondomaine.com francis@domaine1.com #tendance123@domaine2.com Cici est mon message\r\n";
    // let message4 = "SEND 12345@mondomaine.com francis@domaine.com edwin@domaine2.com Cici est mon message\r\n";
    // let message2 = "ECHO 1234 g6server1.godswila.guru\r\n";
    // let message3 = "INVALID MESSAGE\r\n";
    //
    // //TEST SEND
    // if let Some(message_type) = Protocol::from_message(message1) {
    //     println!("Message type: {}", message_type);
    //
    //     let groupes = Protocol::decomposer(message1, "send").unwrap();
    //
    //     let id_domaine = &groupes[0];
    //     let nom_utilisateur_emetteur = &groupes[3];
    //     let domaine_emetteur = &groupes[4];
    //     let nom_domaine_emetteur = groupes[3].to_owned() + "@" + &groupes[4];
    //
    //     let nom_tag_domaine_receveur = &groupes[5];
    //     let nom_tag_receveur = &groupes[8];
    //     let domaine_receveur = &groupes[9];
    //
    //     let message_intenre = &groupes[11];
    //
    //     println!("id_domaine = {}", id_domaine);
    //     println!("nom_utilisateur_emetteur = {}", nom_utilisateur_emetteur);
    //     println!("domaine_emetteur = {}", domaine_emetteur);
    //     println!("nom_domaine_emetteur = {}", nom_domaine_emetteur);
    //
    //     println!("nom_tag_domaine_receveur = {}", nom_tag_domaine_receveur);
    //     println!("nom_tag_receveur = {}", nom_tag_receveur);
    //     println!("domaine_receveur = {}", domaine_receveur);
    //
    //     println!("message_intenre = {}", message_intenre);
    //
    // } else {
    //     println!("Invalid message");
    // }
    //
    // println!("-----------------------");
    //
    // if let Some(message_type) = Protocol::from_message(message4) {
    //     println!("Message type: {}", message_type);
    //
    //
    //     let groupes = Protocol::decomposer(message4, "send").unwrap();
    //
    //     if groupes.len() < 11{
    //         let id_domaine = &groupes[0];
    //         let nom_utilisateur_emetteur = &groupes[3];
    //         let domaine_emetteur = &groupes[4];
    //         let nom_domaine_emetteur = groupes[3].to_owned() + "@" + &groupes[4];
    //
    //         let nom_tag_domaine_receveur = &groupes[5];
    //         let nom_tag_receveur = &groupes[7];
    //         let domaine_receveur = &groupes[8];
    //
    //         let message_intenre = &groupes[9];
    //
    //         println!("id_domaine = {}", id_domaine);
    //         println!("nom_utilisateur_emetteur = {}", nom_utilisateur_emetteur);
    //         println!("domaine_emetteur = {}", domaine_emetteur);
    //         println!("nom_domaine_emetteur = {}", nom_domaine_emetteur);
    //
    //         println!("nom_tag_domaine_receveur = {}", nom_tag_domaine_receveur);
    //         println!("nom_tag_receveur = {}", nom_tag_receveur);
    //         println!("domaine_receveur = {}", domaine_receveur);
    //
    //         println!("message_intenre = {}", message_intenre);
    //     }
    // } else {
    //     println!("Invalid message");
    // }
    //
    // println!("-----------------------");

    //
    // //Test ECHO connexion
    // println!("{}", "----------------ECHO SITUATION-----------------");
    // // Clé valide
    // let echo1 = "ECHO 23106 g6server1.godswila.guru\r\n";
    // // Clé non valide (manquante)
    // let echo2 = "ECHO 1234 g6server2.godswila.guru\r\n";
    // // lire la configuration du fichier
    // let map_server_config = config_reader::read_config("src/ressources/relayConfig.json").unwrap();
    //
    // // créer une instance de ServerConfigManager
    // let mut server_config_manager = ServerConfigManager::new(map_server_config);
    //
    // // tester le premier domaine
    // let domain1 = "g6server1.godswila.guru";
    // let is_valid1 = server_config_manager.server_is_valid(domain1);
    // println!("Domaine {} valide ? {}", domain1, is_valid1);
    //
    // //Tester si le serveur est bien connecté
    // let domain1connected = server_config_manager.get_server_config(domain1).map(|sc| sc.is_connected()).unwrap_or(false);
    // if !domain1connected {
    //     println!("{}", "Serveur non connecté")
    // } else {
    //     println!("{}", "Serveur connecté")
    // }


}
