package forwarder

import (
	"fmt"
	"path/filepath"
	"sync"

	"github.com/HannisLee/PortHannis/config"
)

type Engine struct {
	mu       sync.RWMutex
	rules    map[string]*RuleState
	loggers  map[string]*CircularLogger
	manager  *config.Manager
}

func NewEngine(mgr *config.Manager) *Engine {
	return &Engine{
		rules:   make(map[string]*RuleState),
		loggers: make(map[string]*CircularLogger),
		manager: mgr,
	}
}

func (e *Engine) StartRule(id string) error {
	e.mu.Lock()
	defer e.mu.Unlock()

	if _, ok := e.rules[id]; ok {
		return fmt.Errorf("rule %s is already running", id)
	}

	var rule config.ForwardRule
	found := false
	for _, r := range e.manager.GetConfig().Rules {
		if r.ID == id {
			rule = r
			found = true
			break
		}
	}
	if !found {
		return fmt.Errorf("rule %s not found", id)
	}

	logPath := filepath.Join(config.GetLogsDir(), id+".log")
	logger, err := NewCircularLogger(logPath)
	if err != nil {
		return fmt.Errorf("failed to create logger: %w", err)
	}

	state := NewRuleState(rule, logger)
	if err := state.Start(); err != nil {
		logger.Close()
		return err
	}

	e.rules[id] = state
	e.loggers[id] = logger
	return nil
}

func (e *Engine) StopRule(id string) error {
	e.mu.Lock()
	defer e.mu.Unlock()

	state, ok := e.rules[id]
	if !ok {
		return fmt.Errorf("rule %s is not running", id)
	}

	state.Stop()
	delete(e.rules, id)

	if logger, ok := e.loggers[id]; ok {
		logger.Close()
		delete(e.loggers, id)
	}
	return nil
}

func (e *Engine) RestartRule(id string) error {
	if err := e.StopRule(id); err != nil {
		return err
	}
	return e.StartRule(id)
}

func (e *Engine) GetStatus() map[string]bool {
	e.mu.RLock()
	defer e.mu.RUnlock()

	status := make(map[string]bool)
	for id, state := range e.rules {
		status[id] = state.Running()
	}
	return status
}

func (e *Engine) GetLogs(ruleID string, limit int) ([]config.LogEntry, error) {
	e.mu.RLock()
	logger, ok := e.loggers[ruleID]
	e.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("no logger for rule %s", ruleID)
	}
	return logger.ReadLogs(limit)
}

func (e *Engine) ClearLogs(ruleID string) error {
	e.mu.RLock()
	logger, ok := e.loggers[ruleID]
	e.mu.RUnlock()

	if !ok {
		return fmt.Errorf("no logger for rule %s", ruleID)
	}
	return logger.Clear()
}

func (e *Engine) StopAll() {
	e.mu.Lock()
	defer e.mu.Unlock()

	for id, state := range e.rules {
		state.Stop()
		delete(e.rules, id)
	}
	for id, logger := range e.loggers {
		logger.Close()
		delete(e.loggers, id)
	}
}
