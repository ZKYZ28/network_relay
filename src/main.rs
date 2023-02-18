mod ServerConfig;
mod ConfigReader;

use std::net::{UdpSocket, IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    // Set the multicast address and port to listen on
    let multicast_addr = "224.1.1.255";
    let port = 23106;

    // Create a UDP socket bound to the multicast address and port
    let socket = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port))?;
    socket.join_multicast_v4(&Ipv4Addr::from_str(multicast_addr).unwrap(), &Ipv4Addr::new(0, 0, 0, 0))?;


    let config = ConfigReader::read_config("src/ressources/relayConfig.json").unwrap();
    let test= config.get("g6server1.godswila.guru").unwrap();

    println!("{}", test.get_server_name());


    // Listen for multicast packets
    let mut buf = [0; 1024];
    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, src);
        println!("{}", String::from_utf8_lossy(&buf[..size]));
    }
}
