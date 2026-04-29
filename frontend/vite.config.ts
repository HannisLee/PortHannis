import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      ignored: ['**/wailsjs/**']
    }
  },
  envPrefix: ['VITE_', 'WAILS_'],
  build: {
    outDir: './dist',
    emptyOutDir: true
  }
})
