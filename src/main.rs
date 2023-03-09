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

fn main() {

    //Création des map
    let server_map: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    let server_aes_map = config_reader::read_config("src/ressources/relayConfig.json").unwrap();


    println!("-----Démarrage de l'écoute multicast.");
    receive_multicast(server_map.clone(), server_aes_map).expect("An error occurred while listening in multicast.");
}

/**
 * Méthode qui écoute en boucle pour les echo et qui ajoute dans servermap quand une connection est éffectuée
 */
fn receive_multicast(server_map: Arc<Mutex<HashMap<String, TcpStream>>>, aes_map: HashMap<String, String>) -> Result<(), std::io::Error> {
    let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), PORT))?;    //Création d'un socket UDP et le lie à toutes les interfaces locales en écoutant le port spécifié
    socket.join_multicast_v4(&Ipv4Addr::from_str(MULTICAST_IP).unwrap(), &Ipv4Addr::new(0, 0, 0, 0))?;  //Permet de joindre un groupe de diffusion multicast IPv4 en utilisant l'adresse IP multicast spécifiée (stockée dans la variable MULTICAST_IP) et en écoutant sur toutes les interfaces locales.

    loop {
        let mut buf = [0u8; 1024];                                                       //Crée un tableau mutable de 1024 éléments de type u8, initialisés à zéro, pour stocker des données qui seront lues à partir d'un socket.

        let (size, _) = socket.recv_from(&mut buf)?;                                        //Lit les données reçues sur un socket UDP

        let echo_message = String::from_utf8_lossy(&buf[..size]);                     //Convertit le tableau de byte en String

        if let Some(map) = Protocol::get_echo_map(&echo_message) {      //Via la méthode get_echo_map de Protocol, recupère les info du echo
            let domain = map.get("domain").unwrap().to_string();                        //Domain annoncé dans le echo
            let port = map.get("port").unwrap();                                       //Port annoncé dans le echo

            println!("ECHO received from server {} on port {}.", domain, port);
            println!(" VALEUR &domain : {}", &domain);
            if aes_map.contains_key(&domain) {                                                            //Vérification que le serveur partage bien une clé AES
                let unicast_socket = TcpStream::connect(format!("{}:{}", domain, port))?;
                let mut map = server_map.lock().unwrap();
                map.insert(domain.clone(), unicast_socket.try_clone().expect("Problème lors du clonage du unicast socket"));  //Ajout du socket dans la map de serveur connecté
                println!("Connection établie avec le serveur {}.", domain.clone());

                let aes_key = aes_map.get(&domain).unwrap().to_string();                           //Récupération de la clé AES stockée

                let server_map_clone = server_map.clone();                            //Lancement du thread serveur
                thread::spawn(move || {
                    let server_runnable = ServerRunnable::new(server_map_clone, aes_key);
                    server_runnable.handle_client(&unicast_socket);
                });
            } else {
                println!("ECHO ignoré car le server {} ne partage pas de clé avec ce relay.", domain)
            }
        }
    }
}
