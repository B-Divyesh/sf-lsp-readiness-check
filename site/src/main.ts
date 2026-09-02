import './style.css';

type Status = 'ready' | 'declared' | 'missing' | 'failed';
type Capability = { kind: string; name: string; command: string; status: Status; evidence: string };

const sampleCapabilities: Capability[] = [
  { kind: 'LSP', name: 'TypeScript language server', command: 'typescript-language-server --stdio', status: 'ready', evidence: 'Initialize reply · definition · references · diagnostics' },
  { kind: 'Format', name: 'Prettier', command: 'prettier', status: 'ready', evidence: 'Version 3.6.2' },
  { kind: 'LSP', name: 'Rust language server', command: 'rust-analyzer', status: 'ready', evidence: 'Initialize reply · definition · references · diagnostics' },
  { kind: 'Format', name: 'Rustfmt', command: 'rustfmt', status: 'ready', evidence: 'Version 1.8.0-stable' },
  { kind: 'Tests', name: 'Repository tests', command: 'npm test', status: 'ready', evidence: '42 tests passed' },
];

const routes: Record<string, { title: string; description: string; render: () => string }> = {
  '/': { title: 'LSP Readiness Check — verify repository tooling', description: 'Check language servers, formatters, and tests before an agent changes your repository.', render: landing },
  '/demo': { title: 'Demo — LSP Readiness Check', description: 'Run the LSP Readiness Check sample repository probe.', render: demo },
  '/privacy': { title: 'Privacy — LSP Readiness Check', description: 'How LSP Readiness Check handles repository and license data.', render: privacy },
  '/terms': { title: 'Terms — LSP Readiness Check', description: 'Terms for LSP Readiness Check.', render: terms },
  '/404': { title: 'Page not found — LSP Readiness Check', description: 'This page does not exist.', render: notFound },
};

const app = document.querySelector<HTMLDivElement>('#app')!;
const live = document.createElement('div');
live.className = 'sr-only';
live.setAttribute('aria-live', 'polite');
document.body.append(live);

function shell(content: string, demoMode = false): string {
  return `
    <a class="skip-link" href="#main">Skip to main content</a>
    ${demoMode ? `<aside class="demo-banner" aria-label="Demo status"><span><strong>Demo</strong> — sample data, nothing is saved</span><span class="demo-actions"><button class="text-button" data-reset>Reset demo</button><a href="/" data-link data-start-real>Start for real</a></span></aside>` : ''}
    <header class="site-header">
      <a class="wordmark" href="/" data-link aria-label="LSP Readiness Check home">
        <svg aria-hidden="true" viewBox="0 0 42 42"><path d="M5 21c0-9 7-16 16-16s16 7 16 16-7 16-16 16S5 30 5 21Z"/><path d="M11 21c0-5.5 4.5-10 10-10s10 4.5 10 10-4.5 10-10 10-10-4.5-10-10Z"/><circle cx="21" cy="21" r="3"/></svg>
        <span>LSP <b>Readiness</b></span>
      </a>
      <nav aria-label="Primary navigation"><a href="/demo" data-link>Demo</a><a href="/#how">How it works</a><a href="/#pricing">Pricing</a><a href="/privacy" data-link>Privacy</a></nav>
    </header>
    <main id="main" tabindex="-1">${content}</main>
    <footer class="site-footer">
      <p><strong>LSP Readiness Check</strong><br><span>Verify language tooling before agent edits.</span></p>
      <div><a href="/privacy" data-link>Privacy</a><a href="/terms" data-link>Terms</a><a href="https://hello-factory.sociobot.in" rel="external">Built by Param Factory <span class="sr-only">(external site)</span></a></div>
      <p class="build">v0.1.0 · build 2026.09.02</p>
    </footer>`;
}

