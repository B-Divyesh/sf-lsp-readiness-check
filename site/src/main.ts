import './style.css';
import metadata from '../route-metadata.json';

type Status = 'ready' | 'declared' | 'missing' | 'failed';
type Capability = { kind: string; name: string; command: string; status: Status; evidence: string };
type Route = { title: string; description: string; render: () => string };
type SetupConfig = {
  identity_configured: boolean;
  github_app_configured: boolean;
  subscription_configured: boolean;
  client_id?: string;
  authorize_url?: string;
  token_url?: string;
  scope?: string;
  redirect_url: string;
  api_origin: string;
};

const API_ORIGIN = location.hostname === '127.0.0.1' || location.hostname === 'localhost'
  ? 'http://127.0.0.1:8787'
  : 'https://lsp-readiness-check-api.sociobot.in';
const ACCESS_TOKEN_KEY = 'session:lsp-readiness-check:access-token';
const PKCE_KEY = 'session:lsp-readiness-check:pkce';

const sampleCapabilities: Capability[] = [
  { kind: 'LSP', name: 'TypeScript language server', command: 'typescript-language-server --stdio', status: 'ready', evidence: 'Initialize reply; capabilities: definition, references, diagnostics' },
  { kind: 'Format', name: 'Prettier', command: 'prettier', status: 'ready', evidence: '3.6.2' },
  { kind: 'LSP', name: 'Rust language server', command: 'rust-analyzer', status: 'ready', evidence: 'Initialize reply; capabilities: definition, references, diagnostics' },
  { kind: 'Format', name: 'Rustfmt', command: 'rustfmt', status: 'ready', evidence: 'rustfmt 1.8.0-stable' },
  { kind: 'Tests', name: 'Repository tests', command: 'npm test', status: 'ready', evidence: '42 tests passed' },
];

const routes: Record<string, Route> = {
  '/': { ...metadata['/'], render: landing },
  '/demo': { ...metadata['/demo'], render: demo },
  '/privacy': { ...metadata['/privacy'], render: privacy },
  '/terms': { ...metadata['/terms'], render: terms },
  '/sign-in': { ...metadata['/sign-in'], render: signIn },
  '/app': { ...metadata['/app'], render: appHome },
  '/app/repositories': { ...metadata['/app/repositories'], render: repositories },
  '/app/billing': { ...metadata['/app/billing'], render: billing },
  '/404': { ...metadata['/404'], render: notFound },
};

