package config

import "time"

// ForwardRule 单条端口转发规则
type ForwardRule struct {
	ID         string    `json:"id"`
	LocalPort  int       `json:"localPort"`
	TargetHost string    `json:"targetHost"`
	TargetPort int       `json:"targetPort"`
	Enabled    bool      `json:"enabled"`
	CreatedAt  time.Time `json:"createdAt"`
}

// LogEntry 日志条目
type LogEntry struct {
	Timestamp time.Time `json:"timestamp"`
	Source    string    `json:"source"`
	BytesIn   int64     `json:"bytesIn"`
	BytesOut  int64     `json:"bytesOut"`
	Status    string    `json:"status"`
}

// Config 应用配置
type Config struct {
	Rules []ForwardRule `json:"rules"`
}
