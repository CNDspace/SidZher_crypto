use crate::ckeys::CKeys;
use diesel::prelude::*;
use dotenv::dotenv;
use rand::rngs::OsRng;
use rsa::{/*PaddingScheme, PublicKey, */ RSAPrivateKey, RSAPublicKey};
use std::env;

pub mod ckeys {
    pub struct CKeys {
        pub private_key: rsa::RSAPrivateKey,
        pub public_key: rsa::RSAPublicKey,
    }
    impl CKeys {
        // A public constructor method
        pub fn new(
            private_key_data: rsa::RSAPrivateKey,
            public_key_data: rsa::RSAPublicKey,
        ) -> CKeys {
            CKeys {
                private_key: private_key_data,
                public_key: public_key_data,
            }
        }
    }
}

pub fn init_db_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//TODO: NOT TESTED!!!
pub fn crypto_module_gen() -> CKeys {
    dotenv().ok();

    let mut rng = OsRng;
    let bits = env::var("KEY_LEN")
        .expect("KEY_LEN must be set")
        .parse::<usize>()
        .expect("Cannot parse type");
    let private_key_t = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key_t = RSAPublicKey::from(&private_key_t);
    return ckeys::CKeys::new(private_key_t, public_key_t);
}
