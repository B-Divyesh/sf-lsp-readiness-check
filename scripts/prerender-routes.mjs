import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';

const origin = 'https://lsp-readiness-check.sociobot.in';
const dist = resolve('dist/site');
const routes = JSON.parse(await readFile('site/route-metadata.json', 'utf8'));
const template = await readFile(resolve(dist, 'index.html'), 'utf8');

function escaped(value) {
  return value.replace(/[&<>"']/g, (character) => ({
    '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;',
  })[character]);
}

function replaceAttribute(html, selector, value) {
  const pattern = new RegExp(`(${selector} content=")[^"]*(")`);
  return html.replace(pattern, `$1${escaped(value)}$2`);
}

function pageFor(path, metadata) {
  const canonical = `${origin}${path}`;
  let html = template.replace(/<title>[^<]*<\/title>/, `<title>${escaped(metadata.title)}</title>`);
  html = replaceAttribute(html, '<meta name="description"', metadata.description);
  html = html.replace(/(<link rel="canonical" href=")[^"]*(")/, `$1${canonical}$2`);
  html = replaceAttribute(html, '<meta property="og:title"', metadata.title);
  html = replaceAttribute(html, '<meta property="og:description"', metadata.description);
  html = replaceAttribute(html, '<meta property="og:url"', canonical);
  html = replaceAttribute(html, '<meta property="og:image"', `${origin}/og-image.webp`);
  html = replaceAttribute(html, '<meta name="twitter:title"', metadata.title);
  html = replaceAttribute(html, '<meta name="twitter:description"', metadata.description);
  html = replaceAttribute(html, '<meta name="twitter:image"', `${origin}/og-image.webp`);
  return html;
}

for (const [path, metadata] of Object.entries(routes)) {
  const output = path === '/' ? resolve(dist, 'index.html') : resolve(dist, path.slice(1), 'index.html');
  await mkdir(resolve(output, '..'), { recursive: true });
  await writeFile(output, pageFor(path, metadata));
}
