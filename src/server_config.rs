use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)] //Sérialisation et désérialisation automatique
pub struct ServerConfig {
    server_name: String,
    base64key_aes: String,
    #[serde(skip)] // Ajout de ceci pour ne pas lire le booléen depuis le json
    is_connected: bool, // Ajouter le booléen is_connected
}

impl ServerConfig {
    pub fn new(server_name: String, base64key_aes: String) -> Self {
        Self {
            server_name,
            base64key_aes,
            is_connected: false, // Initialiser is_connected à false par défaut
        }
    }

    pub fn get_server_name(&self) -> &str {
        &self.server_name
    }

    pub fn get_base64_key_aes(&self) -> &str {
        &self.base64key_aes
    }

    // Mettre à jour le booléen is_connected
    pub fn set_is_connected(&mut self, is_connected: bool) {
        self.is_connected = is_connected;
    }

    // Récupérer la valeur de is_connected
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}
