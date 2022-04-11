use bcrypt::{hash, verify, DEFAULT_COST};
use init_lib::ckeys::CKeys;
use init_lib::init_redis_db_connection;
use redis::{Commands, RedisResult};
use rsa::PaddingScheme;
// use rsa::{PaddingScheme, PublicKey};
// extern crate bcrypt;

// pub fn encrypt_data(crypt_info: &mut CKeys, data: &[u8]) -> Vec<u8> {
//     let padding = PaddingScheme::new_pkcs1v15_encrypt();
//     let enc_data = crypt_info
//         .public_key
//         .encrypt(&mut crypt_info.osrng, padding, &data[..])
//         .expect("Failed to encrypt data!");
//     return enc_data;
// }

pub fn decrypt_and_compare_data_auth(
    crypt_info: &mut CKeys,
    enc_data: Vec<u8>,
    username: String,
) -> bool {
    let mut db_connection = init_redis_db_connection().unwrap();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    match crypt_info.private_key.decrypt(padding, &enc_data) {
        Ok(decrypted) => {
            let db_try_get_value: RedisResult<Option<String>> =
                db_connection.get(username.as_str());
            match db_try_get_value {
                Ok(db_value) => {
                    if let Some(password) = db_value {
                        let decrypted_u8: &[u8] = &decrypted;
                        match verify(&decrypted_u8, password.as_str()) {
                            Ok(condition) => return if condition { true } else { false },
                            Err(_) => false,
                        };
                    };
                    return false;
                }
                Err(_) => false,
            }
        }
        Err(_) => false,
    };
    return false;
}

pub fn decrypt_and_compare_data_reg(
    crypt_info: &mut CKeys,
    enc_data: Vec<u8>,
    username: String,
) -> bool {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    match crypt_info.private_key.decrypt(padding, &enc_data) {
        Ok(decrypted) => {
            let hashed_value = hash(decrypted, DEFAULT_COST).unwrap();
            if database::register_user_redis(username, hashed_value) {
                return true;
            }
            return false;
        }
        Err(_) => false,
    };
    return false;
}
