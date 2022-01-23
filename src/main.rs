use base64;
// use crypto_module;
use database;
use init_lib;
use init_lib::ckeys::CKeys;
use rsa::PublicKeyEncoding;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// TODO: Need create struct  and forget variables between first and second step

struct User {
    _username: Option<String>,
    _crypt_info: Option<CKeys>,
    // public_key: Option<RSAPublicKey>,
    // private_key: Option<RSAPrivateKey>,
    // encrypted: Option<Vec<u8>>,
}

impl User {
    pub fn new(
        username_data: Option<String>,
        crypt_info_data: Option<CKeys>,
        // public_key_data: Option<RSAPublicKey>,
        // private_key_data: Option<RSAPrivateKey>,
        // encrypted_data: Option<Vec<u8>>,
    ) -> User {
        User {
            _username: username_data,
            _crypt_info: crypt_info_data
            // public_key: public_key_data,
            // private_key: private_key_data,
            // encrypted: encrypted_data,
        }
    }
    pub fn _default() -> User {
        User {
            _username: None,
            _crypt_info: None,
            // public_key: None,
            // private_key: None,
            // encrypted: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transit {
    step: u8,
    req_type: String,
    user: String,
    data: String,
}

fn _check_username(username_struct: &Option<String>, user_json: &String) -> bool {
    if let Some(username) = username_struct {
        return if user_json.eq(username) { true } else { false };
    }
    false
}

fn parse_data(req_data: &str) -> Result<String> {
    return match serde_json::from_str(req_data) {
        Ok(parsed) => {
            let mut request_json: Transit = parsed;
            // let mut user_struct = User::default();
            match request_json.step {
                1 => {
                    request_json.step = 2;
                    let value_for_user = database::check_user_redis(&request_json.user);
                    if !value_for_user.eq("ERROR") {
                        let encrypt_keys = init_lib::crypto_module_gen();
                        request_json.data =
                            base64::encode(encrypt_keys.public_key.to_pkcs8().unwrap());
                        User::new(
                            Option::from(request_json.user.clone()),
                            Option::from(encrypt_keys),
                            // Option::from(encrypt_keys.public_key.clone()),
                            // Option::from(encrypt_keys.private_key.clone()),
                            // Option::from(crypto_module::encrypt_data(
                            //     &mut encrypt_keys,
                            //     value_for_user.as_bytes(),
                            //     )),
                        );
                        CKeys::flush();
                    }
                }
                3 => {}
                _ => {}
            }
            let response_json = serde_json::to_string_pretty(&request_json);
            response_json
        }
        Err(e) => {
            let error_struct_parse: Transit = Transit {
                step: 0,
                req_type: "".to_string(),
                user: "".to_string(),
                data: format!("Error: {}", e.to_string()),
            };
            let response_json = serde_json::to_string_pretty(&error_struct_parse);
            response_json
        }
    };
}

fn send_data(mut stream: &TcpStream, request_message: String) {
    let response = format!("{}", request_message);
    stream.write(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer).unwrap();

    let serealized_data = parse_data(buffer.as_str());

    match serealized_data {
        Ok(parsed) => send_data(&stream, parsed),
        Err(e) => send_data(&stream, e.to_string()),
    }

    stream.flush().unwrap();
}

fn main() {
    let _connection = init_lib::init_redis_db_connection().unwrap();
    let listener = TcpListener::bind("127.0.0.1:5141");
    if let Ok(listener_ok) = listener {
        for stream in listener_ok.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    } else {
        println!("Error bind listener!")
    }

    // // let _connect = init_lib::init_db_connection();
    // let user: String = String::from("Not_kek");
    // let password: String = String::from("Kek_password");
    // // // register_user(user, password);
    // database::check_user(&user, &password);
    //
    // // Init Crypto
    // let mut crypto_config = init_lib::crypto_module_gen();
    //
    // // Encrypt
    // let data = b"Think_test";
    // let enc_data = crypto_module::encrypt_data(&mut crypto_config, data);
    // println!("{}", String::from_utf8_lossy(&*enc_data));
    //
    // // Decrypt
    // let dec_data = crypto_module::decrypt_data(&mut crypto_config, enc_data);
    // println!("{}", String::from_utf8_lossy(&*dec_data));
}
