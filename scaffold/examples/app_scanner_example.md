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

### 2. 扫码页面
```dart
import 'package:flutter/material.dart';
import 'package:mobile_scanner/mobile_scanner.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';

class QRScannerPage extends StatefulWidget {
  @override
  _QRScannerPageState createState() => _QRScannerPageState();
}

class _QRScannerPageState extends State<QRScannerPage> {
  MobileScannerController cameraController = MobileScannerController();
  bool isProcessing = false;
  String? appToken;

  @override
  void initState() {
    super.initState();
    _loadToken();
  }

  // 加载本地存储的token
  Future<void> _loadToken() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      appToken = prefs.getString('auth_token');
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('扫码登录'),
        backgroundColor: Colors.deepPurple,
      ),
      body: Stack(
        children: [
          // 相机预览
          MobileScanner(
            controller: cameraController,
            onDetect: (capture) {
              if (!isProcessing) {
                final List<Barcode> barcodes = capture.barcodes;
                for (final barcode in barcodes) {
                  if (barcode.rawValue != null) {
                    _handleQRCode(barcode.rawValue!);
                    break;
                  }
                }
              }
            },
          ),
          
          // 扫描框
          Center(
            child: Container(
              width: 250,
              height: 250,
              decoration: BoxDecoration(
                border: Border.all(color: Colors.white, width: 3),
                borderRadius: BorderRadius.circular(12),
              ),
            ),
          ),
          
          // 提示文字
          Positioned(
            bottom: 100,
            left: 0,
            right: 0,
            child: Text(
              '将二维码放入框内',
              textAlign: TextAlign.center,
              style: TextStyle(
                color: Colors.white,
                fontSize: 18,
                fontWeight: FontWeight.bold,
                shadows: [
                  Shadow(
                    blurRadius: 10.0,
                    color: Colors.black,
                    offset: Offset(2.0, 2.0),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  // 处理扫描到的二维码
  void _handleQRCode(String qrData) async {
    if (isProcessing) return;
    
    setState(() {
      isProcessing = true;
    });
    
    // 暂停扫描
    cameraController.stop();

    try {
      // 解析二维码数据
      final data = jsonDecode(qrData);
      final sessionId = data['session_id'];
      final action = data['action'];
      
      if (action != 'login') {
        _showError('无效的二维码');
        return;
      }

      // 检查是否过期
      final expiresAt = data['expires_at'];
      if (DateTime.now().millisecondsSinceEpoch / 1000 > expiresAt) {
        _showError('二维码已过期');
        return;
      }

      // 显示确认对话框
      final confirmed = await _showConfirmDialog(sessionId);
      
      if (confirmed == true) {
        await _confirmLogin(sessionId);
      } else if (confirmed == false) {
        await _rejectLogin(sessionId);
      }
      
    } catch (e) {
      _showError('二维码格式错误: $e');
    } finally {
      setState(() {
        isProcessing = false;
      });
      // 恢复扫描
      cameraController.start();
    }
  }

  // 显示确认对话框
  Future<bool?> _showConfirmDialog(String sessionId) {
    return showDialog<bool>(
      context: context,
      barrierDismissible: false,
      builder: (context) => AlertDialog(
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(15),
        ),
        title: Row(
          children: [
            Icon(Icons.login, color: Colors.deepPurple),
            SizedBox(width: 10),
            Text('确认登录'),
          ],
        ),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('是否同意登录到网页端？'),
            SizedBox(height: 10),
            Container(
              padding: EdgeInsets.all(10),
              decoration: BoxDecoration(
                color: Colors.grey[200],
                borderRadius: BorderRadius.circular(8),
              ),
              child: Text(
                'Session: ${sessionId.substring(0, 12)}...',
                style: TextStyle(fontSize: 12, fontFamily: 'monospace'),
              ),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: Text('取消', style: TextStyle(color: Colors.grey)),
          ),
          ElevatedButton(
            onPressed: () => Navigator.pop(context, true),
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.deepPurple,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(8),
              ),
            ),
            child: Text('同意'),
          ),
        ],
      ),
    );
  }

  // 确认登录
  Future<void> _confirmLogin(String sessionId) async {
    if (appToken == null) {
      _showError('未登录，请先登录App');
      return;
    }

    try {
      final response = await http.post(
        Uri.parse('http://localhost:8080/qr-login/confirm'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({
          'session_id': sessionId,
          'app_token': appToken,
        }),
      );

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        if (data['success']) {
          _showSuccess('登录确认成功');
          Navigator.pop(context);
        } else {
          _showError('确认失败: ${data['message']}');
        }
      } else {
        _showError('请求失败: ${response.statusCode}');
      }
    } catch (e) {
      _showError('网络错误: $e');
    }
  }

  // 拒绝登录
  Future<void> _rejectLogin(String sessionId) async {
    try {
      await http.post(
        Uri.parse('http://localhost:8080/qr-login/reject'),
        headers: {'Content-Type': 'application/json'},
        body: jsonEncode({'session_id': sessionId}),
      );
      
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('已拒绝登录')),
      );
    } catch (e) {
      print('拒绝登录失败: $e');
    }
  }

  // 显示错误消息
  void _showError(String message) {
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

  // 显示成功消息
  void _showSuccess(String message) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Row(
          children: [
            Icon(Icons.check_circle, color: Colors.green),
            SizedBox(width: 10),
            Text('成功'),
          ],
        ),
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

  @override
  void dispose() {
    cameraController.dispose();
    super.dispose();
  }
}
```

