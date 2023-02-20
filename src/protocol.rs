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
    //const NOM_DOMAINE: &'static str = "(?P<nom_utilisateur>(?P<lettre_chiffre1>[A-Za-z0-9]){5,20})@(?P<domaine_nom>(?P<lettre_chiffre3>[A-Za-z0-9]|[.]){5,200})";
    const NOM_DOMAINE: &'static str = "([A-Za-z0-9]{5,20})@([A-Za-z0-9.]{5,200})";
    const TAG_DOMAINE: &'static str = "(?P<tag_domaine>(?P<tag>#[A-Za-z0-9]{5,20})@(?P<domaine_tag>(?P<tag_lettre_chiffre2>[A-Za-z0-9]|[.]){5,200}))";
    const ID_DOMAINE: &'static str = "(?P<id_domaine>[0-9]{1,5}@(?P<domaine1>(?P<lettre_chiffre6>[A-Za-z0-9]|[.]){5,200}))";

    pub fn from_message(message: &str) -> Option<String> {
        let echo_regex = Regex::new(&format!("^ECHO{}{}{}{}{}", Protocol::ESP, Protocol::PORT, Protocol::ESP, Protocol::DOMAINE, Protocol::CRLF)).ok()?;
        let send_regex = Regex::new(&format!("^SEND{}{}{}{}{}(({})|({})){}{}{}", Protocol::ESP, Protocol::ID_DOMAINE, Protocol::ESP, Protocol::NOM_DOMAINE, Protocol::ESP,  Protocol::NOM_DOMAINE, Protocol::TAG_DOMAINE, Protocol::ESP, Protocol::MESSAGE_INTERNE, Protocol::CRLF)).ok()?;

        if send_regex.is_match(message){
            Some("SEND".to_owned())
        } else if echo_regex.is_match(message) {
            Some("ECHO".to_owned())
        } else {
            None
        }
    }

    pub fn decomposer(string: &str, typ: &str) -> Result<Vec<String>, &'static str> {
        let regex_str = match typ {
            "echo" => format!("^ECHO{}{}{}{}{}", Protocol::ESP, Protocol::PORT, Protocol::ESP, Protocol::DOMAINE, Protocol::CRLF),
            "send" => format!("^SEND{}{}{}{}{}(({})|({})){}{}{}", Protocol::ESP, Protocol::ID_DOMAINE, Protocol::ESP, Protocol::NOM_DOMAINE, Protocol::ESP,  "([A-Za-z0-9]{5,20})@([A-Za-z0-9.]{5,200})", Protocol::TAG_DOMAINE, Protocol::ESP, Protocol::MESSAGE_INTERNE, Protocol::CRLF),
            _ => return Err("Type de décomposition non pris en charge"),
        };
        let re = match Regex::new(&regex_str) {
            Ok(re) => re,
            Err(_) => return Err("Erreur lors de la création de l'expression régulière"),
        };

        let captures = match re.captures(string) {
            Some(captures) => captures,
            None => return Err("Aucune capture trouvée"),
        };

        let mut groupes = vec![];

        for i in 1..captures.len() {
            if let Some(capture) = captures.get(i) {
                groupes.push(capture.as_str().to_string());
            }
        }

        Ok(groupes)
    }
}
