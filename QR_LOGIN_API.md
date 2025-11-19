# 扫码登录 API 文档

> REST API 规范文档 v1.0

## 📋 目录

- [API概览](#api概览)
- [认证机制](#认证机制)
- [API端点](#api端点)
- [数据结构](#数据结构)
- [错误码](#错误码)
- [示例代码](#示例代码)

---

## 🌐 API概览

### Base URL

```
Production:  https://api.yourdomain.com
Development: http://localhost:8080
```

### 协议

- **传输协议：** HTTP/HTTPS
- **数据格式：** JSON
- **字符编码：** UTF-8
- **请求方法：** GET, POST

---

## 🔐 认证机制

### JWT Token结构

```json
{
  "user_id": "123",
  "username": "0x1234...5678",
  "role": "user",
  "exp": 1732012345
}
```

### Token类型

| Token类型 | 用途 | 有效期 | 签名算法 |
|-----------|------|--------|----------|
| `app_token` | App端用户身份 | 30天 | EdDSA |
| `web_token` | Web端登录凭证 | 7天 | EdDSA |

---

## 📡 API端点

### 1. 生成二维码

**网页端调用，生成登录二维码**

#### 请求

```http
POST /qr-login/generate
Content-Type: application/json
```

**请求体：**
```json
{
  "client_info": "web"  // 可选，客户端信息
}
```

#### 响应

**成功 (200 OK):**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "qr_image": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA...",
  "qr_data": "{\"session_id\":\"...\",\"action\":\"login\",\"expires_at\":1732012345}",
  "expires_in": 300
}
```

**字段说明：**

| 字段 | 类型 | 说明 | 示例 |
|------|------|------|------|
| `session_id` | String | 会话唯一标识符 | UUID v4 |
| `qr_image` | String | Base64编码的PNG图片（Data URI格式） | `data:image/png;base64,...` |
| `qr_data` | String | 二维码原始数据（JSON字符串） | 包含session_id和expires_at |
| `expires_in` | Number | 过期时间（秒） | `300`（5分钟） |

**失败 (500 Internal Server Error):**
```json
{
  "error": "Failed to create QR session: ..."
}
```

#### 示例

```bash
# curl
curl -X POST http://localhost:8080/qr-login/generate \
  -H "Content-Type: application/json" \
  -d '{"client_info":"web"}' | jq .

# JavaScript
const response = await fetch('http://localhost:8080/qr-login/generate', {
  method: 'POST',
  headers: {'Content-Type': 'application/json'},
  body: JSON.stringify({client_info: 'web'})
});
const data = await response.json();
console.log(data.session_id);
```

---

### 2. 查询登录状态

**网页端轮询调用，检查用户是否完成扫码**

#### 请求

```http
GET /qr-login/status/{session_id}
```

**路径参数：**

| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `session_id` | String | 是 | 会话ID |

#### 响应

**状态：pending（等待扫码）**
```json
{
  "status": "pending",
  "message": "Waiting for user confirmation"
}
```

**状态：confirmed（登录成功）**
```json
{
  "status": "confirmed",
  "web_token": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9...",
  "message": "Login successful"
}
```

**状态：rejected（用户拒绝）**
```json
{
  "status": "rejected",
  "message": "User rejected the login"
}
```

**状态：expired（二维码过期）**
```json
{
  "status": "expired",
  "message": "QR code has expired"
}
```

**失败：会话不存在 (404 Not Found)**
```json
{
  "error": "Session not found"
}
```

#### 轮询建议

- **间隔时间：** 2-3秒
- **超时时间：** 5分钟（与二维码有效期一致）
- **最大次数：** 150次（300秒 / 2秒）

#### 示例

```bash
# curl
curl http://localhost:8080/qr-login/status/550e8400-e29b-41d4-a716-446655440000

# JavaScript
async function pollStatus(sessionId) {
  const interval = setInterval(async () => {
    const res = await fetch(`http://localhost:8080/qr-login/status/${sessionId}`);
    const data = await res.json();
    
    if (data.status === 'confirmed') {
      clearInterval(interval);
      localStorage.setItem('token', data.web_token);
      console.log('登录成功');
    } else if (data.status === 'rejected' || data.status === 'expired') {
      clearInterval(interval);
      console.log('登录失败:', data.status);
    }
  }, 2000);
}
```

---

### 3. 确认登录

**App端调用，用户扫码后确认登录**

#### 请求

```http
POST /qr-login/confirm
Content-Type: application/json
```

**请求体：**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "app_token": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9..."
}
```

**字段说明：**

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `session_id` | String | 是 | 从二维码中解析的会话ID |
| `app_token` | String | 是 | App端用户的JWT token |

#### 响应

**成功 (200 OK):**
```json
{
  "message": "Login confirmed successfully",
  "web_token": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9..."
}
```

**失败：token无效 (401 Unauthorized)**
```json
{
  "error": "Invalid app token"
}
```

**失败：会话不存在 (404 Not Found)**
```json
{
  "error": "Session not found"
}
```

**失败：会话已过期 (400 Bad Request)**
```json
{
  "error": "Session has expired"
}
```

**失败：会话已处理 (400 Bad Request)**
```json
{
  "error": "Session already processed"
}
```

#### 示例

```bash
# curl
curl -X POST http://localhost:8080/qr-login/confirm \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "app_token": "eyJhbGciOiJFZERTQSIs..."
  }'

# Flutter
Future<void> confirmLogin(String sessionId, String appToken) async {
  final response = await http.post(
    Uri.parse('http://localhost:8080/qr-login/confirm'),
    headers: {'Content-Type': 'application/json'},
    body: jsonEncode({
      'session_id': sessionId,
      'app_token': appToken,
    }),
  );
  
  if (response.statusCode == 200) {
    print('登录确认成功');
  }
}
```

---

### 4. 拒绝登录

**App端调用，用户扫码后拒绝登录**

#### 请求

```http
POST /qr-login/reject
Content-Type: application/json
```

**请求体：**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "app_token": "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9..."
}
```

#### 响应

**成功 (200 OK):**
```json
{
  "message": "Login rejected successfully"
}
```

**失败响应与确认接口类似**

#### 示例

```bash
curl -X POST http://localhost:8080/qr-login/reject \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "app_token": "eyJhbGciOiJFZERTQSIs..."
  }'
```

---

## 📊 数据结构

### QR Data格式

**二维码内包含的数据（JSON字符串）：**

```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "action": "login",
  "expires_at": 1732012345
}
```

**字段说明：**

| 字段 | 类型 | 说明 |
|------|------|------|
| `session_id` | String | 会话ID，用于后续确认 |
| `action` | String | 操作类型，固定为"login" |
| `expires_at` | Number | Unix时间戳，过期时间 |

### 会话状态

```typescript
type SessionStatus = 
  | "pending"    // 等待扫码
  | "confirmed"  // 已确认
  | "rejected"   // 已拒绝
  | "expired";   // 已过期
```

### JWT Claims

```typescript
interface Claims {
  user_id: string;      // 用户ID
  username: string;     // 用户名/地址
  role?: string;        // 用户角色
  exp: number;          // 过期时间戳
}
```

---

## ❌ 错误码

### HTTP状态码

| 状态码 | 说明 | 场景 |
|--------|------|------|
| `200` | 成功 | 请求成功处理 |
| `400` | 请求错误 | 参数错误、会话已处理等 |
| `401` | 未授权 | Token无效或过期 |
| `404` | 未找到 | 会话不存在 |
| `500` | 服务器错误 | 数据库错误、内部错误 |

### 错误响应格式

```json
{
  "error": "Error message description"
}
```

### 常见错误

| 错误信息 | 原因 | 解决方案 |
|---------|------|---------|
| `Session not found` | 会话ID不存在 | 检查session_id是否正确 |
| `Invalid app token` | JWT验证失败 | 检查token是否有效 |
| `Session has expired` | 二维码已过期 | 重新生成二维码 |
| `Session already processed` | 会话已被处理 | 使用新的会话 |
| `Failed to create QR session` | 数据库错误 | 检查数据库连接 |

---

## 💡 示例代码

### 完整Web端流程

```javascript
class QRLogin {
  constructor(apiBase = 'http://localhost:8080') {
    this.apiBase = apiBase;
    this.sessionId = null;
    this.pollInterval = null;
  }
  
  // 生成二维码
  async generate() {
    const response = await fetch(`${this.apiBase}/qr-login/generate`, {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({client_info: 'web'})
    });
    
    if (!response.ok) throw new Error('生成二维码失败');
    
    const data = await response.json();
    this.sessionId = data.session_id;
    
    // 显示二维码
    document.getElementById('qrImage').src = data.qr_image;
    
    // 开始轮询
    this.startPolling();
    
    return data;
  }
  
  // 轮询状态
  startPolling() {
    this.pollInterval = setInterval(async () => {
      try {
        const response = await fetch(
          `${this.apiBase}/qr-login/status/${this.sessionId}`
        );
        const data = await response.json();
        
        if (data.status === 'confirmed') {
          this.handleSuccess(data.web_token);
        } else if (data.status === 'rejected') {
          this.handleRejected();
        } else if (data.status === 'expired') {
          this.handleExpired();
        }
      } catch (error) {
        console.error('轮询失败:', error);
      }
    }, 2000);
  }
  
  // 登录成功
  handleSuccess(token) {
    clearInterval(this.pollInterval);
    localStorage.setItem('token', token);
    console.log('✅ 登录成功');
    // 跳转或刷新页面
    window.location.href = '/dashboard';
  }
  
  // 用户拒绝
  handleRejected() {
    clearInterval(this.pollInterval);
    console.log('❌ 用户拒绝登录');
    alert('用户拒绝了登录请求');
  }
  
  // 二维码过期
  handleExpired() {
    clearInterval(this.pollInterval);
    console.log('⏰ 二维码已过期');
    alert('二维码已过期，请重新生成');
  }
  
  // 清理
  cleanup() {
    if (this.pollInterval) {
      clearInterval(this.pollInterval);
    }
  }
}

// 使用
const qrLogin = new QRLogin();
qrLogin.generate();
```

### 完整App端流程（Flutter）

```dart
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:mobile_scanner/mobile_scanner.dart';

class QRLoginService {
  final String apiBase;
  final String appToken;
  
  QRLoginService({
    required this.apiBase,
    required this.appToken,
  });
  
  // 扫描二维码
  Future<void> scanQRCode(BuildContext context) async {
    await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => Scaffold(
          appBar: AppBar(title: Text('扫描二维码')),
          body: MobileScanner(
            onDetect: (capture) async {
              final String? code = capture.barcodes.first.rawValue;
              if (code == null) return;
              
              try {
                final qrData = jsonDecode(code);
                await _handleQRData(context, qrData);
              } catch (e) {
                _showError(context, '二维码格式错误');
              }
            },
          ),
        ),
      ),
    );
  }
  
  // 处理二维码数据
  Future<void> _handleQRData(
    BuildContext context,
    Map<String, dynamic> qrData,
  ) async {
    final sessionId = qrData['session_id'];
    final expiresAt = qrData['expires_at'];
    
    // 检查是否过期
    if (DateTime.now().millisecondsSinceEpoch / 1000 > expiresAt) {
      _showError(context, '二维码已过期');
      return;
    }
    
    // 显示确认对话框
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('确认登录'),
        content: Text('是否在网页端登录？'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text('取消'),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: Text('确认'),
          ),
        ],
      ),
    );
    
    if (confirmed == true) {
      await _confirmLogin(context, sessionId);
    } else {
      await _rejectLogin(context, sessionId);
    }
  }
  
  // 确认登录
  Future<void> _confirmLogin(BuildContext context, String sessionId) async {
    try {
      final response = await http.post(
        Uri.parse('$apiBase/qr-login/confirm'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'session_id': sessionId,
          'app_token': appToken,
        }),
      );
      
      if (response.statusCode == 200) {
        Navigator.pop(context);
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('✅ 登录确认成功')),
        );
      } else {
        throw Exception('确认失败');
      }
    } catch (e) {
      _showError(context, '确认登录失败: $e');
    }
  }
  
  // 拒绝登录
  Future<void> _rejectLogin(BuildContext context, String sessionId) async {
    try {
      await http.post(
        Uri.parse('$apiBase/qr-login/reject'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'session_id': sessionId,
          'app_token': appToken,
        }),
      );
      Navigator.pop(context);
    } catch (e) {
      _showError(context, '操作失败: $e');
    }
  }
  
  void _showError(BuildContext context, String message) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('错误'),
        content: Text(message),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text('确定'),
          ),
        ],
      ),
    );
  }
}

// 使用
final qrService = QRLoginService(
  apiBase: 'http://api.example.com',
  appToken: await getStoredToken(),
);
await qrService.scanQRCode(context);
```

---

## 🔗 相关链接

- **主文档：** [QR_LOGIN.md](./QR_LOGIN.md)
- **改动清单：** [CHANGES.md](./CHANGES.md)
- **测试页面：** [scaffold/examples/qr_login_simple.html](./scaffold/examples/qr_login_simple.html)

---

**API版本：** v1.0  
**最后更新：** 2024-11-19  
**维护者：** pureblackalex@google.com
