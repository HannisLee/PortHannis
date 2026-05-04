import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useEntries, useDeleteEntry, useStartEntry, useStopEntry } from '../hooks/useEntries';
import { isEntryRunning } from '../api/types';
import { StatusBadge } from '../components/StatusBadge';
import { ConfirmDialog } from '../components/ConfirmDialog';
import { EmptyState } from '../components/EmptyState';
import { toast } from '../components/Toast';

export function EntryList() {
  const { data: entries, isLoading } = useEntries();
  const deleteMut = useDeleteEntry();
  const startMut = useStartEntry();
  const stopMut = useStopEntry();
  const navigate = useNavigate();
  const [deleteId, setDeleteId] = useState<string | null>(null);

  const handleDelete = async () => {
    if (!deleteId) return;
    try {
      await deleteMut.mutateAsync({ id: deleteId, cleanupLogs: true });
      toast('删除成功', 'success');
    } catch {
      toast('删除失败', 'error');
    }
    setDeleteId(null);
  };

  if (isLoading) {
    return <div style={{ padding: 48, textAlign: 'center', color: '#9ca3af' }}>加载中...</div>;
  }

  const list = entries ?? [];

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h2 style={{ margin: 0, fontSize: 20, fontWeight: 600 }}>端口转发</h2>
        <button onClick={() => navigate('/entries/new')} style={{
          padding: '8px 18px', borderRadius: 6, border: 'none',
          background: '#3b82f6', color: '#fff', cursor: 'pointer',
          fontSize: 14, fontWeight: 500,
        }}>
          + 新建
        </button>
      </div>

      {list.length === 0 ? (
        <EmptyState title="暂无端口转发条目" subtitle="点击右上角「新建」按钮创建第一个条目" />
      ) : (
        <table style={{
          width: '100%', marginTop: 16, borderCollapse: 'collapse',
          background: '#fff', borderRadius: 8,
          boxShadow: '0 1px 3px rgba(0,0,0,0.06)',
        }}>
          <thead>
            <tr style={{ borderBottom: '1px solid #e5e7eb', fontSize: 13, color: '#6b7280' }}>
              <th style={thStyle}>名称</th>
              <th style={thStyle}>源地址</th>
              <th style={thStyle}>目标地址</th>
              <th style={thStyle}>状态</th>
              <th style={{ ...thStyle, textAlign: 'right' }}>操作</th>
            </tr>
          </thead>
          <tbody>
            {list.map((e) => (
              <tr key={e.id} style={{ borderBottom: '1px solid #f3f4f6', fontSize: 14 }}>
                <td style={tdStyle}>
                  <span style={{ fontWeight: 500, cursor: 'pointer' }}
                    onClick={() => navigate(`/entries/${e.id}`)}>
                    {e.name}
                  </span>
                </td>
                <td style={tdStyle}>{e.source_address}:{e.source_port}</td>
                <td style={tdStyle}>{e.target_address}:{e.target_port}</td>
                <td style={tdStyle}>
                  <StatusBadge status={e.status} />
                </td>
                <td style={{ ...tdStyle, textAlign: 'right' }}>
                  <div style={{ display: 'flex', gap: 4, justifyContent: 'flex-end' }}>
                    {isEntryRunning(e.status) ? (
                      <button onClick={() => stopMut.mutate(e.id)} style={actionBtnStyle('#f59e0b')}>
                        停止
                      </button>
                    ) : (
                      <button onClick={() => startMut.mutate(e.id)} style={actionBtnStyle('#22c55e')}>
                        启动
                      </button>
                    )}
                    <button onClick={() => navigate(`/entries/${e.id}`)} style={actionBtnStyle('#3b82f6')}>
                      编辑
                    </button>
                    <button onClick={() => setDeleteId(e.id)} style={actionBtnStyle('#ef4444')}>
                      删除
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      <ConfirmDialog
        open={!!deleteId}
        title="确认删除"
        message="删除后将同时清理该条目下的所有日志文件，此操作不可撤销。"
        onConfirm={handleDelete}
        onCancel={() => setDeleteId(null)}
      />
    </div>
  );
}

const thStyle: React.CSSProperties = { padding: '10px 14px', textAlign: 'left', fontWeight: 500 };
const tdStyle: React.CSSProperties = { padding: '10px 14px' };

function actionBtnStyle(color: string): React.CSSProperties {
  return {
    padding: '4px 10px', borderRadius: 4, border: 'none',
    background: color, color: '#fff', cursor: 'pointer',
    fontSize: 12, fontWeight: 500, whiteSpace: 'nowrap',
  };
}
