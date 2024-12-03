use jsonwebtoken::{encode, decode, Algorithm, EncodingKey, DecodingKey, Header, Validation, TokenData};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use ring::rand::SystemRandom;

/// 定义 JWT 的负载
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // 用户 ID
    pub role: String,        // 用户角色
    pub exp: usize,          // 过期时间戳 (Unix 时间)
}

fn new_keys () -> (EncodingKey, DecodingKey) {
    // 生成新的密钥对
    let rng = SystemRandom::new();
    let doc = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

    // 私钥和公钥
    let encoding_key = EncodingKey::from_ed_der(doc.as_ref());
    let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
    let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());
    (encoding_key, decoding_key)
}
fn load_keys() -> (EncodingKey, DecodingKey) {
    // 从文件中加载密钥
    // ...
    new_keys()
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
pub fn verify_jwt(token: &str) -> TokenData<Claims> {
    let (_, decoding_key) = load_keys();
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.validate_exp = true;

    decode::<Claims>(token, &decoding_key, &validation).expect("Failed to verify JWT")
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

        let claims = verify_jwt(&token);
        assert_eq!(claims.claims.sub, user_id);
        assert_eq!(claims.claims.role, role);
        assert!(claims.claims.exp > Utc::now().timestamp() as usize);
    }

    #[test]
    fn test_invalid_jwt() {
        let invalid_token = "invalid.token.value";
        let result = std::panic::catch_unwind(|| {
            verify_jwt(invalid_token);
        });
        assert!(result.is_err());
    }
}