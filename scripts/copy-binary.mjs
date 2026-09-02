import { copyFile, mkdir } from 'node:fs/promises';
await mkdir('dist/site/downloads', { recursive: true });
await copyFile('target/release/lsp-readiness', 'dist/site/downloads/lsp-readiness-linux-x86_64');
