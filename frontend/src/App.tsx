import { Routes, Route, Navigate } from 'react-router-dom';
import { Layout } from './components/Layout';
import { Dashboard } from './pages/Dashboard';
import { EntryList } from './pages/EntryList';
import { EntryForm } from './pages/EntryForm';
import { LogPage } from './pages/LogPage';

export default function App() {
  return (
    <Routes>
      <Route element={<Layout />}>
        <Route index element={<Dashboard />} />
        <Route path="entries" element={<EntryList />} />
        <Route path="entries/new" element={<EntryForm />} />
        <Route path="entries/:id" element={<EntryForm />} />
        <Route path="entries/:id/logs" element={<LogPage />} />
      </Route>
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}
