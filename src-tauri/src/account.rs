use crate::crypt::{self, KeyIv};

pub type Accounts = Vec<Account>;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Account {
    pub username: String,
    pub link:     String,
    pub password: String
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct QueryAccount {
    pub username: Option<String>,
    pub link:     Option<String>,
    pub password: Option<String>
}

impl QueryAccount {
    pub fn to_account(&self) -> Account {
        Account {
            username: self.username.clone().unwrap_or_default(),
            link: self.link.clone().unwrap_or_default(),
            password: self.password.clone().unwrap_or_default()
        }
    }

    pub fn is_match(&self, other: &Account) -> bool {
        // match should be 2 because 1 field can only be edited at a time
        let mut match_c = 0;
        if let Some(v) = &self.username {
            if v == &other.username {
                match_c+=1;
            }
        }
        if let Some(v) = &self.link {
            if v == &other.link {
                match_c+=1;
            }
        }
        if let Some(v) = &self.password {
            if v == &other.password {
                match_c+=1;
            }
        }
        match_c >= 2
    }
}


impl Default for Account {
    fn default() -> Self {
        Self {
            username: Default::default(),
            link: Default::default(),
            password: Default::default()
        }
    }
}

impl Account {
    pub fn is_empty(&self) -> bool {
        self.username.is_empty()
        || self.link.is_empty()
        || self.password.is_empty()
    }

    pub fn set_ignore_empty(&mut self, update: Account) {
        if !update.username.is_empty() {
            self.username = update.username;
        }

        if !update.link.is_empty() {
            self.link = update.link;
        }

        if !update.password.is_empty() {
            self.password = update.password;
        }
    }

    pub fn as_encrypted(&self, keyiv: &KeyIv) -> Self {
        let pass = crypt::encrypt(keyiv, &self.password);
        if let Err(e) = pass {
            dbg!(e);
            return Self::default();
        }
        Self {
            username: self.username.clone(),
            link: self.link.clone(),
            password: pass.unwrap()
        }
    }

    pub fn as_decrypted(&self, keyiv: &KeyIv) -> Self {
        let pass = crypt::decrypt(keyiv, &self.password);
        if let Err(e) = pass {
            dbg!(e);
            return Self::default();
        }
        Self {
            username: self.username.clone(),
            link: self.link.clone(),
            password: pass.unwrap()
        }
    }
}
