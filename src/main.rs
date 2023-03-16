mod config_reader;
mod aes_encryptor;
mod protocol;
mod server_runnable;

use std::collections::HashMap;
use std::net::{TcpStream, UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::protocol::Protocol;
use crate::server_runnable::ServerRunnable;


static PORT: u16 = 23106;
static MULTICAST_IP: &str = "224.1.1.255";
static JSON_PATH : &str = "src/ressources/relayConfig.json";



fn main() {

    let server_map: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    let server_aes: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(config_reader::read_config(JSON_PATH).unwrap()));


    println!("-----Démarrage de l'écoute multicast sur l'IP {} et le port {}", PORT, MULTICAST_IP);
    receive_multicast(server_map.clone(), server_aes).expect("Une erreur est survenue lors de l'écoute en multicast.");
}



/// La méthode receive_multicast écoute en permanence les appels multicast sur l'adresse IP et le port définis.
/// Si un serveur s'annonce mais qu'il ne fait pas partie de la liste des serveurs partageant une clé AES avec le relais, il sera ignoré.
/// Sinon, une connexion TCP sera créée sur ce serveur à l'aide des informations annoncées, et un nouveau thread sera créé pour la réception et l'envoi de messages vers ou depuis ce serveur en TCP.
/// Cette méthode doit être appelée une seule fois au lancement du programme et fonctionne même si d'autres threads ont été créés auparavant.
///
fn receive_multicast(server_map: Arc<Mutex<HashMap<String, TcpStream>>>, aes_map: Arc<Mutex<HashMap<String, String>>>) -> Result<(), std::io::Error> {

    let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), PORT))?;            // Création d'un socket UDP et le lie à toutes les interfaces locales en écoutant le port spécifié
    socket.join_multicast_v4(&Ipv4Addr::from_str(MULTICAST_IP).unwrap(), &Ipv4Addr::UNSPECIFIED)?;                          // Permet de joindre un groupe de diffusion multicast IPv4 en utilisant l'adresse IP multicast spécifiée (stockée dans la variable MULTICAST_IP) et en écoutant sur toutes les interfaces locales.

    loop {
        let mut buf = [0u8; 1024];                                                                                                    // Crée un tableau mutable de 1024 éléments de type u8 (Entier non signé de 8 bits), initialisés à zéro, pour stocker des données qui seront lues à partir d'un socket.

        let (size, _) = socket.recv_from(&mut buf)?;                                                                                     // Lit les données reçues sur un socket UDP, bloquant si rien reçu

        let echo_message = String::from_utf8_lossy(&buf[..size]);                                                                  // Convertit un vecteur d'octets en une chaîne de caractères

        if let Some(map) = Protocol::get_echo_map(&echo_message) {                                                     // Via la méthode get_echo_map de Protocol, recupère les info du echo à l'aide d'une map
            let domain = map.get("domain").unwrap().to_string();                                                                      // Domain annoncé dans le echo
            let port = map.get("port").unwrap();                                                                                     // Port annoncé dans le echo

            println!("ECHO received from server {} on port {}.", domain, port);

            // Récupération de la clé AES stockée
            if aes_map.try_lock().unwrap().contains_key(&domain) {                                                                              // Vérification que le serveur partage bien une clé AES avec le serveur ennoncé

                let aes_key = aes_map.try_lock().unwrap().get(&domain).unwrap().to_string();

                if aes_key.len() == 44 {

                    let unicast_socket = TcpStream::connect(format!("{}:{}", domain, port))?;                                      // Connection au ServeurSocket du serveur

                    let mut map = server_map.lock().unwrap();                                                              // Crée un verrou sur la map grâce a lock pour garantir qu'un seul thread peut y accéder à la fois
                    map.insert(domain.clone(), unicast_socket.try_clone().expect("Problème lors du clonage du unicast socket"));       // Ajout du socket dans la map de serveur connecté

                    println!("Connection établie avec le serveur {} !", domain.clone());


                    //Vérification de la validité de la clé
                    let server_map_clone = server_map.clone();
                    let server_aes_clone = aes_map.clone();                                                            // Sans l'utilisation du mot clé move, ces variables ne seraient pas disponibles dans le nouveau thread, car Rust garantit que chaque variable ne peut avoir qu'un propriétaire à la fois. En utilisant le mot clé move, Rust transfère la propriété de server_map_clone et aes_key à la closure du thread, permettant ainsi leur utilisation dans ce nouveau contexte.
                    // Sans l'utilisation du mot clé move, ces variables ne seraient pas disponibles dans le nouveau thread, car Rust garantit que chaque variable ne peut avoir qu'un propriétaire à la fois. En utilisant le mot clé move, Rust transfère la propriété de server_map_clone et aes_key à la closure du thread, permettant ainsi leur utilisation dans ce nouveau contexte.
                    thread::spawn(move || {
                        let mut server_runnable = ServerRunnable::new( server_aes_clone, server_map_clone, aes_key);
                        server_runnable.handle_client(&unicast_socket, &domain);
                    });

                }else {
                    println!("ECHO ignoré car le Relay ne contient pas une clé valide avec le server {}  ", domain)
                }
            } else {
                println!("ECHO ignoré car le server {} n'est pas connu du Relay ", domain)
            }
        }
    }
}








/*
   Dans ce contexte, Arc et Mutex sont utilisés pour partager des données entre plusieurs threads de manière sûre et efficace.

   Arc signifie "Atomic Reference Counting" et est utilisé pour créer une référence partagée entre plusieurs threads.
   Dans cet exemple, server_map est une référence partagée à un HashMap qui stocke des connexions TCP.

   L'accès simultané à HashMap par plusieurs threads pourrait causer des problèmes de concurrence. C'est là que Mutex intervient.
   Le Mutex permet de verrouiller l'accès à une ressource partagée entre plusieurs threads.
   Cela signifie que seul un thread peut accéder à la ressource verrouillée à un moment donné.

   Ainsi, en combinant Arc et Mutex, nous avons une référence partagée à un HashMap qui peut être utilisée par plusieurs threads simultanément, en toute sécurité.
*/
