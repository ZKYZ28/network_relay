use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)] //Serialize, Deserialize automatique
pub struct ServerConfig {
    server_name: String,
    base64key_aes: String,
}

impl ServerConfig {
    fn new(server_name: String, base64key_aes: String) -> Self {
        Self {
            server_name: server_name,
            base64key_aes: base64key_aes,
        }
    }

    pub fn get_server_name(&self) -> &str {
        &self.server_name
    }

    pub fn get_base64_key_aes(&self) -> &str {
        &self.base64key_aes
    }
}