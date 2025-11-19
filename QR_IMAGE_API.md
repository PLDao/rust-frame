# 扫码登录 - 后端直接下发二维码图片

## ✨ 新特性

**后端直接生成二维码PNG图片并base64编码，前端无需任何二维码库！**

## 📡 API变化

### 生成二维码接口

**请求：**
```bash
POST /qr-login/generate
Content-Type: application/json

{
  "client_info": "web"
}
```

**响应（新增 `qr_image` 字段）：**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "qr_image": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA...",
  "qr_data": "{\"session_id\":\"...\",\"action\":\"login\",\"expires_at\":1234567890}",
  "expires_in": 300
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `session_id` | String | 会话唯一ID |
| **`qr_image`** | String | **base64编码的PNG图片（可直接用于img标签）** |
| `qr_data` | String | 二维码原始数据（JSON字符串） |
| `expires_in` | Number | 过期时间（秒） |

## 🎯 前端使用

### 超简单！只需3行代码

```javascript
const response = await fetch('http://localhost:8080/qr-login/generate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ client_info: 'web' })
});

const data = await response.json();

// 直接显示图片，无需任何二维码库！
document.getElementById('qrImage').src = data.qr_image;
```

### 完整HTML示例

```html
<!DOCTYPE html>
<html>
<head>
    <title>扫码登录</title>
</head>
<body>
    <!-- 只需一个img标签 -->
    <img id="qrImage" alt="二维码">
    
    <script>
        async function generateQR() {
            const response = await fetch('http://localhost:8080/qr-login/generate', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({})
            });
            
            const data = await response.json();
            
            // 直接设置src，无需任何库！
            document.getElementById('qrImage').src = data.qr_image;
        }
        
        generateQR();
    </script>
</body>
</html>
```

## 🔧 后端实现

### 技术栈

- `qrcode = "0.14"` - 二维码生成
- `image = "0.25"` - 图片处理
- `base64` - Base64编码（已有）

### 核心代码

```rust
use qrcode::QrCode;
use image::Luma;
use base64::{Engine as _, engine::general_purpose};

fn generate_qr_image(data: &str) -> Result<String, String> {
    // 1. 生成二维码
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| format!("Failed to generate QR code: {}", e))?;
    
    // 2. 渲染为300x300的图像
    let image = code.render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .max_dimensions(300, 300)
        .build();
    
    // 3. 转换为PNG字节
    let mut png_bytes: Vec<u8> = Vec::new();
    image::DynamicImage::ImageLuma8(image)
        .write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;
    
    // 4. Base64编码
    let base64_image = general_purpose::STANDARD.encode(&png_bytes);
    Ok(format!("data:image/png;base64,{}", base64_image))
}
```

## 📊 对比

### 之前的方式（前端生成）

```javascript
// ❌ 需要引入库
<script src="qrcode.js"></script>

// ❌ 前端代码复杂
const qr = new QRCode(document.getElementById('qrcode'), {
    text: qr_data,
    width: 300,
    height: 300
});
```

### 现在的方式（后端生成）

```javascript
// ✅ 无需任何库

// ✅ 前端代码超简单
img.src = data.qr_image;
```

## ✅ 优势

1. **前端零依赖** - 无需引入qrcode.js等库
2. **代码更简洁** - 3行代码搞定
3. **性能更好** - 减少前端计算负担
4. **统一管理** - 二维码样式由后端统一控制
5. **直接可用** - base64格式可直接用于`<img>`标签

## 🚀 快速测试

### 1. 启动服务

```bash
cd scaffold
cargo run -- --pgsql-url "postgres://..." --backend-port 8080
```

### 2. 打开测试页面

```bash
open scaffold/examples/qr_login_simple.html
```

### 3. 点击"生成二维码"

立即看到后端生成的二维码图片！

## 📝 API测试

```bash
# 生成二维码
curl -X POST http://localhost:8080/qr-login/generate \
  -H "Content-Type: application/json" \
  -d '{"client_info":"test"}' | jq .

# 响应示例
{
  "session_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
  "qr_image": "data:image/png;base64,iVBORw0KGgo...(很长的base64字符串)",
  "qr_data": "{\"session_id\":\"...\",\"action\":\"login\",\"expires_at\":1732001234}",
  "expires_in": 300
}
```

## 🎨 自定义二维码样式

在 `generate_qr.rs` 中修改：

```rust
let image = code.render::<Luma<u8>>()
    .min_dimensions(400, 400)  // 修改尺寸
    .max_dimensions(400, 400)
    .dark_color(Luma([0u8]))   // 黑色
    .light_color(Luma([255u8])) // 白色
    .build();
```

## 📦 文件清单

### 新增/修改的文件

```
scaffold/
├── Cargo.toml                                  ✏️ 添加qrcode和image依赖
├── src/backend/api/qr_login/
│   └── generate_qr.rs                          ✏️ 添加图片生成功能
└── examples/
    └── qr_login_simple.html                    ✅ 新增简化版测试页面
```

## 🔍 技术细节

### 生成流程

```
QR数据 → QrCode对象 → Luma图像 → PNG字节 → Base64 → Data URI
```

### Data URI格式

```
data:image/png;base64,iVBORw0KGgoAAAANSUhEUg...
  │      │      │       └─ base64编码的PNG数据
  │      │      └─ 编码方式
  │      └─ MIME类型
  └─ 协议
```

### 图片大小

- 默认：300x300 像素
- 文件大小：约 2-5KB（base64编码后）
- 适合移动端扫描

## 🎉 总结

通过后端直接生成二维码图片，实现了：

✅ **前端极简化** - 无需任何库，只需`<img>`标签
✅ **后端标准化** - 统一的二维码格式和尺寸
✅ **性能优化** - 减少前端计算
✅ **维护方便** - 样式修改只需改后端

---

**开发时间：** 2024-11-19  
**状态：** ✅ 已完成并测试
