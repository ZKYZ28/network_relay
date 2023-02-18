mod server_config;
mod config_reader;
mod aes_encryptor;

use std::str;
use base64::{decode, encode};
use crate::aes_encryptor::AesEncryptor;

use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    // Set the multicast address and port to listen on
    let multicast_addr = "224.1.1.255";
    let port = 23106;

    // Create a UDP socket bound to the multicast address and port
    let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))?;
    socket.join_multicast_v4(&Ipv4Addr::from_str(multicast_addr).unwrap(), &Ipv4Addr::new(0, 0, 0, 0))?;


    let config = config_reader::read_config("src/ressources/relayConfig.json").unwrap();
    let test= config.get("g6server1.godswila.guru").unwrap();

    println!("{}", test.get_server_name());

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
    }


    // Listen for multicast packets
    let mut buf = [0; 1024];
    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, src);
        println!("{}", String::from_utf8_lossy(&buf[..size]));
    }
}
