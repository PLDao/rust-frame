use actix_ws::Session;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// WebSocket连接管理器
/// 用于管理所有活跃的WebSocket连接，并支持向特定会话推送消息
#[derive(Clone)]
pub struct WsManager {
    // session_id -> WebSocket Session
    connections: Arc<RwLock<HashMap<String, Session>>>,
}

impl WsManager {
    /// 创建新的WebSocket管理器
    pub fn new() -> Self {
        info!("🔌 WebSocket Manager initialized");
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 添加新的WebSocket连接
    pub async fn add_connection(&self, session_id: String, session: Session) {
        let mut connections = self.connections.write().await;
        connections.insert(session_id.clone(), session);
        info!("✅ WebSocket connected for session: {}", session_id);
        info!("📊 Active connections: {}", connections.len());
    }

    /// 移除WebSocket连接
    /// 
    /// 注意：如果连接已被其他地方移除（如notify_status），此方法会静默返回
    pub async fn remove_connection(&self, session_id: &str) {
        let mut connections = self.connections.write().await;
        if connections.remove(session_id).is_some() {
            info!("🔌 WebSocket disconnected for session: {}", session_id);
            info!("📊 Active connections: {}", connections.len());
        }
        // 如果连接不存在，说明已被其他地方清理，不需要重复日志
    }

    /// 推送状态更新到指定会话
    /// 
    /// 注意：此方法会：
    /// 1. 从连接管理器中移除连接（避免重复访问）
    /// 2. 发送状态消息
    /// 3. 主动关闭WebSocket连接
    pub async fn notify_status(&self, session_id: &str, status: &str, web_token: Option<&str>) {
        let mut connections = self.connections.write().await;

        if let Some(mut session) = connections.remove(session_id) {
            // 释放锁，避免阻塞其他操作
            drop(connections);
            
            let message = if let Some(token) = web_token {
                format!(
                    r#"{{"status":"{}","web_token":"{}","message":"Login successful"}}"#,
                    status, token
                )
            } else {
                format!(
                    r#"{{"status":"{}","message":"Status updated"}}"#,
                    status
                )
            };

            info!("🔔 Pushing status update to session {}: {}", session_id, status);

            // 发送消息
            if let Err(e) = session.text(message).await {
                info!("❌ Failed to send message: {}", e);
            }

            // 关闭连接（会触发ws_status中的清理逻辑，但连接已从HashMap移除）
            let _ = session.close(None).await;

            info!("✅ Status pushed and connection closed for session: {}", session_id);
        } else {
            info!("⚠️  No active WebSocket connection found for session: {}", session_id);
        }
    }

    /// 获取当前活跃连接数
    pub async fn get_connection_count(&self) -> usize {
        self.connections.read().await.len()
    }

    /// 检查某个会话是否有活跃连接
    pub async fn has_connection(&self, session_id: &str) -> bool {
        self.connections.read().await.contains_key(session_id)
    }
}

impl Default for WsManager {
    fn default() -> Self {
        Self::new()
    }
}
