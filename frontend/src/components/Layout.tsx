import { Outlet } from 'react-router-dom';
import { Sidebar } from './Sidebar';

export function Layout() {
  return (
    <div style={{ display: 'flex', height: '100vh', overflow: 'hidden' }}>
      <Sidebar />
      <main style={{
        flex: 1, overflow: 'auto',
        background: '#f8fafc', padding: '24px 32px',
      }}>
        <Outlet />
      </main>
    </div>
  );
}
