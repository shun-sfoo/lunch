use jsonwebtoken as jwt;

use crate::model::Claims;

const SECRET: &[u8] = b"deadbeef";

pub trait ClaimService {
    fn generate(&self) -> String;
}

impl ClaimService for Claims {
    fn generate(&self) -> String {
        let key = jwt::EncodingKey::from_secret(SECRET);
        let token = jwt::encode(&jwt::Header::default(), &self, &key).unwrap();
        token
    }
}