function landing(): string {
  return shell(`
    <section class="hero contour-field">
      <div class="hero-copy">
        <p class="eyebrow">Repository preflight · CLI</p>
        <h1>Verify tooling before an agent edits</h1>
        <p class="lede">For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.</p>
        <div class="hero-actions"><a class="button primary" href="/demo" data-link>Try it with sample data</a><span>See a finished probe in one click.</span></div>
        <ul class="facts" aria-label="Product facts"><li>Source stays on your machine</li><li>The demo reloads offline after its first visit</li><li>Private CI costs $49 per repository each month</li></ul>
      </div>
      <figure class="hero-map">
        <img src="/topographic-survey.webp" width="768" height="512" alt="Contour lines connect four survey markers around a repository map." fetchpriority="high" />
        <figcaption class="terminal compact"><span class="terminal-bar"><i></i><i></i><i></i><b>northstar-api / preflight</b></span><pre><code><span class="muted">$</span> lsp-readiness demo

<span class="ok">READY</span> — agent edits may start
<span class="ok">PASS</span>  TypeScript LSP
<span class="ok">PASS</span>  Rust LSP
<span class="ok">PASS</span>  Formatters
<span class="ok">PASS</span>  42 tests

<span class="muted">Signed: Ed25519 / Q7wV4rT2…</span></code></pre></figcaption>
      </figure>
    </section>

    <section class="preview section-rule" aria-labelledby="preview-title">
      <div class="section-heading"><p class="eyebrow">Capability packet</p><h2 id="preview-title">Give agents evidence they can read</h2><p>The CLI writes one JSON packet. It records each probe, the repository inventory digest, and an Ed25519 signature.</p></div>
      ${capabilityTable(sampleCapabilities)}
    </section>

    <section id="how" class="route-section section-rule" aria-labelledby="how-title">
      <div class="section-heading"><p class="eyebrow">Three checks</p><h2 id="how-title">How the preflight works</h2><p>Run it in your container, or let the CLI create one from a pinned development image.</p></div>
      <ol class="route-list">
        <li><span>01</span><div><h3>Scan the repository</h3><p>Detect source languages and declared test commands. Ignore dependencies, build output, and source contents.</p></div></li>
        <li><span>02</span><div><h3>Probe each tool</h3><p>Start each detected language server. Check formatter versions and run the test command.</p></div></li>
        <li><span>03</span><div><h3>Sign the result</h3><p>Write a JSON capability packet. Verify its Ed25519 signature before an agent starts work.</p></div></li>
      </ol>
      <div class="install-strip"><div><p class="eyebrow">Install from source</p><code>cargo install --git https://github.com/B-Divyesh/sf-lsp-readiness-check</code></div><div class="install-actions"><button class="copy-button" data-copy="cargo install --git https://github.com/B-Divyesh/sf-lsp-readiness-check">Copy command</button><a href="/downloads/lsp-readiness-linux-x86_64" download>Download Linux binary</a></div></div>
    </section>

    <section class="limits section-rule" aria-labelledby="limits-title">
      <div class="section-heading"><p class="eyebrow">Scope</p><h2 id="limits-title">What the CLI does not do</h2></div>
      <div class="limits-grid"><p>It does not upload source code or repository file contents.</p><p>It does not install or update language servers.</p><p>It does not replace your editor, test runner, or container policy.</p></div>
    </section>

    <section id="pricing" class="pricing section-rule" aria-labelledby="pricing-title">
      <div class="price-mark" aria-hidden="true">49</div>
      <div><p class="eyebrow">Private CI</p><h2 id="pricing-title">Keep readiness history for private repositories</h2><p class="price"><strong>$49</strong> per repository each month</p><ul><li>Required checks for private CI</li><li>Versioned policy templates</li><li>Readiness history for onboarding reviews</li></ul><p class="legal-note">Sociobot, through Dodo, is the merchant of record. See the <a href="/terms" data-link>terms</a>.</p></div>
      <div class="purchase"><a class="button primary" href="https://api.sociobot.in/api/v1/products/lsp-readiness-check/checkout" rel="external">Buy private CI <span class="sr-only">(hosted checkout)</span></a><button class="button secondary" data-show-license>Verify a license</button><p class="license-state" data-license-state aria-live="polite"></p><form class="license-form" hidden><label for="license">License token</label><input id="license" name="license" autocomplete="off" spellcheck="false"><button class="button secondary" type="submit">Verify license</button><p class="form-status" aria-live="polite"></p></form></div>
    </section>`, false);
}

