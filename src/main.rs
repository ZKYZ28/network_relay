mod server_config;
mod config_reader;
mod aes_encryptor;
mod protocol;
mod server_config_manager;
mod server_runnable;

use std::collections::HashMap;
use std::{str, thread};
use base64::{decode, encode};
use crate::aes_encryptor::AesEncryptor;

use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use crate::protocol::Protocol;
use crate::server_config::ServerConfig;
use crate::server_config_manager::ServerConfigManager;
use crate::server_runnable::ServerRunnable;

// TODO : POUR LA CLE NE PAS LA METTRE DANS LE CONSTRUCTEUR MAIS LA METTRE DANS LA METHODE !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

fn main() {
    
    receive_multicast().expect("Une erreur est surevnue lors de l'écoute en multicast.");

     fn receive_multicast() -> Result<(), std::io::Error> {
        // Specify the multicast group address and port.
        let port = 23106;

        // Create a UDP socket and bind it to the multicast address and port.
        let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))?;
        socket.join_multicast_v4(&Ipv4Addr::new(224, 1, 1, 255), &Ipv4Addr::new(0, 0, 0, 0))?;

        // Loop indefinitely to receive messages.
        loop {
            // Create a buffer to hold the received message.
            let mut buf = [0u8; 1024];

            // Receive a message from the socket.
            let (size, _) = socket.recv_from(&mut buf)?;

            // Convert the received bytes into a string.
            let echo_essage = String::from_utf8_lossy(&buf[..size]);

            if let Some(map) = Protocol::get_echo_map(&echo_essage) {
                let domain = map.get("domain").unwrap();
                let port = map.get("port").unwrap();

                println!("ECHO reçu du serveurr {} sur le port {}.", domain, port);

                let mut unicastSocket = TcpStream::connect(domain.to_owned()+":"+"port")?;
            }

            //let mut unicastSocket = TcpStream::connect("127.0.0.1:34254")?;
            // Print the received message.
            println!("Received message: {}", echo_essage);
        }
    }
}
