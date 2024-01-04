use crate::account::Account;

pub static mut KEY_N_IV: (Vec<u8>, Vec<u8>) = (vec![], vec![]);
pub static mut ACCOUNTS: Vec<Account> = vec![];