const policyRoute: Route = {
  title: 'Repository policy — LSP Readiness Check',
  description: 'Set the readiness requirements for a private repository.',
  render: repositoryPolicy,
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
      <nav aria-label="Primary navigation"><a href="/demo" data-link>Demo</a><a href="/#how">How it works</a><a href="/sign-in" data-link>Sign in</a><a href="/privacy" data-link>Privacy</a></nav>
    </header>
    <main id="main" tabindex="-1">${content}</main>
    <footer class="site-footer">
      <p><strong>LSP Readiness Check</strong><br><span>Verify language tooling before agent edits.</span></p>
      <div><a href="/privacy" data-link>Privacy</a><a href="/terms" data-link>Terms</a><a href="https://hello-factory.sociobot.in" rel="external">Built by Param Factory <span class="sr-only">(external site)</span></a></div>
      <p class="build">M2 · CLI v0.1.2</p>
    </footer>`;
}

function landing(): string {
  return shell(`
    <section class="hero contour-field">
      <div class="hero-copy">
        <p class="eyebrow">Repository check · command-line tool</p>
        <h1>Verify tooling before an agent edits</h1>
        <p class="lede">For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.</p>
        <div class="hero-actions"><a class="button primary" href="/?demo=1" data-link>Try it with sample data</a><span>See a finished probe in one click.</span></div>
        <ul class="facts" aria-label="Product facts"><li>Source stays on your machine</li><li>The demo reloads offline after its first visit</li><li>No account is needed for the free CLI</li></ul>
      </div>
      <figure class="hero-map">
        <img src="/topographic-survey.webp" width="768" height="512" alt="Contour lines connect four survey markers around a repository map." fetchpriority="high" />
        <figcaption class="terminal compact"><span class="terminal-bar"><i></i><i></i><i></i><b>northstar-api / repository check</b></span><pre><code><span class="muted">$</span> lsp-readiness demo

<span class="ok">READY</span> — agent edits may start
<span class="ok">PASS</span>  TypeScript LSP
<span class="ok">PASS</span>  Rust LSP
<span class="ok">PASS</span>  Formatters
<span class="ok">PASS</span>  42 tests

<span class="muted">Signed JSON readiness report</span></code></pre></figcaption>
      </figure>
    </section>

    <section class="preview section-rule" aria-labelledby="preview-title">
      <div class="section-heading"><p class="eyebrow">Readiness report</p><h2 id="preview-title">Signed JSON readiness report</h2><p>The CLI writes one signed JSON readiness report. Its signature makes tampering detectable (Ed25519).</p></div>
      ${capabilityTable(sampleCapabilities)}
    </section>

    <section id="how" class="route-section section-rule" aria-labelledby="how-title">
      <div class="section-heading"><p class="eyebrow">Three checks</p><h2 id="how-title">How the repository check works</h2><p>The normal check uses a network-disabled container made from the exact development image you choose.</p></div>
      <ol class="route-list">
        <li><span>01</span><div><h3>Scan the repository</h3><p>Detect source languages and declared test commands. Ignore dependencies, build output, and source contents.</p></div></li>
        <li><span>02</span><div><h3>Probe each tool</h3><p>Start each detected language server. Check formatter versions and run the test command.</p></div></li>
        <li><span>03</span><div><h3>Sign the result</h3><p>Write a signed JSON readiness report. Verify the report’s signature before an agent starts work.</p></div></li>
      </ol>
      <div class="install-strip"><div><p class="eyebrow">Install from source</p><code>cargo install --git https://github.com/B-Divyesh/sf-lsp-readiness-check</code><p class="install-note">Use an image address with a SHA-256 digest so the same tools run each time.</p></div><div class="install-actions"><button class="copy-button" data-copy="cargo install --git https://github.com/B-Divyesh/sf-lsp-readiness-check">Copy command</button><a href="/downloads/lsp-readiness-linux-x86_64" download>Download Linux binary</a></div></div>
    </section>

    <section class="limits section-rule" aria-labelledby="limits-title">
      <div class="section-heading"><p class="eyebrow">Scope</p><h2 id="limits-title">What the CLI does not do</h2></div>
      <div class="limits-grid"><p>It does not upload source code or repository file contents.</p><p>It does not install or update language servers.</p><p>It does not replace your editor, test runner, or container policy.</p></div>
    </section>

    <section id="pricing" class="pricing section-rule" aria-labelledby="pricing-title">
      <div class="price-mark" aria-hidden="true">$49</div>
      <div><p class="eyebrow">Private CI plan</p><h2 id="pricing-title">Private checks for each repository</h2><p class="price"><strong>$49</strong> per repository each month</p><ul><li>Private CI checks</li><li>Repository policy templates</li><li>Readiness history</li></ul><p class="legal-note">Subscriptions are not open yet. CIAM, GitHub App, and billing registration must pass product QA first.</p></div>
      <div class="purchase"><a class="button secondary" href="/sign-in" data-link>Check setup status</a><p class="license-state">The free local CLI stays available without an account.</p></div>
    </section>

    `, false);
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
        <div class="terminal" aria-label="Recorded terminal output"><span class="terminal-bar"><i></i><i></i><i></i><b>lsp-readiness demo</b></span><pre id="terminal-output" aria-live="polite" tabindex="0"><code><span class="muted">$</span> lsp-readiness demo
<span class="muted">Scanning northstar-api…</span></code></pre><div class="terminal-controls"><button class="button terminal-button" data-run-demo>Run sample probe</button><button class="text-button on-dark" data-replay>Replay output</button></div></div>
        <aside class="demo-summary"><p class="eyebrow">Readiness contract</p><strong class="score">5/5</strong><p>required checks pass</p><dl><div><dt>Languages</dt><dd>TypeScript, Rust</dd></div><div><dt>Readiness report</dt><dd>Signed JSON (Ed25519)</dd></div><div><dt>Source digest</dt><dd><code>sha256:6ad036fe…</code></dd></div></dl><a class="button secondary" href="/sample/northstar-api.lsp-readiness.json" download>Download sample JSON</a></aside>
      </div>
      <section class="demo-results" aria-labelledby="results-title"><div class="section-heading"><p class="eyebrow">Probe evidence</p><h2 id="results-title">Each required check passed</h2></div>${capabilityTable(sampleCapabilities)}</section>
    </section>`, true);
}

