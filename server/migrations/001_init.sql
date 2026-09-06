PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS schema_migrations (
  version INTEGER PRIMARY KEY,
  applied_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  ciam_subject TEXT NOT NULL UNIQUE,
  display_name TEXT,
  email TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS organizations (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS memberships (
  organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  role TEXT NOT NULL CHECK (role IN ('owner', 'member')),
  created_at INTEGER NOT NULL,
  PRIMARY KEY (organization_id, user_id)
);

CREATE TABLE IF NOT EXISTS github_installations (
  id TEXT PRIMARY KEY,
  organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  github_installation_id INTEGER NOT NULL UNIQUE,
  account_login TEXT NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS github_connect_states (
  state_hash TEXT PRIMARY KEY,
  organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  expires_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS repositories (
  id TEXT PRIMARY KEY,
  organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  github_installation_id TEXT REFERENCES github_installations(id) ON DELETE SET NULL,
  github_repository_id INTEGER NOT NULL,
  owner TEXT NOT NULL,
  name TEXT NOT NULL,
  private INTEGER NOT NULL CHECK (private IN (0, 1)),
  report_token_hash TEXT NOT NULL UNIQUE,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE (organization_id, github_repository_id)
);

CREATE TABLE IF NOT EXISTS policies (
  id TEXT PRIMARY KEY,
  organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  repository_id TEXT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
  required_lsp INTEGER NOT NULL DEFAULT 1 CHECK (required_lsp IN (0, 1)),
  required_formatters INTEGER NOT NULL DEFAULT 1 CHECK (required_formatters IN (0, 1)),
  required_tests INTEGER NOT NULL DEFAULT 1 CHECK (required_tests IN (0, 1)),
  version INTEGER NOT NULL DEFAULT 1,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE (organization_id, repository_id)
);

CREATE TABLE IF NOT EXISTS runs (
  id TEXT PRIMARY KEY,
  organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  repository_id TEXT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
  pull_request INTEGER,
  external_run_id TEXT,
  schema_url TEXT NOT NULL,
  report_repository TEXT NOT NULL,
  generated_at INTEGER NOT NULL,
  ready INTEGER NOT NULL CHECK (ready IN (0, 1)),
  languages_json TEXT NOT NULL,
  capabilities_json TEXT NOT NULL,
  source_digest TEXT NOT NULL,
  algorithm TEXT NOT NULL,
  public_key TEXT NOT NULL,
  signature TEXT NOT NULL,
  received_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS subscriptions (
  id TEXT PRIMARY KEY,
  organization_id TEXT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  repository_id TEXT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
  provider_reference TEXT NOT NULL UNIQUE,
  status TEXT NOT NULL,
  expires_at INTEGER,
  verified_at INTEGER NOT NULL,
  UNIQUE (organization_id, repository_id)
);

CREATE INDEX IF NOT EXISTS repositories_org_idx ON repositories(organization_id, id);
CREATE INDEX IF NOT EXISTS policies_org_repo_idx ON policies(organization_id, repository_id);
CREATE INDEX IF NOT EXISTS runs_org_repo_idx ON runs(organization_id, repository_id, received_at DESC);
CREATE INDEX IF NOT EXISTS memberships_user_idx ON memberships(user_id, organization_id);

INSERT OR IGNORE INTO schema_migrations(version, applied_at)
VALUES (1, unixepoch());
