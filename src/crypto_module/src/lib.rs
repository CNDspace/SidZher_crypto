use init_lib::ckeys::CKeys;
use rsa::{PaddingScheme, PublicKey};

pub fn encrypt_data(crypt_info: &mut CKeys, data: &[u8]) -> Vec<u8> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = crypt_info
        .public_key
        .encrypt(&mut crypt_info.osrng, padding, &data[..])
        .expect("Failed to encrypt data!");
    return enc_data;
}

pub fn decrypt_data(crypt_info: &mut CKeys, enc_data: Vec<u8>) -> Vec<u8> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = crypt_info
        .private_key
        .decrypt(padding, &enc_data)
        .expect("Failed to decrypt");
    return dec_data;
}
