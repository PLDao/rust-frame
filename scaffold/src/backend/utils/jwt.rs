use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use std::env;
use crate::backend::models::sea_orm_active_enums::RoleType;

/// 定义 JWT 的负载
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String, // 用户 ID
    pub username: String, // 用户链上地址
    pub role: Option<RoleType>,
    pub exp: usize,  // 过期时间戳 (Unix 时间)
}

fn load_keys() -> (EncodingKey, DecodingKey) {
    let private_key_str = env::var("JWT_PRIVATE_KEY")
        .unwrap_or_else(|_| "GD8M1Qm17WXoukx8QqqfvYtM9zCSR83R1yZSuMbZ9JJtwayF39rabnwd26jMsLLw8LkHLT31x1TLZYT6ypKpPMgW1apMno2LrB4UBL56pZff5DukXkTf".to_string());
    let public_key_str = env::var("JWT_PUBLIC_KEY")
        .unwrap_or_else(|_| "Hnh4C68tZtSHurUuLzNt265EwyTyy1i6Qdg5Umjo995F".to_string());

    let private_key_bytes = bs58::decode(private_key_str).into_vec().expect("Failed to decode jwt private key");
    let public_key_bytes = bs58::decode(public_key_str).into_vec().expect("Failed to decode jwt public key");

    let encoding_key = EncodingKey::from_ed_der(&private_key_bytes);
    let decoding_key = DecodingKey::from_ed_der(&public_key_bytes);

    (encoding_key, decoding_key)
}

/// 生成 JWT
pub fn create_jwt(new_user: &Claims) -> String {
    let header = Header::new(Algorithm::EdDSA);
    let (encoding_key, _) = load_keys();

    encode(&header, &new_user, &encoding_key).expect("Failed to create JWT")
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
        let new_user = Claims {
            user_id: "user123".to_string(),
            username: "test_user".to_string(),
            role: Some(RoleType::Admin),
            exp: Utc::now().timestamp() as usize + 60 * 60 * 24,
        };

        let token = create_jwt(&new_user);
        assert!(!token.is_empty());

        let claims = verify_jwt(&token).expect("Failed to verify valid JWT");
        assert_eq!(claims.claims.user_id, new_user.user_id);
        assert_eq!(claims.claims.role, new_user.role);
        assert_eq!(claims.claims.username, new_user.username);
        assert!(claims.claims.exp > Utc::now().timestamp() as usize);
    }
}