import { useState } from 'react';

interface ToastItem {
  id: number;
  message: string;
  type: 'success' | 'error' | 'info';
}

let toastId = 0;
let addToastFn: ((msg: string, type: ToastItem['type']) => void) | null = null;

export function toast(message: string, type: ToastItem['type'] = 'info') {
  addToastFn?.(message, type);
}

export function ToastContainer() {
  const [toasts, setToasts] = useState<ToastItem[]>([]);

  addToastFn = (message: string, type: ToastItem['type']) => {
    const id = ++toastId;
    setToasts((prev) => [...prev, { id, message, type }]);
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 4000);
  };

  return (
    <div style={{
      position: 'fixed', top: 16, right: 16, zIndex: 9999,
      display: 'flex', flexDirection: 'column', gap: 8,
    }}>
      {toasts.map((t) => (
        <div key={t.id} style={{
          padding: '10px 18px',
          borderRadius: 6,
          color: '#fff',
          fontSize: 14,
          boxShadow: '0 2px 8px rgba(0,0,0,0.2)',
          background: t.type === 'success' ? '#22c55e' : t.type === 'error' ? '#ef4444' : '#3b82f6',
          animation: 'slideIn 0.3s ease',
        }}>
          {t.message}
        </div>
      ))}
    </div>
  );
}
