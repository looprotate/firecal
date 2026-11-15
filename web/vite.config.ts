import { defineConfig } from 'vite';
import { serviceWorkerSource } from './app/service-worker.ts';

export default defineConfig({
  plugins: [{
    name: 'firecal-service-worker',
    configureServer(server) {
      server.middlewares.use('/service-worker.js', (_request, response) => {
        response.setHeader('Content-Type', 'text/javascript');
        response.end(serviceWorkerSource(['/', '/firecal.wasm']));
      });
    },
    generateBundle(_options, bundle) {
      const assets = Object.keys(bundle).map((file) => `/${file}`);
      const shell = ['/', '/firecal.wasm', ...assets];
      this.emitFile({ type: 'asset', fileName: 'service-worker.js', source: serviceWorkerSource(shell) });
    },
  }],
  // The app draws with the site's hand (`site/js/draw.js`), one folder up.
  server: { fs: { allow: ['..'] } },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
});
