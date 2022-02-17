use base64;
use crypto_module;
use database;
use init_lib;
use init_lib::ckeys::CKeys;
use redis::Connection as RedisConnection;
use rsa::PublicKeyPemEncoding;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::prelude::*;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::u8;

const FIRST_STEP: u8 = 1;
const SECOND_STEP: u8 = 2;
const THIRD_STEP: u8 = 3;
const FOUR_STEP: u8 = 4;

struct User {
    username: Option<String>,
    crypt_info: Option<CKeys>,
}

impl User {
    pub fn new(username_data: Option<String>, crypt_info_data: Option<CKeys>) -> User {
        User {
            username: username_data,
            crypt_info: crypt_info_data,
        }
    }
    pub fn default() -> User {
        User {
            username: None,
            crypt_info: None,
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

impl Transit {
    pub fn error(req_type_data: String) -> Transit {
        Transit {
            step: 0,
            req_type: req_type_data,
            user: String::from("Not_exist!"),
            data: String::from("ERROR"),
        }
    }
}

fn check_username(username_struct: &Option<String>, user_json: &String) -> bool {
    if let Some(username) = username_struct {
        return if user_json.eq(username) { true } else { false };
    }
    false
}

fn parse_data(
    req_data: &str,
    mut user_struct: User,
    db_connection: &mut RedisConnection,
) -> Result<String> {
    return match serde_json::from_str(req_data) {
        Ok(parsed) => {
            let mut request_json: Transit = parsed;
            match request_json.step {
                FIRST_STEP => {
                    CKeys::flush();
                    request_json.step = SECOND_STEP;
                    let value_for_user = database::check_user_redis(&request_json.user);
                    if !value_for_user.eq("ERROR") {
                        let encrypt_keys = init_lib::crypto_module_gen();
                        request_json.data = encrypt_keys.public_key.to_pem_pkcs8().unwrap();
                        // base64::encode(encrypt_keys.public_key.to_pkcs8().unwrap());
                        user_struct = User::new(
                            Option::from(request_json.user.clone()),
                            Option::from(encrypt_keys),
                        );
                        let _read_struct = user_struct; // Костыль ебаный, нужно от него избавиться
                    } else {
                        request_json = Transit::error(request_json.req_type);
                    };
                }
                THIRD_STEP => {
                    request_json.step = FOUR_STEP;
                    if check_username(&user_struct.username, &request_json.user) {
                        if let Some(mut encrypt_key) = user_struct.crypt_info {
                            let json_data = request_json.data.clone();
                            let username = request_json.user.clone();
                            if crypto_module::decrypt_and_compare_data(
                                &mut encrypt_key,
                                base64::decode(json_data).unwrap(),
                                username,
                                db_connection,
                            ) {
                                request_json.data = "OK".to_string()
                            } else {
                                request_json.data = "WRONG PASSWORD!".to_string();
                            }
                        };
                    } else {
                        request_json.data = "ERROR CHECK USERNAME!".to_string()
                    }
                }
                _ => {}
            }
            let response_json = serde_json::to_string(&request_json);
            response_json
        }
        Err(e) => {
            let error_struct_parse: Transit = Transit {
                step: 0,
                req_type: "".to_string(),
                user: "".to_string(),
                data: format!("Error: {}", e.to_string()),
            };
            let response_json = serde_json::to_string(&error_struct_parse);
            response_json
        }
    };
}

fn send_data(mut stream: &TcpStream, request_message: String) {
    let response = format!("{}", request_message);
    stream.write(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream, db_connection: &mut RedisConnection) {
    let mut buffer = [0 as u8; 2048];

    stream
        .set_nonblocking(true)
        .expect("Failed to set nonblocking mode");

    loop {
        match stream.read(&mut buffer) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }

    let string_buffer = String::from_utf8_lossy(&buffer)
        .trim_matches(char::from(0))
        .to_string();

    println!("Received from front:\n{}", string_buffer);

    let serialized_data = parse_data(string_buffer.as_str(), User::default(), db_connection);

    match serialized_data {
        Ok(parsed) => send_data(&stream, parsed),
        Err(e) => send_data(&stream, e.to_string()),
    }
    stream.flush().unwrap();
}

fn main() {
    match init_lib::init_redis_db_connection() {
        Ok(mut connect) => {
            let listener = TcpListener::bind("127.0.0.1:5141");
            if let Ok(listener_ok) = listener {
                for stream in listener_ok.incoming() {
                    let stream = stream.unwrap();
                    handle_connection(stream, &mut connect);
                }
            } else {
                println!("Error bind listener!")
            }
        }
        Err(error) => println!("Failed connect to database!\n{}", error),
    }
}
