use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)] //Serialize, Deserialize automatique
pub struct ServerConfig {
    serverName: String,
    base64KeyAES: String,
}

impl ServerConfig {
    fn new(serverName: String, base64KeyAES: String) -> Self {
        Self {
            serverName,
            base64KeyAES,
        }
    }

    pub fn get_server_name(&self) -> &str {
        &self.serverName
    }

    pub fn get_base64_key_aes(&self) -> &str {
        &self.base64KeyAES
    }
}