function privacy(): string {
  return shell(`<article class="legal"><p class="eyebrow">Policy · effective 6 September 2026</p><h1>Your repository stays on your machine</h1><p>LSP Readiness Check is a local command-line tool. It reads file names, manifest files, and command output to build a signed JSON readiness report.</p><h2>Data the free CLI handles</h2><p>Normal checks run repository commands only inside a network-disabled container. The CLI skips source-tree symlinks and keeps your signing key on the host.</p><p>The CLI stores its signing key and readiness report in paths you choose. It sends no repository data to us.</p><h2>Private CI account data</h2><p>After sign-in, the service stores your CIAM subject, team membership, connected repository names, policies, and readiness report metadata.</p><p>The service does not store repository files, diffs, raw test logs, CIAM passwords, or payment card details. Access tokens stay in your browser session.</p><p>Account owners can export or delete their team data from the private app. Deletion removes repositories, policies, runs, and subscription records.</p><h2>Demo data</h2><p>The website demo uses bundled sample data. It stores only the demo state under the <code>demo:</code> browser storage prefix.</p><h2>Contact</h2><p>Send access, export, or deletion questions to <a href="mailto:privacy@sociobot.in">privacy@sociobot.in</a>.</p></article>`);
}

function terms(): string {
  return shell(`<article class="legal"><p class="eyebrow">Terms · effective 6 September 2026</p><h1>Terms for LSP Readiness Check</h1><p>LSP Readiness Check reports the tools it can observe. A passing result does not guarantee correct code or safe agent changes.</p><h2>Free CLI</h2><p>The open-source CLI is provided under the MIT License. You control where it runs and which test commands it executes.</p><h2>Private CI subscription</h2><p>The planned paid offer costs $49 per repository each month. It includes private CI checks, policy templates, and readiness history.</p><p>The subscription is not available until hosted billing and entitlement checks pass product QA. No payment is taken on this site today.</p><h2>Acceptable use</h2><p>Do not use the service to probe systems you do not own or have permission to test.</p><h2>Contact</h2><p>Questions can be sent to <a href="mailto:support@sociobot.in">support@sociobot.in</a>.</p></article>`);
}

function signIn(): string {
  return shell(`<section class="account-page contour-field"><p class="eyebrow">Private repository account</p><h1>Sign in for private repository checks</h1><p>Team owners use CIAM to connect a GitHub App and keep repository results separate.</p><div class="setup-panel" data-setup-panel aria-live="polite"><p>Checking account setup…</p></div><div class="account-actions"><button class="button primary" data-sign-in disabled>Sign in with your team account</button><a href="/demo" data-link>Try the sample instead</a></div></section>`);
}

function appHome(): string {
  return shell(`<section class="account-page contour-field"><p class="eyebrow">Private CI setup</p><h1>Set up private repository checks</h1><p>Connect an authorized repository, choose its checks, then send signed readiness reports from your own CI runner.</p>${appNavigation()}<div class="setup-panel" data-account-content aria-live="polite"><p>Checking your account…</p></div></section>`);
}

function repositories(): string {
  return shell(`<section class="account-page contour-field"><p class="eyebrow">Private repositories</p><h1>Choose a private repository</h1><p>The GitHub App lists only repositories approved during installation. Source code is not copied here.</p>${appNavigation()}<div class="setup-panel" data-account-content aria-live="polite"><p>Loading repositories…</p></div></section>`);
}

