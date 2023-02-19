use std::collections::HashMap;
use crate::server_config::ServerConfig;

struct ServerConfigManager {
    my_hashmap: HashMap<String, ServerConfig>,
}

impl MyHashMapClass {
    fn new(hashmap: HashMap<String, ServerConfig>) -> Self {
        Self { my_hashmap: hashmap }
    }

    fn server_is_valid(key: &str, map: &HashMap<String, server_config>) -> bool {
        if map.contains_key(key){
            let server_config = mapServerConfig.get(domaine);
            let key = server_config.unwrap().get_base64_key_aes();

            if !key.is_empty(){
                // + mettre à jour le isConnected
                return true
            }
        }
        return false
    }
}