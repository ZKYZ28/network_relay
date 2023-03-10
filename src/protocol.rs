use regex::Regex;
use std::collections::HashMap;

pub struct Protocol {}


/// La classe Protocol a pour rôle d'extraire les informations contenues dans un message respectant la grammaire définie.
/// Elle permet ainsi de récupérer les différentes données nécessaires pour la communication entre les différents serveurs du réseau.
///
impl Protocol {
    const CRLF: &'static str = "\\r\\n";
    const ESP: &'static str = "\\x20";
    const DOMAINE: &'static str = "(?P<domaine2>(?P<lettre_chiffre5>[A-Za-z0-9]|[.]){5,200})";
    const PORT: &'static str = "(?P<port>[0-9]{1,5})";

    const NOM_DOMAINE: &'static str = "([A-Za-z0-9]{5,20})@([A-Za-z0-9.]{5,200})";
    const ID_DOMAINE: &'static str = "([0-9]{1,5})@([A-Za-z0-9.]{5,200})";
    const DEST_DOMAINE: &'static str = "#?([A-Za-z0-9]{5,20})@(?P<dest_domain>[A-Za-z0-9.]{5,200})";
    const MESSAGE_INTERNE: &'static str = "(?P<message_interne>[\\x20-\\xFF]{1,500})";


    /// Cette fonction permet de récupérer les informations d'un message ECHO et de les stocker dans une HashMap.
    ///
    pub fn get_echo_map(msg: &str) -> Option<HashMap<String, String>> {
        let regex = Regex::new(&format!(                                                             // Création de l'expression régulière pour récupérer les informations de l'écho
            "^ECHO{}{}{}{}{}",
            Protocol::ESP,
            Protocol::PORT,
            Protocol::ESP,
            Protocol::DOMAINE,
            Protocol::CRLF
        )).ok()?;

        let captures = regex.captures(msg)?;                                                      // Récupération des informations en utilisant l'expression régulière

        let mut map = HashMap::new();                                                       // Stockage des informations dans une HashMap
        map.insert(
            "domain".to_owned(),
            captures.name("domaine2").unwrap().as_str().to_owned(),
        );
        map.insert(
            "port".to_owned(),
            captures.name("port").unwrap().as_str().to_owned(),
        );

        Some(map)                                                                                               // Retourne l'option contenant la HashMap des informations de l'écho
    }



    /// La fonction get_receiving_domain permet d'extraire le nom de domaine de destination à partir d'un message reçu.
    /// Elle prend en paramètre une référence à une chaîne de caractères msg, qui représente le message reçu.
    ///
    pub fn get_receiving_domain(msg: &str) -> Option<String> {
        let regex = Regex::new(&format!(                                                               // Création de l'expression régulière pour récupérer les informations du send
            "^SEND{}{}{}{}{}{}{}{}{}",
            Protocol::ESP,
            Protocol::ID_DOMAINE,
            Protocol::ESP,
            Protocol::NOM_DOMAINE,
            Protocol::ESP,
            Protocol::DEST_DOMAINE,
            Protocol::ESP,
            Protocol::MESSAGE_INTERNE,
            Protocol::CRLF
        )).ok()?;

        let captures = regex.captures(msg)?;                                                        // Récupération des informations en utilisant l'expression régulière

        let destination_domain = captures.name("dest_domain").unwrap().as_str().to_owned();                // Stockage du résultat du groupement dans une variable

        Some(destination_domain)                                                                                 // Retourne l'option contenant le domain de destination
    }
}