function repositoryPolicy(): string {
  return shell(`<section class="account-page contour-field"><p class="eyebrow">Repository requirements</p><h1>Set repository requirements</h1><p>Choose which readiness checks must be present in each signed report.</p>${appNavigation()}<div class="setup-panel" data-account-content aria-live="polite"><p>Loading the repository policy…</p></div></section>`);
}

function billing(): string {
  return shell(`<section class="account-page contour-field"><p class="eyebrow">Private CI subscription</p><h1>Manage repository subscriptions</h1><p>Each private repository will require its own monthly subscription.</p>${appNavigation()}<div class="setup-panel" data-account-content aria-live="polite"><p>Checking subscription setup…</p></div></section>`);
}

function appNavigation(): string {
  return `<nav class="app-navigation" aria-label="Account navigation"><a href="/app" data-link>Setup</a><a href="/app/repositories" data-link>Repositories</a><a href="/app/billing" data-link>Billing</a><button class="text-button" data-sign-out>Sign out</button></nav>`;
}

function notFound(): string {
  return shell(`<section class="not-found contour-field"><p class="eyebrow">404</p><h1>Page not found</h1><p>The address may be old or incomplete.</p><a class="button primary" href="/" data-link>Return home</a></section>`);
}

function currentPath(): string {
  if (window.location.pathname === '/' && new URLSearchParams(window.location.search).get('demo') === '1') return '/demo';
  return window.location.pathname.replace(/\/$/, '') || '/';
}

function currentRoute(path: string): Route {
  if (/^\/app\/repositories\/[^/]+\/policy$/.test(path)) return policyRoute;
  return routes[path] ?? routes['/404'];
}

function render(focus = false): void {
  const path = currentPath();
  const route = currentRoute(path);
  document.title = route.title;
  document.querySelector('meta[name="description"]')?.setAttribute('content', route.description);
  const canonicalPath = route === routes['/404'] ? '/404' : path;
  const canonical = `https://lsp-readiness-check.sociobot.in${canonicalPath}`;
  document.querySelector('link[rel="canonical"]')?.setAttribute('href', canonical);
  document.querySelector('meta[property="og:title"]')?.setAttribute('content', route.title);
  document.querySelector('meta[property="og:description"]')?.setAttribute('content', route.description);
  document.querySelector('meta[property="og:url"]')?.setAttribute('content', canonical);
  document.querySelector('meta[property="og:image"]')?.setAttribute('content', 'https://lsp-readiness-check.sociobot.in/og-image.webp');
  document.querySelector('meta[name="twitter:title"]')?.setAttribute('content', route.title);
  document.querySelector('meta[name="twitter:description"]')?.setAttribute('content', route.description);
  document.querySelector('meta[name="twitter:image"]')?.setAttribute('content', 'https://lsp-readiness-check.sociobot.in/og-image.webp');
  app.innerHTML = route.render();
  document.querySelector('h1')?.setAttribute('tabindex', '-1');
  bindActions();
  if (path === '/demo') runDemo();
  if (path === '/sign-in') void prepareSignIn();
  if (path.startsWith('/app')) void loadAccountPage(path);
  live.textContent = document.querySelector('h1')?.textContent ?? '';
  if (focus) document.querySelector<HTMLElement>('h1')?.focus({ preventScroll: false });
}

function navigate(path: string): void {
  history.pushState({}, '', path);
  window.scrollTo(0, 0);
  render(true);
}

function bindActions(): void {
  document.querySelectorAll<HTMLAnchorElement>('[data-link]').forEach((link) => link.addEventListener('click', (event) => {
    event.preventDefault();
    const url = new URL(link.href);
    navigate(`${url.pathname}${url.search}`);
  }));
  document.querySelector('[data-reset]')?.addEventListener('click', () => { localStorage.removeItem('demo:lsp-readiness-check'); location.reload(); });
  document.querySelector('[data-start-real]')?.addEventListener('click', () => localStorage.removeItem('demo:lsp-readiness-check'));
  document.querySelector('[data-run-demo]')?.addEventListener('click', runDemo);
  document.querySelector('[data-replay]')?.addEventListener('click', runDemo);
  document.querySelector('[data-sign-in]')?.addEventListener('click', () => void beginSignIn().catch((error) => {
    const panel = document.querySelector<HTMLElement>('[data-setup-panel]');
    if (panel) panel.innerHTML = `<p class="error-text">${escapeHtml(errorMessage(error, 'Sign-in could not start.'))}</p>`;
  }));
  document.querySelector('[data-sign-out]')?.addEventListener('click', () => {
    sessionStorage.removeItem(ACCESS_TOKEN_KEY);
    navigate('/');
  });
  document.querySelector('[data-copy]')?.addEventListener('click', async (event) => {
    const button = event.currentTarget as HTMLButtonElement;
    try { await navigator.clipboard.writeText(button.dataset.copy ?? ''); button.textContent = 'Copied'; }
    catch { button.textContent = 'Copy failed'; }
  });
}

