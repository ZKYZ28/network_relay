use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct ServerRunnable<'a> {
    handle: Option<thread::JoinHandle<()>>,
    connected_server: &'a mut HashMap<String, TcpStream>,
    tcp_stream: TcpStream,
}

impl<'a> ServerRunnable<'a> {
    pub(crate) fn new(connected_server: &mut HashMap<String, TcpStream>, tcp_stream: TcpStream) -> ServerRunnable {
        ServerRunnable {
            handle: None,
            connected_server,
            tcp_stream,
        }
    }

    pub(crate) fn start(&mut self) {
        let tcp_stream = self.tcp_stream.try_clone().unwrap(); // Cloner la TcpStream pour la passer au Thread
        // Cloner la TcpStream pour la passer au Thread
        let handle = thread::spawn(move || {
            // Code exécuté dans le Thread
            Self::handle_client(tcp_stream);
        });

        self.handle = Some(handle);
    }

    pub(crate) fn join(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }

    fn handle_client(tcp_stream: TcpStream) {
        let stream = tcp_stream.try_clone().unwrap();
        let mut reader = BufReader::new(stream);
        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer) {
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
                _ => {}
            }
        }
    }
}