function capabilityTable(capabilities: Capability[]): string {
  return `<div class="capability-table" role="region" aria-label="Readiness results" tabindex="0">
    <div class="capability-row heading" aria-hidden="true"><span>State</span><span>Capability</span><span>Evidence</span></div>
    ${capabilities.map((cap) => `<div class="capability-row"><span><b class="status ${cap.status}">${cap.status === 'ready' ? 'Pass' : cap.status}</b><small>${cap.kind}</small></span><span><strong>${cap.name}</strong><code>${cap.command}</code></span><span>${cap.evidence}</span></div>`).join('')}
  </div>`;
}

function demo(): string {
  return shell(`
    <section class="demo-page contour-field">
      <div class="demo-heading"><p class="eyebrow">Sample repository · northstar-api</p><h1>Review a completed readiness probe</h1><p>This replay uses bundled TypeScript and Rust sample data. It does not inspect your device.</p></div>
      <div class="demo-layout">
        <div class="terminal" aria-label="Recorded terminal output"><span class="terminal-bar"><i></i><i></i><i></i><b>lsp-readiness demo</b></span><pre id="terminal-output" aria-live="polite"><code><span class="muted">$</span> lsp-readiness demo
<span class="muted">Scanning northstar-api…</span></code></pre><div class="terminal-controls"><button class="button terminal-button" data-run-demo>Run sample probe</button><button class="text-button on-dark" data-replay>Replay output</button></div></div>
        <aside class="demo-summary"><p class="eyebrow">Readiness contract</p><strong class="score">5/5</strong><p>required checks pass</p><dl><div><dt>Languages</dt><dd>TypeScript, Rust</dd></div><div><dt>Packet</dt><dd>Ed25519 signed</dd></div><div><dt>Source digest</dt><dd><code>sha256:1df019b6…</code></dd></div></dl><a class="button secondary" href="/sample/northstar-api.lsp-readiness.json" download>Download sample JSON</a></aside>
      </div>
      <section class="demo-results" aria-labelledby="results-title"><div class="section-heading"><p class="eyebrow">Probe evidence</p><h2 id="results-title">Each required check passed</h2></div>${capabilityTable(sampleCapabilities)}</section>
    </section>`, true);
}

function privacy(): string {
  return shell(`<article class="legal"><p class="eyebrow">Policy · effective 2 September 2026</p><h1>Your repository stays on your machine</h1><p>LSP Readiness Check is a local command-line tool. It reads file names, manifest files, and command output to build a capability packet.</p><h2>Data the free CLI handles</h2><p>The CLI stores its signing key and output in paths you choose. It sends no repository data to us.</p><h2>Demo data</h2><p>The website demo uses bundled sample data. It stores only the demo state under the <code>demo:</code> browser storage prefix.</p><h2>License checks</h2><p>If you add a license, your browser sends that token to the Sociobot billing API. The token and its last result stay in your browser. Remove the license from browser storage to delete it.</p><h2>Contact</h2><p>Questions can be sent to <a href="mailto:privacy@sociobot.in">privacy@sociobot.in</a>.</p></article>`);
}

