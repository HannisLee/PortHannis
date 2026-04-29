<script lang="ts">
  import { api } from '../lib/api'

  let password = ''
  let error = ''
  let submitting = false

  async function handleSubmit(e: Event) {
    e.preventDefault()
    error = ''
    submitting = true
    try {
      await api.login(password)
      window.location.reload()
    } catch (e: any) {
      if (e?.message === 'UNAUTHORIZED') {
        error = '密码错误'
      } else {
        error = e?.message || '登录失败'
      }
    } finally {
      submitting = false
    }
  }
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
  <form
    on:submit={handleSubmit}
    class="bg-white rounded-lg shadow-xl p-8 w-96"
  >
    <h2 class="text-lg font-semibold text-gray-800 mb-6">PortHannis 登录</h2>

    {#if error}
      <div class="mb-4 p-3 bg-red-50 text-red-600 text-sm rounded">{error}</div>
    {/if}

    <div class="mb-4">
      <label class="block text-sm text-gray-600 mb-1">访问密码</label>
      <input
        type="password"
        bind:value={password}
        class="w-full px-3 py-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
        placeholder="请输入密码"
        autofocus
      />
    </div>

    <button
      type="submit"
      disabled={submitting || !password}
      class="w-full py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
    >
      {submitting ? '登录中...' : '登录'}
    </button>
  </form>
</div>
