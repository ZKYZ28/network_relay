use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use Aes256GcmEncryptor::Aes256GcmEncryptor;
fn main() -> std::io::Result<()> {
    // Set the multicast address and port to listen on
    let multicast_addr = "224.1.1.255";
    let port = 23106;

    // Create a UDP socket bound to the multicast address and port
    let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))?;
    socket.join_multicast_v4(&Ipv4Addr::from_str(multicast_addr).unwrap(), &Ipv4Addr::new(0, 0, 0, 0))?;


    //TEST AES256
    let key = [0u8; 32];
    let encryptor = Aes256GcmEncryptor::new(key);

    let plaintext = "Hello, world!";
    let ciphertext = encryptor.encrypt(plaintext);
    let decrypted = encryptor.decrypt(&ciphertext);

    assert_eq!(decrypted, plaintext);
    //FI?


    // Listen for multicast packets
    let mut buf = [0; 1024];
    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, src);
        println!("{}", String::from_utf8_lossy(&buf[..size]));
    }
}
