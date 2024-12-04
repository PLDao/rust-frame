use ring::signature::{Ed25519KeyPair, KeyPair};
use ring::rand::SystemRandom;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::Write;

// 定义结构用于序列化密钥对
#[derive(Serialize, Deserialize, Debug)]
struct KeyPairJson {
    private_key: String,
    public_key: String,
}

// 生成新的密钥对并存储为 JSON
pub fn new_keys() -> (String, String) {
    // 生成新的密钥对
    let rng = SystemRandom::new();
    let doc = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

    // 提取私钥和公钥
    let private_key = bs58::encode(doc.as_ref()).into_string();
    let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
    let public_key = bs58::encode(pair.public_key().as_ref()).into_string();

    // 创建 JSON 对象
    let key_pair_json = KeyPairJson {
        private_key: private_key.clone(),
        public_key: public_key.clone(),
    };

    // 保存到 JSON 文件
    let json_string = serde_json::to_string_pretty(&key_pair_json).unwrap();
    let mut file = File::create("keypair.json").unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    (private_key, public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_keys() {
        let (private_key, public_key) = new_keys();
        println!("Private Key: {:?}", private_key);
        println!("Public Key: {:?}", public_key);

        // 检查文件是否正确保存
        let file_content = std::fs::read_to_string("keypair.json").unwrap();
        println!("Saved JSON: {}", file_content);
    }
}