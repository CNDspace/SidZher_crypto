// use crypto_module;
// use database;
// use init_lib;
// use net_module;

// use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5141");
    if let Ok(listener_ok) = listener {
        for stream in listener_ok.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
    } else {
        println!("Error bind!")
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
