package webui

import (
	"crypto/rand"
	"crypto/subtle"
	"encoding/hex"
	"net/http"
	"sync"

	"github.com/labstack/echo/v4"
)

const sessionCookieName = "porthannis_session"

var (
	sessions   = map[string]bool{}
	sessionsMu sync.RWMutex
)

// setupAuth 注册认证路由和中间件
func (s *Server) setupAuth() {
	s.echo.POST("/api/login", s.handleLogin)
	s.echo.POST("/api/logout", s.handleLogout)

	// 认证中间件：检查除 login/logout 外的所有 /api/* 请求
	s.echo.Use(func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c echo.Context) error {
			path := c.Request().URL.Path
			if path == "/api/login" || path == "/api/logout" {
				return next(c)
			}
			if !s.pathNeedsAuth(path) {
				return next(c)
			}
			if !s.isAuthenticated(c) {
				return c.JSON(http.StatusUnauthorized, map[string]string{"error": "unauthorized"})
			}
			return next(c)
		}
	})
}

// pathNeedsAuth 判断路径是否需要认证
func (s *Server) pathNeedsAuth(path string) bool {
	return len(path) >= 5 && path[:5] == "/api/"
}

// isAuthenticated 检查请求是否已认证
func (s *Server) isAuthenticated(c echo.Context) bool {
	password := s.getPassword()
	if password == "" {
		return true
	}

	cookie, err := c.Cookie(sessionCookieName)
	if err != nil {
		return false
	}

	sessionsMu.RLock()
	ok := sessions[cookie.Value]
	sessionsMu.RUnlock()
	return ok
}

// handleLogin 处理登录
func (s *Server) handleLogin(c echo.Context) error {
	password := s.getPassword()
	if password == "" {
		return c.JSON(http.StatusOK, map[string]string{"status": "ok"})
	}

	var body struct {
		Password string `json:"password"`
	}
	if err := c.Bind(&body); err != nil {
		return c.JSON(http.StatusBadRequest, map[string]string{"error": "invalid request"})
	}

	if subtle.ConstantTimeCompare([]byte(body.Password), []byte(password)) != 1 {
		return c.JSON(http.StatusUnauthorized, map[string]string{"error": "wrong password"})
	}

	token := generateToken()
	sessionsMu.Lock()
	sessions[token] = true
	sessionsMu.Unlock()

	c.SetCookie(&http.Cookie{
		Name:     sessionCookieName,
		Value:    token,
		Path:     "/",
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
	})

	return c.JSON(http.StatusOK, map[string]string{"status": "ok"})
}

// handleLogout 处理登出
func (s *Server) handleLogout(c echo.Context) error {
	cookie, err := c.Cookie(sessionCookieName)
	if err == nil {
		sessionsMu.Lock()
		delete(sessions, cookie.Value)
		sessionsMu.Unlock()
	}

	c.SetCookie(&http.Cookie{
		Name:     sessionCookieName,
		Value:    "",
		Path:     "/",
		HttpOnly: true,
		MaxAge:   -1,
	})

	return c.JSON(http.StatusOK, map[string]string{"status": "ok"})
}

// getPassword 获取当前配置的密码
func (s *Server) getPassword() string {
	cfg := s.app.GetWebUI()
	return cfg.Password
}

// generateToken 生成随机 session token
func generateToken() string {
	b := make([]byte, 32)
	rand.Read(b)
	return hex.EncodeToString(b)
}
