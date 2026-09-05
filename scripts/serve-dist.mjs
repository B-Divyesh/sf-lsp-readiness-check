import { createReadStream, existsSync, readFileSync, statSync } from 'node:fs';
import { createServer } from 'node:http';
import { extname, join, normalize, resolve } from 'node:path';

const dist = resolve('dist/site');
const config = JSON.parse(readFileSync(join(dist, 'staticwebapp.config.json'), 'utf8'));
const rewrites = new Map((config.routes ?? [])
  .filter((route) => route.rewrite)
  .map((route) => [route.route, route.rewrite]));
const notFound = config.responseOverrides?.['404']?.rewrite ?? '/404.html';
const types = {
  '.css': 'text/css; charset=utf-8', '.html': 'text/html; charset=utf-8', '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8', '.svg': 'image/svg+xml', '.webp': 'image/webp', '.woff2': 'font/woff2',
  '.xml': 'application/xml; charset=utf-8', '.txt': 'text/plain; charset=utf-8',
};

function fileFor(pathname) {
  const rewritten = rewrites.get(pathname) ?? pathname;
  const relative = rewritten === '/' ? '/index.html' : rewritten;
  const file = resolve(dist, `.${relative}`);
  if (!file.startsWith(`${dist}/`) || !existsSync(file)) return undefined;
  return statSync(file).isDirectory() ? join(file, 'index.html') : file;
}

createServer((request, response) => {
  const pathname = decodeURIComponent(new URL(request.url ?? '/', 'http://localhost').pathname);
  const file = fileFor(pathname);
  const status = file ? 200 : 404;
  const served = file ?? resolve(dist, `.${notFound}`);
  response.writeHead(status, { 'content-type': types[extname(served)] ?? 'application/octet-stream' });
  createReadStream(served).pipe(response);
}).listen(4173, '127.0.0.1');
