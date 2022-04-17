/*
    Библиотека используется для дешифрования и хеширования полученных данных
*/

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

// функция используется для дешифрования данных при аутентификации и авторизации пользователя
pub fn decrypt_and_compare_data_auth(
    crypt_info: &mut CKeys,
    enc_data: Vec<u8>,
    username: String,
) -> bool {
    // создаём соединение с базой данных
    let mut db_connection = init_redis_db_connection().unwrap();
    // создаём padding
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    // дешифруем полученные данные
    match crypt_info.private_key.decrypt(padding, &enc_data) {
        Ok(decrypted) => {
            // если дешифровали данные успешно,
            // то пытаемся получить пароль пользователя используя его имя
            let db_try_get_value: RedisResult<Option<String>> =
                db_connection.get(username.as_str());
            match db_try_get_value {
                Ok(db_value) => {
                    if let Some(password) = db_value {
                        // если данные получили успешно, то пытаемся сверить пароль пользователя
                        let decrypted_u8: &[u8] = &decrypted;
                        // функция verify используется для проверки хеша пароля пользователя
                        // с полученным дешифрованным паролем пользователя
                        match verify(&decrypted_u8, password.as_str()) {
                            // если верификация прошла успешно, возвращаем true, иначе false
                            Ok(condition) => return if condition { true } else { false },
                            // произошла ошибка - возвращаем false
                            Err(_) => false,
                        };
                    };
                    // если не смогли получить пароль пользователя - возвращаем false
                    return false;
                }
                // если возникла ошибка при получении данных из базы - возвращаем  false
                Err(_) => false,
            }
        }
        // если возникла ошибка при дешифровке, возвращаем false
        Err(_) => false,
    };
    // при любой иной ошибке также возвращаем false
    return false;
}

// функция используется для дешифровавки данных при регистрации пользователя
pub fn decrypt_and_compare_data_reg(
    crypt_info: &mut CKeys,
    enc_data: Vec<u8>,
    username: String,
) -> bool {
    // создаём padding
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    match crypt_info.private_key.decrypt(padding, &enc_data) {
        Ok(decrypted) => {
            // если данные дешифровали успешно, то создаём хеш на основе полученного пароля
            let hashed_value = hash(decrypted, DEFAULT_COST).unwrap();
            // вызываем функцию для регистрации пользователя в базе данных
            if database::register_user_redis(username, hashed_value) {
                // если данные занесены в базу данных успешно, то возвращаем true
                return true;
            }
            // если данные не получилось занести в базу данные - возвращаем true
            return false;
        }
        // если дешифровать данные не получилось - возвращаем false
        Err(_) => false,
    };
    // при любой иной ошибке также возвращаем false
    return false;
}
