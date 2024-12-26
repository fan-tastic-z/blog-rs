use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use super::Error;

pub fn compute_password_hash(password: &str) -> Result<String, Error> {
    let slat = SaltString::generate(&mut rand::thread_rng());
    let password = Argon2::default()
        .hash_password(password.as_bytes(), &slat)?
        .to_string();
    Ok(password)
}

pub fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Error> {
    let expected_password_hash = PasswordHash::new(&expected_password_hash)?;
    Argon2::default().verify_password(password_candidate.as_bytes(), &expected_password_hash)?;
    Ok(())
}
