extern crate bcrypt;
use bcrypt::verify;
use init_lib::ckeys::CKeys;
use redis::{Commands, Connection as RedisConnection};
use rsa::{PaddingScheme, PublicKey};

pub fn encrypt_data(crypt_info: &mut CKeys, data: &[u8]) -> Vec<u8> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = crypt_info
        .public_key
        .encrypt(&mut crypt_info.osrng, padding, &data[..])
        .expect("Failed to encrypt data!");
    return enc_data;
}

pub fn decrypt_and_compare_data(
    crypt_info: &mut CKeys,
    enc_data: Vec<u8>,
    username: String,
    db_connection: &mut RedisConnection,
) -> bool {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    match crypt_info.private_key.decrypt(padding, &enc_data) {
        Ok(decrypted) => {
            let db_value: Option<String> = db_connection.get(username.as_str()).unwrap();
            if let Some(password) = db_value {
                return match verify(decrypted, password.as_str()) {
                    Ok(_) => true,
                    Err(_) => false,
                };
                // return verify(decrypted, password.as_str()).unwrap();
            }
            return false;
        }
        Err(_) => false,
    };
    return false;
}
