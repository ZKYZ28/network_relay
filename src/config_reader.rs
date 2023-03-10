use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;


/// La méthode load_servers permet de récupérer la map de clés aes du relay qui sont stockée dans un fichier json.
///
pub fn read_config(path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}
