pub struct PlaintextEncryption {}

impl PlaintextEncryption {
    pub fn new() -> Self {
        Self {}
    }
}

impl super::Encryption for PlaintextEncryption {
    fn encrypt(&mut self, contents: &super::EncryptedContents) -> String {
        format!("{}||{}||{}", contents.first_name, contents.last_name, contents.pronouns)
    }

    fn decrypt(&mut self, text: &str) -> Option<super::EncryptedContents> {
        let vec: Vec<&str> = text.split("||").collect();
        if vec.len() != 3 {
            return None
        }

        Some(super::EncryptedContents {
            first_name: vec[0].to_string(),
            last_name: vec[1].to_string(),
            pronouns: vec[2].to_string(),
        })
    }
}