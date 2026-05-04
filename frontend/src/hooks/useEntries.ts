import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '../api/client';
import type { EntryRequest } from '../api/types';

export function useEntries() {
  return useQuery({
    queryKey: ['entries'],
    queryFn: api.listEntries,
    refetchInterval: 5000,
  });
}

export function useEntry(id: string | undefined) {
  return useQuery({
    queryKey: ['entries', id],
    queryFn: () => api.getEntry(id!),
    enabled: !!id,
  });
}

export function useEntryStatus(id: string | undefined) {
  return useQuery({
    queryKey: ['entries', id, 'status'],
    queryFn: () => api.getEntryStatus(id!),
    enabled: !!id,
    refetchInterval: 3000,
  });
}

export function useCreateEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (req: EntryRequest) => api.createEntry(req),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['entries'] }),
  });
}

export function useUpdateEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, req }: { id: string; req: EntryRequest }) =>
      api.updateEntry(id, req),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['entries'] }),
  });
}

export function useDeleteEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, cleanupLogs }: { id: string; cleanupLogs?: boolean }) =>
      api.deleteEntry(id, cleanupLogs),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['entries'] }),
  });
}

export function useStartEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.startEntry(id),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['entries'] });
    },
  });
}

export function useStopEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => api.stopEntry(id),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['entries'] });
    },
  });
}
