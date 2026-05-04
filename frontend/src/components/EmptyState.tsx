interface Props {
  icon?: string;
  title: string;
  subtitle?: string;
}

export function EmptyState({ icon = '📋', title, subtitle }: Props) {
  return (
    <div style={{
      display: 'flex', flexDirection: 'column', alignItems: 'center',
      justifyContent: 'center', padding: 48, color: '#9ca3af',
    }}>
      <span style={{ fontSize: 40, marginBottom: 12 }}>{icon}</span>
      <span style={{ fontSize: 15, fontWeight: 500 }}>{title}</span>
      {subtitle && <span style={{ fontSize: 13, marginTop: 4 }}>{subtitle}</span>}
    </div>
  );
}
