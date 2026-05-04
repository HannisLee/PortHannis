import { useState, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { useEntry, useCreateEntry, useUpdateEntry } from '../hooks/useEntries';
import type { EntryRequest } from '../api/types';
import { toast } from '../components/Toast';

export function EntryForm() {
  const { id } = useParams<{ id: string }>();
  const isEdit = !!id;
  const { data: existing } = useEntry(id);
  const createMut = useCreateEntry();
  const updateMut = useUpdateEntry();
  const navigate = useNavigate();

  const [form, setForm] = useState<EntryRequest>({
    name: '',
    source_address: '0.0.0.0',
    source_port: 8080,
    target_address: '',
    target_port: 80,
    log_directory: '',
    enabled: true,
  });
  const [errors, setErrors] = useState<Record<string, string>>({});

  useEffect(() => {
    if (existing) {
      setForm({
        name: existing.name,
        source_address: existing.source_address,
        source_port: existing.source_port,
        target_address: existing.target_address,
        target_port: existing.target_port,
        log_directory: existing.log_directory ?? '',
        enabled: existing.enabled,
      });
    }
  }, [existing]);

  const validate = (): boolean => {
    const e: Record<string, string> = {};
    if (!form.name.trim()) e.name = '名称不能为空';
    if (form.source_port < 1 || form.source_port > 65535) e.source_port = '端口范围 1-65535';
    if (!form.target_address.trim()) e.target_address = '目标地址不能为空';
    if (form.target_port < 1 || form.target_port > 65535) e.target_port = '端口范围 1-65535';
    setErrors(e);
    return Object.keys(e).length === 0;
  };

  const handleSubmit = async () => {
    if (!validate()) return;
    try {
      if (isEdit) {
        await updateMut.mutateAsync({ id: id!, req: form });
        toast('更新成功', 'success');
      } else {
        await createMut.mutateAsync(form);
        toast('创建成功', 'success');
      }
      navigate('/entries');
    } catch {
      toast(isEdit ? '更新失败' : '创建失败', 'error');
    }
  };

  const field = (label: string, key: keyof EntryRequest, type = 'text', placeholder = '') => (
    <div style={{ marginBottom: 16 }}>
      <label style={{ display: 'block', fontSize: 13, fontWeight: 500, marginBottom: 4, color: '#374151' }}>
        {label}
      </label>
      <input
        type={type}
        value={String(form[key] ?? '')}
        placeholder={placeholder}
        onChange={(ev) => {
          const val = type === 'number' ? Number(ev.target.value) || 0 : ev.target.value;
          setForm((f) => ({ ...f, [key]: val }));
        }}
        style={{
          width: '100%', padding: '8px 10px', borderRadius: 6,
          border: errors[key] ? '1px solid #ef4444' : '1px solid #d1d5db',
          fontSize: 14, boxSizing: 'border-box', outline: 'none',
        }}
      />
      {errors[key] && <span style={{ color: '#ef4444', fontSize: 12 }}>{errors[key]}</span>}
    </div>
  );

  return (
    <div style={{ maxWidth: 560 }}>
      <h2 style={{ margin: '0 0 20px', fontSize: 20, fontWeight: 600 }}>
        {isEdit ? '编辑转发条目' : '新建转发条目'}
      </h2>

      <div style={{ background: '#fff', borderRadius: 8, padding: 24, boxShadow: '0 1px 3px rgba(0,0,0,0.06)' }}>
        {field('名称', 'name', 'text', '例如：数据库转发')}
        {field('源监听地址', 'source_address', 'text')}
        {field('源监听端口', 'source_port', 'number')}
        {field('目标地址', 'target_address', 'text')}
        {field('目标端口', 'target_port', 'number')}
        {field('日志目录 (可选)', 'log_directory', 'text', '留空则自动生成')}

        <div style={{ marginBottom: 16 }}>
          <label style={{ display: 'flex', alignItems: 'center', gap: 8, cursor: 'pointer', fontSize: 14 }}>
            <input
              type="checkbox"
              checked={form.enabled}
              onChange={(ev) => setForm((f) => ({ ...f, enabled: ev.target.checked }))}
            />
            <span>启用该转发</span>
          </label>
        </div>

        <div style={{ display: 'flex', gap: 8, justifyContent: 'flex-end', marginTop: 24 }}>
          <button onClick={() => navigate('/entries')} style={{
            padding: '8px 18px', borderRadius: 6, border: '1px solid #d1d5db',
            background: '#fff', cursor: 'pointer', fontSize: 14,
          }}>
            取消
          </button>
          <button onClick={handleSubmit} style={{
            padding: '8px 18px', borderRadius: 6, border: 'none',
            background: '#3b82f6', color: '#fff', cursor: 'pointer', fontSize: 14,
            fontWeight: 500,
          }}>
            {isEdit ? '保存修改' : '创建条目'}
          </button>
        </div>
      </div>
    </div>
  );
}
