import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
import { resolve } from 'path'

export default defineConfig({
  plugins: [
    tailwindcss(),
  ],
  // build: {
  //   rollupOptions: {
  //     input: {
  //       main: resolve(__dirname, 'src/index.html'),
  //       login: resolve(__dirname, 'src/login.html')
  //     }
  //   }
  // },
  root: 'src'
})
