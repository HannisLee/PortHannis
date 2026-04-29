package config

import (
	"encoding/json"
	"os"
	"path/filepath"
	"sync"
)

const (
	appName    = "porthannis"
	configFile = "rules.json"
)

type Manager struct {
	mu     sync.Mutex
	config *Config
	dir    string
}

func NewManager() *Manager {
	return &Manager{
		config: &Config{Rules: []ForwardRule{}, WebUI: defaultWebUIConfig()},
		dir:    GetConfigDir(),
	}
}

func defaultWebUIConfig() WebUIConfig {
	return WebUIConfig{
		Enabled:  true,
		Port:     18080,
		Password: "",
	}
}

func GetConfigDir() string {
	if xdg := os.Getenv("XDG_CONFIG_HOME"); xdg != "" {
		return filepath.Join(xdg, appName)
	}
	home, err := os.UserHomeDir()
	if err != nil {
		home = "."
	}
	return filepath.Join(home, ".config", appName)
}

func GetLogsDir() string {
	return filepath.Join(GetConfigDir(), "logs")
}

func (m *Manager) LoadConfig() error {
	m.mu.Lock()
	defer m.mu.Unlock()

	path := filepath.Join(m.dir, configFile)
	data, err := os.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			m.config = &Config{Rules: []ForwardRule{}}
			return nil
		}
		return err
	}

	var cfg Config
	if err := json.Unmarshal(data, &cfg); err != nil {
		return err
	}
	if cfg.Rules == nil {
		cfg.Rules = []ForwardRule{}
	}
	if cfg.WebUI.Port == 0 {
		cfg.WebUI = defaultWebUIConfig()
	}
	m.config = &cfg
	return nil
}

func (m *Manager) SaveConfig() error {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.saveLocked()
}

func (m *Manager) saveLocked() error {
	if err := os.MkdirAll(m.dir, 0755); err != nil {
		return err
	}
	data, err := json.MarshalIndent(m.config, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(m.dir, configFile), data, 0644)
}

func (m *Manager) GetConfig() *Config {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.config
}

func (m *Manager) AddRule(rule ForwardRule) error {
	m.mu.Lock()
	m.config.Rules = append(m.config.Rules, rule)
	err := m.saveLocked()
	m.mu.Unlock()
	return err
}

func (m *Manager) DeleteRule(id string) error {
	m.mu.Lock()
	for i, r := range m.config.Rules {
		if r.ID == id {
			m.config.Rules = append(m.config.Rules[:i], m.config.Rules[i+1:]...)
			break
		}
	}
	err := m.saveLocked()
	m.mu.Unlock()
	return err
}

func (m *Manager) UpdateRule(id string, enabled bool) error {
	m.mu.Lock()
	for i, r := range m.config.Rules {
		if r.ID == id {
			m.config.Rules[i].Enabled = enabled
			break
		}
	}
	err := m.saveLocked()
	m.mu.Unlock()
	return err
}

func (m *Manager) GetWebUIConfig() WebUIConfig {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.config.WebUI
}

func (m *Manager) UpdateWebUIConfig(cfg WebUIConfig) error {
	m.mu.Lock()
	m.config.WebUI = cfg
	err := m.saveLocked()
	m.mu.Unlock()
	return err
}
