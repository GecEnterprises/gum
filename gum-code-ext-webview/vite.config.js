import { defineConfig } from 'vite';
import preact from '@preact/preset-vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [preact()],
  build: {
    rollupOptions: {
      output: {
        entryFileNames: 'index.js', // Set the JavaScript output filename
        assetFileNames: (assetInfo) => {
          if (assetInfo.name === 'style.css') {
            return 'index.css'; // Set the CSS output filename
          }
          return assetInfo.name;
        },
      },
    },
  },
});
