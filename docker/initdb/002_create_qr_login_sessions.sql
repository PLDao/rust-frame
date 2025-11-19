-- 创建扫码登录会话表
CREATE TABLE IF NOT EXISTS qr_login_sessions (
    id BIGSERIAL PRIMARY KEY,
    session_id TEXT NOT NULL UNIQUE,
    user_id TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    web_token TEXT,
    app_token TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT fk_qr_login_user FOREIGN KEY (user_id) 
        REFERENCES users(user_id) 
        ON UPDATE NO ACTION 
        ON DELETE CASCADE
);

-- 为常用查询字段添加索引
CREATE INDEX idx_qr_login_session_id ON qr_login_sessions(session_id);
CREATE INDEX idx_qr_login_status ON qr_login_sessions(status);
CREATE INDEX idx_qr_login_expires_at ON qr_login_sessions(expires_at);

---- 添加注释
--COMMENT ON TABLE qr_login_sessions IS '扫码登录会话表';
--COMMENT ON COLUMN qr_login_sessions.id IS '主键ID';
--COMMENT ON COLUMN qr_login_sessions.session_id IS '登录会话唯一ID（UUID）';
--COMMENT ON COLUMN qr_login_sessions.user_id IS 'App端用户ID（确认后填充）';
--COMMENT ON COLUMN qr_login_sessions.status IS '登录状态: pending, scanned, confirmed, rejected, expired';
--COMMENT ON COLUMN qr_login_sessions.web_token IS '生成的Web端JWT（确认后生成）';
--COMMENT ON COLUMN qr_login_sessions.app_token IS 'App端提交的身份信息';
--COMMENT ON COLUMN qr_login_sessions.created_at IS '创建时间';
--COMMENT ON COLUMN qr_login_sessions.expires_at IS '过期时间';
--COMMENT ON COLUMN qr_login_sessions.updated_at IS '更新时间';
