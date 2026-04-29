package webui

import (
	"log"
	"net/http"
	"time"

	"github.com/gorilla/websocket"
	"github.com/labstack/echo/v4"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool { return true },
}

// registerWebSocket 注册 WebSocket 端点
func (s *Server) registerWebSocket() {
	s.echo.GET("/api/ws/status", s.handleStatusWS)
}

// handleStatusWS WebSocket 状态推送
func (s *Server) handleStatusWS(c echo.Context) error {
	ws, err := upgrader.Upgrade(c.Response(), c.Request(), nil)
	if err != nil {
		return err
	}
	defer ws.Close()

	done := make(chan struct{})
	go func() {
		defer close(done)
		for {
			if _, _, err := ws.ReadMessage(); err != nil {
				return
			}
		}
	}()

	ticker := time.NewTicker(2 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-done:
			return nil
		case <-ticker.C:
			status := s.app.GetStatus()
			if err := ws.WriteJSON(status); err != nil {
				log.Printf("WebSocket write error: %v", err)
				return nil
			}
		}
	}
}
