import type { EntryStatus } from '../api/types';
import { getStatusLabel, isEntryRunning, isEntryError } from '../api/types';

interface Props {
  status?: EntryStatus;
}

export function StatusBadge({ status }: Props) {
  const running = isEntryRunning(status);
  const error = isEntryError(status);
  const color = running ? '#22c55e' : error ? '#ef4444' : '#9ca3af';
  const pulse = running;

  return (
    <span style={{
      display: 'inline-flex', alignItems: 'center', gap: 6,
      fontSize: 13, fontWeight: 500,
    }}>
      <span style={{
        width: 8, height: 8, borderRadius: '50%',
        backgroundColor: color,
        animation: pulse ? 'pulse 1.5s infinite' : undefined,
      }} />
      {getStatusLabel(status)}
    </span>
  );
}
