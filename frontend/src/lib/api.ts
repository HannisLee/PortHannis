// 检测运行模式：Wails 桌面 or 浏览器 WebUI
const isWails = typeof window !== 'undefined' && !!(window as any).__WAILS__

export interface ForwardRule {
  id: string
  localPort: number
  targetHost: string
  targetPort: number
  enabled: boolean
  createdAt: string
}

export interface LogEntry {
  timestamp: string
  source: string
  bytesIn: number
  bytesOut: number
  status: string
}

export interface WebUIConfig {
  enabled: boolean
  port: number
  password: string
}

// ============ Wails 模式 ============
async function wailsGetRules(): Promise<ForwardRule[]> {
  const { GetRules } = await import('../../wailsjs/go/main/App')
  return GetRules()
}
async function wailsAddRule(localPort: number, targetHost: string, targetPort: number): Promise<void> {
  const { AddRule } = await import('../../wailsjs/go/main/App')
  return AddRule(localPort, targetHost, targetPort)
}
async function wailsDeleteRule(id: string): Promise<void> {
  const { DeleteRule } = await import('../../wailsjs/go/main/App')
  return DeleteRule(id)
}
async function wailsToggleRule(id: string, enabled: boolean): Promise<void> {
  const { ToggleRule } = await import('../../wailsjs/go/main/App')
  return ToggleRule(id, enabled)
}
async function wailsGetLogs(ruleId: string, limit: number): Promise<LogEntry[]> {
  const { GetLogs } = await import('../../wailsjs/go/main/App')
  return GetLogs(ruleId, limit)
}
async function wailsClearLogs(ruleId: string): Promise<void> {
  const { ClearLogs } = await import('../../wailsjs/go/main/App')
  return ClearLogs(ruleId)
}
async function wailsGetStatus(): Promise<{ [key: string]: boolean }> {
  const { GetStatus } = await import('../../wailsjs/go/main/App')
  return GetStatus()
}

// ============ HTTP 模式 ============
async function httpRequest<T>(path: string, opts?: RequestInit): Promise<T> {
  const res = await fetch(path, {
    headers: { 'Content-Type': 'application/json', ...opts?.headers },
    ...opts,
  })
  if (res.status === 401) {
    throw new Error('UNAUTHORIZED')
  }
  if (!res.ok) {
    const body = await res.json().catch(() => ({}))
    throw new Error(body.error || res.statusText)
  }
  return res.json()
}

function httpGetRules(): Promise<ForwardRule[]> {
  return httpRequest<ForwardRule[]>('/api/rules')
}
function httpAddRule(localPort: number, targetHost: string, targetPort: number): Promise<void> {
  return httpRequest<void>('/api/rules', {
    method: 'POST',
    body: JSON.stringify({ localPort, targetHost, targetPort }),
  })
}
function httpDeleteRule(id: string): Promise<void> {
  return httpRequest<void>(`/api/rules/${id}`, { method: 'DELETE' })
}
function httpToggleRule(id: string, enabled: boolean): Promise<void> {
  return httpRequest<void>(`/api/rules/${id}/toggle`, {
    method: 'PUT',
    body: JSON.stringify({ enabled }),
  })
}
function httpGetLogs(ruleId: string, limit: number): Promise<LogEntry[]> {
  return httpRequest<LogEntry[]>(`/api/rules/${ruleId}/logs?limit=${limit}`)
}
function httpClearLogs(ruleId: string): Promise<void> {
  return httpRequest<void>(`/api/rules/${ruleId}/logs`, { method: 'DELETE' })
}
function httpGetStatus(): Promise<{ [key: string]: boolean }> {
  return httpRequest<{ [key: string]: boolean }>('/api/status')
}
function httpLogin(password: string): Promise<void> {
  return httpRequest<void>('/api/login', {
    method: 'POST',
    body: JSON.stringify({ password }),
  })
}
function httpLogout(): Promise<void> {
  return httpRequest<void>('/api/logout', { method: 'POST' })
}
function httpGetWebUIConfig(): Promise<WebUIConfig> {
  return httpRequest<WebUIConfig>('/api/webui-config')
}
function httpUpdateWebUIConfig(cfg: Partial<WebUIConfig>): Promise<void> {
  return httpRequest<void>('/api/webui-config', {
    method: 'PUT',
    body: JSON.stringify(cfg),
  })
}

// ============ 统一导出 ============
export const api = isWails
  ? {
      getRules: wailsGetRules,
      addRule: wailsAddRule,
      deleteRule: wailsDeleteRule,
      toggleRule: wailsToggleRule,
      getLogs: wailsGetLogs,
      clearLogs: wailsClearLogs,
      getStatus: wailsGetStatus,
      login: async (_password: string) => {},
      logout: async () => {},
      getWebUIConfig: async () => ({ enabled: true, port: 18080, password: '' }) as WebUIConfig,
      updateWebUIConfig: async (_cfg: Partial<WebUIConfig>) => {},
      isWailsMode: true as const,
    }
  : {
      getRules: httpGetRules,
      addRule: httpAddRule,
      deleteRule: httpDeleteRule,
      toggleRule: httpToggleRule,
      getLogs: httpGetLogs,
      clearLogs: httpClearLogs,
      getStatus: httpGetStatus,
      login: httpLogin,
      logout: httpLogout,
      getWebUIConfig: httpGetWebUIConfig,
      updateWebUIConfig: httpUpdateWebUIConfig,
      isWailsMode: false as const,
    }
