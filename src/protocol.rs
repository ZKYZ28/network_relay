use regex::Regex;
use std::collections::HashMap;

pub struct Protocol {}

impl Protocol {
    const CRLF: &'static str = "\\r\\n";
    const ESP: &'static str = "\\x20";
    const DOMAINE: &'static str = "(?P<domaine2>(?P<lettre_chiffre5>[A-Za-z0-9]|[.]){5,200})";
    const PORT: &'static str = "(?P<port>[0-9]{1,5})";

    const NOM_DOMAINE: &'static str = "([A-Za-z0-9]{5,20})@([A-Za-z0-9.]{5,200})";
    const ID_DOMAINE: &'static str = "([0-9]{1,5})@([A-Za-z0-9.]{5,200})";
    const DEST_DOMAINE: &'static str = "([A-Za-z0-9]{5,20})@(?P<dest_domain>[A-Za-z0-9.]{5,200})";
    const MESSAGE_INTERNE: &'static str = "(?P<message_interne>[\\x20-\\xFF]{1,500})";

    pub fn get_echo_map(msg: &str) -> Option<HashMap<String, String>> {
        let regex = Regex::new(&format!(
            "^ECHO{}{}{}{}{}",
            Protocol::ESP,
            Protocol::PORT,
            Protocol::ESP,
            Protocol::DOMAINE,
            Protocol::CRLF
        )).ok()?;

        let captures = regex.captures(msg)?;

        let mut map = HashMap::new();
        map.insert(
            "domain".to_owned(),
            captures.name("domaine2").unwrap().as_str().to_owned(),
        );
        map.insert(
            "port".to_owned(),
            captures.name("port").unwrap().as_str().to_owned(),
        );

        Some(map)
    }


    pub fn get_receiving_domain(msg: &str) -> Option<String> {
        let regex = Regex::new(&format!(
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

        let captures = regex.captures(msg)?;

        let destination_domain = captures.name("dest_domain").unwrap().as_str().to_owned();

        println!("Domaine de destination : {}", destination_domain);

        Some(destination_domain)
    }
}
