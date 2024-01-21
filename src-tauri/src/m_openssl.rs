use std::fs::File;
use std::io::Read;
use openssl::base64;
use openssl::rand::rand_bytes;
use openssl::error::ErrorStack;
use openssl::symm::{Cipher, Mode};
use openssl::symm::Crypter;


pub fn gen_random_bytes() -> Result<Vec<u8>, ErrorStack> {
    // 128 bits
    let mut buf = [0u8; 16];
    rand_bytes(&mut buf)?;
    Ok(Vec::from(buf))
}

// key, iv
pub fn open_key(file: &mut File) -> (Vec<u8>, Vec<u8>) {
    let mut buf = [0u8; 32];
    if let Err(e) = file.read_exact(&mut buf) {
        println!("{}",e.to_string());
        return (vec![], vec![]);
    }
    (Vec::from(&buf[0..16]), Vec::from(&buf[16..32]))
}

pub fn encrypt(key: &[u8], iv: &[u8], data: &str) -> Result<String, ErrorStack> {
    let mut crypter = Crypter::new( Cipher::aes_128_cbc(), Mode::Encrypt, key.as_ref(), Some(iv.as_ref()) )?;
    let block_size = Cipher::aes_128_cbc().block_size();
    let mut out = vec![0; data.len() + block_size];
    let mut count = crypter.update(data.as_bytes(), &mut out)?;
    count += crypter.finalize(&mut out[count..])?;
    out.truncate(count);
    Ok(base64::encode_block(out.as_slice()))
}

pub fn decrypt(key: &[u8], iv: &[u8], data_b64: &str) -> Result<String, ErrorStack> {
    let data = base64::decode_block(data_b64)?;
    let mut crypter = Crypter::new( Cipher::aes_128_cbc(), Mode::Decrypt, key.as_ref(), Some(iv.as_ref()) )?;
    let block_size = Cipher::aes_128_cbc().block_size();
    let mut out = vec![0; data.len() + block_size];
    let mut count = crypter.update(data.as_slice(), &mut out)?;
    count += crypter.finalize(&mut out[count..])?;
    out.truncate(count);
    Ok(std::str::from_utf8(out.as_slice()).unwrap().to_string())
}
