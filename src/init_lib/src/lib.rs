use crate::ckeys::CKeys;
use diesel::prelude::*;
use dotenv::dotenv;
use rand::rngs::OsRng;
use redis::{Connection as RedisConnection, RedisResult};
use rsa::{RSAPrivateKey, RSAPublicKey};
use std::env;

pub mod ckeys {
    use crate::crypto_module_gen;

    pub struct CKeys {
        pub osrng: rand::rngs::OsRng,
        pub private_key: rsa::RSAPrivateKey,
        pub public_key: rsa::RSAPublicKey,
    }
    impl CKeys {
        pub fn new(
            osrng_data: rand::rngs::OsRng,
            private_key_data: rsa::RSAPrivateKey,
            public_key_data: rsa::RSAPublicKey,
        ) -> CKeys {
            CKeys {
                osrng: osrng_data,
                private_key: private_key_data,
                public_key: public_key_data,
            }
        }
        pub fn flush() -> CKeys {
            crypto_module_gen()
        }
    }
}

pub fn init_db_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn init_redis_db_connection() -> RedisResult<RedisConnection> {
    return redis::Client::open("redis://127.0.0.1:6379/")?.get_connection();
}

//TODO: add args for generate new or update
pub fn crypto_module_gen() -> CKeys {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key_t = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key_t = RSAPublicKey::from(&private_key_t);
    return ckeys::CKeys::new(rng, private_key_t, public_key_t);
}
