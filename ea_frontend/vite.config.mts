import { defineConfig } from 'npm:vite@^5.0.10'
import vue from 'npm:@vitejs/plugin-vue@^4.5.2'
import Vuetify, { transformAssetUrls } from 'npm:vite-plugin-vuetify'

import 'npm:vue@^3.3.13'

import { fileURLToPath, URL } from 'node:url';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        vue({template: { transformAssetUrls },}),
        Vuetify()
    ],
    resolve: {
        alias: {
          '@': fileURLToPath(new URL('./src', import.meta.url)),
        },
        extensions: [
          '.js',
          '.json',
          '.jsx',
          '.mjs',
          '.ts',
          '.tsx',
          '.vue',
        ],
      },
    css: {
        preprocessorOptions: {
            css: { charset: false }
        }
    },
    server: {
        port: 3000,
    },
})
