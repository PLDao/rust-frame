use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
use once_cell::sync::Lazy;
use std::env;

// 使用 Lazy 定义全局的 SMTP 客户端
static MAILER: Lazy<SmtpTransport> = Lazy::new(|| {
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not set");
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");

    SmtpTransport::relay(&smtp_server)
        .expect("Failed to create SMTP relay")
        .credentials(Credentials::new(smtp_username, smtp_password))
        .build()
});

// 获取全局的 SMTP 客户端
pub fn get_mailer() -> &'static SmtpTransport {
    &MAILER
}
