pub type Accounts = Vec<Account>;

pub struct Account {
    username: String,
    link:     String,
    password: String
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
