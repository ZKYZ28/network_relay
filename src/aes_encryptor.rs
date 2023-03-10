use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::{Aead, generic_array::{GenericArray}};
use aes_gcm::aead::rand_core::RngCore;
use base64;
use base64::{Engine};
use base64::engine::general_purpose;


pub struct AesEncryptor;


/// La classe AesEncryptor est responsable du chiffrement et déchiffrement de messages en utilisatant AES-256-GCM
///
impl AesEncryptor {


    /// La méthode encrypt prend en entrée une clé de chiffrement encodée en base64 et une chaîne de caractères à chiffrer.
    /// Elle renvoie une chaîne de caractères encodée en base64 qui représente le résultat du chiffrement AES 256.
    ///
    pub fn encrypt(key_base64: &str, message: String) -> String {
        let key_decoded = general_purpose::STANDARD.decode(key_base64).unwrap();                                                            // Décodage de la clé de chiffrement à partir de son encodage en base64.

        let mut key = [0u8; 32];                                                                                                                   // Initialisation d'une clé de chiffrement de 32 octets à partir des 32 premiers octets de la clé décodée.
        key.copy_from_slice(&key_decoded[..32]);

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));                                                           // Initialisation d'un chiffreur AES-256-GCM avec la clé de chiffrement.

        let mut iv = [0u8; 12];                                                                                                                    // Génération d'un vecteur d'initialisation aléatoire de 12 octets.
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut iv);

        let iv = GenericArray::from_slice(&iv);                                                                                    // Conversion d'vecteur d'initialisation en une structure GenericArray.

        let ciphertext = cipher.encrypt(iv, message.as_bytes()).expect("L'encryption du message à échoué.");                   // Chiffrement du message en utilisant le vecteur d'initialisation et la clé de chiffrement.

        let mut result = Vec::new();                                                                                                              // Création d'un vecteur qui contient le vecteur d'initialisation suivi du texte chiffré.
        result.extend_from_slice(&iv);
        result.extend_from_slice(&ciphertext);

        general_purpose::STANDARD.encode(&result)                                                                                                    // Encodage du vecteur résultant en base64 et retour de la chaîne encodée.
    }




    /// La méthode decrypt prend en entrée une clé de déchiffrement sous forme de base64 et un texte chiffré,
    /// et renvoie le texte déchiffré en cas de succès ou un message d'erreur en cas d'échec.
    ///
    pub fn decrypt(key_base64: &str, ciphertext: &Vec<u8>) -> Result<String, String> {

        let key_decoded = general_purpose::STANDARD.decode(key_base64).unwrap();                                                             // Décodage la clé de déchiffrement à partir de son encodage en base64.

        let mut key = [0u8; 32];                                                                                                                    // Initialisation d'une clé de déchiffrement de 32 octets à partir des 32 premiers octets de la clé décodée.
        key.copy_from_slice(&key_decoded[..32]);

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));                                                            // Initialisation d'un déchiffreur AES-256-GCM avec la clé de déchiffrement

        let iv = GenericArray::from_slice(&ciphertext[..12]);                                                                      // Extraction du vecteur d'initialisation du texte chiffré.

        let ciphertext = &ciphertext[12..];                                                                                                          // Extraction du vecteur d'initialisation du texte chiffré.

        match cipher.decrypt(iv, &*ciphertext) {                                                                                                      // Déchiffrement du texte en utilisant le vecteur d'initialisation et la clé de déchiffrement.
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(s) => Ok(s),
                Err(_) => Err("Erreur de décryptage : La chaine UTF-8 est invalide.".to_owned()),
            },
            Err(_) => Err("Erreur de décryptage : La clé est incorrecte ou le message à été altéré.".to_owned()),
        }
    }
}
