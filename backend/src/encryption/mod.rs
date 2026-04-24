pub mod plaintext;

pub use plaintext::PlaintextEncryption;

pub struct EncryptedContents {
    pub first_name: String,
    pub last_name: String,
    pub pronouns: String,
}

pub trait Encryption: Send + Sync {
    fn encrypt(&mut self, contents: &EncryptedContents) -> String;
    fn decrypt(&mut self, text: &str) -> Option<EncryptedContents>;
}