# 扫码登录功能 - 快速开始

## ✨ 核心特性

**后端直接生成二维码PNG图片，前端零依赖！**

## 🚀 快速开始（3步）

### 1. 运行数据库迁移

```bash
psql -U postgres -d your_db -f scaffold/migrations/001_create_qr_login_sessions.sql
```

### 2. 启动服务

```bash
cd scaffold
cargo run -- --pgsql-url "postgres://postgres:postgres@localhost/your_db" --backend-port 8080
```

### 3. 测试

```bash
# 方式1：打开测试页面
open scaffold/examples/qr_login_simple.html

# 方式2：命令行测试
curl -X POST http://localhost:8080/qr-login/generate \
  -H "Content-Type: application/json" \
  -d '{"client_info":"test"}' | jq .
```

## 📡 API接口

### 1. 生成二维码（网页端）

```http
POST /qr-login/generate
Content-Type: application/json

{"client_info": "web"}
```

**响应：**
```json
{
  "session_id": "uuid",
  "qr_image": "data:image/png;base64,...",  ← PNG图片！
  "qr_data": "{...}",
  "expires_in": 300
}
```

### 2. 查询状态（网页端轮询）

```http
GET /qr-login/status/{session_id}
```

**响应：**
```json
{
  "status": "confirmed",
  "web_token": "eyJhbGc...",
  "message": "Login successful"
}
```

### 3. 确认登录（App端）

```http
POST /qr-login/confirm
Content-Type: application/json

{
  "session_id": "uuid",
  "app_token": "eyJhbGc..."  ← App的JWT token
}
```

## 💻 前端代码（超简单）

### HTML
```html
<img id="qrImage" alt="二维码">

<script>
async function generateQR() {
    const res = await fetch('http://localhost:8080/qr-login/generate', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({})
    });
    const data = await res.json();
    
    // 直接显示，无需任何库！
    document.getElementById('qrImage').src = data.qr_image;
}
</script>
```

### React
```jsx
function QRLogin() {
  const [qrImage, setQrImage] = useState('');
  
  const generateQR = async () => {
    const res = await fetch('http://localhost:8080/qr-login/generate', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({})
    });
    const data = await res.json();
    setQrImage(data.qr_image);
  };
  
  return <img src={qrImage} alt="二维码" />;
}
```

### Vue
```vue
<template>
  <img :src="qrImage" alt="二维码">
</template>

<script setup>
import { ref } from 'vue'

const qrImage = ref('')

const generateQR = async () => {
  const res = await fetch('http://localhost:8080/qr-login/generate', {
    method: 'POST',
    headers: {'Content-Type': 'application/json'},
    body: JSON.stringify({})
  })
  const data = await res.json()
  qrImage.value = data.qr_image
}
</script>
```

## 📱 App端集成

### 1. 扫描二维码获取session_id

```dart
// Flutter示例
import 'package:mobile_scanner/mobile_scanner.dart';

MobileScanner(
  onDetect: (capture) {
    final qrData = jsonDecode(capture.barcodes.first.rawValue!);
    final sessionId = qrData['session_id'];
    // 显示确认对话框
    showConfirmDialog(sessionId);
  },
)
```

### 2. 用户确认后调用API

```dart
Future<void> confirmLogin(String sessionId) async {
  final appToken = await getStoredToken(); // 获取App的JWT
  
  final response = await http.post(
    Uri.parse('http://api.example.com/qr-login/confirm'),
    headers: {'Content-Type': 'application/json'},
    body: jsonEncode({
      'session_id': sessionId,
      'app_token': appToken,
    }),
  );
  
  if (response.statusCode == 200) {
    showSuccess('登录确认成功');
  }
}
```

## 🔄 完整流程

```
网页端                     后端                      App端
  │                        │                         │
  │──1. POST /generate────→│                         │
  │←─── qr_image ──────────│                         │
  │                        │                         │
  │  显示二维码             │                         │
  │                        │                         │
  │  开始轮询状态           │                         │
  │──2. GET /status/{id}──→│                         │
  │←─── pending ───────────│                         │
  │                        │                         │
  │                        │←─3. 扫码获取session_id──│
  │                        │                         │
  │                        │                    显示确认对话框
  │                        │                         │
  │                        │←─4. POST /confirm──────│
  │                        │   (session_id+app_token)│
  │                        │                         │
  │                        │  验证app_token           │
  │                        │  生成web_token           │
  │                        │  更新状态为confirmed     │
  │                        │                         │
  │                        │─── success ────────────→│
  │                        │                         │
  │──5. GET /status/{id}──→│                         │
  │←─── confirmed ─────────│                         │
  │    + web_token         │                         │
  │                        │                         │
  保存token，登录成功       │                         │
```

## 📦 依赖说明

### 后端（Rust）
```toml
uuid = "1.10"        # 生成session_id
qrcode = "0.14"      # 生成二维码
image = "0.25"       # 图片处理
base64 = "0.22"      # Base64编码
```

### 前端
**无需任何依赖！** 只需要浏览器原生支持的 `<img>` 标签

## 📚 文档

- **快速开始：** 本文件
- **API详细文档：** [QR_IMAGE_API.md](./QR_IMAGE_API.md)
- **实现说明：** [QR_LOGIN_IMPLEMENTATION.md](./QR_LOGIN_IMPLEMENTATION.md)
- **改动清单：** [CHANGES.md](./CHANGES.md)
- **测试页面：** [scaffold/examples/qr_login_simple.html](./scaffold/examples/qr_login_simple.html)

## ✅ 优势总结

| 特性 | 传统方式 | 现在 |
|------|---------|------|
| 前端依赖 | 需要qrcode.js | ✅ 零依赖 |
| 代码复杂度 | ~20行 | ✅ 3行 |
| 性能 | 前端计算 | ✅ 后端计算 |
| 样式统一 | 难以控制 | ✅ 后端统一 |
| 维护成本 | 高 | ✅ 低 |

## 🎉 完成状态

- [x] 数据库模型和迁移
- [x] 生成二维码API（含图片）
- [x] 查询状态API
- [x] 确认登录API
- [x] 前端测试页面
- [x] App示例代码
- [x] 完整文档
- [x] 编译通过

**立即可用！**

---

**最后更新：** 2024-11-19  
**状态：** ✅ 生产就绪
