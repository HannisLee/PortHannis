import { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useLogs } from '../hooks/useLogs';
import { useEntry, useEntryStatus, useStartEntry, useStopEntry } from '../hooks/useEntries';
import { LogViewer } from '../components/LogViewer';
import { StatusBadge } from '../components/StatusBadge';
import { isEntryRunning } from '../api/types';
import { toast } from '../components/Toast';

export function LogPage() {
  const { id } = useParams<{ id: string }>();
  const { data: entry } = useEntry(id);
  const { data: status } = useEntryStatus(id);
  const [offset, setOffset] = useState(0);
  const { data: logData, isLoading } = useLogs(id, offset);
  const startMut = useStartEntry();
  const stopMut = useStopEntry();
  const navigate = useNavigate();

  const lines = logData?.lines ?? [];
  const hasMore = logData ? lines.length < (logData.total_bytes > 0 ? 10000 : 0) || lines.length >= 500 : false;

  const handleToggle = async () => {
    if (!id) return;
    try {
      if (isEntryRunning(status)) {
        await stopMut.mutateAsync(id);
        toast('已停止转发', 'success');
      } else {
        await startMut.mutateAsync(id);
        toast('已启动转发', 'success');
      }
    } catch {
      toast('操作失败', 'error');
    }
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: 'calc(100vh - 100px)' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 16 }}>
        <div>
          <h2 style={{ margin: 0, fontSize: 20, fontWeight: 600 }}>
            {entry?.name ?? '日志'}
          </h2>
          <div style={{ fontSize: 12, color: '#9ca3af', marginTop: 2 }}>
            {entry && `${entry.source_address}:${entry.source_port} → ${entry.target_address}:${entry.target_port}`}
          </div>
        </div>
        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
          <StatusBadge status={status} />
          <button onClick={handleToggle} style={{
            padding: '6px 16px', borderRadius: 6, border: 'none',
            background: isEntryRunning(status) ? '#ef4444' : '#22c55e',
            color: '#fff', cursor: 'pointer', fontSize: 13, fontWeight: 500,
          }}>
            {isEntryRunning(status) ? '停止' : '启动'}
          </button>
          <button onClick={() => navigate(`/entries/${id}`)} style={{
            padding: '6px 16px', borderRadius: 6, border: '1px solid #d1d5db',
            background: '#fff', cursor: 'pointer', fontSize: 13,
          }}>
            编辑
          </button>
        </div>
      </div>

      <div style={{ flex: 1 }}>
        <LogViewer
          lines={lines}
          loading={isLoading}
          hasMore={hasMore}
          onLoadMore={() => setOffset((o) => o + 500)}
        />
      </div>

      {logData && (
        <div style={{ fontSize: 11, color: '#9ca3af', marginTop: 8, textAlign: 'right' }}>
          日志大小: {formatBytes(logData.total_bytes)} / {formatBytes(logData.max_bytes)}
        </div>
      )}
    </div>
  );
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}
