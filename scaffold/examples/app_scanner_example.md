# App端扫码登录实现示例

## 概述
此文档提供App端实现扫码登录的参考代码，支持Flutter、React Native等主流框架。

## 前提条件
- App端用户已登录，持有有效的JWT token
- 集成了二维码扫描库

---

## Flutter实现 (完整示例)

### 1. 添加依赖 (pubspec.yaml)
```yaml
dependencies:
  flutter:
    sdk: flutter
  mobile_scanner: ^5.0.0  # 二维码扫描
  http: ^1.1.0            # HTTP请求
  shared_preferences: ^2.2.2  # 本地存储
```

---

---

## 测试步骤

### 1. 准备工作
1. 确保后端服务正在运行: `http://localhost:8080`
2. 创建测试用户并获取JWT token
3. 在App中保存该token

### 2. 测试流程
1. 打开网页测试页面: `scaffold/examples/qr_login_test.html`
2. 点击"生成二维码"
3. 使用App扫描二维码
4. 在App中点击"同意"
5. 观察网页自动登录

### 3. 调试技巧
- 使用 `console.log` / `print` 打印二维码内容
- 检查网络请求是否成功
- 验证token格式是否正确
- 确认API地址配置正确（Android模拟器使用 `10.0.2.2` 代替 `localhost`）

---

## 安全建议

1. **HTTPS**: 生产环境必须使用HTTPS
2. **Token验证**: 严格验证App端token
3. **过期时间**: 合理设置二维码过期时间（建议3-5分钟）
4. **一次性**: 每个二维码只能使用一次
5. **设备绑定**: 可以绑定设备信息增加安全性
6. **日志记录**: 记录所有登录尝试

---

## 常见问题

### Q: App如何获取初始token？
A: 用户首次在App中通过用户名密码登录，获取JWT token后保存。

### Q: token过期怎么办？
A: 实现token刷新机制，或要求用户重新登录App。

### Q: 如何处理网络超时？
A: 设置合理的超时时间，失败后提示用户重试。

### Q: 支持多平台吗？
A: 是的，只要App能发送HTTP请求和扫描二维码即可。
