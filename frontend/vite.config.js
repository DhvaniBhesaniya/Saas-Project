import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    host: false,
    staticPort: true,
    port: 5170,
    proxy:{
      '/api': {
       target:  'http://localhost:4000',
       changeOrigin:true,
       secure:false,
      }
    },
  },
})
