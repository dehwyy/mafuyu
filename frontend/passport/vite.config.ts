import { enhancedImages } from '@sveltejs/enhanced-img'
import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'

import { purgeCss } from 'vite-plugin-tailwind-purgecss'

export default defineConfig({
  plugins: [
    enhancedImages(),
    sveltekit(),
    purgeCss({
      safelist: {
        // any selectors that begin with "hljs-" will not be purged
        greedy: [/^hljs-/]
      }
    })
  ]
})
