interface Props {
  open: boolean;
  title: string;
  message: string;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({ open, title, message, onConfirm, onCancel }: Props) {
  if (!open) return null;

  return (
    <div style={{
      position: 'fixed', inset: 0, zIndex: 1000,
      display: 'flex', alignItems: 'center', justifyContent: 'center',
      backgroundColor: 'rgba(0,0,0,0.4)',
    }}>
      <div style={{
        background: '#fff', borderRadius: 8, padding: 24,
        minWidth: 360, boxShadow: '0 4px 24px rgba(0,0,0,0.15)',
      }}>
        <h3 style={{ margin: 0, fontSize: 16, fontWeight: 600 }}>{title}</h3>
        <p style={{ color: '#6b7280', fontSize: 14, marginTop: 8 }}>{message}</p>
        <div style={{ display: 'flex', justifyContent: 'flex-end', gap: 8, marginTop: 20 }}>
          <button onClick={onCancel} style={{
            padding: '6px 16px', borderRadius: 6, border: '1px solid #d1d5db',
            background: '#fff', cursor: 'pointer', fontSize: 14,
          }}>
            取消
          </button>
          <button onClick={onConfirm} style={{
            padding: '6px 16px', borderRadius: 6, border: 'none',
            background: '#ef4444', color: '#fff', cursor: 'pointer', fontSize: 14,
          }}>
            确认
          </button>
        </div>
      </div>
    </div>
  );
}
