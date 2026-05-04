import { NavLink } from 'react-router-dom';

export function Sidebar() {
  const linkStyle = (isActive: boolean): React.CSSProperties => ({
    display: 'flex', alignItems: 'center', gap: 8,
    padding: '8px 16px', borderRadius: 6,
    fontSize: 14, fontWeight: 500,
    textDecoration: 'none',
    color: isActive ? '#fff' : '#94a3b8',
    background: isActive ? '#3b82f6' : 'transparent',
    transition: 'all 0.15s',
  });

  return (
    <aside style={{
      width: 200, minWidth: 200,
      background: '#0f172a', color: '#e2e8f0',
      display: 'flex', flexDirection: 'column',
      padding: '16px 12px', gap: 4,
    }}>
      <div style={{
        fontSize: 16, fontWeight: 700, padding: '8px 16px',
        marginBottom: 16, letterSpacing: '-0.02em',
      }}>
        ⚡ PortHannis
      </div>

      <NavLink to="/" end style={({ isActive }) => linkStyle(isActive)}>
        <span>📊</span> 仪表盘
      </NavLink>
      <NavLink to="/entries" style={({ isActive }) => linkStyle(isActive)}>
        <span>🔀</span> 端口转发
      </NavLink>
    </aside>
  );
}
