# Mail for Agents

Mail for Agents is the public, self-hostable email automation product. The same
multi-workspace binary runs the hosted service; the private cloud controller
provisions paid accounts and operates the hosted deployment.

This repository currently provides the first deployment slice: an Axum and
Askama operator console, SQLite persistence in WAL mode, tenant foundation
tables, health checks, and an explicit capability manifest. Authentication,
billing, and workflow actions are intentionally reported as unavailable until
their end-to-end implementations exist.

## Run locally

```bash
cp .env.example .env
set -a
source .env
set +a
cargo run --package mailforagents
```

Open <http://127.0.0.1:3200>. The SQLite database is created at
`data/app.sqlite` by default.

## Container

```bash
docker build --tag mailforagents:local .
docker run --rm --publish 3200:3200 \
  --volume mailforagents-data:/app/data \
  mailforagents:local
```

Use a persistent volume for `/app/data`. The application enables foreign keys,
WAL mode, a busy timeout, and schema initialization at startup.

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `MFA_APP_ADDR` | `127.0.0.1:3200` | Listener address |
| `MFA_DATABASE_PATH` | `data/app.sqlite` | SQLite database path |
| `MFA_DEPLOYMENT_MODE` | `self-hosted` | `self-hosted` or `cloud` presentation mode |
| `RUST_LOG` | product and HTTP info | Rust tracing filter |

The hosted instance uses `MFA_DEPLOYMENT_MODE=cloud`; this does not unlock a
different product edition. Cloud-only billing, provisioning, fleet management,
and operational data remain in the private `mailforagents-cloud` repository.

## Deployment

GitHub Actions verifies the Rust workspace, publishes immutable images to
`ghcr.io/mailforagents/mailforagents`, points the Coolify resource at the
commit-specific image, enforces `https://app.mailforagents.com`, deploys it,
and waits for Coolify to report success.

Production deployment expects:

- GitHub environment secrets `COOLIFY_URL` and `COOLIFY_TOKEN`;
- GitHub environment variable `COOLIFY_RESOURCE_UUID`;
- repository variable `COOLIFY_DEPLOY_ENABLED=true`.

## License

Copyright © 2026 Mail for Agents contributors.

Mail for Agents is free software licensed under the
[GNU Affero General Public License version 3 or later](LICENSE). If you run a
modified version as a network service, the AGPL requires you to offer its
corresponding source code to users of that service. The private cloud control
plane is a separate work and is not included in this repository or license.
