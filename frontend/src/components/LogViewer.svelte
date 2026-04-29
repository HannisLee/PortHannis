<script lang="ts">
  import type { LogEntry } from '../lib/api'
  import { api } from '../lib/api'
  import { onMount, createEventDispatcher } from 'svelte'

  export let ruleId = ''

  const dispatch = createEventDispatcher<{ close: void }>

  let logs: LogEntry[] = []
  let loading = true
  let container: HTMLDivElement

  async function loadLogs() {
    try {
      logs = await api.getLogs(ruleId, 500)
    } catch (e) {
      console.error('load logs failed:', e)
    } finally {
      loading = false
    }
    scrollToBottom()
  }

  async function handleClear() {
    try {
      await api.clearLogs(ruleId)
      logs = []
    } catch (e) {
      console.error('clear logs failed:', e)
    }
  }

  function scrollToBottom() {
    setTimeout(() => {
      if (container) container.scrollTop = container.scrollHeight
    }, 0)
  }

  function formatTime(ts: string): string {
    const d = new Date(ts)
    return d.toLocaleString('zh-CN', { hour12: false })
  }

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return bytes + ' B'
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
    return (bytes / 1024 / 1024).toFixed(1) + ' MB'
  }

  function statusLabel(status: string): string {
    switch (status) {
      case 'connected': return '已连接'
      case 'closed': return '已断开'
      case 'error': return '错误'
      default: return status
    }
  }

  onMount(loadLogs)
</script>

<div class="fixed inset-0 bg-black/40 flex items-center justify-center z-50" on:click={() => dispatch('close')}>
  <div class="bg-white rounded-lg shadow-xl w-[640px] max-h-[80vh] flex flex-col" on:click|stopPropagation>
    <div class="flex items-center justify-between px-5 py-3 border-b">
      <h2 class="text-sm font-semibold">日志查看</h2>
      <div class="flex gap-2">
        <button on:click={loadLogs} class="text-xs text-gray-500 hover:text-gray-700 px-2 py-1">刷新</button>
        <button on:click={handleClear} class="text-xs text-red-500 hover:text-red-700 px-2 py-1">清空</button>
        <button on:click={() => dispatch('close')} class="text-xs text-gray-400 hover:text-gray-600 px-2 py-1">关闭</button>
      </div>
    </div>

    <div bind:this={container} class="overflow-y-auto flex-1 text-xs font-mono">
      {#if loading}
        <div class="text-center text-gray-400 py-8">加载中...</div>
      {:else if logs.length === 0}
        <div class="text-center text-gray-400 py-8">暂无日志</div>
      {:else}
        <table class="w-full">
          <thead class="sticky top-0 bg-gray-50 text-gray-500">
            <tr>
              <th class="text-left px-4 py-2 font-normal">时间</th>
              <th class="text-left px-4 py-2 font-normal">来源</th>
              <th class="text-right px-4 py-2 font-normal">入站</th>
              <th class="text-right px-4 py-2 font-normal">出站</th>
              <th class="text-left px-4 py-2 font-normal">状态</th>
            </tr>
          </thead>
          <tbody>
            {#each logs as log}
              <tr class="border-t hover:bg-gray-50">
                <td class="px-4 py-1.5 text-gray-600">{formatTime(log.timestamp)}</td>
                <td class="px-4 py-1.5">{log.source}</td>
                <td class="px-4 py-1.5 text-right text-gray-600">{formatBytes(log.bytesIn)}</td>
                <td class="px-4 py-1.5 text-right text-gray-600">{formatBytes(log.bytesOut)}</td>
                <td class="px-4 py-1.5 {log.status === 'error' ? 'text-red-500' : 'text-gray-600'}">{statusLabel(log.status)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      {/if}
    </div>
  </div>
</div>