async function fetchSetup(): Promise<SetupConfig> {
  const response = await fetch(`${API_ORIGIN}/api/v1/config`, { headers: { Accept: 'application/json' } });
  if (!response.ok) throw new Error('The private CI service is not available.');
  return response.json() as Promise<SetupConfig>;
}

async function prepareSignIn(): Promise<void> {
  const panel = document.querySelector<HTMLElement>('[data-setup-panel]');
  const button = document.querySelector<HTMLButtonElement>('[data-sign-in]');
  if (!panel || !button) return;
  const parameters = new URLSearchParams(location.search);
  if (parameters.has('error')) {
    panel.innerHTML = '<p class="error-text">Sign-in was not completed. Return here and try again.</p>';
    return;
  }
  if (parameters.has('code')) {
    panel.innerHTML = '<p>Completing sign-in…</p>';
    try {
      await completeSignIn(parameters.get('code')!, parameters.get('state') ?? '');
      history.replaceState({}, '', '/app');
      render(true);
    } catch (error) {
      panel.innerHTML = `<p class="error-text">${escapeHtml(errorMessage(error, 'Sign-in could not be completed. Start again.'))}</p>`;
    }
    return;
  }
  try {
    const config = await fetchSetup();
    if (!config.identity_configured) {
      panel.innerHTML = '<p><strong>Sign-in is not open.</strong> The product CIAM registration still needs operator setup and hosted QA.</p>';
      return;
    }
    panel.innerHTML = '<p><strong>Account setup is ready.</strong> Your password stays with the identity provider.</p>';
    button.disabled = false;
  } catch (error) {
    panel.innerHTML = `<p class="error-text">${escapeHtml(errorMessage(error, 'The account service could not be reached. Try the sample instead.'))}</p>`;
  }
}

async function beginSignIn(): Promise<void> {
  const config = await fetchSetup();
  if (!config.identity_configured || !config.client_id || !config.authorize_url || !config.scope) {
    throw new Error('Sign-in needs the product CIAM registration.');
  }
  const verifier = randomUrlToken(48);
  const state = randomUrlToken(24);
  const challengeBytes = await crypto.subtle.digest('SHA-256', new TextEncoder().encode(verifier));
  const challenge = base64Url(new Uint8Array(challengeBytes));
  sessionStorage.setItem(PKCE_KEY, JSON.stringify({ verifier, state, token_url: config.token_url, client_id: config.client_id, redirect_url: config.redirect_url }));
  const authorize = new URL(config.authorize_url);
  authorize.searchParams.set('client_id', config.client_id);
  authorize.searchParams.set('response_type', 'code');
  authorize.searchParams.set('redirect_uri', config.redirect_url);
  authorize.searchParams.set('scope', `openid profile email ${config.scope}`);
  authorize.searchParams.set('code_challenge', challenge);
  authorize.searchParams.set('code_challenge_method', 'S256');
  authorize.searchParams.set('state', state);
  location.assign(authorize);
}

