use argon2::{
    password_hash::{
        errors::Error, rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
pub struct Argon2PasswordHash {}

impl Argon2PasswordHash {
    pub fn hash_password(password: String) -> Result<String, Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password = password.as_bytes();
        let password_hash = argon2.hash_password(password, &salt);
        match password_hash {
            Ok(password_hash) => {
                return Ok(password_hash.to_string());
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
    pub fn verify_password(password: String, hashed_password: String) -> bool {
        let argon2 = Argon2::default();
        let password = password.as_bytes();
        let parsed_hash = PasswordHash::new(&hashed_password);
        if let Ok(parsed_hash) = parsed_hash {
            return argon2.verify_password(password, &parsed_hash).is_ok();
        }
        return false;
    }
}
