use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, TokenData,
    Validation,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Error;

const JWT_ALGORITHM: Algorithm = Algorithm::HS512;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub pid: String,
    exp: u64,
    pub claims: Option<Value>,
}

#[derive(Debug)]
pub struct JWT {
    secret: String,
    algorithm: Algorithm,
}

impl JWT {
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
            algorithm: JWT_ALGORITHM,
        }
    }

    pub fn algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    pub fn generate_token(
        &self,
        expiration: &u64,
        pid: String,
        claims: Option<Value>,
    ) -> Result<String, Error> {
        let exp = get_current_timestamp().saturating_add(*expiration);

        let claims = UserClaims { pid, exp, claims };

        let token = encode(
            &Header::new(self.algorithm),
            &claims,
            &EncodingKey::from_base64_secret(&self.secret)?,
        )?;
        Ok(token)
    }

    pub fn validate(&self, token: &str) -> Result<TokenData<UserClaims>, Error> {
        let mut validate = Validation::new(self.algorithm);
        validate.leeway = 0;

        let res = decode::<UserClaims>(
            token,
            &DecodingKey::from_base64_secret(&self.secret)?,
            &validate,
        )?;
        Ok(res)
    }
}
