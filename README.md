# rust-frame
this is rust frame,use actix web &amp;&amp; seaorm &amp;&amp; pg

## ✨ 新功能: 扫码登录

**核心特性：后端直接生成PNG二维码图片，前端零依赖！**

已完成扫码登录功能的实现，支持网页端生成二维码，App扫码确认登录。

### 📖 文档

| 文档 | 说明 |
|------|------|
| **[QR_LOGIN.md](./QR_LOGIN.md)** | 📖 完整文档（架构图、流程图、快速开始、前端/App集成） |
| **[QR_LOGIN_API.md](./QR_LOGIN_API.md)** | 📡 API详细文档（接口定义、请求响应、示例代码） |
| [CHANGES.md](./CHANGES.md) | 📝 改动清单 |

### 🚀 快速开始

```bash
# 1. 运行数据库迁移
psql -U postgres -d your_database -f scaffold/migrations/001_create_qr_login_sessions.sql

# 2. 启动服务
cd scaffold
cargo run --release -- --backend-port 8080

# 3. 测试
curl -X POST http://localhost:8080/qr-login/generate \
  -H "Content-Type: application/json" \
  -d '{}' | jq .

# 4. 打开测试页面
open scaffold/examples/qr_login_simple.html
```

### 📡 API端点

| 端点 | 方法 | 调用方 | 说明 |
|------|------|--------|------|
| `/qr-login/generate` | POST | Web | 生成二维码（**含PNG图片**） |
| `/qr-login/status/{id}` | GET | Web | 查询登录状态 |
| `/qr-login/confirm` | POST | App | 确认登录 |
| `/qr-login/reject` | POST | App | 拒绝登录 |

### 💻 前端集成（仅需3行代码）

```javascript
const res = await fetch('http://localhost:8080/qr-login/generate', {
    method: 'POST', headers: {'Content-Type': 'application/json'}, body: '{}'
});
const data = await res.json();
document.getElementById('qrImage').src = data.qr_image; // ✨ 直接显示，无需任何库！
```

---

## 🛠️ 开发工具

sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5432/postgres 
