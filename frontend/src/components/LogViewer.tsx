import { useEffect, useRef, useState } from 'react';
import type { LogLine } from '../api/types';

interface Props {
  lines: LogLine[];
  loading?: boolean;
  onLoadMore?: () => void;
  hasMore?: boolean;
}

export function LogViewer({ lines, loading, onLoadMore, hasMore }: Props) {
  const bottomRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [autoScroll, setAutoScroll] = useState(true);
  const prevLen = useRef(lines.length);

  useEffect(() => {
    if (autoScroll && lines.length > prevLen.current) {
      bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }
    prevLen.current = lines.length;
  }, [lines, autoScroll]);

  const handleScroll = () => {
    const el = containerRef.current;
    if (!el) return;
    const atBottom = el.scrollHeight - el.scrollTop - el.clientHeight < 40;
    setAutoScroll(atBottom);
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
      <div
        ref={containerRef}
        onScroll={handleScroll}
        style={{
          flex: 1, overflow: 'auto',
          background: '#1e1e2e', color: '#cdd6f4',
          fontFamily: 'Consolas, "Courier New", monospace',
          fontSize: 13, lineHeight: 1.6, padding: '8px 0',
          borderRadius: 6, minHeight: 300,
        }}
      >
        {lines.length === 0 && !loading && (
          <div style={{ padding: 24, textAlign: 'center', color: '#6c7086' }}>
            暂无日志
          </div>
        )}
        {lines.map((line, i) => {
          const isError = line.level === 'error';
          const ts = line.timestamp.replace('T', ' ').substring(0, 23);
          return (
            <div key={i} style={{
              padding: '1px 12px',
              color: isError ? '#f38ba8' : '#cdd6f4',
              background: isError ? 'rgba(243,139,168,0.08)' : undefined,
            }}>
              <span style={{ color: '#6c7086', marginRight: 8 }}>{ts}</span>
              <span style={{
                display: 'inline-block', width: 40,
                color: isError ? '#f38ba8' : '#a6e3a1',
                fontWeight: 600,
              }}>
                [{line.level.toUpperCase()}]
              </span>
              <span>{line.message}</span>
            </div>
          );
        })}
        {hasMore && (
          <div style={{ textAlign: 'center', padding: 8 }}>
            <button
              onClick={onLoadMore}
              disabled={loading}
              style={{
                background: 'transparent', border: '1px solid #45475a',
                color: '#cdd6f4', borderRadius: 4, padding: '4px 12px',
                cursor: 'pointer', fontSize: 12,
              }}
            >
              {loading ? '加载中...' : '加载更多'}
            </button>
          </div>
        )}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
