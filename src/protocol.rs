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
    const DOMAINE: &'static str = "(?P<domaine2>(?P<lettre_chiffre5>[A-Za-z0-9]|[.]){5,200})";
    const PORT: &'static str = "(?P<port>[0-9]{1,5})";
    const MESSAGE: &'static str = "(?P<message>[\\x20-\\xFF]{1,200})";
    const MESSAGE_INTERNE: &'static str = "(?P<message_interne>[\\x20-\\xFF]{1,500})";
    const NOM_UTILISATEUR: &'static str = "(?P<nom_utilisateur>(?P<lettre_chiffre>[A-Za-z0-9]){5,20})";
    const TAG: &'static str = "#(?P<tag>(?P<lettre_chiffre4>[A-Za-z0-9]){5,20})";
    //const NOM_DOMAINE: &'static str = "(?P<nom_domaine>(?P<nom_utilisateur>[A-Za-z0-9]{5,20})@(?P<domaine>(?P<lettre_chiffre>[A-Za-z0-9]|[.]){5,200}))";
    const NOM_DOMAINE: &'static str = "(?P<nom_utilisateur>(?P<lettre_chiffre1>[A-Za-z0-9]){5,20})@(?P<domaine_nom>(?P<lettre_chiffre3>[A-Za-z0-9]|[.]){5,200})";
    const TAG_DOMAINE: &'static str = "(?P<tag_domaine>(?P<tag>#[A-Za-z0-9]{5,20})@(?P<domaine_tag>(?P<tag_lettre_chiffre2>[A-Za-z0-9]|[.]){5,200}))";
    const ID_DOMAINE: &'static str = "(?P<id_domaine>[0-9]{1,5}@(?P<domaine1>(?P<lettre_chiffre6>[A-Za-z0-9]|[.]){5,200}))";
    const SEND : &'static str = "^SEND\\x20(?P<id_domaine>[0-9]{1,5}@(\\P{Cc}|\\p{M}|\\p{N}|\\p{P}|\\p{S}){5,200})((#(?P<tag>[A-Za-z0-9]{5,20})@(?P<domaine_tag>(\\P{Cc}|\\p{M}|\\p{N}|\\p{P}|\\p{S}){5,200}))|((?P<nom_utilisateur>(?P<lettre_chiffre1>[A-Za-z0-9]){5,20})@(?P<domaine_nom>(\\P{Cc}|\\p{M}|\\p{N}|\\p{P}|\\p{S}){5,200})))(\\x20(?P<message_interne>(\\P{Cc}|\\p{M}|\\p{N}|\\p{P}|\\p{S}){1,500}))?\\r\\n";


    pub fn from_message(message: &str) -> Option<String> {
        //let send_regex = Regex::new(&format!("^SEND{}{}(({})|({})){}{}(({})|({})){}{}", Protocol::ESP, Protocol::ID_DOMAINE, Protocol::NOM_DOMAINE, Protocol::TAG_DOMAINE, Protocol::ESP, Protocol::MESSAGE_INTERNE, Protocol::ADRESSE_EMAIL, Protocol::ADRESSE_EMAIL, Protocol::ESP, Protocol::CRLF));
       // let send_regex = Regex::new(&format!("^SEND{}{}{}(({})|({})){}{}(({})|({})){}{}", Protocol::ESP, Protocol::ID_DOMAINE, Protocol::ESP, Protocol::NOM_DOMAINE, Protocol::TAG_DOMAINE, Protocol::ESP, Protocol::MESSAGE_INTERNE, Protocol::ADRESSE_EMAIL, Protocol::ADRESSE_EMAIL, Protocol::ESP, Protocol::CRLF));
        let send_regex = Regex::new(&format!("^SEND{}{}{}(({})|({})){}{}{}", Protocol::ESP, Protocol::ID_DOMAINE, Protocol::ESP, Protocol::NOM_DOMAINE, Protocol::TAG_DOMAINE, Protocol::ESP, Protocol::MESSAGE_INTERNE, Protocol::CRLF)).ok()?;
        let echo_regex = Regex::new(&format!("^ECHO{}{}{}{}{}", Protocol::ESP, Protocol::PORT, Protocol::ESP, Protocol::DOMAINE, Protocol::CRLF)).ok()?;
        if send_regex.is_match(message) {
            Some("SEND".to_owned())
        } else if echo_regex.is_match(message) {
            Some("ECHO".to_owned())
        } else {
            None
        }
    }

}