function terms(): string {
  return shell(`<article class="legal"><p class="eyebrow">Terms · effective 2 September 2026</p><h1>Use the check as one safety signal</h1><p>LSP Readiness Check reports the tools it can observe. A passing result does not guarantee correct code or safe agent changes.</p><h2>Free CLI</h2><p>The open-source CLI is provided under the MIT License. You control where it runs and which test commands it executes.</p><h2>Private CI plan</h2><p>Private CI costs $49 per repository each month. It includes private required checks, policy templates, and readiness history.</p><p>Sociobot, through Dodo, is the merchant of record. Billing, cancellations, and refunds follow the terms shown at hosted checkout. A refund or cancellation can end license access.</p><h2>Acceptable use</h2><p>Do not use the service to probe systems you do not own or have permission to test.</p><h2>Contact</h2><p>Questions can be sent to <a href="mailto:support@sociobot.in">support@sociobot.in</a>.</p></article>`);
}

function notFound(): string {
  return shell(`<section class="not-found contour-field"><p class="eyebrow">Map edge · 404</p><h1>This route is not on the map</h1><p>The address may be old or incomplete.</p><a class="button primary" href="/" data-link>Return home</a></section>`);
}

function currentPath(): string {
  const path = window.location.pathname.replace(/\/$/, '') || '/';
  return routes[path] ? path : '/404';
}

function render(focus = false): void {
  const path = currentPath();
  const route = routes[path];
  document.title = route.title;
  document.querySelector('meta[name="description"]')?.setAttribute('content', route.description);
  document.querySelector('link[rel="canonical"]')?.setAttribute('href', `https://lsp-readiness-check.sociobot.in${path === '/' ? '/' : path}`);
  app.innerHTML = route.render();
  document.querySelector('h1')?.setAttribute('tabindex', '-1');
  bindActions();
  showLicenseState();
  if (path === '/demo') runDemo();
  live.textContent = document.querySelector('h1')?.textContent ?? '';
  if (focus) document.querySelector<HTMLElement>('h1')?.focus({ preventScroll: false });
}

function navigate(path: string): void {
  history.pushState({}, '', path);
  window.scrollTo(0, 0);
  render(true);
}

function bindActions(): void {
  document.querySelectorAll<HTMLAnchorElement>('[data-link]').forEach((link) => link.addEventListener('click', (event) => { event.preventDefault(); navigate(new URL(link.href).pathname); }));
  document.querySelector('[data-reset]')?.addEventListener('click', () => { localStorage.removeItem('demo:lsp-readiness-check'); location.reload(); });
  document.querySelector('[data-start-real]')?.addEventListener('click', () => localStorage.removeItem('demo:lsp-readiness-check'));
  document.querySelector('[data-run-demo]')?.addEventListener('click', runDemo);
  document.querySelector('[data-replay]')?.addEventListener('click', runDemo);
  document.querySelector('[data-copy]')?.addEventListener('click', async (event) => {
    const button = event.currentTarget as HTMLButtonElement;
    try { await navigator.clipboard.writeText(button.dataset.copy ?? ''); button.textContent = 'Copied'; }
    catch { button.textContent = 'Copy failed'; }
  });
  document.querySelector('[data-show-license]')?.addEventListener('click', () => {
    const form = document.querySelector<HTMLFormElement>('.license-form')!;
    form.hidden = false; form.querySelector<HTMLInputElement>('input')?.focus();
  });
  document.querySelector<HTMLFormElement>('.license-form')?.addEventListener('submit', verifyLicenseForm);
}

async function runDemo(): Promise<void> {
  const output = document.querySelector<HTMLElement>('#terminal-output');
  if (!output) return;
  localStorage.setItem('demo:lsp-readiness-check', JSON.stringify({ sample: 'northstar-api', ran: true }));
  output.innerHTML = `<code><span class="muted">$</span> lsp-readiness demo\n<span class="muted">Scanning northstar-api…</span></code>`;
  await wait(reducedMotion() ? 0 : 180);
  output.innerHTML = `<code><span class="muted">$</span> lsp-readiness demo

<span class="ok">READY</span> — agent edits may start
<span class="ok">PASS</span>  lsp         TypeScript language server
<span class="ok">PASS</span>  formatter   prettier 3.6.2
<span class="ok">PASS</span>  lsp         Rust language server
<span class="ok">PASS</span>  formatter   rustfmt 1.8.0-stable
<span class="ok">PASS</span>  tests       42 tests passed

<span class="muted">Signature: Ed25519 / Q7wV4rT2…
Sample packet is stored only in this demo.</span></code>`;
}

