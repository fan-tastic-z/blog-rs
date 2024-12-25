use argon2::{password_hash::SaltString, Argon2, PasswordHasher};

use super::Error;

pub fn compute_password_hash(password: &str) -> Result<String, Error> {
    let slat = SaltString::generate(&mut rand::thread_rng());
    let password = Argon2::default()
        .hash_password(password.as_bytes(), &slat)?
        .to_string();
    Ok(password)
}
