use crate::account::Account;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Global {
    pub key_iv: Mutex<(Vec<u8>, Vec<u8>)>,
    pub accounts: Mutex<Vec<Account>>
}
