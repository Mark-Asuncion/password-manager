#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Account {
    pub username: String,
    pub link: String,
    pub password: String
}

impl Account {
    pub fn new(username: &str, link: &str, password: &str) -> Account {
        Account {
            username: username.trim().to_string(),
            link: link.trim().to_string(),
            password: password.trim().to_string(),
        }
    }

    pub fn get_pass_decrypted(&self, key: &[u8], iv: &[u8]) -> Result<String, String> {
        use crate::m_openssl;
        match m_openssl::decrypt(key, iv, &self.password) {
            Ok(v) => {
                return Ok(v);
            },
            Err(e) => return Err(e.to_string()),
        }
    }
}