async function completeSignIn(code: string, returnedState: string): Promise<void> {
  const stored = sessionStorage.getItem(PKCE_KEY);
  if (!stored) throw new Error('This sign-in was not started in this browser session.');
  const pkce = JSON.parse(stored) as { verifier: string; state: string; token_url?: string; client_id: string; redirect_url: string };
  if (pkce.state !== returnedState || !pkce.token_url) throw new Error('The sign-in state did not match. Start again.');
  const body = new URLSearchParams({
    client_id: pkce.client_id,
    grant_type: 'authorization_code',
    code,
    redirect_uri: pkce.redirect_url,
    code_verifier: pkce.verifier,
  });
  const response = await fetch(pkce.token_url, { method: 'POST', headers: { 'Content-Type': 'application/x-www-form-urlencoded' }, body });
  if (!response.ok) throw new Error('The identity provider did not accept the sign-in response.');
  const token = await response.json() as { access_token?: string };
  if (!token.access_token) throw new Error('The identity provider returned no access token.');
  sessionStorage.setItem(ACCESS_TOKEN_KEY, token.access_token);
  sessionStorage.removeItem(PKCE_KEY);
}

async function apiRequest(path: string, init: RequestInit = {}): Promise<Response> {
  const token = sessionStorage.getItem(ACCESS_TOKEN_KEY);
  if (!token) throw new Error('Sign in to open this page.');
  const headers = new Headers(init.headers);
  headers.set('Authorization', `Bearer ${token}`);
  headers.set('Accept', 'application/json');
  if (init.body) headers.set('Content-Type', 'application/json');
  const response = await fetch(`${API_ORIGIN}/api/v1${path}`, { ...init, headers });
  if (response.status === 401) {
    sessionStorage.removeItem(ACCESS_TOKEN_KEY);
    throw new Error('Your session ended. Sign in again.');
  }
  return response;
}

async function loadAccountPage(path: string): Promise<void> {
  const content = document.querySelector<HTMLElement>('[data-account-content]');
  if (!content) return;
  if (!sessionStorage.getItem(ACCESS_TOKEN_KEY)) {
    content.innerHTML = '<p><strong>Sign in is required.</strong> Your free CLI and sample remain available without an account.</p><a class="button secondary" href="/sign-in" data-link>Open sign-in status</a>';
    bindActions();
    return;
  }
  try {
    if (path === '/app') await loadDashboard(content);
    else if (path === '/app/repositories') await loadRepositories(content);
    else if (path === '/app/billing') await loadBilling(content);
    else if (/^\/app\/repositories\/[^/]+\/policy$/.test(path)) await loadPolicy(content, path.split('/')[3]);
  } catch (error) {
    content.innerHTML = `<p class="error-text">${escapeHtml(errorMessage(error, 'This account page could not be loaded. Try again.'))}</p><a href="/sign-in" data-link>Return to sign-in</a>`;
    bindActions();
  }
}

async function loadDashboard(content: HTMLElement): Promise<void> {
  const response = await apiRequest('/session');
  if (!response.ok) throw await apiError(response);
  const session = await response.json() as { user: { display_name?: string }; organization: { name: string } };
  content.innerHTML = `<p><strong>${escapeHtml(session.organization.name)}</strong></p><p>Signed in as ${escapeHtml(session.user.display_name ?? 'team owner')}.</p><ol class="setup-steps"><li>Connect the registered GitHub App.</li><li>Choose a repository policy.</li><li>Send its signed readiness report from CI.</li></ol><div class="account-actions"><a class="button secondary" href="/app/repositories" data-link>View repositories</a><button class="text-button" data-export-account>Export team data</button><button class="text-button danger-link" data-delete-account>Delete team data</button></div><p class="form-status" data-account-status aria-live="polite"></p>`;
  content.querySelector('[data-export-account]')?.addEventListener('click', () => void exportAccount(content));
  content.querySelector('[data-delete-account]')?.addEventListener('click', () => void deleteAccount(session.organization.name, content));
  bindActions();
}

async function exportAccount(content: HTMLElement): Promise<void> {
  const status = content.querySelector<HTMLElement>('[data-account-status]')!;
  status.textContent = 'Preparing the export…';
  const response = await apiRequest('/account/export');
  if (!response.ok) { status.textContent = errorMessage(await apiError(response), 'The export could not be prepared.'); return; }
  const blob = await response.blob();
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = 'lsp-readiness-team-export.json';
  link.click();
  URL.revokeObjectURL(url);
  status.textContent = 'Team data exported.';
}