---

## React Native实现

### 1. 安装依赖
```bash
npm install react-native-camera-kit axios @react-native-async-storage/async-storage
```

### 2. 扫码组件
```typescript
import React, { useState, useEffect } from 'react';
import { View, Text, Alert, StyleSheet, ActivityIndicator } from 'react-native';
import { Camera, CameraType } from 'react-native-camera-kit';
import AsyncStorage from '@react-native-async-storage/async-storage';
import axios from 'axios';

const API_BASE = 'http://localhost:8080';

export const QRScannerScreen = ({ navigation }) => {
  const [appToken, setAppToken] = useState<string | null>(null);
  const [isScanning, setIsScanning] = useState(true);
  const [isProcessing, setIsProcessing] = useState(false);

  useEffect(() => {
    loadToken();
  }, []);

  const loadToken = async () => {
    try {
      const token = await AsyncStorage.getItem('auth_token');
      setAppToken(token);
    } catch (error) {
      console.error('加载token失败:', error);
    }
  };

  const handleQRCodeScanned = async (event: any) => {
    if (!isScanning || isProcessing) return;
    
    setIsScanning(false);
    setIsProcessing(true);

    try {
      const qrData = JSON.parse(event.nativeEvent.codeStringValue);
      const sessionId = qrData.session_id;
      
      // 检查过期
      if (Date.now() / 1000 > qrData.expires_at) {
        Alert.alert('错误', '二维码已过期');
        resetScanning();
        return;
      }

      // 显示确认对话框
      Alert.alert(
        '确认登录',
        `是否同意登录到网页端？\n\nSession: ${sessionId.substring(0, 12)}...`,
        [
          {
            text: '取消',
            onPress: () => {
              rejectLogin(sessionId);
              resetScanning();
            },
            style: 'cancel'
          },
          {
            text: '同意',
            onPress: () => confirmLogin(sessionId)
          }
        ]
      );
    } catch (error) {
      Alert.alert('错误', '无效的二维码');
      resetScanning();
    }
  };

  const confirmLogin = async (sessionId: string) => {
    if (!appToken) {
      Alert.alert('错误', '未登录，请先登录App');
      resetScanning();
      return;
    }

    try {
      const response = await axios.post(`${API_BASE}/qr-login/confirm`, {
        session_id: sessionId,
        app_token: appToken
      });

      if (response.data.success) {
        Alert.alert('成功', '登录确认成功', [
          { text: '确定', onPress: () => navigation.goBack() }
        ]);
      } else {
        Alert.alert('失败', response.data.message);
        resetScanning();
      }
    } catch (error) {
      Alert.alert('错误', '网络请求失败');
      resetScanning();
    } finally {
      setIsProcessing(false);
    }
  };

  const rejectLogin = async (sessionId: string) => {
    try {
      await axios.post(`${API_BASE}/qr-login/reject`, {
        session_id: sessionId
      });
    } catch (error) {
      console.error('拒绝登录失败:', error);
    }
  };

  const resetScanning = () => {
    setIsScanning(true);
    setIsProcessing(false);
  };

  return (
    <View style={styles.container}>
      <Camera
        scanBarcode={isScanning}
        onReadCode={handleQRCodeScanned}
        showFrame={true}
        laserColor="red"
        frameColor="white"
        style={styles.camera}
      />
      
      <View style={styles.overlay}>
        <Text style={styles.title}>扫描二维码登录</Text>
        <View style={styles.scanArea} />
        <Text style={styles.hint}>将二维码放入框内</Text>
        
        {isProcessing && (
          <View style={styles.loading}>
            <ActivityIndicator size="large" color="#fff" />
            <Text style={styles.loadingText}>处理中...</Text>
          </View>
        )}
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  camera: {
    flex: 1,
  },
  overlay: {
    ...StyleSheet.absoluteFillObject,
    justifyContent: 'center',
    alignItems: 'center',
  },
  title: {
    position: 'absolute',
    top: 100,
    fontSize: 24,
    fontWeight: 'bold',
    color: '#fff',
    textShadowColor: 'rgba(0, 0, 0, 0.75)',
    textShadowOffset: { width: -1, height: 1 },
    textShadowRadius: 10,
  },
  scanArea: {
    width: 250,
    height: 250,
    borderWidth: 3,
    borderColor: '#fff',
    borderRadius: 12,
    backgroundColor: 'transparent',
  },
  hint: {
    position: 'absolute',
    bottom: 100,
    fontSize: 18,
    color: '#fff',
    fontWeight: 'bold',
    textShadowColor: 'rgba(0, 0, 0, 0.75)',
    textShadowOffset: { width: -1, height: 1 },
    textShadowRadius: 10,
  },
  loading: {
    position: 'absolute',
    alignItems: 'center',
  },
  loadingText: {
    marginTop: 10,
    color: '#fff',
    fontSize: 16,
  },
});
```

