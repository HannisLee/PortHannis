import type { ForwardingEntry, EntryRequest, LogResponse, EntryStatus } from './types';

declare global {
  interface Window {
    __PORTHANNIS_API_PORT__?: number;
  }
}

function getBaseUrl(): string {
  const port = window.__PORTHANNIS_API_PORT__;
  if (port) {
    return `http://127.0.0.1:${port}`;
  }
  return '';
}

class ApiError extends Error {
  status: number;
  body: string;

  constructor(status: number, message: string) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.body = message;
  }
}

async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${getBaseUrl()}/api${path}`, {
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    ...options,
  });
  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new ApiError(res.status, body.message ?? res.statusText);
  }
  if (res.status === 204) return undefined as T;
  return res.json();
}

export const api = {
  health: () => request<{ ok: boolean }>('/health'),

  listEntries: () => request<ForwardingEntry[]>('/entries'),

  createEntry: (req: EntryRequest) =>
    request<ForwardingEntry>('/entries', {
      method: 'POST',
      body: JSON.stringify(req),
    }),

  getEntry: (id: string) => request<ForwardingEntry>(`/entries/${id}`),

  updateEntry: (id: string, req: EntryRequest) =>
    request<ForwardingEntry>(`/entries/${id}`, {
      method: 'PUT',
      body: JSON.stringify(req),
    }),

  deleteEntry: (id: string, cleanupLogs?: boolean) =>
    request<void>(`/entries/${id}?cleanup_logs=${cleanupLogs ?? false}`, {
      method: 'DELETE',
    }),

  startEntry: (id: string) =>
    request<EntryStatus>(`/entries/${id}/start`, { method: 'POST' }),

  stopEntry: (id: string) =>
    request<EntryStatus>(`/entries/${id}/stop`, { method: 'POST' }),

  getEntryStatus: (id: string) =>
    request<EntryStatus>(`/entries/${id}/status`),

  getLogs: (id: string, offset = 0, limit = 500) =>
    request<LogResponse>(`/entries/${id}/logs?offset=${offset}&limit=${limit}`),
};

export { ApiError, getBaseUrl };