async function deleteAccount(organizationName: string, content: HTMLElement): Promise<void> {
  if (!confirm(`Delete all stored data for ${organizationName}? This cannot be undone.`)) return;
  const status = content.querySelector<HTMLElement>('[data-account-status]')!;
  status.textContent = 'Deleting team data…';
  const response = await apiRequest('/account', { method: 'DELETE' });
  if (!response.ok) { status.textContent = errorMessage(await apiError(response), 'Team data could not be deleted.'); return; }
  sessionStorage.removeItem(ACCESS_TOKEN_KEY);
  navigate('/');
}

async function loadRepositories(content: HTMLElement): Promise<void> {
  const [repositoriesResponse, setup] = await Promise.all([apiRequest('/repositories'), fetchSetup()]);
  if (!repositoriesResponse.ok) throw await apiError(repositoriesResponse);
  const result = await repositoriesResponse.json() as { repositories: Array<{ id: string; owner: string; name: string; private: boolean }> };
  const list = result.repositories.length
    ? `<ul class="repository-list">${result.repositories.map((repository) => `<li><span><strong>${escapeHtml(repository.owner)}/${escapeHtml(repository.name)}</strong><small>${repository.private ? 'Private repository' : 'Public repository'}</small></span><a href="/app/repositories/${encodeURIComponent(repository.id)}/policy" data-link>Set requirements</a></li>`).join('')}</ul>`
    : '<p>No repositories are connected. An approved GitHub App installation will list them here.</p>';
  const action = setup.github_app_configured
    ? '<button class="button primary" data-connect-github>Connect GitHub App</button>'
    : '<p class="dependency-note"><strong>GitHub connection is not open.</strong> The product GitHub App registration still needs operator setup and hosted QA.</p>';
  content.innerHTML = `${list}${action}`;
  content.querySelector('[data-connect-github]')?.addEventListener('click', () => void connectGithub(content));
  bindActions();
}

async function connectGithub(content: HTMLElement): Promise<void> {
  const response = await apiRequest('/github/connect', { method: 'POST' });
  if (!response.ok) throw await apiError(response);
  const body = await response.json() as { url: string };
  content.innerHTML = '<p>Opening the GitHub App installation…</p>';
  location.assign(body.url);
}

async function loadPolicy(content: HTMLElement, repositoryId: string): Promise<void> {
  const [repositoryResponse, policyResponse] = await Promise.all([
    apiRequest(`/repositories/${encodeURIComponent(repositoryId)}`),
    apiRequest(`/repositories/${encodeURIComponent(repositoryId)}/policy`),
  ]);
  if (!repositoryResponse.ok) throw await apiError(repositoryResponse);
  if (!policyResponse.ok) throw await apiError(policyResponse);
  const repository = await repositoryResponse.json() as { owner: string; name: string };
  const policy = await policyResponse.json() as { required_lsp: boolean; required_formatters: boolean; required_tests: boolean; version: number };
  content.innerHTML = `<form class="policy-form" data-policy-form><fieldset><legend>${escapeHtml(repository.owner)}/${escapeHtml(repository.name)}</legend><label><input type="checkbox" name="required_lsp" ${policy.required_lsp ? 'checked' : ''}> Require language servers</label><label><input type="checkbox" name="required_formatters" ${policy.required_formatters ? 'checked' : ''}> Require formatters</label><label><input type="checkbox" name="required_tests" ${policy.required_tests ? 'checked' : ''}> Require tests</label></fieldset><button class="button primary" type="submit">Save requirements</button><p class="form-status" data-policy-status aria-live="polite">${policy.version ? `Policy version ${policy.version}` : 'Default requirements are shown.'}</p></form><section class="ci-token" aria-labelledby="ci-token-title"><h2 id="ci-token-title">Send reports from CI</h2><p>Create a repository token after your GitHub connection is approved. The token appears once.</p><button class="button secondary" data-create-report-token>Create CI report token</button><div data-token-result aria-live="polite"></div></section>`;
  const form = content.querySelector<HTMLFormElement>('[data-policy-form]')!;
  form.addEventListener('submit', async (event) => {
    event.preventDefault();
    const status = form.querySelector<HTMLElement>('[data-policy-status]')!;
    status.textContent = 'Saving requirements…';
    const data = new FormData(form);
    const response = await apiRequest(`/repositories/${encodeURIComponent(repositoryId)}/policy`, { method: 'PUT', body: JSON.stringify({ required_lsp: data.has('required_lsp'), required_formatters: data.has('required_formatters'), required_tests: data.has('required_tests') }) });
    if (!response.ok) { status.textContent = errorMessage(await apiError(response), 'Requirements could not be saved.'); return; }
    const saved = await response.json() as { version: number };
    status.textContent = `Saved policy version ${saved.version}.`;
  });
  content.querySelector('[data-create-report-token]')?.addEventListener('click', () => void createReportToken(content, repositoryId));
}

