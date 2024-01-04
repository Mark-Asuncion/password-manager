#[cfg(test)]
mod tests {
    use crate::{m_openssl, file};

    #[test]
    fn genkey_encrypt_decrypt() {
        let unecrypted_text = "Hello World".to_string();
        let key = m_openssl::gen_random_bytes().unwrap();
        let iv = m_openssl::gen_random_bytes().unwrap();
        let encrypted = m_openssl::encrypt(&key, &iv, &unecrypted_text).unwrap();

        let decrypted_text = m_openssl::decrypt(&key, &iv, &encrypted).unwrap();
        assert_eq!(unecrypted_text, decrypted_text);
    }

    #[test]
    fn open_encrypt_decrypt() {
        let mut p = file::udata_path().unwrap();
        p.push(file::constants::F_KEY);
        let mut p = file::open(p.as_path()).unwrap();
        let k = m_openssl::open_key(&mut p);

        let mut p = file::udata_path().unwrap();
        p.push(file::constants::F_ACCOUNT);
        let accounts = file::read_csv(p.as_path()).unwrap();

        for acc in accounts {
            acc.get_pass_decrypted(&k.0, &k.1).unwrap();
        }
    }
}
