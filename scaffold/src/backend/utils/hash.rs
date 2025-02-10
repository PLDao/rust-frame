use core::fmt;

use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use sha2::{Digest, Sha256};

pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let bytes = hasher.finalize().to_vec();
    hex::encode(bytes)
}

pub fn hash_str(data: &str) -> String {
    hash_bytes(data.as_bytes())
}

pub trait Hash: fmt::Display {
    fn hash(&self) -> String {
        hash_bytes(self.to_string().as_bytes())
    }
    fn b_hash(&self) -> Result<String> {
        Ok(hash(self.to_string(), DEFAULT_COST)?)
    }
    fn b_verify(&self, password: &str) -> Result<bool> {
        Ok(verify(password, &self.to_string())?)
    }
}

impl Hash for String {}

impl Hash for &str {}

// password hash

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_hash() {
        assert_eq!(
            "+86 15219903461".hash(),
            "040b75454a8a0072958d6a05f34da8bdb03546eb006d9dc546e87d89b6c0c9c5"
        );
        assert_eq!(
            "1286735237@qq.com".hash(),
            "6c3c8821e6986f2934d6bd1e7a158689d4fc98752207942b7e7ea3409cb6c725"
        );
        let hash = "+86 13292080358".hash();
        println!("{}", hash);
        let hash = "+86 13818658534".hash();
        println!("{}", hash);
    }
    #[test]
    fn test_hash_password() {
        let password = "123456";
        let hash = hash_password(password);
        println!("{:?}", hash);
        assert!(verify_password(password, &hash.unwrap()));
    }
}