async function createReportToken(content: HTMLElement, repositoryId: string): Promise<void> {
  const result = content.querySelector<HTMLElement>('[data-token-result]')!;
  result.innerHTML = '<p>Creating a new token…</p>';
  const response = await apiRequest(`/repositories/${encodeURIComponent(repositoryId)}/report-token`, { method: 'POST' });
  if (!response.ok) { result.innerHTML = `<p class="error-text">${escapeHtml(errorMessage(await apiError(response), 'The token could not be created.'))}</p>`; return; }
  const body = await response.json() as { token: string };
  result.innerHTML = `<p><strong>Copy this token now.</strong> It will not be shown again.</p><code class="token-value">${escapeHtml(body.token)}</code><p>Store it as <code>LSP_READINESS_REPORT_TOKEN</code> in your CI secret store.</p>`;
}

async function loadBilling(content: HTMLElement): Promise<void> {
  const response = await apiRequest('/billing');
  if (!response.ok) throw await apiError(response);
  const billing = await response.json() as { available: boolean; price_minor: number; currency: string; interval: string; paid_features: string[] };
  content.innerHTML = `<p class="price"><strong>$${(billing.price_minor / 100).toFixed(0)}</strong> per repository each ${escapeHtml(billing.interval)}</p><ul>${billing.paid_features.map((feature) => `<li>${escapeHtml(feature)}</li>`).join('')}</ul><p class="dependency-note"><strong>Subscriptions are not open.</strong> Sociobot subscription registration and entitlement QA are still required.</p>`;
}

async function apiError(response: Response): Promise<Error> {
  const body = await response.json().catch(() => ({})) as { message?: string };
  return new Error(body.message ?? `Request failed with status ${response.status}.`);
}

function errorMessage(error: unknown, fallback: string): string {
  return error instanceof Error && error.message ? error.message : fallback;
}

function randomUrlToken(length: number): string {
  const bytes = new Uint8Array(length);
  crypto.getRandomValues(bytes);
  return base64Url(bytes);
}

function base64Url(bytes: Uint8Array): string {
  let binary = '';
  bytes.forEach((byte) => { binary += String.fromCharCode(byte); });
  return btoa(binary).replaceAll('+', '-').replaceAll('/', '_').replaceAll('=', '');
}

function escapeHtml(value: string): string {
  return value.replace(/[&<>"']/g, (character) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[character]!);
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

<span class="muted">Tamper check: Ed25519 signature
Sample readiness report is shown only in this demo.</span></code>`;
}

function reducedMotion() { return matchMedia('(prefers-reduced-motion: reduce)').matches; }
function wait(ms: number) { return new Promise((resolve) => setTimeout(resolve, ms)); }

document.addEventListener('click', (event) => {
  const anchor = (event.target as Element).closest<HTMLAnchorElement>('a[href^="/#"]');
  if (!anchor || location.pathname !== '/') return;
  event.preventDefault(); document.querySelector(anchor.hash)?.scrollIntoView({ behavior: reducedMotion() ? 'auto' : 'smooth' });
});
window.addEventListener('popstate', () => render(true));
render();
if ('serviceWorker' in navigator) window.addEventListener('load', () => navigator.serviceWorker.register('/service-worker.js').catch(() => undefined));
