// @ts-check
import { defineConfig } from 'astro/config';

import node from '@astrojs/node';
import svelte from '@astrojs/svelte';

import tailwindcss from '@tailwindcss/vite';
import wasm from 'vite-plugin-wasm';

// https://astro.build/config
export default defineConfig({
  adapter: node({ mode: 'standalone' }),
  integrations: [svelte()],

  vite: {
    plugins: [tailwindcss(), wasm()]
  }
});
