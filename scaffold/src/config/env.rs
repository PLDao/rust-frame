use dotenv::dotenv;
use std::env;
use tracing::error;

/// 初始化环境变量
pub fn load_env() {
    dotenv().ok(); // 加载 .env 文件，忽略加载失败
    verify_essential_env_vars(); // 验证必要的环境变量是否存在
}

/// 验证必要的环境变量
fn verify_essential_env_vars() {
    // 必要的环境变量列表
    let required_vars = vec![
        "JWT_PRIVATE_KEY",
        "JWT_PUBLIC_KEY",
    ];

    for var in required_vars {
        if env::var(var).is_err() {
            error!("❌ Missing required environment variable: {}", var);
            panic!("❌ Missing required environment variable: {}", var);
        }
    }
}