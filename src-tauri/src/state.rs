use std::sync::Mutex;
use crate::account::Accounts;

pub struct MState {
    accounts: Mutex< Accounts > // for now
}

impl Default for MState {
    fn default() -> Self {
        Self { accounts: Default::default() }
    }
}
