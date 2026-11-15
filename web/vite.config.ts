import { defineConfig } from 'vite';
import { SCREEN_PATHS } from './app/screens.ts';
import { serviceWorkerSource } from './app/service-worker.ts';

export default defineConfig({
  plugins: [{
    name: 'firecal-service-worker',
    enforce: 'post',
    configureServer(server) {
      server.middlewares.use('/service-worker.js', (_request, response) => {
        response.setHeader('Content-Type', 'text/javascript');
        response.end(serviceWorkerSource(['/', '/firecal.wasm']));
      });
    },
    generateBundle(_options, bundle) {
      const index = bundle['index.html'];
      if (!index || index.type !== 'asset') throw new Error('The application entry page is missing.');
      for (const path of Object.values(SCREEN_PATHS)) {
        if (path !== '/index.html') this.emitFile({ type: 'asset', fileName: path.slice(1), source: index.source });
      }
      const assets = [...Object.keys(bundle), ...Object.values(SCREEN_PATHS).map(path => path.slice(1))].map((file) => `/${file}`);
      const shell = ['/', '/firecal.wasm', '/photography/forest-hero.png', '/favicon.svg', '/branding/firecal-tree.png', ...assets];
      this.emitFile({ type: 'asset', fileName: 'service-worker.js', source: serviceWorkerSource(shell) });
    },
  }],
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
});
