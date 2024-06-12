use std::fs::File;
use std::io::Read;
use openssl::base64;
use openssl::rand::rand_bytes;
use openssl::error::ErrorStack;
use openssl::symm::{Cipher, Mode};
use openssl::symm::Crypter;

pub struct KeyIv {
    pub key: Vec<u8>,
    pub iv: Vec<u8>
}

impl KeyIv {
    pub fn from(buf: Vec<u8>) -> Self {
        Self {
            key: Vec::from(&buf[0..16]),
            iv: Vec::from(&buf[16..32])
        }
    }

    pub fn is_empty(&self) -> bool {
        self.key.is_empty() || self.iv.is_empty()
    }
}

impl Default for KeyIv {
    fn default() -> Self {
        Self { key: Default::default(), iv: Default::default() }
    }
}

pub fn gen_32_bytes() -> Result< Vec<u8>, ErrorStack > {
    // 256 bits
    let mut buf = [0u8; 32];
    rand_bytes(&mut buf)?;
    Ok(Vec::from(buf))
}

pub fn read_key(file: &mut File) -> KeyIv {
    let mut buf = [0u8; 32];
    if let Err(e) = file.read_exact(&mut buf) {
        dbg!(e);
        return KeyIv::default();
    }
    KeyIv {
        key: Vec::from(&buf[0..16]),
        iv: Vec::from(&buf[16..32])
    }
}

pub fn encrypt(keyiv: &KeyIv, data: &str) -> Result<String, ErrorStack> {
    let key = keyiv.key.as_slice();
    let iv = keyiv.iv.as_slice();
    let mut crypter = Crypter::new( Cipher::aes_128_cbc(), Mode::Encrypt, key, Some(iv) )?;
    let block_size = Cipher::aes_128_cbc().block_size();
    let mut out = vec![0; data.len() + block_size];
    let mut count = crypter.update(data.as_bytes(), &mut out)?;
    count += crypter.finalize(&mut out[count..])?;
    out.truncate(count);
    Ok(base64::encode_block(out.as_slice()))
}

pub fn decrypt(keyiv: &KeyIv, data_b64: &str) -> Result<String, ErrorStack> {
    let key = keyiv.key.as_slice();
    let iv = keyiv.iv.as_slice();
    let data = base64::decode_block(data_b64)?;
    let mut crypter = Crypter::new( Cipher::aes_128_cbc(), Mode::Decrypt, key, Some(iv) )?;
    let block_size = Cipher::aes_128_cbc().block_size();
    let mut out = vec![0; data.len() + block_size];
    let mut count = crypter.update(data.as_slice(), &mut out)?;
    count += crypter.finalize(&mut out[count..])?;
    out.truncate(count);
    Ok(std::str::from_utf8(out.as_slice()).unwrap().to_string())
}
