import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useEntries } from '../hooks/useEntries';
import { api } from '../api/client';
import { isEntryRunning, isEntryError, type EntryStatus } from '../api/types';
import { StatusBadge } from '../components/StatusBadge';
import { EmptyState } from '../components/EmptyState';

export function Dashboard() {
  const { data: entries, isLoading } = useEntries();
  const [statuses, setStatuses] = useState<Record<string, EntryStatus>>({});
  const navigate = useNavigate();

  useEffect(() => {
    if (!entries) return;
    const fetchStatuses = async () => {
      const map: Record<string, EntryStatus> = {};
      await Promise.all(
        entries.map(async (e) => {
          try {
            map[e.id] = await api.getEntryStatus(e.id);
          } catch {
            map[e.id] = { stopped: {} };
          }
        })
      );
      setStatuses(map);
    };
    fetchStatuses();
    const interval = setInterval(fetchStatuses, 4000);
    return () => clearInterval(interval);
  }, [entries]);

  if (isLoading) {
    return <div style={{ padding: 48, textAlign: 'center', color: '#9ca3af' }}>加载中...</div>;
  }

  const list = entries ?? [];
  const runningCount = list.filter((e) => isEntryRunning(statuses[e.id])).length;
  const errorCount = list.filter((e) => isEntryError(statuses[e.id])).length;
  const stoppedCount = list.length - runningCount - errorCount;

  const cards = [
    { label: '总数', value: list.length, color: '#3b82f6' },
    { label: '运行中', value: runningCount, color: '#22c55e' },
    { label: '已停止', value: stoppedCount, color: '#9ca3af' },
    { label: '错误', value: errorCount, color: '#ef4444' },
  ];

  return (
    <div>
      <h2 style={{ margin: 0, fontSize: 20, fontWeight: 600 }}>仪表盘</h2>

      <div style={{ display: 'flex', gap: 16, marginTop: 20 }}>
        {cards.map((c) => (
          <div key={c.label} style={{
            flex: 1, background: '#fff', borderRadius: 8,
            padding: '16px 20px', boxShadow: '0 1px 3px rgba(0,0,0,0.06)',
          }}>
            <div style={{ fontSize: 13, color: '#6b7280' }}>{c.label}</div>
            <div style={{ fontSize: 28, fontWeight: 700, color: c.color, marginTop: 4 }}>
              {c.value}
            </div>
          </div>
        ))}
      </div>

      <h3 style={{ marginTop: 32, fontSize: 16, fontWeight: 600 }}>所有条目</h3>

      {list.length === 0 ? (
        <EmptyState title="暂无端口转发条目" subtitle="在「端口转发」页面创建第一个条目" />
      ) : (
        <div style={{ marginTop: 12 }}>
          {list.map((e) => (
            <div key={e.id} onClick={() => navigate(`/entries/${e.id}`)} style={{
              display: 'flex', alignItems: 'center', justifyContent: 'space-between',
              background: '#fff', borderRadius: 8, padding: '12px 16px',
              marginBottom: 8, cursor: 'pointer',
              boxShadow: '0 1px 3px rgba(0,0,0,0.04)',
              transition: 'box-shadow 0.15s',
            }}
              onMouseEnter={(ev) => ev.currentTarget.style.boxShadow = '0 2px 8px rgba(0,0,0,0.08)'}
              onMouseLeave={(ev) => ev.currentTarget.style.boxShadow = '0 1px 3px rgba(0,0,0,0.04)'}
            >
              <div>
                <div style={{ fontWeight: 600, fontSize: 14 }}>{e.name}</div>
                <div style={{ fontSize: 12, color: '#9ca3af', marginTop: 2 }}>
                  {e.source_address}:{e.source_port} → {e.target_address}:{e.target_port}
                </div>
              </div>
              <StatusBadge status={statuses[e.id]} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
