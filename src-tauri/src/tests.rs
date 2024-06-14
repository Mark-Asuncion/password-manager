#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use crate::{crypt::{gen_32_bytes, encrypt, decrypt}, state::MState, account::{Account, TAccounts}};

    #[test]
    fn _t_crypt() {
        let keyiv = gen_32_bytes().unwrap();
        let keyiv = (Vec::from(&keyiv[0..16]), Vec::from(&keyiv[16..32]));
        let data = "password123";

        let encryptedb64 = encrypt(&keyiv.0, &keyiv.1, &data).unwrap();

        let decryptedb = decrypt(&keyiv.0, &keyiv.1, &encryptedb64).unwrap();

        assert_eq!(data, decryptedb);
    }

    #[test]
    fn _t_accs() {
        // let mut state = MState::default();
        // let acc = Account {
        //     username: "username".into(),
        //     link: "link".into(),
        //     password: "password".into()
        // };
        //
        // state.accs_push(acc);
        //
        // let acc = state.accs_get(0);
        //
    }
}
