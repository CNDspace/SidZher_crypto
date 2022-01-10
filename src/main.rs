// use crypto_module;
// use database;
// use init_lib;
use net_module;

fn main() {
    net_module::connect()

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
