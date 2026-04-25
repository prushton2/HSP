pub mod plaintext;

pub use plaintext::PlaintextEncryption;

pub struct EncryptedContents {
    pub first_name: String,
    pub last_name: String,
    pub pronouns: String,
}

impl Default for EncryptedContents {
    fn default() -> Self {
        Self {
            first_name: "".to_string(),
            last_name: "".to_string(),
            pronouns: "".to_string()
        }
    }
}

pub trait Encryption: Send + Sync {
    fn encrypt(&mut self, contents: &EncryptedContents) -> String;
    fn decrypt(&mut self, text: &str) -> EncryptedContents;
    fn hash(&mut self, text: &str, salt: &str) -> String;
    fn random_string(&mut self, len: usize) -> String;
}