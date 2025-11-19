# rust-frame
this is rust frame,use actix web &amp;&amp; seaorm &amp;&amp; pg

## ✨ 新功能: 扫码登录

已完成扫码登录功能的实现，支持网页端生成二维码，App扫码确认登录。

### 📖 文档
- [扫码登录实现指南](./QR_LOGIN_GUIDE.md) - 详细的API文档和前端集成示例
- [架构设计文档](./QR_LOGIN_ARCHITECTURE.md) - 系统架构和技术设计
- [App端示例代码](./scaffold/examples/app_scanner_example.md) - Flutter/React Native/Android示例

### 🚀 快速开始

1. **运行数据库迁移**:
```bash
psql -U postgres -d your_database -f scaffold/migrations/001_create_qr_login_sessions.sql
```

2. **启动服务**:
```bash
cd scaffold
cargo run -- --pgsql-url "postgres://postgres:postgres@localhost:5432/postgres" --backend-port 8080
```

3. **测试网页端**:
打开 `scaffold/examples/qr_login_test.html` 在浏览器中测试

### 📡 API接口

- `POST /qr-login/generate` - 生成二维码
- `GET /qr-login/status/{session_id}` - 查询登录状态  
- `POST /qr-login/confirm` - App端确认登录
- `POST /qr-login/reject` - App端拒绝登录

---

## 🛠️ 开发工具

sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/postgres 
