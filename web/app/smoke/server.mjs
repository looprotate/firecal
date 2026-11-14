// A static host for the built app and fixture tiles. Stopping it is the network going away for the page.
import { readFileSync, statSync } from 'node:fs';
import { createServer } from 'node:http';
import { extname, join, normalize } from 'node:path';

const TYPES = {
  '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.wasm': 'application/wasm',
  '.json': 'application/json', '.bin': 'application/octet-stream', '.svg': 'image/svg+xml',
};

/**
 * Serves `root` (the built app) and the in-memory `files` (path to bytes), answering a missing path with 404
 * as a static host does. Resolves to its address and a `stop` that closes every open connection.
 */
export async function serve(root, files) {
  const server = createServer((request, response) => {
    const path = new URL(request.url ?? '/', 'http://localhost').pathname;
    const body = files[path] ?? readStatic(root, path === '/' ? '/index.html' : path);
    if (body === null) {
      response.writeHead(404).end();
      return;
    }
    response.writeHead(200, { 'Content-Type': TYPES[extname(path)] ?? 'text/html' }).end(body);
  });
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  const { port } = server.address();
  return {
    origin: `http://127.0.0.1:${port}`,
    stop: () => new Promise((resolve) => {
      server.closeAllConnections();
      server.close(resolve);
    }),
  };
}

function readStatic(root, path) {
  const file = normalize(join(root, path));
  if (!file.startsWith(normalize(root))) return null;
  try {
    return statSync(file).isFile() ? readFileSync(file) : null;
  } catch {
    return null;
  }
}
