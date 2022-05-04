import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';
import * as path from 'path';

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: 'build',
  },
  resolve: {
    alias: {
      components: path.resolve(__dirname, 'src/components'),
      layout: path.resolve(__dirname, 'src/layout'),
      lib: path.resolve(__dirname, 'src/lib'),
      style: path.resolve(__dirname, 'src/style'),
    },
  },
});
