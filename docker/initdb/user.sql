-- 创建用户角色 ENUM 类型
CREATE TYPE user_role_type AS ENUM ('admin', 'user');

-- 用户表
CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  user_id TEXT NOT NULL UNIQUE, -- 链用户 ID
  password_hash TEXT NOT NULL,  -- 密码哈希
  email TEXT UNIQUE,            -- 邮箱（唯一）
  phone TEXT UNIQUE,            -- 手机号（唯一，可选）
  role user_role_type NOT NULL DEFAULT 'user', -- 角色（默认普通用户）
  is_active BOOLEAN DEFAULT TRUE,   -- 是否启用
  is_verified BOOLEAN DEFAULT FALSE,-- 邮箱是否已验证
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP  -- 更新时间
);

-- 用户表索引优化
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_phone ON users(phone);

-- 认证会话表（存储 JWT 或 Session Token）
CREATE TABLE auth_sessions (
  id BIGSERIAL PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  token TEXT NOT NULL UNIQUE,    -- 存 JWT 或 Session Token
  user_agent TEXT,               -- 设备信息
  expires_at TIMESTAMP NOT NULL, -- 过期时间
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 用户日志表（记录用户行为）
CREATE TYPE log_action_type AS ENUM ('LOGIN', 'LOGOUT', 'CHANGE_PASSWORD', 'RESET_PASSWORD', 'UPDATE_PROFILE');

CREATE TABLE user_logs (
  id BIGSERIAL PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  action log_action_type NOT NULL,  -- 记录动作
  ip_address INET NOT NULL,         -- 用户 IP
  user_agent TEXT,                  -- 设备信息
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 密码重置表
CREATE TABLE password_resets (
  id BIGSERIAL PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  reset_token VARCHAR(64) NOT NULL UNIQUE,  -- 限制 Token 长度
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 邮箱验证码表
CREATE TABLE email_verifications (
  id BIGSERIAL PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
  email TEXT NOT NULL,
  code VARCHAR(6) NOT NULL,   -- 6 位验证码
  is_used BOOLEAN DEFAULT FALSE, -- 是否已使用，防止重复
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

