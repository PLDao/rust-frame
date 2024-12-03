CREATE TYPE role_type AS ENUM ('admin', 'user');
CREATE TABLE api__users (
  id BIGSERIAL PRIMARY KEY,
  user_id TEXT NOT NULL UNIQUE, -- chainless chain id
  username TEXT NOT NULL UNIQUE, -- 用户名，唯一
  email TEXT UNIQUE, -- 邮箱，唯一
  phone TEXT UNIQUE, -- 手机号，唯一，可选
  password_hash TEXT NOT NULL, -- 用户密码的哈希值
  verified BOOLEAN DEFAULT FALSE NOT NULL, -- 是否已通过验证
  role role_type DEFAULT 'user', -- 用户角色
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 注册时间
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 更新时间
  last_login_at TIMESTAMP -- 上次登录时间
);

INSERT INTO api__users (
    user_id,
    username,
    email,
    phone,
    password_hash,
    verified,
    role,
    created_at,
    updated_at,
    last_login_at
)
VALUES (
    'explorer.chainless',
    'explorer',
    'explorer@example.com',  -- 这里使用一个示例邮箱
    '1234567890',            -- 这里使用一个示例手机号
    '$2a$12$Fv.ljHbVhzzVHa6JYq6pmOl5FhzYmmY3H0jt6UBFOaTXZ40AZY7OC',  -- 已经哈希后的密码
    TRUE,                    -- 设为已验证
    'admin',                 -- 角色是管理员
    CURRENT_TIMESTAMP,       -- 注册时间
    CURRENT_TIMESTAMP,       -- 更新时间
    CURRENT_TIMESTAMP       -- 上次登录时间
);

CREATE TABLE api__plans (
  id SERIAL PRIMARY KEY, -- 唯一标识符
  title TEXT NOT NULL, -- 计划名称
  description TEXT, -- 计划描述
  limit_per_second INTEGER DEFAULT 0, -- 每秒调用限制，0 表示无限制
  limit_per_minute INTEGER DEFAULT 0, -- 每分钟调用限制
  limit_per_day INTEGER DEFAULT 0, -- 每日调用限制
  limit_per_month INTEGER DEFAULT 0, -- 每月调用限制
  duration_days INTEGER NOT NULL, -- 计划有效期（天）
  price_monthly INTEGER, -- 月费（分为单位）
  price_annually INTEGER, -- 年费（分为单位）
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

CREATE TABLE api__keys (
  id BIGSERIAL PRIMARY KEY, -- 唯一标识符
  user_id BIGINT NOT NULL REFERENCES api__users(id) ON DELETE CASCADE, -- 所属用户
  plan_id INTEGER NOT NULL REFERENCES api__plans(id) ON DELETE SET NULL, -- 关联计划
  start_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 开始日期
  end_date TIMESTAMP NOT NULL, -- 结束日期
  name TEXT NOT NULL, -- API Key 的名称
  token TEXT NOT NULL UNIQUE, -- API Key 值，唯一且随机生成
  usage_limit INTEGER DEFAULT 100000, -- 调用次数限制
  current_usage INTEGER DEFAULT 0, -- 当前已使用次数
  active BOOLEAN DEFAULT TRUE NOT NULL, -- 是否启用
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

CREATE TABLE api__usage_logs (
  id BIGSERIAL PRIMARY KEY, -- 唯一标识符
  key_id BIGINT NOT NULL REFERENCES api__keys(id) ON DELETE CASCADE, -- 所属 Key
  timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 调用时间
  endpoint TEXT NOT NULL, -- 调用的 API 端点
  response_time_ms INTEGER, -- 响应时间（毫秒）
  status_code INTEGER -- 返回状态码
);

DELETE FROM api__usage_logs WHERE timestamp < NOW() - INTERVAL '30 days';