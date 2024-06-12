#[cfg(test)]
mod tests {
    use crate::crypt::{gen_32_bytes, encrypt, decrypt};

    #[test]
    fn _t_crypt() {
        let keyiv = gen_32_bytes().unwrap();
        let keyiv = (Vec::from(&keyiv[0..16]), Vec::from(&keyiv[16..32]));
        let data = "password123";

        let encryptedb64 = encrypt(&keyiv.0, &keyiv.1, &data).unwrap();

        let decryptedb = decrypt(&keyiv.0, &keyiv.1, &encryptedb64).unwrap();

        assert_eq!(data, decryptedb);
    }
}
