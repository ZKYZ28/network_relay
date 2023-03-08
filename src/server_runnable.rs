use std::collections::HashMap;
use std::io::BufReader;
use std::net::TcpListener;
use std::thread;
use tokio::net::TcpStream;

pub struct ServerRunnable<'a> {
    handle: Option<thread::JoinHandle<()>>,
    connected_server: &'a mut HashMap<String, TcpStream>,
    tcp_stream: TcpStream,
}

impl<'a> ServerRunnable<'a> {
    pub(crate) fn new(connected_server: &'a mut HashMap<String, TcpStream>, tcp_stream: TcpStream) -> ServerRunnable<'a> {
        ServerRunnable {
            handle: None,
            connected_server,
            tcp_stream,
        }
    }

    pub(crate) fn start(&mut self) {

        let handle = thread::spawn(move || {
            // Code exécuté dans le Thread
            handle_client(self.tcp_stream.clone());
        });

        self.handle = Some(handle);
    }

    pub(crate) fn join(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }

    fn handle_client(stream: TcpStream) {
        let mut reader = BufReader::new(stream);
        loop {
            let mut buffer = String::new();
            match reader.readline(&mut buffer) {
                Ok(0) => break, // Connexion fermée
                Ok(1) => {
                    // Ligne lue avec succès, faire quelque chose avec la ligne
                    println!("Ligne lue : {}", buffer);
                }
                Err(e) => {
                    // Erreur de lecture, faire quelque chose avec l'erreur
                    println!("Erreur de lecture : {}", e);
                    break;
                }
            }
        }
    }
}