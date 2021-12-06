use init_lib;
use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};

fn main() {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RSAPublicKey::from(&private_key);

    let _connect = init_lib::main();

    // Encrypt
    let data = b"Think_test";
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key
        .encrypt(&mut rng, padding, &data[..])
        .expect("failed to encrypt");
    println!("{}", String::from_utf8_lossy(&*enc_data));

    // Decrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key
        .decrypt(padding, &enc_data)
        .expect("failed to decrypt");
    println!("{}", String::from_utf8_lossy(&*dec_data));
}
