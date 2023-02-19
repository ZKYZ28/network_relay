use std::collections::HashMap;
use crate::server_config::ServerConfig;

pub struct ServerConfigManager {
    my_hashmap: HashMap<String, ServerConfig>,
}

impl ServerConfigManager {
    pub(crate) fn new(hashmap: HashMap<String, ServerConfig>) -> Self {
        Self { my_hashmap: hashmap }
    }

    /// Vérifie si le domaine spécifié existe dans la hashmap de configurations de serveurs
    ///
    /// # Arguments
    ///
    /// * `domain` - Le domaine à vérifier
    ///
    /// # Returns
    ///
    /// * `true` si le domaine est présent dans la hashmap et qu'une clé AES est associée à la configuration correspondante, `false` sinon.
    pub(crate) fn server_is_valid(&mut self, domain: &str) -> bool {
        if self.my_hashmap.contains_key(domain) {
            let server_config = self.my_hashmap.get(domain);
            let key = server_config.unwrap().get_base64_key_aes();

            if !key.is_empty() {
                // mettre à jour le isConnected
                let server_config = self.my_hashmap.get_mut(domain).unwrap();
                server_config.set_is_connected(true);
                println!("{}", "Serveur connecté first");
                return true;
            }
        }
       return false
    }

    ///
    /// #Returns l'instance de Server Option
    pub(crate) fn get_server_config(&self, domain: &str) -> Option<&ServerConfig> {
        self.my_hashmap.get(domain)
    }
}
