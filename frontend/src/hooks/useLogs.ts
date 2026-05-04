import { useQuery } from '@tanstack/react-query';
import { api } from '../api/client';

export function useLogs(id: string | undefined, offset = 0, limit = 500) {
  return useQuery({
    queryKey: ['logs', id, offset, limit],
    queryFn: () => api.getLogs(id!, offset, limit),
    enabled: !!id,
    refetchInterval: 3000,
  });
}