---

## 原生Android实现 (Kotlin)

### 1. 添加依赖 (build.gradle)
```groovy
dependencies {
    implementation 'com.google.zxing:core:3.5.1'
    implementation 'com.journeyapps:zxing-android-embedded:4.3.0'
    implementation 'com.squareup.okhttp3:okhttp:4.11.0'
    implementation 'com.google.code.gson:gson:2.10.1'
}
```

### 2. 扫码Activity
```kotlin
import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import com.google.zxing.integration.android.IntentIntegrator
import com.google.gson.Gson
import okhttp3.*
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import java.io.IOException

data class QRData(
    val session_id: String,
    val action: String,
    val expires_at: Long
)

data class ConfirmRequest(
    val session_id: String,
    val app_token: String
)

class QRScanActivity : AppCompatActivity() {
    private val API_BASE = "http://10.0.2.2:8080" // Android模拟器localhost
    private val client = OkHttpClient()
    private val gson = Gson()
    private var appToken: String? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // 加载token
        val prefs = getSharedPreferences("app_prefs", MODE_PRIVATE)
        appToken = prefs.getString("auth_token", null)
        
        if (appToken == null) {
            Toast.makeText(this, "请先登录", Toast.LENGTH_SHORT).show()
            finish()
            return
        }
        
        // 启动扫描
        startQRScanner()
    }

    private fun startQRScanner() {
        val integrator = IntentIntegrator(this)
        integrator.setDesiredBarcodeFormats(IntentIntegrator.QR_CODE)
        integrator.setPrompt("扫描登录二维码")
        integrator.setCameraId(0)
        integrator.setBeepEnabled(true)
        integrator.setBarcodeImageEnabled(false)
        integrator.initiateScan()
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        val result = IntentIntegrator.parseActivityResult(requestCode, resultCode, data)
        
        if (result != null) {
            if (result.contents == null) {
                Toast.makeText(this, "扫描取消", Toast.LENGTH_SHORT).show()
                finish()
            } else {
                handleQRCode(result.contents)
            }
        } else {
            super.onActivityResult(requestCode, resultCode, data)
        }
    }

    private fun handleQRCode(qrContent: String) {
        try {
            val qrData = gson.fromJson(qrContent, QRData::class.java)
            
            // 检查过期
            if (System.currentTimeMillis() / 1000 > qrData.expires_at) {
                Toast.makeText(this, "二维码已过期", Toast.LENGTH_SHORT).show()
                finish()
                return
            }
            
            // 显示确认对话框
            showConfirmDialog(qrData.session_id)
            
        } catch (e: Exception) {
            Toast.makeText(this, "无效的二维码", Toast.LENGTH_SHORT).show()
            finish()
        }
    }

    private fun showConfirmDialog(sessionId: String) {
        AlertDialog.Builder(this)
            .setTitle("确认登录")
            .setMessage("是否同意登录到网页端？\n\nSession: ${sessionId.take(12)}...")
            .setPositiveButton("同意") { _, _ ->
                confirmLogin(sessionId)
            }
            .setNegativeButton("取消") { _, _ ->
                rejectLogin(sessionId)
                finish()
            }
            .setCancelable(false)
            .show()
    }

    private fun confirmLogin(sessionId: String) {
        val requestBody = gson.toJson(
            ConfirmRequest(sessionId, appToken!!)
        ).toRequestBody("application/json".toMediaType())
        
        val request = Request.Builder()
            .url("$API_BASE/qr-login/confirm")
            .post(requestBody)
            .build()
        
        client.newCall(request).enqueue(object : Callback {
            override fun onFailure(call: Call, e: IOException) {
                runOnUiThread {
                    Toast.makeText(this@QRScanActivity, "网络错误", Toast.LENGTH_SHORT).show()
                    finish()
                }
            }
            
            override fun onResponse(call: Call, response: Response) {
                runOnUiThread {
                    if (response.isSuccessful) {
                        Toast.makeText(this@QRScanActivity, "登录确认成功", Toast.LENGTH_SHORT).show()
                    } else {
                        Toast.makeText(this@QRScanActivity, "确认失败", Toast.LENGTH_SHORT).show()
                    }
                    finish()
                }
            }
        })
    }

    private fun rejectLogin(sessionId: String) {
        val requestBody = gson.toJson(
            mapOf("session_id" to sessionId)
        ).toRequestBody("application/json".toMediaType())
        
        val request = Request.Builder()
            .url("$API_BASE/qr-login/reject")
            .post(requestBody)
            .build()
        
        client.newCall(request).enqueue(object : Callback {
            override fun onFailure(call: Call, e: IOException) {}
            override fun onResponse(call: Call, response: Response) {}
        })
    }
}
```

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