async function verifyLicenseForm(event: SubmitEvent): Promise<void> {
  event.preventDefault();
  const form = event.currentTarget as HTMLFormElement;
  const input = form.elements.namedItem('license') as HTMLInputElement;
  const status = form.querySelector<HTMLElement>('.form-status')!;
  const token = input.value.trim();
  if (!token) { status.textContent = 'Enter the license token from your receipt.'; return; }
  status.textContent = 'Checking the license…';
  try {
    const response = await fetch(`https://api.sociobot.in/api/v1/products/lsp-readiness-check/verify?license=${encodeURIComponent(token)}`);
    const result = await response.json() as { valid?: boolean };
    if (!result.valid) throw new Error('inactive');
    localStorage.setItem('sb_license:lsp-readiness-check', token);
    localStorage.setItem('sb_license_verdict:lsp-readiness-check', JSON.stringify({ valid: true, checkedAt: Date.now() }));
    status.textContent = 'License active. Private CI setup is available.';
    showLicenseState();
  } catch { status.textContent = 'The license is not active. Check the token or buy a new plan.'; }
}

function acceptReturnedLicense(): void {
  const params = new URLSearchParams(location.search);
  const token = params.get('license');
  if (!token) return;
  localStorage.setItem('sb_license:lsp-readiness-check', token);
  params.delete('license');
  history.replaceState({}, '', `${location.pathname}${params.size ? `?${params}` : ''}${location.hash}`);
}

async function verifyStoredLicense(token: string): Promise<void> {
  const verdictKey = 'sb_license_verdict:lsp-readiness-check';
  const cached = JSON.parse(localStorage.getItem(verdictKey) ?? 'null') as { valid?: boolean; checkedAt?: number } | null;
  if (cached?.checkedAt && Date.now() - cached.checkedAt < 86_400_000) return;
  try {
    const response = await fetch(`https://api.sociobot.in/api/v1/products/lsp-readiness-check/verify?license=${encodeURIComponent(token)}`);
    const result = await response.json() as { valid?: boolean };
    localStorage.setItem(verdictKey, JSON.stringify({ valid: result.valid === true, checkedAt: Date.now() }));
    showLicenseState();
  } catch { /* Keep the last verdict while offline. */ }
}

function showLicenseState(): void {
  const element = document.querySelector<HTMLElement>('[data-license-state]');
  if (!element) return;
  const verdict = JSON.parse(localStorage.getItem('sb_license_verdict:lsp-readiness-check') ?? 'null') as { valid?: boolean } | null;
  if (verdict?.valid === true) element.textContent = 'License active on this device.';
  if (verdict?.valid === false) element.textContent = 'License no longer active. Buy a new plan to restore private CI.';
}

function reducedMotion() { return matchMedia('(prefers-reduced-motion: reduce)').matches; }
function wait(ms: number) { return new Promise((resolve) => setTimeout(resolve, ms)); }

document.addEventListener('click', (event) => {
  const anchor = (event.target as Element).closest<HTMLAnchorElement>('a[href^="/#"]');
  if (!anchor || location.pathname !== '/') return;
  event.preventDefault(); document.querySelector(anchor.hash)?.scrollIntoView({ behavior: reducedMotion() ? 'auto' : 'smooth' });
});
window.addEventListener('popstate', () => render(true));
acceptReturnedLicense();
render();
const storedLicense = localStorage.getItem('sb_license:lsp-readiness-check');
if (storedLicense) verifyStoredLicense(storedLicense);
if ('serviceWorker' in navigator) window.addEventListener('load', () => navigator.serviceWorker.register('/service-worker.js').catch(() => undefined));
