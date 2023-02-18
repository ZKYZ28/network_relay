use regex::Regex;

pub struct Protocol {}

impl Protocol {
    const CHIFFRE: &'static str = "[0-9]";
    const LETTRE: &'static str = "[A-Za-z]";
    const LETTRE_CHIFFRE: &'static str = "(?P<lettre_chiffre>[A-Za-z0-9])";
    const CARACTERE_IMPRIMABLE: &'static str = "[\\x20-\\xFF]";
    const CRLF: &'static str = "\\r\\n";
    const SYMBOLE: &'static str = "[!-\\/:-@\\[-`{-~]";
    const ESP: &'static str = "\\x20";
    const DOMAINE: &'static str = "(?P<domaine>(?P<lettre_chiffre>[A-Za-z0-9]|[.]){5,200})";
    const PORT: &'static str = "(?P<port>[0-9]{1,5})";
    const MESSAGE: &'static str = "(?P<message>[\\x20-\\xFF]{1,200})";
    const MESSAGE_INTERNE: &'static str = "(?P<message_interne>[\\x20-\\xFF]{1,500})";
    const NOM_UTILISATEUR: &'static str = "(?P<nom_utilisateur>(?P<lettre_chiffre>[A-Za-z0-9]){5,20})";
    const TAG: &'static str = "#(?P<tag>(?P<lettre_chiffre>[A-Za-z0-9]){5,20})";
    const NOM_DOMAINE: &'static str = "(?P<nom_domaine>(?P<nom_utilisateur>[A-Za-z0-9]{5,20})@(?P<domaine>(?P<lettre_chiffre>[A-Za-z0-9]|[.]){5,200}))";
    const TAG_DOMAINE: &'static str = "(?P<tag_domaine>(?P<tag>#[A-Za-z0-9]{5,20})@(?P<domaine>(?P<lettre_chiffre>[A-Za-z0-9]|[.]){5,200}))";
    const ID_DOMAINE: &'static str = "(?P<id_domaine>[0-9]{1,5}@(?P<domaine>(?P<lettre_chiffre>[A-Za-z0-9]|[.]){5,200}))";

    pub fn from_message(message: &str) -> Option<ProtocolMessage> {
        let send_regex = Regex::new(&format!("^SEND{}{}{}({}|{}){}{}{}", ESP, ID_DOMAINE, ESP, NOM_DOMAINE, TAG_DOMAINE, ESP, MESSAGE_INTERNE, CRLF)).unwrap();
        let echo_regex = Regex::new(&format!("^ECHO{}{}{}{}{}", ESP, PORT, ESP, DOMAINE, CRLF)).unwrap();


        if send_regex.is_match(message) {
            let captures = send_regex.captures(message).unwrap();
            let id_domaine = captures.name("id_domaine").unwrap().as_str().to_owned();
            let nom_domaine = captures.name("nom_domaine").or(captures.name("tag_domaine")).unwrap().as_str().to_owned();
            Some(ProtocolMessage::Send(id_domaine, nom_domaine))
        } else if echo_regex.is_match(message) {
            let captures = echo_regex.captures(message).unwrap();
            let port = captures.name("port").unwrap().as_str().parse().unwrap();
            let domaine = captures.name("domaine").unwrap().as_str().to_owned();
            Some(ProtocolMessage::Echo(port, domaine))
        } else {
            None
        }
    }
}