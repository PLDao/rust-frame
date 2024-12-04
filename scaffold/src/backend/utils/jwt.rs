use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::env;

/// 定义 JWT 的负载
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户 ID
    pub role: String, // 用户角色
    pub exp: usize,  // 过期时间戳 (Unix 时间)
}

fn load_keys() -> (EncodingKey, DecodingKey) {

    let private_key_str = env::var("JWT_PRIVATE_KEY")
        .expect("JWT_PRIVATE_KEY is not set in .env");
    let public_key_str = env::var("JWT_PUBLIC_KEY")
        .expect("JWT_PUBLIC_KEY is not set in .env");

    let private_key_bytes = bs58::decode(private_key_str).into_vec().expect("Failed to decode jwt private key");
    let public_key_bytes = bs58::decode(public_key_str).into_vec().expect("Failed to decode jwt public key");

    // 创建签名和解码密钥
    let encoding_key = EncodingKey::from_ed_der(&private_key_bytes);
    let decoding_key = DecodingKey::from_ed_der(&public_key_bytes);

    (encoding_key, decoding_key)
}

/// 生成 JWT
pub fn create_jwt(user_id: &str, role: &str, expiration: usize) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp: expiration,
    };

    let header = Header::new(Algorithm::EdDSA);
    let (encoding_key, _) = load_keys();

    encode(&header, &claims, &encoding_key).expect("Failed to create JWT")
}

/// 验证 JWT
pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let (_, decoding_key) = load_keys();
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.validate_exp = true;

    decode::<Claims>(token, &decoding_key, &validation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_create_jwt() {
        let user_id = "user123";
        let role = "admin";
        let expiration = Utc::now().timestamp() as usize + 60 * 60 * 24;

        let token = create_jwt(user_id, role, expiration);
        assert!(!token.is_empty());

        let claims = verify_jwt(&token).expect("Failed to verify valid JWT");
        assert_eq!(claims.claims.sub, user_id);
        assert_eq!(claims.claims.role, role);
        assert!(claims.claims.exp > Utc::now().timestamp() as usize);
    }

}