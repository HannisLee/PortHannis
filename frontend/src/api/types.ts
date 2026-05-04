export type EntryId = string;

export type EntryStatus =
  | { running: Record<string, never> }
  | { stopped: Record<string, never> }
  | { error: { message: string } };

export interface ForwardingEntry {
  id: EntryId;
  name: string;
  source_address: string;
  source_port: number;
  target_address: string;
  target_port: number;
  log_directory: string;
  enabled: boolean;
  created_at: string;
  updated_at: string;
  status?: EntryStatus;
}

export interface EntryRequest {
  name: string;
  source_address: string;
  source_port: number;
  target_address: string;
  target_port: number;
  log_directory?: string;
  enabled: boolean;
}

export interface LogLine {
  timestamp: string;
  level: string;
  message: string;
}

export interface LogResponse {
  entry_id: EntryId;
  lines: LogLine[];
  total_bytes: number;
  max_bytes: number;
}

export interface ApiErrorBody {
  error: string;
  message: string;
}

export function isEntryRunning(status?: EntryStatus): boolean {
  if (!status) return false;
  return 'running' in status;
}

export function isEntryError(status?: EntryStatus): boolean {
  if (!status) return false;
  return 'error' in status;
}

export function getStatusLabel(status?: EntryStatus): string {
  if (!status) return '已停止';
  if ('running' in status) return '运行中';
  if ('stopped' in status) return '已停止';
  if ('error' in status) return '错误';
  return '未知';
}
