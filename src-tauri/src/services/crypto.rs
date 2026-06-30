use aes::Aes128;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use cbc::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use num_bigint::BigUint;
use num_traits::Num;
use rand::Rng;

type Aes128CbcEnc = cbc::Encryptor<Aes128>;

const PRESET_KEY: &[u8; 16] = b"0CoJUm6Qyw8W8jud";
const IV: &[u8; 16] = b"0102030405060708";
const RSA_EXPONENT: &str = "010001";
const RSA_MODULUS: &str = "00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615bb7b725152b3ab17a876aea8a5aa76d2e417629ec4ee341f56135fccf695280104e0312ecbda92557c93870114af6c9d05c4f7f0c3685b7a46bee255932575cce10b424d813cfe4875d3e82047b97ddef52741d546b8e289dc6935b3ece0462db0a22b8e7";

const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn random_key(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

fn aes_cbc_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // Allocate buffer with space for PKCS7 padding (up to one extra block)
    let block_size = 16usize;
    let pad_len = block_size - (data.len() % block_size);
    let total = data.len() + pad_len;
    let mut buf = vec![0u8; total];
    buf[..data.len()].copy_from_slice(data);
    let encryptor = Aes128CbcEnc::new(key.into(), IV.into());
    let encrypted = encryptor
        .encrypt_padded_mut::<Pkcs7>(&mut buf, data.len())
        .unwrap();
    encrypted.to_vec()
}

fn rsa_encrypt(secret_key: &str) -> String {
    let reversed: Vec<u8> = secret_key.bytes().rev().collect();
    let message = BigUint::from_bytes_be(&reversed);
    let modulus = BigUint::from_str_radix(RSA_MODULUS, 16).unwrap();
    let exponent = BigUint::from_str_radix(RSA_EXPONENT, 16).unwrap();
    let result = message.modpow(&exponent, &modulus);
    format!("{:0256x}", result)
}

/// Encrypt request body using Netease's weapi scheme.
/// Returns (params, encSecKey) as form-encoded fields.
pub fn weapi_encrypt(text: &str) -> (String, String) {
    let secret = random_key(16);
    let first = aes_cbc_encrypt(text.as_bytes(), PRESET_KEY);
    let second = aes_cbc_encrypt(&first, secret.as_bytes());
    let params = BASE64.encode(&second);
    let enc_sec_key = rsa_encrypt(&secret);
    (params, enc_sec_key)
}
