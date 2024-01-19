use serde_json::{Value, json};

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

    pub fn as_json(&self, id: usize) -> Value {
        json!({
            "id": id,
            "username": self.username,
            "link": self.link,
            "password": self.password
        })
    }

    pub fn as_json_decrypted(&self, id: usize, key: &[u8], iv: &[u8]) -> Result<Value, String> {
        let password = self.get_pass_decrypted(key, iv)?;
        Ok(json!({
            "id": id,
            "username": self.username,
            "link": self.link,
            "password": password
        }))
    }

    pub fn csv_header() -> [String; 3] {
        [String::from("username"), String::from("link"), String::from("password")]
    }

    pub fn json(id: usize, username: &str, link: &str, password: &str) -> Value {
        json!({
            "id": Value::Number(id.into()),
            "username": Value::String(username.into()),
            "link": Value::String(link.into()),
            "password": Value::String(password.into())
        })
    }
}
