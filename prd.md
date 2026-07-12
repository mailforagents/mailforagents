# Mail for Agents
## Product Requirements Document

**Product descriptor:** Open-source, agent-native lifecycle email automation with AWS SES and installed business playbooks  
**Domain:** `mailforagents.com`  
**Document version:** 0.1  
**Status:** Build-ready product draft  
**Date:** 12 July 2026  
**Primary implementation:** Rust, Axum, HTMX, SQLite, SeaORM  
**Recommended licence:** AGPL-3.0 for the application core, subject to legal review

---

## Contents

- [Strategy and product definition](#1-executive-summary) — Sections 1–10
- [Technical architecture and domain model](#11-technical-architecture) — Sections 11–16
- [Complete playbook library](#17-complete-v1-playbook-library) — Sections 17–18
- [AWS SES, messaging, agents, and API](#19-aws-ses-integration) — Sections 19–23
- [Information architecture and visual design](#24-information-architecture) — Sections 24–28
- [Migration, analytics, security, reliability, and deployment](#29-migration-from-incumbent-platforms) — Sections 29–35
- [Open-source strategy, roadmap, acceptance criteria, and decisions](#36-open-source-distribution-and-go-to-market) — Sections 36–42
- [Appendices and references](#appendix-a--representative-database-tables)

---

## 1. Executive summary

Mail for Agents is an open-source alternative to ActiveCampaign automations, HubSpot workflows, Mailchimp customer journeys, and Customer.io-style lifecycle messaging. It is built for software companies, technical service businesses, consultants, agencies, and AI agents.

The product combines five ideas:

1. **Installed business playbooks instead of a blank automation canvas.** A workspace begins with opinionated, evidence-informed lifecycle systems for activation, trials, annual plans, failed payments, consulting qualification, project-to-retainer conversion, value reporting, referrals, appointments, and related use cases.
2. **Agent-native operation.** Every meaningful function is available through a public API, MCP surface, CLI, webhooks, and a versioned workflow format. Agents may inspect, draft, simulate, and propose changes. Bulk sending and workflow activation remain behind explicit approval by default.
3. **AWS SES as the default transport.** Customers connect their own AWS account and pay AWS directly for delivery. Mail for Agents supplies orchestration, contacts, consent, templates, analytics, suppression, event processing, and deliverability UX.
4. **A simple, self-hostable Rust architecture.** The community edition is one application binary plus one SQLite database. It does not require Redis, Temporal, Kafka, PostgreSQL, or a JavaScript application server.
5. **The Postiz open-source growth model.** The complete useful core is open source; the paid product sells managed hosting, upgrades, backups, collaboration, and support rather than withholding the core workflow engine.

### Product promise

> Connect AWS SES, describe how customers succeed, and install the lifecycle system.

### Primary homepage message

> **Email automation built for agents.**  
> Install proven lifecycle playbooks, connect your own AWS SES account, and operate every workflow through the web, API, CLI, or MCP.

### Supporting message

> No contact-based pricing. No blank canvas. No vendor lock-in.

### Important terminology

- **Playbook:** A packaged business strategy containing instrumentation, workflow logic, message drafts, measurement, economics, and guardrails.
- **Workflow:** A deterministic, versioned state machine executed by the application.
- **Agent:** An AI or software operator with scoped permissions to inspect or change the system.
- **Event:** A fact about customer or business behaviour, such as `account.created`, `report.generated`, or `invoice.payment_failed`.
- **Message:** One rendered email for one recipient.
- **Broadcast:** A one-time or scheduled send to an audience.
- **Topic:** A consent and preference category such as `product_updates`, `education`, or `billing`.

---

## 2. Strategic reference: what to copy from Postiz

Postiz is the strategic benchmark, not a feature blueprint.

As of July 2026, its public GitHub repository showed roughly **33,000 stars and 6,200 forks**. Its README stated that the project had almost **six million downloads** and approximately **20,000 repository views per month**. The repository is AGPL-3.0 and the product offers a hosted service alongside self-hosting.[1] Postiz now presents itself as an agentic scheduler supporting 30+ networks and explicitly promotes operation through agents, a CLI, public API, n8n, Make, and Zapier.[2]

The commercial figures are founder-reported rather than independently audited. The founder reported approximately **$14,200 in monthly subscription revenue in November 2025**, after increasing focus on n8n, the public API, automation users, and affiliates.[3] A June 2026 Indie Hackers profile reported approximately **$113,000 MRR / $1.3 million ARR** after two years, while describing a cloud product that mirrors the open-source version.[4]

### Lessons applied to Mail for Agents

| Postiz lesson | Mail for Agents application |
|---|---|
| A full open-source product can create trust and developer distribution | Ship contacts, workflows, SES support, playbooks, API, and agent tools in the community edition |
| Hosted convenience is a valid product even when self-hosting is free | Sell managed hosting, upgrades, backups, monitoring, collaboration, and support |
| Integrations unlock adjacent audiences | Launch with SES, webhooks, Stripe examples, n8n, MCP, CLI, and SDKs |
| Public APIs create durable use and retention | Make every web action available through stable API primitives |
| Agentic repositioning can create a new category around an existing tool | Lead with agent-operable lifecycle automation, not merely an open-source ESP |
| A large integration/template surface creates compounding search demand | Publish one indexable page per playbook, integration, migration path, and event recipe |
| A visible open-source project becomes a marketing asset | Maintain an excellent README, public roadmap, changelog, demo data, and one-command deployment |

### What not to copy

- Do not copy Postiz’s user interface, branding, wording, or source implementation.
- Do not expand to every channel before email workflows are excellent.
- Do not make self-hosting require a collection of infrastructure services.
- Do not treat GitHub stars as proof of product-market fit or revenue.
- Do not use unqualified founder-reported revenue as an audited claim.
- Do not allow an “agentic” label to replace deterministic execution and safety controls.

---

## 3. Problem statement

Existing email automation products create five recurring problems.

### 3.1 Blank-canvas failure

Most platforms give users a visual workflow canvas but little help deciding:

- which customer state matters;
- which events must be instrumented;
- when a human should intervene;
- what stopping condition prevents irrelevant email;
- how to measure economic value;
- how to distinguish a useful lifecycle message from another generic drip.

The difficult work is business-system design, not drawing arrows between boxes.

### 3.2 Contact-based pricing

Customers often pay more merely because they retain a larger audience, even when most contacts receive no message. This makes a customer database economically punitive and encourages deletion rather than useful retention.

### 3.3 Cheap delivery, expensive orchestration

AWS SES supplies scalable email transport, identity verification, quotas, event publishing, suppression features, and subscription management. It does not provide a complete opinionated lifecycle automation product. Customers can obtain inexpensive transport but must build the operational layer themselves.

### 3.4 Human-only administration

Traditional tools assume that a person repeatedly clicks through a web interface. Modern software teams increasingly use coding agents, internal agents, n8n, and command-line workflows. Existing tools often expose incomplete APIs and opaque workflow state.

### 3.5 Opaque execution and lock-in

Users struggle to answer:

- Why did this person receive this email?
- Which version of the automation ran?
- What changed last Tuesday?
- Can the workflow be reviewed in Git?
- Can it be exported or recreated elsewhere?
- What happens to contacts already inside a workflow after an edit?

Mail for Agents makes the workflow a portable, inspectable document and retains a full execution trace.

---

## 4. Product vision

Mail for Agents becomes the default lifecycle layer for AI-native and technically operated businesses.

A founder connects the application, SES, billing, and product events. Mail for Agents identifies missing instrumentation, recommends appropriate playbooks, creates original message drafts, simulates customer paths, estimates affected recipients, and requests approval. Deterministic workflows then run continuously while agents monitor outcomes and propose improvements.

### Example product experience

A user writes:

> I run a B2B SaaS. A customer succeeds after connecting a domain and generating a first report. Accounts with more than 20 employees are worth personal onboarding. Install the recommended activation system.

Mail for Agents responds with:

1. A proposed lifecycle model: `signed_up → domain_connected → report_generated → paid`.
2. A missing event checklist.
3. SDK snippets for `domain.connected` and `report.generated`.
4. An activation workflow with state-aware branches.
5. Original email drafts.
6. A high-value account alert.
7. A simulation using sample contacts.
8. A measurable goal and simple economics model.
9. A publish request requiring human approval.

---

## 5. Goals and non-goals

### 5.1 Goals

1. Deploy the community edition using one binary or one Docker Compose command.
2. Connect a customer-owned AWS SES account without requiring broad AWS administrator credentials.
3. Send a verified test email within ten minutes after usable AWS credentials and DNS are available.
4. Ingest contacts and behavioural events through a clear API and SDK.
5. Install a playbook with an instrumentation checklist, message drafts, workflow, goal, and dashboard.
6. Execute durable waits, branches, retries, stopping conditions, and goal exits without an external workflow service.
7. Explain every send using an immutable workflow version and execution trace.
8. Let agents inspect, draft, simulate, and propose through MCP and API.
9. Require explicit approval for risky actions by default.
10. Support broadcasts as well as event-driven automations.
11. Preserve consent, topic preferences, bounces, complaints, and suppressions across all sends.
12. Import contacts, suppression data, templates, and eventually automations from major incumbents.
13. Provide a dark, precise, developer-console interface inspired by the restraint and information density of the xAI Console, without copying its protected assets or exact layouts.

### 5.2 Non-goals for v1

- A general CRM with deals, pipelines, forecasting, and call logging.
- A landing-page or website builder.
- SMS, WhatsApp, push notifications, or social media.
- A full Gmail-style shared inbox or help desk.
- Hosting an SMTP server or becoming an inbox provider.
- A complex freeform workflow canvas requiring a large client-side framework.
- Arbitrary user-supplied code in workflow conditions.
- Fully autonomous bulk sending by an LLM.
- Guaranteed inbox placement.
- Multi-region active-active operation.
- Horizontal writes against one SQLite file.
- A proprietary library marketed with Patrick McKenzie’s name or endorsement without permission.

---

## 6. Product principles

1. **Install a business system, not a diagram.** The first screen is a playbook recommendation, not an empty canvas.
2. **Behaviour beats elapsed time.** Time delays are useful, but state and event conditions determine relevance.
3. **Deterministic execution; agent-assisted design.** Agents may propose. A typed workflow engine executes.
4. **Every action is explainable.** A recipient timeline must show trigger, conditions, workflow version, message version, and outcome.
5. **No hidden transport tax.** BYO SES is the default commercial and self-hosted path.
6. **One useful binary.** The default install has no mandatory external queue, cache, or database server.
7. **Safe by default.** Marketing sends require consent checks, frequency limits, suppression checks, previews, and approval policies.
8. **Portable by construction.** Playbooks and workflows have stable schemas and export cleanly.
9. **Open core means the real core.** Self-hosters get the workflow engine, SES integration, API, and playbooks.
10. **Evidence is labelled honestly.** “Evidence-informed,” “community-validated,” and “workspace-proven” are distinct states.
11. **Original copy, attributed ideas.** Public expert ideas may be cited; protected course material and distinctive copy are not reproduced.
12. **The operator remains accountable.** The product assists compliance and deliverability but does not pretend to replace legal or deliverability expertise.

---

## 7. Target users and jobs to be done

### 7.1 Technical SaaS founder

**Context:** Has product events, Stripe, and an SES or transactional email setup; dislikes per-contact pricing.

**Jobs:**

- Turn product events into activation and conversion workflows.
- See where users stall.
- Install a reasonable lifecycle system without hiring a specialist.
- Let a coding agent wire events and maintain workflows.
- Retain control of data and delivery infrastructure.

### 7.2 Growth engineer or technical marketer

**Context:** Owns onboarding, lifecycle messaging, experiments, and reporting.

**Jobs:**

- Define segments and state transitions precisely.
- Simulate changes before publishing.
- Version and review workflows.
- Measure goal completion rather than vanity opens.
- Coordinate human outreach for valuable accounts.

### 7.3 Consultant or productized-service operator

**Context:** Sells audits, discovery, implementation, and recurring retainers.

**Jobs:**

- Qualify enquiries.
- Convert free diagnostics into paid work.
- Automate paid discovery and proposal follow-up.
- turn completed projects into recurring retainers.
- Produce value reports and request testimonials/referrals at the right moment.

### 7.4 Agency

**Context:** Manages several client businesses and sending identities.

**Jobs:**

- Reuse playbooks while keeping client data isolated.
- Connect a separate SES account per client.
- Apply approval policies and generate client-facing reporting.
- Export or transfer a client workspace cleanly.

### 7.5 Self-hosting operator

**Context:** Values sovereignty, predictable infrastructure, and simple upgrades.

**Jobs:**

- Run one container and one durable database file.
- Back up and restore reliably.
- Inspect logs and health without cloud-only tooling.
- Upgrade without manual schema work.

### 7.6 AI or software agent

**Context:** Operates under delegated access from a human or service account.

**Jobs:**

- Discover event schemas and available playbooks.
- Propose a workflow or message change.
- Run a simulation and retrieve a structured trace.
- Request approval.
- Diagnose why a contact did or did not receive a message.
- Read metrics and recommend an experiment within explicit permissions.

---

## 8. Success metrics

### 8.1 North-star metric

**Weekly successful playbook outcomes per active workspace.**

A successful outcome is the goal event defined by an installed playbook, such as activation, recovered payment, annual upgrade, booked discovery, completed appointment, or accepted retainer.

### 8.2 Activation funnel

1. Workspace created.
2. SES connected.
3. Sending identity verified.
4. First test message delivered.
5. First contact or event ingested.
6. First playbook installed.
7. Simulation completed.
8. First workflow published.
9. First goal completed.

### 8.3 Product metrics

- Median time from account creation to first delivered test.
- Median time from first event to first published playbook.
- Percentage of new workspaces installing a playbook before creating a custom workflow.
- Weekly active workflows.
- Percentage of workflow changes simulated before publishing.
- Percentage of sends with a complete execution explanation.
- Playbook goal conversion and incremental lift where a holdout exists.
- Failed-payment recovery amount.
- Annual-plan cash collected.
- Retainer attach rate for service-business workspaces.

### 8.4 Commercial metrics

- Open-source installations that connect to update checks, measured only with opt-in telemetry.
- Community-to-cloud conversion.
- Cloud trial-to-paid conversion.
- Cloud gross retention and net revenue retention.
- Hosted BYO-SES margin.
- Support hours per active workspace.

### 8.5 Guardrails

- Duplicate message incidents: target zero.
- Complaint and hard-bounce rates, monitored per workspace and sending identity.
- Marketing sends without a valid unsubscribe mechanism: target zero.
- Agent-initiated bulk sends without required approval: target zero.
- Suppression processing latency.
- Ambiguous send retries.
- Unexplained workflow executions.

---

## 9. Evidence model for “proven” playbooks

The product must not imply that every template guarantees a result. Each playbook carries an evidence status.

| Status | Meaning | UI label |
|---|---|---|
| Evidence-informed | Derived from public expert advice, established lifecycle practice, or a documented case | “Evidence-informed” |
| Community-validated | Opt-in aggregate results across multiple unrelated workspaces show useful outcomes | “Community-validated” |
| Workspace-proven | The current workspace has measured improvement through a valid comparison, holdout, or clear operational baseline | “Proven here” |

Every playbook manifest stores:

- source titles and URLs;
- source type;
- original author;
- date reviewed;
- which claims are directly supported;
- which parts are Mail for Agents’ synthesis;
- a warning that outcomes depend on audience, offer, timing, consent, and execution.

Patrick McKenzie’s public lifecycle email writing supports the core emphasis on micro-targeted messages based on a customer’s current relationship with a software business, educating prospects before presenting the product, annual-plan conversion, and systematic sales/customer-success operations.[5][6][7] His public writing on productizing consulting supports recurring retainers, value-added reporting, and capturing repeated expertise into reusable artefacts.[8]

The commercial product must use original playbook wording and original email drafts. “Patio11,” “Patrick McKenzie,” and related identity should not appear in product naming or imply endorsement without explicit permission. Public source attribution may appear in a playbook’s evidence panel.

---

## 10. Business model and licensing

### 10.1 Recommended model

- **Community:** Free, self-hosted, full workflow engine, SES connection, API, MCP, CLI, playbooks, contacts, broadcasts, and analytics.
- **Cloud BYO SES:** Managed Mail for Agents; customer pays AWS directly for sending.
- **Managed delivery:** Later option for customers who do not want an AWS account.
- **Agency:** Multiple workspaces, reusable private playbooks, client approvals, consolidated reporting, and workspace transfer.
- **Enterprise:** SSO, audit export, private networking, managed deployment, support, and contractual controls.

### 10.2 Pricing hypothesis

| Plan | Hypothesis | Commercial unit |
|---|---:|---|
| Community | Free | Self-hosted |
| Cloud Starter | £19/month | One workspace, BYO SES |
| Cloud Growth | £49/month | More team members, retention, experiments, support |
| Agency | £149/month | Multiple client workspaces and approvals |
| Enterprise | Custom | Security, SSO, private hosting, support |

Pricing should be tested. The principal message is:

> Pay for the automation software, not for people sitting in your database.

Fair-use limits may apply to hosted event retention, storage, and compute, but not as disguised contact-based pricing.

### 10.3 Licence

AGPL-3.0 is the recommended starting point because it aligns with the open-source/cloud strategy used by Postiz. The final choice requires legal review, particularly for hosted-network use, contributor agreements, third-party dependencies, and any future enterprise modules.


---

## 11. Technical architecture

### 11.1 Architectural decision

The v1 community edition is a **single Rust application** with three internal roles:

- HTTP server;
- durable scheduler;
- background worker.

They run together by default and may be selected with process flags for development or operational isolation. SQLite remains single-writer storage; running multiple writers against one database file over a network filesystem is unsupported.

The application is multi-workspace by design in both community and hosted modes. A workspace is the v1 tenant/organization boundary. One user may belong to several workspaces through memberships, and one application instance may serve many workspaces from the same SQLite database. Self-hosters receive this capability without an artificial single-workspace restriction.

### 11.2 System diagram

```text
┌───────────────────────────────────────────────────────────────────┐
│ Browser / CLI / SDK / MCP Client                                 │
└──────────────────────────────┬────────────────────────────────────┘
                               │ HTTPS
                               ▼
┌───────────────────────────────────────────────────────────────────┐
│ Axum application                                                  │
│                                                                   │
│  HTML routes ── Askama templates ── HTMX fragments                │
│  REST API ───── JSON                                               │
│  MCP endpoint ─ JSON-RPC / streamable HTTP                         │
│  Webhooks ───── event and provider ingestion                       │
│  SSE ────────── live jobs, events, broadcasts, simulations         │
└─────────────┬────────────────┬───────────────────┬─────────────────┘
              │                │                   │
              ▼                ▼                   ▼
       Domain services   Workflow runtime     Mail renderer
              │                │                   │
              └────────────────┴──────────┬────────┘
                                          ▼
                              SeaORM repositories
                                          │
                                          ▼
                                 SQLite in WAL mode
                                          │
                             durable jobs / outbox / audit
                                          │
                                          ▼
                              AWS SES v2 / AWS STS
                                          │
                                          ▼
                             recipient mail providers
                                          │
                                          ▼
                                  SES event publishing
                                          │
                                          ▼
                                      SNS webhook
                                          │
                                          └──► Axum event normalizer
```

### 11.3 Technology choices

| Area | Choice | Requirement |
|---|---|---|
| Language | Stable Rust | Pin toolchain in `rust-toolchain.toml` and dependencies in `Cargo.lock` |
| HTTP | Axum 0.8.x | Use typed extractors, Tower middleware, structured errors, and shared application state |
| Async runtime | Tokio | One runtime; bounded background concurrency |
| HTML templates | Askama or equivalent compile-time Rust templating | Full-page and fragment templates share components |
| Browser interaction | Stable HTMX 2.x | Pin and vendor assets; do not depend on htmx 4 beta for v1 |
| Database | SQLite | WAL, foreign keys, busy timeout, explicit migration discipline |
| ORM | SeaORM 2.0.x | Entities, transactions, typed queries, migrations, future backend portability |
| AWS | Official AWS SDK for Rust | SES v2 and STS; use standard credential chain in self-hosted mode |
| Email rendering | MJML-compatible Rust renderer plus MiniJinja-style variables | Produce HTML and plain text; deterministic previews |
| MIME | Rust MIME builder | Raw SES messages where custom headers are required |
| API schema | OpenAPI generated from Rust types | Documentation and SDK generation |
| Logging | `tracing` | JSON in production, readable local mode, no secrets |
| Static assets | Embedded at compile time | One-binary deployment; CSP-compatible JavaScript modules |
| CSS | Hand-authored design system using CSS custom properties | No required Node runtime; optional build-time minification only |

Axum’s current documentation emphasises ergonomic routing, typed extraction, predictable errors, and Tower integration.[9] SeaORM 2.0 supports SQLite connection strings and owns an underlying SQLx connection pool.[10] HTMX supports server-driven AJAX, CSS transitions, WebSockets, and server-sent events directly from HTML attributes; v1 should use its stable 2.x line rather than the developing 4.x beta.[11]

### 11.4 Rust workspace layout

```text
mailforagents/
├── Cargo.toml
├── crates/
│   ├── app/                 # binary, startup, configuration, task supervision
│   ├── domain/              # domain types and invariants
│   ├── db/                  # SeaORM entities, migrations, repositories
│   ├── web/                 # Axum HTML routes, templates, HTMX fragments
│   ├── api/                 # REST API, OpenAPI, authentication
│   ├── mcp/                 # MCP server and permission translation
│   ├── workflow/            # compiler, evaluator, simulator, runtime
│   ├── scheduler/           # durable job leasing and retry policy
│   ├── mail/                # rendering, MIME, personalization, preflight
│   ├── ses/                 # AWS adapter, onboarding, quotas, event normalizer
│   ├── contacts/            # identities, properties, segments, consent
│   ├── playbooks/           # manifest parser, installer, evidence metadata
│   ├── analytics/           # goals, cohorts, aggregates, experiments
│   ├── agents/              # proposals, approvals, audit, model adapters
│   ├── integrations/        # Stripe/webhook/n8n/import adapters
│   └── cli/                 # `mailfa` command line client
├── playbooks/               # bundled public playbook definitions
├── templates/               # Askama templates
├── static/                  # vendored HTMX, JS modules, CSS, icons
├── examples/
├── docs/
└── docker/
```

### 11.5 Application modes

```bash
mailforagents serve --mode all       # default: HTTP + scheduler + workers
mailforagents serve --mode web       # HTTP only; development/support mode
mailforagents serve --mode worker    # supported only when sharing a local host/database safely
mailforagents migrate
mailforagents doctor
mailforagents backup
mailforagents restore
```

The public CLI should be named `mailfa`, not `mfa`, because “MFA” is strongly associated with multi-factor authentication.

---

## 12. SQLite and persistence requirements

### 12.1 Default settings

At application startup:

```sql
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;
PRAGMA synchronous = NORMAL;
PRAGMA busy_timeout = 5000;
PRAGMA temp_store = MEMORY;
```

The exact production values must be benchmarked and configurable. The application must refuse to start if foreign keys cannot be enabled or migrations are inconsistent.

### 12.2 Write discipline

- All writes go through repository methods and explicit transactions.
- A process-local semaphore bounds concurrent writers.
- Job claiming uses a short `BEGIN IMMEDIATE` transaction.
- Long-running analytics never hold a write transaction.
- Network calls, SES calls, rendering, and agent calls never occur inside a database transaction.
- Bulk imports use bounded batches and checkpoints.
- Every destructive migration includes backup guidance and a rollback or restore path.

### 12.3 Durable job table

```text
jobs
├── id
├── workspace_id
├── kind
├── payload_json
├── run_at
├── priority
├── status              queued | leased | succeeded | failed | cancelled | ambiguous
├── attempts
├── max_attempts
├── locked_by
├── locked_until
├── dedupe_key
├── last_error_code
├── last_error_message
├── created_at
└── updated_at
```

Required indexes:

- `(status, run_at, priority)`;
- `(workspace_id, status, run_at)`;
- unique partial or equivalent constraint on active `dedupe_key`;
- `(locked_until)` for lease recovery.

### 12.4 Delivery semantics

The scheduler is **at-least-once**, while business operations are made idempotent.

A send step creates a `messages` row with a unique key derived from:

```text
workspace_id + workflow_enrollment_id + workflow_step_id + step_execution_number
```

If SES returns an accepted message ID, the provider ID is stored. If a network timeout occurs after the request may have reached SES, the message becomes `ambiguous`. The system must not blindly retry an ambiguous marketing send because SES does not provide a general application idempotency key for the operation. The operator sees a reconciliation action and may deliberately retry.

### 12.5 Backup and restore

Community requirements:

- Online consistent backup command.
- Optional encrypted S3 backup target.
- Retention policy configuration.
- Restore verification using a temporary database.
- `doctor` command checks integrity, migrations, WAL health, disk space, and backup age.
- Before automatic upgrade migrations, create a local pre-migration backup unless explicitly disabled.

### 12.6 Hosted multi-workspace and scaling boundary

SQLite is a deliberate v1 simplification, not a claim of unlimited distributed scalability.

Initial hosted operation uses:

- one public Mail for Agents application binary serving many workspaces;
- one shared application SQLite database containing tenant-scoped product data for those workspaces;
- one private cloud-controller binary with its own SQLite database for public signup, subscriptions, entitlements, provisioning jobs, Stripe webhook idempotency, and instance routing;
- explicit `workspace_id` scoping on every tenant-owned row, query, job, event, API key, webhook, export, and cache key;
- compound uniqueness and foreign-key constraints that include the workspace boundary where identity is tenant-local;
- locked cross-workspace isolation tests for reads, writes, jobs, webhooks, exports, and agent operations;
- object storage for backups and exports;
- bounded event retention;
- one active writer process per SQLite database.

Paying for hosting creates a workspace and owner membership through an authenticated internal API; it does not create a container or database per customer. The same public binary and multi-workspace model are available to self-hosters. Public signup, Stripe credentials, hosted entitlement policy, abuse controls, fleet management, and support tooling remain responsibilities of the separate private cloud controller.

The v1 product, marketing site where persistence is ever required, and private cloud controller use SQLite exclusively. PostgreSQL, MySQL, Redis, and external queues are not v1 dependencies or hidden requirements. “Unlimited customers” means no product-level customer cap, not infinite capacity from one server. When measured limits are reached, the hosted service adds another complete single-writer application instance and database, assigns new workspaces to it, and moves existing workspaces only through a tested tenant export/import procedure. SeaORM may preserve future backend portability, but no alternative database is designed or operated in v1.

---

## 13. Core domain model

### 13.1 Workspace and access

| Entity | Purpose |
|---|---|
| `workspaces` | Tenant boundary and business configuration |
| `users` | Human identities |
| `memberships` | User-to-workspace role assignment |
| `service_accounts` | Non-human API/MCP identities |
| `sessions` | Browser sessions |
| `api_keys` | Hashed public API credentials and scopes |
| `approval_policies` | Rules determining which changes require approval |
| `approvals` | Pending, approved, rejected, expired requests |
| `audit_logs` | Immutable security and change trail |

Default roles:

- Owner
- Administrator
- Marketer
- Developer
- Analyst
- Reviewer
- Read-only
- Agent service account

### 13.2 Contacts and consent

| Entity | Purpose |
|---|---|
| `contacts` | Primary profile, lifecycle state, external identifiers |
| `contact_identities` | Email addresses and source-system identities |
| `field_definitions` | Typed custom property definitions |
| `contact_field_values` | Typed property values where indexing is required |
| `contact_tags` | Lightweight labels |
| `topics` | Preference and consent categories |
| `consent_records` | Source, purpose, timestamp, evidence, and status |
| `contact_topic_preferences` | Subscribed, unsubscribed, unknown, or required transactional state |
| `suppressions` | Hard bounce, complaint, manual, global, workspace, or category blocks |
| `segments` | Saved audience definitions |
| `segment_versions` | Immutable segment rules |
| `segment_memberships` | Materialized or cached membership when needed |

Email addresses are normalized for comparison while preserving the original form for display. The application must not invent provider-specific normalization such as stripping dots from local parts.

### 13.3 Events

| Entity | Purpose |
|---|---|
| `events` | Append-only business/customer events |
| `event_schemas` | Expected properties and documentation |
| `event_idempotency_keys` | Duplicate prevention |
| `event_aggregates` | Counts, first/last occurrence, and rolling summaries |
| `event_dead_letters` | Invalid events retained for diagnosis |

Every event includes:

```json
{
  "id": "evt_...",
  "workspace_id": "ws_...",
  "name": "report.generated",
  "occurred_at": "2026-07-12T10:15:00Z",
  "received_at": "2026-07-12T10:15:01Z",
  "contact": {
    "external_id": "usr_123",
    "email": "person@example.com"
  },
  "properties": {
    "report_type": "market_scan"
  },
  "context": {
    "source": "sdk-js",
    "idempotency_key": "order-or-event-key"
  }
}
```

### 13.4 Content and messages

| Entity | Purpose |
|---|---|
| `email_templates` | Stable template identity |
| `email_template_versions` | Subject, preheader, HTML/MJML, text, variables, metadata |
| `message_topics` | Message classification and preference mapping |
| `messages` | One recipient/render/send record |
| `message_attempts` | Provider attempts and errors |
| `message_events` | Normalized delivery, bounce, complaint, open, click, and subscription events |
| `links` | Canonical links and tracking metadata |
| `broadcasts` | One-time audience sends |
| `broadcast_recipients` | Frozen send audience and status |

### 13.5 Workflows and playbooks

| Entity | Purpose |
|---|---|
| `playbooks` | Installed or bundled strategy identity |
| `playbook_versions` | Immutable manifest and evidence metadata |
| `playbook_installations` | Workspace-specific configuration and status |
| `workflows` | Stable workflow identity |
| `workflow_versions` | Immutable compiled definition |
| `workflow_enrollments` | One subject progressing through one workflow |
| `workflow_step_runs` | Step-level execution trace |
| `workflow_waits` | Event/time waits and deadlines |
| `workflow_goals` | Goal definitions and completion records |
| `experiments` | Split or holdout configuration |
| `experiment_assignments` | Stable assignment per subject |

### 13.6 Provider and integration state

| Entity | Purpose |
|---|---|
| `ses_connections` | Region, credential mode, role ARN, external ID, status |
| `sending_identities` | Domain/email identity, DKIM, MAIL FROM, verification state |
| `ses_configuration_sets` | Managed configuration set metadata |
| `webhook_endpoints` | Outgoing endpoint definitions |
| `webhook_deliveries` | Attempts, signatures, response codes |
| `integration_connections` | Stripe, n8n, imports, and future systems |
| `secrets` | Encrypted secret material references |

---

## 14. Event ingestion and customer state

### 14.1 Ingestion endpoints

```text
POST /v1/events
POST /v1/events/batch
POST /v1/contacts
PATCH /v1/contacts/{id}
POST /v1/contacts/{id}/identify
POST /v1/contacts/imports
```

### 14.2 Required behaviour

1. Authenticate and scope the request to a workspace.
2. Validate event name, timestamp, identity, and schema.
3. Apply idempotency.
4. Upsert or resolve the contact identity.
5. Store the raw normalized event.
6. Update event aggregates.
7. Evaluate workflow triggers and waiting enrollments.
8. Enqueue resulting jobs.
9. Return a stable event ID and validation warnings.

### 14.3 Event schema registry

Each playbook may declare required and optional events. The registry shows:

- description;
- required properties;
- example payload;
- SDK snippets;
- first and last received times;
- event volume;
- invalid-event count;
- playbooks and workflows depending on the event.

A workflow cannot publish when a required event has never been observed unless the user explicitly acknowledges the risk.

### 14.4 Identity resolution

Default rules:

- `external_id` is authoritative within a workspace and source namespace.
- Verified email may join identities only under explicit workspace rules.
- Conflicting identities create a review item rather than silently merging.
- Contact merges are audited and reversible through an alias record.
- Suppression and consent records survive merges.

---

## 15. Workflow definition and runtime

### 15.1 Canonical format

Workflows are stored as a typed document. YAML is the human-facing export; JSON is accepted through the API. The database stores the original document, compiled representation, hash, schema version, and validation output.

```yaml
schema_version: 1
id: saas_activation
name: SaaS activation
subject: contact
reentry: never

trigger:
  event: account.created

exit_when:
  any:
    - event_occurred: subscription.started
    - contact_suppressed: true

steps:
  - id: welcome
    send_email:
      template: activation_welcome
      topic: onboarding

  - id: wait_for_setup
    wait_for:
      event: domain.connected
      timeout: 2h

  - id: setup_branch
    branch:
      when:
        event_occurred: domain.connected
      then:
        - goto: wait_for_value
      else:
        - send_email:
            template: connect_domain_help
            topic: onboarding

  - id: wait_for_value
    wait_for:
      event: report.generated
      timeout: 1d

  - id: value_branch
    branch:
      when:
        event_occurred: report.generated
      then:
        - complete_goal: activated
      else:
        - send_email:
            template: generate_first_report
            topic: onboarding
        - branch:
            when:
              property:
                name: company_size
                op: greater_than
                value: 20
            then:
              - request_human_action:
                  queue: high_value_onboarding
```

### 15.2 Supported triggers

- Contact created.
- Contact property changed.
- Event received.
- Segment entered.
- Segment exited.
- Date/time reached.
- Recurring schedule.
- Email event received.
- Webhook received.
- Manual/API enrollment.
- Playbook-specific business trigger.

### 15.3 Supported steps

- Send email.
- Wait for duration.
- Wait until date/time.
- Wait for event with timeout.
- Branch on typed condition.
- Multi-way switch.
- Add/remove tag.
- Set or clear property.
- Add/remove topic preference only when policy permits.
- Call webhook.
- Notify human queue.
- Notify agent.
- Create approval request.
- Start another workflow.
- Stop another workflow.
- Complete goal.
- Enter experiment split or holdout.
- Stop enrollment.
- Add note/annotation.

### 15.4 Condition language

Conditions compile to a constrained abstract syntax tree. No arbitrary JavaScript, SQL, shell, or Rust is accepted.

Supported primitives:

- equality and inequality;
- numeric and date comparison;
- string contains/prefix/suffix;
- existence and absence;
- AND, OR, NOT;
- event occurred/never occurred;
- event count within a window;
- first/last event time;
- segment membership;
- message sent/delivered/clicked;
- topic preference;
- suppression state;
- local time and quiet hours;
- experiment assignment.

### 15.5 Versioning

- Draft workflow versions are mutable.
- Published versions are immutable.
- Publishing creates a semantic diff and risk summary.
- Each enrollment is pinned to one workflow version.
- Editing a workflow does not silently mutate active enrollments.
- The publisher chooses whether a new version applies only to new enrollments or offers a reviewed migration for existing enrollments.
- Rollback creates a new publication pointing to a prior definition; history is never rewritten.

### 15.6 Simulation

Simulation is a release gate for playbooks and a recommended gate for all changes.

The simulator accepts:

- a saved contact;
- a synthetic contact;
- a historical event stream;
- a generated scenario from a playbook;
- a simulated clock.

Output includes:

- trigger match;
- each evaluated condition and value;
- waits and deadlines;
- rendered message previews;
- suppressions and frequency-cap decisions;
- goal completion;
- final state;
- warnings;
- a machine-readable trace.

### 15.7 Frequency and relevance controls

Workspace controls:

- maximum marketing messages per contact per day/week;
- quiet hours and timezone fallback;
- minimum spacing between workflow messages;
- campaign priority;
- transactional bypass policy;
- topic-specific caps;
- global kill switch.

When two messages conflict, the runtime applies explicit priority and records the decision.

---

## 16. Playbook package specification

### 16.1 Directory structure

```text
playbooks/saas/activation/
├── manifest.yaml
├── workflow.yaml
├── messages/
│   ├── welcome.mjml
│   ├── connect-domain-help.mjml
│   └── first-report.mjml
├── text/
│   ├── welcome.txt
│   ├── connect-domain-help.txt
│   └── first-report.txt
├── instrumentation.md
├── strategy.md
├── economics.yaml
├── dashboard.yaml
├── scenarios/
│   ├── activates-immediately.yaml
│   ├── stalls-before-setup.yaml
│   └── high-value-stalled.yaml
└── evidence.yaml
```

### 16.2 Manifest

```yaml
id: saas.activation
version: 1.0.0
title: SaaS activation
category: activation
evidence_status: evidence_informed
business_models: [b2b_saas, self_serve_saas]

outcome:
  goal_event: report.generated
  primary_metric: activation_rate_7d

required_events:
  - account.created
  - domain.connected
  - report.generated

optional_properties:
  - company_size
  - plan
  - owner_email

message_topics:
  - onboarding

approval_policy:
  publish: required
  bulk_send: required

sources:
  - id: public-lifecycle-email-source
    type: public_article
    attribution: Patrick McKenzie
```

### 16.3 Installation interview

Every playbook asks only questions needed to adapt it. Examples:

- What is the first valuable customer result?
- Which event proves that result?
- Where do customers commonly stall?
- What account characteristic justifies human intervention?
- What is the offer or next commercial step?
- Which topic and consent basis should these messages use?
- What sender identity and reply-to address should be used?
- What conversion window is meaningful?
- What frequency cap applies?

### 16.4 Installation output

- Event/property mapping.
- Missing instrumentation checklist.
- SDK snippets.
- Adapted workflow draft.
- Original email drafts using workspace language.
- Simulation scenarios.
- Goal dashboard.
- Economics calculator.
- Evidence panel.
- Publish checklist.

### 16.5 Playbook linter

The linter warns when:

- the workflow never clearly asks for the intended action;
- no stopping condition exists;
- an email may be sent after goal completion;
- a required event is absent;
- a marketing topic lacks unsubscribe support;
- a template references missing variables;
- high-value human follow-up has no owner;
- the sequence contains too many messages for the workspace cap;
- conversion is measured only with opens or clicks;
- an agent-created draft includes unsupported factual claims;
- imported copy appears duplicated from a protected source.


---

## 17. Complete v1 playbook library

The following is the complete v1 catalogue established by this PRD. It combines publicly documented lifecycle-email and software-business practices with standard operational safeguards. Each playbook ships with original message drafts, required instrumentation, simulations, an economics model, and an evidence panel.

### Acquisition and education

#### PB-01 — Lead magnet nurture

- **Objective:** Turn a requested guide, checklist, template, benchmark, or resource into an educational relationship and an appropriate commercial next step.
- **Trigger:** `lead_magnet.requested` or form submission.
- **Required signals:** Asset, source, topic, consent status, intended problem, purchase/booking event.
- **Core flow:** Deliver immediately; explain one useful idea; offer a practical win; ask one qualifying question; branch on engagement or product state; make a clear offer; stop on purchase, booking, unsubscribe, or inactivity policy.
- **Primary metric:** Qualified next-step conversion within the configured window.
- **Guardrails:** Never hold the promised asset hostage; do not infer sensitive attributes; every educational message must stand on its own.

#### PB-02 — Free tool or diagnostic to paid product

- **Objective:** Convert users of a calculator, audit, scanner, grader, assessment, or report into customers of the implementation product or service.
- **Trigger:** `diagnostic.completed`.
- **Required signals:** Result summary, severity, category, recommended action, account value indicator, paid conversion event.
- **Core flow:** Deliver result; explain the most consequential finding; teach one improvement; contextualize business cost; provide additional insight; present the paid product as the implementation path; alert a human for high-value severe findings.
- **Primary metric:** Diagnostic-to-paid or diagnostic-to-qualified-call conversion.
- **Guardrails:** Results must be truthful and reproducible; do not fabricate urgency or exaggerate severity.

#### PB-03 — Teach, then sell

- **Objective:** Educate a prospect enough to understand the problem, evaluate the category, and make a confident purchase.
- **Trigger:** Topic-specific opt-in, content request, or qualified product interest.
- **Required signals:** Topic, persona, product fit, offer, objection categories, purchase event.
- **Core flow:** Define problem; correct misconception; give tactical win; demonstrate before/after; quantify cost carefully; introduce solution; address primary objection; explicitly ask for the sale.
- **Primary metric:** Offer conversion and reply quality.
- **Guardrails:** No endless nurture; no unsupported ROI claim; stop when the person buys or becomes ineligible.

#### PB-04 — Webinar, workshop, or training

- **Objective:** Improve attendance, follow-up based on observed participation, and convert appropriate attendees to the next step.
- **Trigger:** `event.registered`.
- **Required signals:** Event time/timezone, attendance, join/leave time, questions, replay view, offer, purchase or booking.
- **Core flow:** Confirmation; calendar action; preparation; reminder sequence; branch into attended, no-show, left early, asked question, viewed replay; deliver relevant follow-up; make offer; stop on conversion.
- **Primary metric:** Attendance and post-event conversion.
- **Guardrails:** Timezone-safe reminders; no reminders after cancellation; cap urgency messages.

#### PB-05 — Productized-service funnel

- **Objective:** Sell a defined service with clear fit, scope, process, proof, price, and intake.
- **Trigger:** Service page enquiry, diagnostic result, or direct lead.
- **Required signals:** Problem, fit criteria, budget band, timeline, service package, proof asset, checkout/booking.
- **Core flow:** Explain outcome; state what is included and excluded; identify ideal/non-ideal customer; provide relevant evidence; present price or range; collect structured intake; accept payment or booking; begin onboarding.
- **Primary metric:** Qualified enquiry-to-paid engagement conversion.
- **Guardrails:** No bait-and-switch scope; preserve a human review option for unusual work.

### SaaS activation and conversion

#### PB-06 — SaaS activation

- **Objective:** Move a new account to the first meaningful customer outcome.
- **Trigger:** `account.created` or `user.signed_up`.
- **Required signals:** Setup events, activation event, plan, company size, account owner, subscription state.
- **Core flow:** Welcome; wait for first setup; send contextual help only if missing; wait for value event; branch on progress; trigger concierge support for valuable stalled accounts; stop on activation or conversion.
- **Primary metric:** Activation rate within the configured window.
- **Guardrails:** Do not continue generic onboarding after activation; do not define activation as an email click.

#### PB-07 — Stalled onboarding recovery

- **Objective:** Help users who began setup but stopped at a known friction point.
- **Trigger:** Setup started plus required next event absent after a threshold.
- **Required signals:** Current onboarding state, last successful step, error category, account value.
- **Core flow:** Name the exact unfinished step; give a short fix; link to the relevant screen; offer data import or human help; suppress when the step completes; escalate repeated technical failures.
- **Primary metric:** Stalled-to-activated conversion.
- **Guardrails:** Never tell a person to repeat a completed step; include current state in every decision.

#### PB-08 — Team invitation and collaborative activation

- **Objective:** Complete multi-party onboarding where the inviter and invitee must both act.
- **Trigger:** `team.invitation_sent`.
- **Required signals:** Inviter, invitee, invitation accepted, invitee activation, team size, account plan.
- **Core flow:** Confirm invitation; remind invitee appropriately; notify inviter when help is needed; stop reminders on acceptance; prompt the inviter toward the next team outcome; alert customer success for valuable blocked teams.
- **Primary metric:** Invitation-to-collaborative-value conversion.
- **Guardrails:** Respect invitee consent and invitation expiry; avoid exposing private account details.

#### PB-09 — Trial conversion state machine

- **Objective:** Convert trials according to actual product state rather than a uniform timer.
- **Trigger:** `trial.started`.
- **Required signals:** Activation, usage intensity, team adoption, pricing views, trial end, subscription start, account value.
- **Core flow:** Branch among no setup, setup without activation, activated, high usage, buying intent, expiring, expired-after-value, and expired-without-value; send context-specific help or offer; stop on purchase.
- **Primary metric:** Trial-to-paid conversion, segmented by initial state.
- **Guardrails:** Never send “you have not tried X” when event history proves otherwise; no false deadlines.

#### PB-10 — High-value trial concierge

- **Objective:** Route economically valuable or complex accounts to timely human assistance.
- **Trigger:** Trial/account meets value heuristic and has not achieved a required milestone.
- **Required signals:** Company size, declared use case, expected volume, domain/company data, product progress, owner availability.
- **Core flow:** Score account; create a human task; offer setup assistance; sync lifecycle context to the owner; suspend redundant automated asks after a human accepts ownership.
- **Primary metric:** Qualified high-value account conversion and time to first human contact.
- **Guardrails:** Keep scoring explainable; prohibit protected or sensitive discriminatory attributes.

#### PB-11 — Pricing intent and explicit ask for the sale

- **Objective:** Ensure commercially interested prospects receive a clear, relevant next action.
- **Trigger:** Repeated pricing visits, proposal request, activated trial, or completion of education sequence.
- **Required signals:** Offer, plan fit, account state, objection, buying route, existing sales ownership.
- **Core flow:** Summarize achieved or expected value; name the recommended offer; state price or next step; provide direct CTA; address principal objection; follow up; close the loop respectfully.
- **Primary metric:** Pricing-intent-to-purchase or booked-call conversion.
- **Guardrails:** The playbook linter must flag fake scarcity, hidden pricing claims, and repeated asks after decline.

### Revenue, retention, and expansion

#### PB-12 — Annual-plan conversion

- **Objective:** Offer annual prepayment to established customers for mutual operational and financial benefit.
- **Trigger:** Customer reaches a tenure, usage, or success threshold while paying monthly.
- **Required signals:** Monthly price, tenure, value milestone, payment status, annual offer, prior decline.
- **Core flow:** Select loyal/successful customers; explain the offer plainly; calculate exact saving; allow one-click or low-friction conversion; do not disrupt the monthly plan if declined; suppress repeat offers for a defined period.
- **Primary metric:** Eligible monthly customers converted and cash collected.
- **Guardrails:** Show accurate arithmetic; distinguish cash collection from recognized revenue; avoid targeting unhappy customers.

#### PB-13 — Failed-payment recovery

- **Objective:** Recover involuntary churn while minimizing customer disruption.
- **Trigger:** `invoice.payment_failed`.
- **Required signals:** Retry schedule, invoice amount, payment method status, account value, eventual success/cancellation.
- **Core flow:** Immediate factual notice; update-payment link; retry reminders tied to actual processor state; grace-period warning; human outreach for strategic accounts; stop instantly on payment success.
- **Primary metric:** Recovered revenue and accounts retained.
- **Guardrails:** Never claim a retry happened when it did not; protect billing links; separate service consequences from threats.

#### PB-14 — Voluntary cancellation save and reason routing

- **Objective:** Learn why a customer intends to cancel and offer an appropriate alternative only where useful.
- **Trigger:** Cancellation flow started or completed.
- **Required signals:** Reason, usage, tenure, plan, missing feature, price concern, current success, cancellation effective date.
- **Core flow:** Collect reason; branch to pause, downgrade, help, missing-feature follow-up, or clean cancellation; confirm outcome; schedule reason-specific win-back only when appropriate.
- **Primary metric:** Saved accounts without increased complaint/support burden, plus reason completeness.
- **Guardrails:** Cancellation must remain easy; no dark patterns; do not force a sales call to cancel.

#### PB-15 — Renewal and expiry

- **Objective:** Prevent accidental expiry and make upcoming renewal terms clear.
- **Trigger:** Contract, subscription, certificate, licence, or service renewal date approaching.
- **Required signals:** Renewal date, price, auto-renew status, owner, usage/value, payment status.
- **Core flow:** Early notice; value summary; action required if any; procurement reminder; final notice; success confirmation; route strategic renewals to an owner.
- **Primary metric:** On-time renewal and reduced surprise-related support.
- **Guardrails:** Disclose dates and amounts accurately; never obscure auto-renewal status.

#### PB-16 — Dormant-user and former-customer reactivation

- **Objective:** Bring back a previously relevant user for a credible new reason.
- **Trigger:** Meaningful inactivity threshold or a change that resolves a prior objection.
- **Required signals:** Prior activation, last use, prior plan, cancellation reason, new capability, seasonality, consent.
- **Core flow:** Segment never-activated, formerly active, former paid, seasonal, missing-feature, price-related, and service-failure cases; send reason-specific message; offer help; stop on return or decline.
- **Primary metric:** Meaningful return and reactivation-to-value conversion.
- **Guardrails:** A “we miss you” message alone is insufficient; state the actual reason to return.

#### PB-17 — Price increase communication

- **Objective:** Communicate a price change plainly while protecting valuable relationships and operational readiness.
- **Trigger:** Approved pricing change with effective date.
- **Required signals:** Current price, new price, effective date, grandfathering, account value, annual option, owner.
- **Core flow:** Segment affected customers; explain change and date; state exact new amount; provide annual/prepay option where genuine; route strategic objections to humans; confirm effective change.
- **Primary metric:** Retention, annual conversion, and support load after change.
- **Guardrails:** No euphemistic or hidden increase; legal and contractual review remains outside the automation engine.

### Consulting and service-business systems

#### PB-18 — Consultant lead qualification

- **Objective:** Identify commercial urgency, fit, authority, expected value, and ability to proceed before consuming scarce delivery time.
- **Trigger:** New enquiry.
- **Required signals:** Problem, why now, desired outcome, consequence of inaction, decision process, budget category, timeline, prior attempts.
- **Core flow:** Acknowledge; collect structured answers; score transparently; send relevant case study; route qualified leads to discovery; politely close or nurture unsuitable leads.
- **Primary metric:** Qualified enquiry-to-paid-discovery or proposal conversion.
- **Guardrails:** Scoring must be explainable; do not claim a lead is “unqualified” based on protected traits.

#### PB-19 — Paid discovery

- **Objective:** Convert complex pre-sales diagnosis into a defined paid engagement rather than unlimited free consulting.
- **Trigger:** Qualified lead requires analysis before implementation scope is known.
- **Required signals:** Qualification status, discovery package, price, deliverables, payment, intake, scheduling.
- **Core flow:** Explain why discovery is separate; present scope and deliverables; collect payment; send intake; gather artefacts; schedule session; deliver outcome; offer implementation next step.
- **Primary metric:** Qualified lead-to-paid-discovery and discovery-to-larger-engagement conversion.
- **Guardrails:** Deliver genuine standalone value; do not make paid discovery a disguised non-refundable sales call.

#### PB-20 — Proposal and abandoned sales conversation

- **Objective:** Progress or close opportunities that have gone quiet without manipulative “breakup email” tactics.
- **Trigger:** Proposal sent, meeting completed, or agreed next step missed.
- **Required signals:** Proposal view/acceptance, objection, intended decision date, owner, next step.
- **Core flow:** Simple reminder; useful clarification; relevant proof; objection-specific response; decision-date reminder; permission to close file; scheduled reactivation for genuine future timing.
- **Primary metric:** Proposal decision rate and sales-cycle duration.
- **Guardrails:** Stop after explicit decline; no invented competing demand or false expiry.

#### PB-21 — Project-to-retainer conversion

- **Objective:** Turn a successful one-off project into recurring continuity, optimisation, maintenance, or reserved availability.
- **Trigger:** Project reaches a successful milestone or scheduled completion review.
- **Required signals:** Delivered outcome, baseline, ongoing risks/opportunities, client owner, potential retainer models, project end date.
- **Core flow:** Summarize achieved value; identify what will continue changing; explain risk of no ongoing ownership; present clear recurring options; follow up before project closure; schedule a later check-in if declined.
- **Primary metric:** Retainer attach rate and recurring gross profit.
- **Guardrails:** Position around ongoing value and risk, not arbitrary unused hours; do not imply ongoing work is mandatory when it is not.

#### PB-22 — Automated value reporting

- **Objective:** Translate operational or technical results into business outcomes that a client or internal champion can share.
- **Trigger:** Monthly/quarterly reporting date or material milestone.
- **Required signals:** Baseline, current metric, business-value assumptions, confidence/limitations, recipients.
- **Core flow:** Pull data; compare with baseline; calculate value using explicit assumptions; generate concise report; deliver to champion; invite correction; surface expansion or risk; notify owner when results are unusually strong or weak.
- **Primary metric:** Report engagement, renewal/expansion influence, and assumption accuracy.
- **Guardrails:** Label estimates; show formula and data period; never present inferred revenue as audited fact.

### Advocacy and operational follow-up

#### PB-23 — Testimonial and case-study capture

- **Objective:** Turn a demonstrated positive outcome into reusable proof with explicit permission.
- **Trigger:** Success milestone, positive reply, renewal, expansion, or measured ROI.
- **Required signals:** Outcome, quote source, identity permission, company permission, evidence, approval state.
- **Core flow:** Ask for a short statement; offer to draft from the customer’s own comments; collect naming/logo permission; invite case-study interview; generate draft; request approval; publish only approved material.
- **Primary metric:** Approved testimonials/case studies and sales influence.
- **Guardrails:** Never fabricate or materially rewrite meaning; permission is granular and revocable operationally.

#### PB-24 — Referral request

- **Objective:** Ask satisfied customers for an introduction at a moment of demonstrated value.
- **Trigger:** Positive outcome, renewal, unsolicited praise, successful support resolution, or high satisfaction signal.
- **Required signals:** Success event, relationship owner, prior referral request, reward rules if any.
- **Core flow:** Acknowledge result; ask whether someone else has the same problem; provide optional forwardable language; make introduction easy; stop after decline or successful referral.
- **Primary metric:** Qualified referrals and referral-to-customer conversion.
- **Guardrails:** Do not ask during unresolved issues; do not condition essential support on advocacy.

#### PB-25 — Appointment confirmation, no-show prevention, and recall

- **Objective:** Reduce missed appointments, improve preparation, recover cancellations/no-shows, and prompt appropriate repeat booking.
- **Trigger:** `appointment.booked`.
- **Required signals:** Appointment time/timezone, location/link, preparation instructions, confirmation, reschedule/cancel, attendance, recommended recall interval.
- **Core flow:** Immediate confirmation; preparation details; timed reminders; reschedule/cancel links; stop on cancellation; no-show follow-up; repeat booking or recall at the appropriate interval; optional review request after successful service.
- **Primary metric:** Attendance rate, recovered appointments, and repeat booking.
- **Guardrails:** Protect sensitive appointment data; use minimal necessary detail in subject lines and previews; timezone correctness is mandatory.

### Cross-playbook “ask for the sale” audit

In addition to PB-11, every acquisition or education playbook is checked for:

- a specific offer;
- a clear outcome;
- an explicit price or next step where appropriate;
- a direct call to action;
- a reason the recipient is eligible now;
- stopping conditions;
- objection handling;
- a finite follow-up policy.

The UI warning is direct:

> This sequence contains educational messages but never clearly asks the eligible recipient to buy, book, or reply.

---

## 18. Flagship playbook installation examples

### 18.1 SaaS activation interview

```text
1. What event represents account creation?
2. What must the user configure before receiving value?
3. What event proves first value?
4. How long does successful activation normally take?
5. Which accounts justify human help?
6. What product state should stop onboarding messages?
7. What sender and reply-to identity should be used?
8. Which onboarding topic and frequency cap apply?
```

Generated result:

```yaml
activation_event: report.generated
high_value_when:
  property: company_size
  op: greater_than
  value: 20
stages:
  - account.created
  - domain.connected
  - report.generated
  - subscription.started
```

Economics panel:

```text
Eligible trials × expected activation lift × activated-to-paid rate × customer value
```

The user must provide or approve each assumption. The application never describes the result as guaranteed.

### 18.2 Free diagnostic to paid service

Generated state model:

```text
Diagnostic completed
  ├── severe + high value ──► educational result + human alert
  ├── severe + normal value ─► result + implementation sequence
  ├── mild ──────────────────► self-service advice + later check-in
  └── already customer ──────► product success guidance, no acquisition pitch
```

### 18.3 Project-to-retainer

Generated options may include:

- reserved availability;
- maintenance and incident response;
- conversion optimisation;
- reporting and analytics;
- security/backup verification;
- ongoing lifecycle operations;
- fractional leadership.

The installation wizard asks for the project’s demonstrated outcome and ongoing risk before drafting any retainer message.


---

## 19. AWS SES integration

AWS SES is the only required delivery provider in v1. The internal provider interface remains abstract enough for later adapters, but no other provider may dilute the first release.

### 19.1 Connection modes

#### Hosted BYO SES

Recommended connection:

- Mail for Agents creates a unique external ID.
- The user launches a CloudFormation template in their AWS account.
- The stack creates a least-privilege IAM role trusted by the hosted Mail for Agents AWS account.
- The hosted application calls AWS STS `AssumeRole` when needed.
- No long-lived customer access key is stored.

#### Self-hosted

Supported credential sources:

- default AWS SDK credential chain;
- instance/task role;
- environment variables;
- explicitly configured profile;
- restricted static credentials as a last-resort local option.

The UI must never encourage administrator credentials.

### 19.2 CloudFormation onboarding stack

The generated stack may create or configure:

- IAM role and policy;
- external-ID trust policy;
- SES configuration set;
- reputation metrics;
- event destination;
- SNS topic;
- HTTPS subscription to the Mail for Agents event endpoint;
- optional custom tracking domain resources where feasible;
- outputs required by the setup wizard.

The stack must be reviewable and contain no unrelated permissions.

### 19.3 Minimum permission categories

Exact AWS actions must be generated from the implementation and reviewed. Categories include:

- inspect SES account status and quotas;
- list/get/create email identities where the user requests managed setup;
- inspect DKIM and verification state;
- inspect/create/update configuration sets and event destinations;
- send email through SES v2;
- inspect suppression/reputation data when enabled;
- use STS only for the configured role path.

The hosted application must scope assumptions to one workspace and one role ARN.

### 19.4 SES setup wizard

```text
Step 1  Select AWS region
Step 2  Connect IAM role or local credentials
Step 3  Inspect SES account and production-access state
Step 4  Select or verify sending domain
Step 5  Publish DKIM records
Step 6  Configure custom MAIL FROM domain
Step 7  Check SPF and DMARC posture
Step 8  Create configuration set and event destination
Step 9  Confirm SNS subscription and event ingestion
Step 10 Send a test message
Step 11 Classify marketing and transactional streams
```

### 19.5 Domain health screen

| Check | States |
|---|---|
| SES identity | Missing / pending / verified / failed |
| DKIM | Missing / pending / verified / rotated / failed |
| Custom MAIL FROM | Missing / pending / verified / failed |
| SPF posture | Valid / warning / unknown |
| DMARC posture | Valid / warning / unknown |
| SES production access | Sandbox / production / paused |
| Sending enabled | Yes / no |
| Daily quota | Current usage and remaining |
| Maximum send rate | Current provider limit |
| Event destination | Healthy / no recent event / failed |
| Bounce processing | Healthy / delayed / unknown |
| Complaint processing | Healthy / delayed / unknown |
| One-click unsubscribe | Configured / warning |

### 19.6 Configuration sets and tags

Default configuration sets:

```text
mailfa-marketing
mailfa-transactional
```

Where provider/account constraints make two sets inappropriate, use one configuration set with explicit tags. Each message includes safe, low-cardinality tags such as:

- workspace public ID;
- message class;
- workflow public ID;
- campaign public ID;
- playbook ID;
- template public ID;
- environment.

Do not place recipient email, names, free text, or secrets in SES tags.

AWS documents configuration sets as the mechanism for event publishing and message tags, including delivery, bounce, complaint, delay, open, click, and subscription events.[12]

### 19.7 Event ingestion

Normalized provider events:

```rust
pub enum DeliveryEventKind {
    Send,
    Delivery,
    DeliveryDelay,
    HardBounce,
    SoftBounce,
    Complaint,
    Reject,
    RenderingFailure,
    Open,
    Click,
    SubscriptionChange,
}
```

SNS ingestion requirements:

- verify AWS SNS message signatures;
- handle subscription confirmation safely;
- reject unexpected topic ARNs;
- store raw provider payload for a configurable diagnostic period;
- normalize into `message_events`;
- deduplicate repeated notifications;
- update message status monotonically where appropriate;
- create suppressions immediately for hard bounce or complaint according to policy;
- wake waiting workflows;
- emit an internal event;
- respond quickly before expensive secondary work.

### 19.8 Suppression hierarchy

Mail for Agents is the application source of truth even when SES suppression is enabled.

Order of checks before send:

1. Global safety block.
2. Workspace suppression.
3. Contact hard-bounce or complaint suppression.
4. Message-class restriction.
5. Topic preference and consent rule.
6. Campaign/workflow exclusion.
7. Frequency cap.
8. Provider suppression insight.

Suppression reasons:

```text
hard_bounce
complaint
manual_global
manual_workspace
marketing_unsubscribe
topic_unsubscribe
administrative_hold
invalid_address
legal_or_policy_hold
provider_suppressed
```

SES supports account-level, configuration-set-level, and newer tenant-level suppression options. Mail for Agents should expose the connected account’s settings while retaining its own explicit suppression model.[13]

### 19.9 Marketing and transactional separation

Message classes:

- `transactional_required`
- `transactional_optional`
- `onboarding`
- `education`
- `marketing`
- `sales_assisted`
- `billing`

The workspace maps each class to:

- sending identity;
- configuration set;
- topic;
- unsubscribe behaviour;
- frequency cap;
- quiet-hours policy;
- default approval threshold.

Recommended domain pattern:

```text
mail.example.com           marketing and lifecycle
notify.example.com         transactional and operational
```

Larger customers may connect separate SES accounts.

### 19.10 Unsubscribe and preferences

For applicable subscribed or marketing messages:

- render a visible preference/unsubscribe footer;
- add `List-Unsubscribe`;
- add `List-Unsubscribe-Post: List-Unsubscribe=One-Click`;
- sign and validate the token;
- accept an HTTPS POST without a redirect;
- update the application preference synchronously;
- record audit evidence;
- prevent later sends immediately.

RFC 8058 defines the one-click POST mechanism and requires relevant headers to be covered by a valid DKIM signature for receivers to offer the feature.[14] SES also offers native subscription management, but v1 should keep the application consent ledger authoritative and treat SES-native list management as an optional adapter.[15]

### 19.11 Sending pipeline

```text
Workflow or broadcast decides to send
        ↓
Consent, suppression, cap, and policy preflight
        ↓
Freeze template and workflow versions
        ↓
Render variables, MJML/HTML, text, links, headers
        ↓
Validate output and recipient
        ↓
Create durable message/outbox record
        ↓
Rate-limit by workspace + SES account + region
        ↓
Call SES v2 SendEmail
        ↓
Store SES message ID or ambiguous state
        ↓
Consume provider events
        ↓
Update analytics, suppressions, and waiting workflows
```

### 19.12 Rate limiting and quotas

- Retrieve and cache SES account quota and send-rate information.
- Use a token bucket per SES connection.
- Enforce a separate workspace safety limit.
- Back off on throttling.
- Do not treat a provider quota error as a permanent recipient failure.
- Pause campaigns before exhausting disk or queue capacity.
- Show estimated completion time before a broadcast begins.

### 19.13 Deliverability health and automatic actions

Configurable thresholds may:

- warn;
- require approval;
- pause marketing sends;
- pause one campaign;
- disable one sending identity;
- preserve transactional-required sends;
- notify owners and agents.

No automatic resume after a complaint-rate or security pause. Resume requires human approval and a recorded note.

---

## 20. Email templates, rendering, and content safety

### 20.1 Template fields

- Internal name.
- Stable public ID.
- Message topic and class.
- From name/address.
- Reply-to.
- Subject.
- Preheader.
- MJML or block representation.
- Generated HTML.
- Plain-text body.
- Required variables and defaults.
- Footer policy.
- Tracking policy.
- Locale.
- Version notes.

### 20.2 Editor modes

1. **Guided blocks:** Heading, paragraph, button, image, divider, callout, signature, footer.
2. **MJML/code:** For technical operators.
3. **Plain text:** First-class, not generated as an afterthought.
4. **Agent draft:** Natural-language request produces a proposed diff, never an immediate publication.

### 20.3 Preview

Preview options:

- synthetic contact;
- selected real contact with masked sensitive fields;
- desktop width;
- mobile width;
- plain text;
- dark-mode approximation;
- raw headers;
- unresolved variables;
- link inventory;
- estimated payload size.

HTML preview runs in a sandboxed iframe. Scripts, forms, and active content are rejected or removed.

### 20.4 Personalization

Use a constrained templating language with:

- escaped variables by default;
- explicit safe HTML helper restricted to trusted blocks;
- defaults;
- simple conditionals;
- loops only over bounded lists;
- date/number formatting;
- no filesystem, network, environment, or arbitrary function access.

A send fails preflight rather than emitting broken placeholders when a required variable is missing.

### 20.5 Links and tracking

- Rewrite links only when tracking is enabled.
- Keep unsubscribe and security-sensitive links outside general tracking.
- Record canonical destination and rendered destination.
- Prevent `javascript:`, dangerous data URLs, and unsupported schemes.
- Let workspaces disable open tracking globally.
- Describe opens as directional rather than definitive.
- Prefer business goal events as the primary conversion measure.

### 20.6 Message preflight

Preflight blocks or warns on:

- missing sender identity;
- unverified domain;
- missing topic;
- missing unsubscribe for applicable classes;
- suppressed recipient;
- missing required variables;
- malformed link;
- absent plain text;
- empty subject;
- excessive payload;
- conflicting workflow goal state;
- frequency-cap violation;
- unresolved agent factual claim marker;
- unsafe HTML;
- test recipient accidentally included in production audience.

---

## 21. Broadcast campaigns

Mail for Agents must support one-time campaigns because an ActiveCampaign/Mailchimp alternative is incomplete without them. Broadcasts remain secondary to lifecycle workflows.

### 21.1 Creation flow

1. Select audience or segment version.
2. Select topic and message class.
3. Select or create template version.
4. Preview representative contacts.
5. Freeze estimated audience.
6. Show exclusions and reasons.
7. Send tests.
8. Run preflight.
9. Request approval if policy requires.
10. Send now or schedule.

### 21.2 Audience freezing

At approval/scheduling time, create a recipient snapshot with:

- contact ID;
- segment version;
- inclusion reason;
- exclusion reason where applicable;
- topic preference state;
- template variant;
- experiment assignment.

Immediately before each send, recheck suppression and consent. A person who unsubscribes after audience freezing must not receive the message.

### 21.3 Campaign controls

- Pause.
- Cancel unsent recipients.
- Resume only after approval when paused by health policy.
- Dry run with counts only.
- Test mode routing every message to a safe address.
- Maximum recipients per approval policy.
- Scheduled local-time delivery as a later enhancement.

### 21.4 Results

- Selected.
- Excluded by reason.
- Queued.
- Accepted by SES.
- Delivered.
- Delayed.
- Bounced.
- Complained.
- Unsubscribed.
- Clicked/opened where enabled.
- Goal converted.
- Revenue or value attributed under explicit model.

---

## 22. Agent-native product surface

### 22.1 Safety model

Agents receive explicit scopes and action levels.

| Level | Capability |
|---|---|
| Observe | Read schemas, contacts, workflows, traces, and metrics |
| Draft | Create message/workflow drafts and analyses |
| Propose | Submit a versioned diff and approval request |
| Operate limited | Pause a workflow, cancel queued work, or send a test under policy |
| Publish | Publish a workflow after policy permits |
| Bulk send | Schedule or activate a broadcast; disabled by default for agents |
| Administer | Manage credentials, users, or compliance settings; never granted by default |

### 22.2 Agent design rule

No LLM sits in the required execution path for deterministic waits, conditions, or sends. An optional AI workflow step must be explicit, bounded, separately priced/credentialed, and able to fail safely.

### 22.3 MCP tools

```text
playbooks.list
playbooks.get
playbooks.recommend
playbooks.install_draft

schemas.list_events
schemas.get_event
schemas.generate_instrumentation
schemas.validate_payload

workflows.list
workflows.get
workflows.create_draft
workflows.propose_diff
workflows.simulate
workflows.request_publish
workflows.pause
workflows.explain_enrollment

messages.create_draft
messages.preview
messages.send_test
messages.get

broadcasts.create_draft
broadcasts.estimate_audience
broadcasts.request_approval
broadcasts.pause

contacts.search
contacts.get
contacts.explain_state
contacts.get_timeline

metrics.query
metrics.playbook_outcomes
metrics.find_dropoffs
metrics.detect_anomaly

ses.get_status
ses.get_identity_status
ses.get_quotas
ses.run_health_check

approvals.list
approvals.get
approvals.request
approvals.approve
approvals.reject

audit.search
```

Tool names and schemas must remain stable after v1. Breaking changes require a versioned MCP namespace.

### 22.4 Agent proposal object

```json
{
  "proposal_id": "prop_123",
  "actor": "agent:lifecycle-optimizer",
  "target": "workflow:saas_activation",
  "base_version": 7,
  "summary": "Stop the second reminder when report.generated occurs",
  "rationale": "18% of recipients currently achieve the goal during the wait",
  "diff": {},
  "simulation": {},
  "estimated_affected_contacts": 382,
  "risk_flags": [],
  "required_approval": "reviewer",
  "expires_at": "2026-07-19T00:00:00Z"
}
```

### 22.5 Agent console

The web UI shows:

- connected agents/service accounts;
- scopes;
- last activity;
- pending proposals;
- accepted/rejected changes;
- send and publish attempts;
- anomalies detected;
- token/cost usage where an LLM provider is connected;
- emergency revoke.

### 22.6 Model providers

The product remains model-provider-neutral. A workspace may connect an OpenAI-compatible endpoint, xAI, Anthropic, or another supported provider. BYO keys are encrypted. Provider support must not alter workflow portability.

Agent-generated content must identify factual placeholders and claims requiring operator verification. The application must never silently invent customer results, testimonials, prices, legal statements, deadlines, or product capabilities.

---

## 23. Public API and SDK requirements

### 23.1 API principles

- Stable `/v1` namespace.
- JSON only for public API.
- Consistent cursor pagination.
- Structured error codes.
- Idempotency keys on event ingestion, sends, imports, and mutating agent operations.
- Workspace-scoped API keys with least privilege.
- Request IDs returned and logged.
- OpenAPI documentation generated from implementation types.
- Webhooks signed with timestamped HMAC.

### 23.2 Core endpoints

```text
/v1/contacts
/v1/events
/v1/segments
/v1/templates
/v1/messages
/v1/broadcasts
/v1/playbooks
/v1/workflows
/v1/simulations
/v1/approvals
/v1/metrics
/v1/ses
/v1/webhooks
/v1/audit
```

### 23.3 Initial SDKs

- JavaScript/TypeScript.
- Rust.
- curl examples for every endpoint.

Python follows after the API stabilizes.

### 23.4 CLI examples

```bash
mailfa login --server https://app.mailforagents.com
mailfa events schema list
mailfa events send report.generated --contact usr_123 --property report_type=market_scan
mailfa playbooks recommend --business b2b-saas
mailfa playbooks install saas.activation --draft
mailfa workflows simulate saas_activation --scenario stalled-high-value
mailfa workflows diff saas_activation
mailfa approvals request workflow:saas_activation
mailfa ses doctor
```

### 23.5 n8n and automation ecosystem

A first-party n8n node should expose:

- send event;
- upsert contact;
- add/remove tag;
- send transactional message;
- enroll in workflow;
- create approval request;
- query contact state;
- receive Mail for Agents webhook trigger.

This directly applies the Postiz lesson that adjacent automation communities can become a high-retention distribution channel.


---

## 24. Information architecture

### 24.1 Primary navigation

```text
Overview
Playbooks
Workflows
Broadcasts
Contacts
Events
Messages
Deliverability
Agents
Settings
```

Workspace switcher appears at the top of the navigation. Documentation, system status, version, and account controls appear at the bottom.

### 24.2 Secondary navigation

#### Playbooks

- Recommended
- Installed
- Catalogue
- Private
- Evidence

#### Workflows

- Active
- Drafts
- Paused
- Archived
- Simulations
- Approvals

#### Contacts

- All contacts
- Segments
- Topics and preferences
- Suppressions
- Imports

#### Events

- Live stream
- Schemas
- Invalid events
- Sources

#### Messages

- All messages
- Templates
- Tests
- Transactional
- Delivery events

#### Deliverability

- Health
- Sending identities
- AWS SES
- Quotas
- Bounces and complaints
- Unsubscribe health
- DNS

#### Agents

- Overview
- Service accounts
- MCP
- API keys
- Proposals
- Approvals
- Activity

### 24.3 Command palette

`⌘K` / `Ctrl+K` opens a command palette with:

- navigation;
- contact search;
- workflow search;
- install playbook;
- send test;
- ingest sample event;
- open SES health;
- create draft;
- show pending approvals;
- ask agent.

Commands display required permission and whether approval will be needed.

---

## 25. Visual design direction

### 25.1 Design intent

The interface should feel like a precise infrastructure console rather than a cheerful newsletter builder. Public xAI Console material shows a restrained developer dashboard with compact navigation, cards, usage views, filters, charts, API-key management, and team/billing surfaces.[16][17] Mail for Agents may borrow those broad design principles—clarity, monochrome surfaces, compact information density, and a warm chart/action accent—but must use an original component system, spacing, iconography, information architecture, and brand.

### 25.2 Brand descriptor

Always pair the name with a descriptor until category recognition exists:

> **Mail for Agents**  
> Agent-native lifecycle email automation

This prevents the domain from being mistaken for an inbox product that creates email accounts for autonomous agents.

### 25.3 Visual character

- Dark-first, not dark-only.
- Near-black background and subtly elevated surfaces.
- Thin borders rather than heavy shadows.
- White and grey typography with one warm signal accent.
- Data-dense tables and timelines.
- Monospace for identifiers, events, code, and numeric telemetry.
- Minimal decorative illustration.
- Clear status language; colour never carries meaning alone.
- Charts use direct labels and restrained grid lines.
- Original wordmark and icon; no resemblance to xAI’s mark.

### 25.4 Design tokens

```css
:root {
  --bg-canvas: #090909;
  --bg-sidebar: #0c0c0c;
  --surface-1: #111111;
  --surface-2: #171717;
  --surface-3: #1d1d1d;
  --border-subtle: #242424;
  --border-strong: #343434;

  --text-primary: #f5f5f5;
  --text-secondary: #b0b0b0;
  --text-tertiary: #777777;
  --text-inverse: #0b0b0b;

  --accent: #ff8a5b;
  --accent-hover: #ff9b73;
  --accent-muted: rgba(255, 138, 91, 0.14);

  --success: #65c18c;
  --warning: #e4b85f;
  --danger: #ef7070;
  --info: #79a8ff;

  --radius-sm: 6px;
  --radius-md: 10px;
  --radius-lg: 14px;

  --space-1: 4px;
  --space-2: 8px;
  --space-3: 12px;
  --space-4: 16px;
  --space-5: 20px;
  --space-6: 24px;
  --space-8: 32px;
  --space-10: 40px;

  --font-sans: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont,
               "Segoe UI", sans-serif;
  --font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas,
               "Liberation Mono", monospace;
}
```

A light theme may be included after dark-theme accessibility is complete. The application does not distribute proprietary font files.

### 25.5 Typography

| Element | Size | Weight | Notes |
|---|---:|---:|---|
| Page title | 24–28px | 600 | Tight line height |
| Section title | 17–20px | 600 | No ornamental uppercase |
| Body | 14px | 400 | Default console density |
| Table/controls | 13px | 400–500 | IDs may use monospace |
| Label | 12px | 500 | Muted, sentence case |
| Metric | 24–32px | 550 | Tabular numbers |
| Code/event | 12–13px | 400 | Monospace |

### 25.6 Layout

- Desktop sidebar: 224px.
- Collapsed sidebar: 64px.
- Context header: 56px.
- Main content max width: 1480px, with full-width exceptions for workflow and tables.
- Default content padding: 24px desktop, 16px tablet/mobile.
- Card grid: 12-column, 16px gaps.
- Table row: 44px standard, 36px compact.
- Right inspector/drawer: 400–480px.

### 25.7 Components

#### Buttons

- Primary: light foreground or warm accent only for the single page action.
- Secondary: surface with strong border.
- Ghost: navigation and low-risk actions.
- Danger: red text/border; destructive confirmation required.
- Agent proposal: accent-muted background plus sparkle/agent glyph, never visually identical to an approved action.

#### Status chips

Status includes icon + label:

```text
● Active
◐ Draft
Ⅱ Paused
! Needs setup
✓ Verified
× Failed
⌁ Agent proposal
```

#### Cards

- 1px subtle border.
- No more than one primary metric per card.
- Context and comparison appear beneath the value.
- Entire cards are not clickable unless affordance is clear.

#### Tables

- Sticky header.
- Column chooser.
- Saved views.
- Keyboard row navigation.
- Bulk actions appear only after selection.
- IDs and event names use monospace.
- No pagination count hidden behind an icon.

#### Drawers and dialogs

- Right drawer for inspect/edit without losing list context.
- Modal only for short blocking decisions.
- Full-page stepper for SES setup, playbook installation, and migrations.

#### Empty states

Every empty state offers one relevant next action and, where useful, a sample-data option.

### 25.8 Motion

- 120–180ms transitions.
- Respect `prefers-reduced-motion`.
- No ambient animation.
- Live-event rows may briefly highlight, then settle.
- Progress indicators show actual states, not indefinite animation where progress can be measured.

### 25.9 Accessibility

- WCAG 2.2 AA target.
- Full keyboard operation.
- Visible focus ring.
- Proper landmarks and heading order.
- Form errors linked with `aria-describedby`.
- Dialog focus trapping and restoration.
- Charts accompanied by a table or textual summary.
- Colour-independent status.
- Minimum 44px touch targets on mobile.

---

## 26. Key screen requirements and wireframes

### 26.1 Overview

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Mail for Agents / Acme SaaS              Production       ⌘K     Ask agent │
├───────────────┬──────────────────────────────────────────────────────────────┤
│ Overview      │ Good morning. Your lifecycle is healthy.                   │
│ Playbooks     │                                                              │
│ Workflows     │ ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐ │
│ Broadcasts    │ │ Activated  │ │ Recovered  │ │ Delivered  │ │ Complaints │ │
│ Contacts      │ │  428       │ │ £4,820     │ │ 98.7%      │ │ 0.04%      │ │
│ Events        │ └────────────┘ └────────────┘ └────────────┘ └────────────┘ │
│ Messages      │                                                              │
│ Deliverability│ Recommended next action                                     │
│ Agents        │ ┌──────────────────────────────────────────────────────────┐ │
│ Settings      │ │ Instrument `team.invitation_accepted` to install the    │ │
│               │ │ collaborative activation playbook. [View setup]         │ │
│               │ └──────────────────────────────────────────────────────────┘ │
│               │                                                              │
│               │ Active playbooks                 Recent delivery health      │
│               │ SaaS activation       12.8%      line/bar chart + table      │
│               │ Failed payment        £4,820                                  │
│ v0.1.0        │ Annual conversion     38 accounts                             │
└───────────────┴──────────────────────────────────────────────────────────────┘
```

Requirements:

- Business outcomes precede open/click counts.
- A single recommended next action is generated from missing setup, health, or lifecycle gaps.
- Health summary links to exact evidence.
- “Ask agent” opens a drawer without navigating away.

### 26.2 Playbook catalogue

```text
┌ Playbooks ───────────────────────────────────────────────────────────────────┐
│ Recommended   Installed   Catalogue   Private   Evidence                    │
│                                                                            │
│ Business type [B2B SaaS ▾]   Goal [Activation ▾]   Evidence [Any ▾]        │
│                                                                            │
│ ┌ SaaS activation ───────────────────────────────────────────────────────┐  │
│ │ Evidence-informed   3 required events   5 messages   ~15 min setup     │  │
│ │ Move accounts to their first valuable result.                          │  │
│ │ Missing: report.generated                           [Review playbook]    │  │
│ └────────────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────────┘
```

Each card shows:

- outcome;
- business fit;
- evidence status;
- events/properties required;
- estimated setup work;
- installed state;
- primary metric;
- source/evidence access.

### 26.3 Playbook detail and installer

Three-column desktop layout:

```text
Strategy / Evidence      Workflow preview          Installation readiness
Why it exists            vertical state diagram    ✓ account.created
Who it fits               messages and branches     ✓ domain.connected
Economics                 goal/stop conditions       ! report.generated
Sources                                              company_size optional
                                                     [Generate SDK snippet]
                                                     [Install as draft]
```

The installer never publishes automatically.

### 26.4 Workflow editor

The v1 editor is a structured step stack, not an unrestricted node canvas.

```text
┌ SaaS activation / Draft v8 ─────────────── Simulate   Compare   Publish ────┐
├──────────────┬───────────────────────────────────────────┬───────────────────┤
│ Step palette │ Trigger: account.created                  │ Inspector         │
│              │       │                                   │                   │
│ Message      │ [1] Send “Welcome”                        │ Step: wait_value  │
│ Wait         │       │                                   │ Wait for event    │
│ Branch       │ [2] Wait for domain.connected — 2 hours  │ report.generated  │
│ Webhook      │       ├── received ─────────────────┐     │ Timeout: 1 day    │
│ Human task   │       └── timeout → Send help       │     │                   │
│ Goal         │                                      │     │ On event          │
│ Experiment   │ [3] Wait for report.generated — 1 day    │ complete goal     │
│ Stop         │       ├── received → Goal: activated      │                   │
│              │       └── timeout → Branch: company > 20  │ [Save step]       │
├──────────────┴───────────────────────────────────────────┴───────────────────┤
│ 2 warnings: report.generated has not been received; message variable ...   │
└──────────────────────────────────────────────────────────────────────────────┘
```

Requirements:

- Click step to load inspector via HTMX.
- Add/reorder through buttons and keyboard; drag-and-drop is progressive enhancement.
- Branches are visually nested but remain readable as a list.
- A read-only graph view may be generated as SVG.
- Publish button opens diff, simulations, affected counts, and approval.
- Autosave is debounced but explicit saved state is always visible.

### 26.5 Simulation console

```text
Scenario: High-value account stalls before activation
Clock: 12 Jul 09:00 ─────────────────────────────────────── 14 Jul 09:00

09:00 trigger matched: account.created
09:00 message rendered: activation_welcome
09:00 suppressed? no
09:01 queued → SES accepted: 0102...
11:00 wait timeout: domain.connected absent
11:00 message rendered: connect_domain_help
...

Final state: waiting for report.generated
Goal: not completed
Warnings: company_size=50 will create human task at next timeout
```

The trace can be downloaded as JSON and copied into an issue.

### 26.6 Contact detail

```text
Header: person@example.com  Active customer  Topic preferences  Suppress

Summary | Timeline | Workflows | Messages | Properties | Consent

Timeline
12 Jul  report.generated
12 Jul  Goal completed: SaaS activation
12 Jul  Workflow stopped before reminder #2
11 Jul  Email delivered: Generate your first report
11 Jul  domain.connected
```

A prominent **Why did this happen?** action explains any message or exclusion.

### 26.7 Event debugger

- Live SSE stream.
- Filter by event name, contact, source, validity.
- Expand raw/normalized payload.
- Schema warnings.
- Replay into a development environment only.
- Copy SDK snippet.
- Show dependent workflows.

### 26.8 Deliverability console

```text
AWS SES / eu-west-1                     Healthy

Identity: mail.example.com              Verified
DKIM                                    Verified
Custom MAIL FROM                        Verified
DMARC                                   Warning: monitoring policy
Production access                       Enabled
Daily quota                             42,810 / 100,000 remaining
Send rate                               18 / 50 per second
Event ingestion                         Last event 12 seconds ago

Marketing health                        Transactional health
Bounce 0.42%                             Bounce 0.08%
Complaint 0.04%                          Complaint 0.00%

[Run health check] [View DNS] [Pause marketing]
```

### 26.9 Agent proposal drawer

```text
Ask Mail for Agents
────────────────────────────────────────
“Reduce unnecessary activation email.”

Proposal
Stop reminder 2 when report.generated occurs during the wait.

Evidence
69 of 382 recipients completed the goal before reminder 2 last month.

Simulation
✓ 6 scenarios pass
✓ No increase in maximum send count

Affected
New enrollments only

[Reject] [Edit draft] [Request approval]
```

The agent’s explanation and evidence are visually separate from an approved change.

### 26.10 Mobile

Mobile supports monitoring, approvals, pausing, contact lookup, and simple edits. Complex workflow authoring may display a message recommending a larger viewport, but all critical safety actions remain available.

---

## 27. HTMX interaction architecture

### 27.1 Rendering model

- Direct navigation returns the complete page shell.
- HTMX navigation returns the main fragment.
- Responses vary on `HX-Request` where caching is present.
- `hx-push-url` maintains usable browser history.
- Out-of-band swaps update toast regions, counters, health badges, and pending approval counts.
- Critical forms use server validation and return field-level fragments with HTTP 422.
- No business state exists only in the browser.

### 27.2 Route examples

```text
GET  /workflows
GET  /workflows/{id}
GET  /workflows/{id}/steps/{step_id}/edit
POST /workflows/{id}/steps
PUT  /workflows/{id}/steps/{step_id}
POST /workflows/{id}/simulate
POST /workflows/{id}/publish-request

GET  /fragments/workflow-step/{step_id}
GET  /fragments/contact-timeline/{contact_id}
GET  /fragments/ses-health
GET  /streams/events
GET  /streams/jobs/{job_id}
```

### 27.3 Live updates

Use server-sent events for:

- live event debugger;
- import progress;
- simulation progress;
- broadcast progress;
- SES health check progress;
- agent proposal generation;
- approval notifications.

SSE is one-way and simpler than WebSockets for these use cases. Mutation remains normal HTTP.

### 27.4 JavaScript policy

- Vendor HTMX locally with an integrity hash in the release process.
- Use small ES modules for command palette, code editor enhancement, graph pan/zoom, clipboard, and progressive drag ordering.
- Initialize modules from a global `htmx:load` listener; do not depend on inline scripts in swapped fragments.
- Enforce a strict Content Security Policy without `unsafe-inline` where feasible.
- All core workflows remain usable without the enhancement modules.

### 27.5 Request safety

- CSRF token on browser mutations.
- SameSite cookies.
- Idempotency token on publish, send, import, and approval actions.
- Double-submit prevention through disabled controls plus server idempotency.
- `hx-sync` or equivalent cancellation for rapid search/filter requests.
- No optimistic UI for sends, permissions, consent, suppression, or publication.

---

## 28. Onboarding experience

### 28.1 First-run self-hosted setup

1. Create owner account.
2. Name workspace and set canonical URL.
3. Configure encryption master key status.
4. Connect AWS SES.
5. Verify domain/event pipeline.
6. Send test message.
7. Choose business type.
8. Select first recommended playbook.
9. Map events/properties.
10. Install draft and simulate.

### 28.2 “Useful before perfect” path

A user without product events may still:

- import contacts;
- connect SES;
- send a test;
- create a broadcast;
- install a date/form-triggered service-business playbook;
- generate instrumentation snippets for later.

The product must not force a SaaS event model on every business.

### 28.3 Demo workspace

A one-click demo creates synthetic:

- contacts;
- events;
- SaaS activation workflow;
- failed payment workflow;
- consultant qualification workflow;
- messages and delivery events;
- dashboards.

Demo sends are disabled. The workspace can be deleted cleanly.


---

## 29. Migration from incumbent platforms

Migration is a primary acquisition feature, not an administrative afterthought.

### 29.1 Import sequence

1. Connect source account or upload exports.
2. Inventory source objects.
3. Map contacts and identifiers.
4. Map custom fields.
5. Map tags/lists to tags, segments, or topics.
6. Import consent and suppression evidence.
7. Import templates and assets.
8. Translate workflows into an intermediate representation.
9. Produce a compatibility report.
10. Create disabled drafts.
11. Run simulations.
12. Approve cutover.
13. Freeze source sends where required.
14. Verify Mail for Agents sends/events.
15. Record migration report.

### 29.2 Initial migration support

#### MVP

- CSV contacts.
- CSV suppressions/unsubscribes.
- Generic HTML and plain-text template import.
- Mail for Agents workflow YAML/JSON import/export.

#### Post-MVP priority

1. ActiveCampaign.
2. Mailchimp.
3. HubSpot workflows and marketing contacts.
4. Customer.io.
5. Kit/ConvertKit.

### 29.3 Normalized migration representation

```text
Source trigger
  ├── event/property/list/date
Conditions
  ├── boolean expression
Actions
  ├── send/wait/update/webhook/goal
Source-specific unsupported nodes
Consent and audience semantics
```

The compatibility report labels each item:

- exact translation;
- translated with semantic difference;
- requires manual mapping;
- unsupported;
- unsafe to infer.

Imported workflows are never activated automatically.

### 29.4 Consent and suppression integrity

- Preserve original status, timestamp, source, and evidence where supplied.
- Unknown consent remains unknown; it is not converted to subscribed.
- A source unsubscribe imports as a block for the relevant topic or marketing class.
- Complaints and hard bounces import as suppressions.
- The cutover report shows counts before and after.

### 29.5 Migration landing pages and tools

Public distribution pages:

- Open-source ActiveCampaign alternative.
- ActiveCampaign automation visualizer.
- Migrate ActiveCampaign automations to AWS SES.
- Self-hosted HubSpot workflows alternative.
- HubSpot workflow compatibility checker.
- Mailchimp customer journey importer.
- Mailchimp contact-cost calculator.
- Customer.io migration guide.

Free tools should lead naturally into a workspace without locking exports.

---

## 30. Analytics, goals, and experiments

### 30.1 Analytics hierarchy

1. Business goal completion.
2. Progression between lifecycle states.
3. Human task outcome.
4. Delivery and failure.
5. Click.
6. Open.

The UI should not make open rate the largest number by default.

### 30.2 Playbook dashboard

Each installed playbook includes:

- eligible population;
- enrolled population;
- goal completions;
- time to goal;
- branch distribution;
- human escalations;
- sends per goal;
- suppression/exclusion counts;
- delivery health;
- experiment or holdout result;
- economic estimate with visible assumptions.

### 30.3 Attribution

Supported v1 attribution:

- direct goal after enrollment within a configured window;
- last workflow touch;
- experiment/holdout comparison;
- explicit external conversion event.

The application must label correlation and attribution clearly. It must not claim an email caused a purchase merely because it preceded one.

### 30.4 Experiments

Supported:

- subject/content variant;
- timing variant;
- workflow branch variant;
- holdout/no-message group;
- human-outreach variant.

Requirements:

- Stable assignment.
- Declared primary metric before start.
- Minimum runtime and sample warnings.
- No automatic winner declaration from open rate alone.
- Preserve assignment in exports.
- Stop/rollback safety.

### 30.5 Economics model

A playbook may define formulas using approved inputs. Example:

```yaml
economics:
  output: estimated_incremental_value
  formula:
    multiply:
      - eligible_contacts
      - measured_incremental_goal_rate
      - value_per_goal
  assumptions:
    - value_per_goal
```

The UI displays:

- input source;
- time window;
- formula;
- uncertainty/limitations;
- whether the value is measured or assumed.

### 30.6 Data retention

Configurable retention tiers:

- raw event payload;
- normalized event;
- aggregate;
- provider payload;
- message body;
- audit record.

Deletion or compaction must preserve legally/operationally required suppression and audit records according to workspace policy.

---

## 31. Security, privacy, and governance

### 31.1 Secret handling

- Encrypt stored provider and model credentials with an application master key.
- Prefer AWS role assumption so hosted BYO-SES stores no long-lived AWS access key.
- Store API keys as a prefix plus one-way hash.
- Never write secrets, authorization headers, full session tokens, or AWS credentials to logs.
- Redact secrets from support bundles.
- Rotate workspace secrets without restarting where practical.

### 31.2 Authentication

Community v1:

- Owner bootstrap flow.
- Email/password with Argon2id.
- Recovery codes or administrator reset path.
- Optional magic link after a sending identity exists.
- Optional OIDC after v1.
- Session revocation and device list.

Hosted v1 should include verified email and rate-limited recovery. Enterprise SSO is later.

### 31.3 Authorization

Every service method receives an actor and workspace context. HTML routes, API, CLI, and MCP all invoke the same authorization layer.

High-risk permissions are separate:

- manage sending identity;
- export contacts;
- reveal personal data;
- publish workflow;
- send broadcast;
- change consent/suppression;
- approve agent proposal;
- manage credentials;
- delete workspace.

### 31.4 Audit log

Audit records include:

- actor type and public ID;
- workspace;
- action;
- target;
- before/after hashes or safe diff;
- request ID;
- source IP where appropriate;
- timestamp;
- approval reference;
- result.

Message content and secrets should not be duplicated into audit logs.

### 31.5 Web security

- TLS required outside localhost.
- Secure, HttpOnly, SameSite cookies.
- CSRF protection.
- Strict CSP.
- Frame restrictions except sandboxed internal previews.
- Output escaping by default.
- Rate limiting for authentication, ingestion, unsubscribe, and public tracking endpoints.
- SSRF controls for outgoing webhooks and imported URLs.
- File upload type, size, and content validation.
- Safe redirect allowlists.

### 31.6 Public unsubscribe security

- Signed opaque token with workspace, contact identity, topic, message, issued time, and expiry policy.
- One-click POST does not require login.
- Endpoint returns success even for an already-unsubscribed contact.
- Do not reveal whether an arbitrary address exists.
- Rate limit abuse without blocking valid inbox-provider POSTs.
- No redirect for the RFC 8058 POST path.

### 31.7 Agent security

- Treat event properties, contact fields, imported email text, and webhook content as untrusted data.
- Never let untrusted content redefine agent system instructions or permissions.
- Separate retrieved data from control instructions.
- Require explicit target and base version for mutations.
- Show structured diff, not only prose.
- Enforce policy server-side after model output.
- Prevent an agent from approving its own proposal unless a narrowly defined policy explicitly permits it.
- Revoke service account instantly.

### 31.8 Privacy controls

- Export contact data and history.
- Delete or anonymize contacts subject to retention/suppression policy.
- Disable open tracking globally or by message class.
- Mask personal data in agent context by default.
- Configurable data retention.
- Separate test and production environments/workspaces.
- No opt-out telemetry in self-hosted edition; any usage telemetry is explicit opt-in.

### 31.9 Compliance posture

The product supplies mechanisms for consent evidence, topic preferences, unsubscribe, suppression, retention, audit, and sender identity. It must avoid claiming that enabling a checkbox makes a workspace compliant in every jurisdiction. Legal requirements and lawful basis remain the customer’s responsibility.

---

## 32. Reliability and observability

### 32.1 Health endpoints

```text
GET /health/live
GET /health/ready
GET /health/details      authenticated
```

Readiness checks:

- database connectivity;
- migrations current;
- writable data directory;
- scheduler heartbeat;
- encryption key availability;
- disk-space threshold;
- worker heartbeat.

SES and external integration health should not make the core HTTP service unready unless configured as a hard dependency.

### 32.2 Structured logs

Fields:

- timestamp;
- level;
- service/component;
- request ID;
- workspace public ID where safe;
- actor public ID;
- job ID;
- workflow/enrollment/message public ID;
- provider error code;
- latency;
- outcome.

No recipient email by default. A time-limited diagnostic mode may reveal more to an authorized administrator with an audit record.

### 32.3 Metrics

- HTTP request count/latency/error.
- SQLite busy duration and transaction latency.
- job queue depth and oldest age.
- job success/failure/ambiguous count.
- event ingestion rate and invalid count.
- workflow enrollment and step latency.
- send attempts and SES response classes.
- provider event lag.
- suppression processing latency.
- SSE connections.
- disk usage and backup age.
- agent requests, cost, and approval rate.

Prometheus-format export is optional in community v1 but recommended.

### 32.4 Tracing

A trace should connect:

```text
incoming event → contact resolution → workflow trigger → job → message → SES ID → provider event → goal
```

### 32.5 Support bundle

`mailforagents doctor --bundle` creates a redacted archive containing:

- version and build metadata;
- configuration with secrets removed;
- migration status;
- database integrity summary;
- recent sanitized errors;
- queue summary;
- SES status without credentials;
- OS/runtime information;
- optional specific trace IDs.

The operator reviews the manifest before export.

---

## 33. Non-functional requirements

These are release targets to benchmark, not promises that every machine or SQLite deployment will achieve them.

### 33.1 Capacity target for one v1 instance

- 100,000 active contacts.
- 5 million retained normalized events.
- 100,000 recipients in one broadcast.
- 50 dispatch attempts per second, subject to SES quota and host capacity.
- 100 event ingestions per second sustained in a benchmark workload.
- 25 concurrent workflow simulations.
- 20 concurrent browser users.

Larger usage should prompt sharding, retention adjustment, or a future database backend.

### 33.2 Latency targets

- Common HTML/API reads: p95 under 250 ms on reference hardware.
- Contact search: p95 under 500 ms for target dataset.
- Event acknowledgement: p95 under 300 ms before asynchronous downstream work.
- Suppression application after provider event: p95 under 60 seconds, target under 10 seconds.
- Pause broadcast: prevent new send leases within 5 seconds.
- Agent proposal UI: stream progress; no fake instant response requirement.

### 33.3 Availability and recovery

- Clean restart recovers expired job leases.
- No workflow state exists only in memory.
- Crash tests prove no automatic duplicate after known accepted send.
- Ambiguous send states are visible and counted.
- Restore procedure tested in CI and release candidates.
- Default hosted objective may be 99.9%, documented separately from community software.

### 33.4 Browser support

Latest two stable versions of:

- Chrome/Chromium;
- Firefox;
- Safari;
- Edge.

Core monitoring and approval views must work on current mobile Safari and Chrome.

---

## 34. Testing strategy

### 34.1 Unit tests

- Condition evaluator.
- Workflow compiler.
- Template variable validation.
- Consent and suppression precedence.
- Frequency cap logic.
- SES event normalization.
- Signature and token verification.
- Pricing/economics formulas.
- Permission checks.

### 34.2 Property-based tests

- Workflow execution does not send after terminal stop/goal where prohibited.
- Replaying the same event with the same idempotency key produces no extra enrollment or send.
- Consent/suppression precedence remains monotonic toward safety.
- Serialization round-trips workflow documents.
- Time advancement does not skip due waits.

### 34.3 Golden tests

- Rendered HTML and plain text.
- MIME headers.
- HTMX fragments.
- Playbook installation output.
- Workflow semantic diffs.
- Migration compatibility reports.

### 34.4 Integration tests

- Axum routes with temporary SQLite database.
- SeaORM migrations from each supported prior release.
- SES adapter against mocks plus an AWS sandbox test account.
- SNS signature and event fixtures.
- Complete event-to-message-to-bounce flow.
- Backup/restore.
- Agent proposal and approval policy.

### 34.5 End-to-end tests

Critical paths:

1. First-run owner setup.
2. Connect SES using test credentials/fixture.
3. Verify identity state.
4. Import contact.
5. Install playbook.
6. Ingest event.
7. Simulate.
8. Publish.
9. Send test.
10. Process delivery/bounce/complaint.
11. Unsubscribe.
12. Verify future send exclusion.
13. Agent proposes change.
14. Human approves.
15. Audit trace is complete.

### 34.6 Time testing

The workflow runtime depends on an injectable clock. Tests can advance days instantly and deterministically.

### 34.7 Crash and ambiguity tests

Inject failures:

- before message record;
- after message record, before SES call;
- during SES call;
- after SES acceptance, before provider ID commit;
- after provider ID commit;
- during SNS event processing;
- during workflow step completion.

Release is blocked if any path creates an unbounded automatic duplicate-send risk.

### 34.8 Security tests

- CSRF.
- session fixation.
- privilege escalation.
- cross-workspace access.
- stored XSS through contact/event/template data.
- SSRF in webhooks and imports.
- unsubscribe token tampering.
- SNS signature bypass.
- API key leakage in logs.
- agent permission bypass and prompt injection.

---

## 35. Deployment and operations

### 35.1 Single binary

Release targets:

- Linux x86_64.
- Linux ARM64.
- macOS for local development/preview.
- Container image.

The binary embeds migrations and static assets.

### 35.2 Docker Compose

Default services:

```yaml
services:
  mailforagents:
    image: ghcr.io/mailforagents/mailforagents:latest
    ports:
      - "8080:8080"
    volumes:
      - ./data:/var/lib/mailforagents
    environment:
      MAILFA_BASE_URL: https://mail.example.com
      MAILFA_DATABASE_URL: sqlite:///var/lib/mailforagents/app.db?mode=rwc
      MAILFA_MASTER_KEY: ${MAILFA_MASTER_KEY}
```

No external database is included because SQLite is the required v1 database.

### 35.3 Configuration

- Environment variables for secrets and deployment-critical values.
- Optional TOML file for non-secret operational configuration.
- Admin-managed workspace settings in the database.
- Startup validation with actionable error messages.
- `mailforagents config print --redacted`.

### 35.4 Upgrades

- Semantic versions.
- Release notes with migration impact.
- Automatic pre-migration backup by default.
- Forward-only migrations within a release line; documented restore rollback.
- Startup lock prevents two processes migrating the same database.
- UI shows current and available version only when update checking is enabled.

### 35.5 Demo and development

```bash
cargo run -- demo
```

Creates a temporary database, seeded workspace, fake SES adapter, and browser URL.

---

## 36. Open-source distribution and go-to-market

### 36.1 Repository promise

README opening:

> **Mail for Agents is the open-source, agent-native alternative to ActiveCampaign and HubSpot workflows. Connect AWS SES, install lifecycle playbooks, and operate everything through the web, API, CLI, or MCP.**

Above the fold:

1. Product screenshot.
2. 45-second activation demo.
3. One-command Docker install.
4. Natural language → draft workflow → simulation → approval.
5. SES domain health screen.
6. ActiveCampaign/Mailchimp migration path.
7. Hosted option.

### 36.2 Public repository structure

```text
mailforagents/mailforagents
mailforagents/playbooks
mailforagents/sdk-js
mailforagents/sdk-rust
mailforagents/n8n-node
mailforagents/examples
mailforagents/docs
```

The playbook repository may live inside the monorepo initially to reduce release complexity, then split once community contribution volume justifies it.

### 36.3 Launch demo

Prompt:

> Recover failed subscriptions. Email immediately, follow up after three days, stop once payment succeeds, and notify me when an account worth more than £500 MRR remains unpaid for seven days.

Demo sequence:

1. Agent proposes event mapping and workflow.
2. Visual step stack appears.
3. Original messages render.
4. Simulation shows payment success stopping future email.
5. High-value branch creates a human task.
6. Human approves.
7. SES test send succeeds.

### 36.4 Content and programmatic pages

- One page per playbook.
- One page per required event recipe.
- One page per migration pair.
- One page per SES setup issue.
- One page per business model.
- Public workflow YAML with diagrams.
- Economics calculators.
- SPF/DKIM/DMARC checker.
- Email HTML previewer.
- Workflow visualizer.
- ActiveCampaign pricing/contact calculator.
- Mailchimp journey exporter/import checker.

Pages must be genuinely useful and exportable, not thin SEO shells.

### 36.5 Community contribution model

A playbook pull request includes:

- manifest;
- original copy;
- sources/evidence;
- simulations;
- linter pass;
- security/consent review;
- screenshots or generated preview;
- version notes.

Maintain a review badge for official playbooks. Community playbooks are clearly distinguished.

### 36.6 Hosted conversion

The application may show a non-intrusive hosted option in documentation and settings. It must not degrade, nag, or intentionally complicate self-hosting. The paid value is:

- no maintenance;
- automatic backups/upgrades;
- monitored event delivery;
- managed TLS;
- team collaboration;
- support;
- optional managed AWS setup.

The hosted service runs the same multi-workspace public application binary available to self-hosters. A small private cloud controller owns public registration, Stripe billing, hosted entitlements, workspace creation, suspension/reactivation, abuse controls, and instance assignment. A customer signup creates tenant records inside a running application instance; it does not provision a dedicated product fork, container, or database.

### 36.7 Distribution milestones inspired by Postiz

- Excellent self-hosted launch before paid acquisition.
- Launch to self-hosted and Rust communities.
- Publish official n8n node.
- Publish MCP server and agent skill.
- Create template videos and community examples.
- List in open-source/self-hosted directories.
- Encourage integrators and agencies with reusable playbooks.
- Build an affiliate programme only after activation and retention are healthy.

---

## 37. Delivery roadmap

Durations are planning hypotheses for a focused small team, not commitments.

### Phase 0 — Foundations

**Exit criteria:**

- Rust workspace and CI.
- Axum shell and xAI-console-inspired design system.
- SeaORM migrations and SQLite settings.
- Authentication, multi-workspace membership, tenant-scoped roles, cross-workspace isolation tests, and audit.
- Background job engine.
- Docker/single-binary release skeleton.

### Phase 1 — SES and first message

**Exit criteria:**

- SES connection modes.
- Identity and domain health.
- Configuration set/SNS setup.
- Template rendering and test send.
- Provider event ingestion.
- Suppression and unsubscribe.
- Deliverability console.

### Phase 2 — Contacts, events, and deterministic workflows

**Exit criteria:**

- Contacts/properties/tags/topics.
- Event API/schema debugger.
- Workflow compiler/runtime.
- Waits, conditions, email, webhook, human task, goal.
- Versioning and simulation.
- Contact timeline/explanation.

### Phase 3 — Playbook product

**Exit criteria:**

- Manifest/install system.
- Recommendation interview.
- First 12 flagship playbooks fully authored.
- Evidence model.
- Economics and dashboards.
- Playbook linter.

Recommended first 12:

1. SaaS activation.
2. Stalled onboarding.
3. Trial conversion.
4. High-value concierge.
5. Lead magnet nurture.
6. Free diagnostic to paid.
7. Annual-plan conversion.
8. Failed-payment recovery.
9. Cancellation save.
10. Consultant qualification.
11. Project-to-retainer.
12. Testimonial/referral capture.

### Phase 4 — Agent surface

**Exit criteria:**

- Public API complete for core web actions.
- MCP tools.
- Service accounts and scopes.
- Proposal/diff/approval flow.
- Agent drawer.
- JavaScript and Rust SDKs.
- CLI.

### Phase 5 — Broadcasts, analytics, and migration

**Exit criteria:**

- Broadcast audience freeze/send controls.
- Experiment/holdout basics.
- Goal dashboards.
- CSV import.
- ActiveCampaign importer proof of concept.
- Public migration report.

### Phase 6 — Hosted launch

**Exit criteria:**

- Private cloud controller deployed separately from the public product.
- Public signup, Stripe Checkout, signed and idempotent webhooks, Customer Portal, subscriptions, and hosted entitlements.
- Paid workspace creation through an authenticated internal API into the running multi-workspace public application.
- Account, workspace, subscription, provisioning, suspension, reactivation, and cancellation state machines.
- Instance assignment and tenant export/import procedure for later whole-instance sharding; no per-customer container provisioning.
- Automated backups and restore drills.
- Managed upgrades.
- Abuse controls.
- Support tooling.
- Published service limits and status page.

### Phase 7 — Complete playbook catalogue

Ship the remaining v1 catalogue through PB-25, plus vertical variants for appointments, SaaS, consulting, and agencies.

---

## 38. MVP release definition

The MVP is complete when a new user can:

1. Deploy with one Docker command.
2. Create a workspace and owner.
3. Connect a restricted AWS SES account.
4. verify a domain and event destination.
5. Send a test message.
6. Create/import a contact.
7. Ingest a product event.
8. Install SaaS activation or consultant qualification as a draft.
9. Map required events/properties.
10. Preview original message copy.
11. Simulate at least three paths.
12. Publish after approval.
13. Execute a wait and conditional branch durably.
14. Deliver through SES.
15. Process delivery, hard bounce, complaint, and unsubscribe.
16. Explain the message from the contact timeline.
17. Pause the workflow.
18. Export the workflow and playbook configuration.

MVP does not require a freeform visual canvas, multiple email providers, a CRM pipeline, or every importer.

---

## 39. Acceptance criteria by capability

### 39.1 SES

- Least-privilege hosted role connection works with external ID.
- Self-hosted standard AWS credential chain works.
- Region and account state are visible.
- Test send records provider message ID.
- SNS signatures are verified.
- Bounce/complaint creates suppression before later queued marketing send is leased.
- Marketing pause does not block explicitly configured required transactional messages.

### 39.2 Workflows

- Published versions are immutable.
- Enrollment is pinned to a version.
- Duplicate trigger/event does not duplicate enrollment where re-entry is disabled.
- Goal event cancels prohibited future sends.
- Crash/restart resumes waits.
- Every step has a trace.
- Simulation and runtime use the same evaluator.

### 39.3 Playbooks

- Each official playbook has strategy, instrumentation, workflow, messages, scenarios, dashboard, economics, and evidence.
- Installation creates a draft only.
- Missing required events are obvious.
- Copy is original and source ideas are attributed.
- Evidence status is shown.

### 39.4 Agents

- An observe-only agent cannot mutate.
- A draft agent cannot publish.
- A proposal contains base version, diff, simulation, recipient estimate, and risk flags.
- Permission is checked after model output.
- Agent cannot approve its own proposal under default policy.
- Revocation immediately blocks new requests.

### 39.5 Consent and unsubscribe

- Topic preference is checked immediately before send.
- One-click unsubscribe accepts POST without redirect.
- Already-unsubscribed response is idempotently successful.
- Hard bounce and complaint block all configured classes.
- Every exclusion appears in recipient/contact explanation.

### 39.6 UI

- All critical actions work with keyboard.
- HTMX navigation preserves URLs and back/forward.
- Validation errors survive fragment swaps.
- Live streams reconnect with bounded backoff.
- No inline script is required for swapped fragments.
- Workflow and deliverability pages remain usable at 1280px width.

---

## 40. Major risks and mitigations

| Risk | Mitigation |
|---|---|
| “Mail for Agents” sounds like inboxes for agents | Always pair with “agent-native lifecycle email automation”; lead demos with workflows and SES |
| SQLite becomes a hosted scaling bottleneck | Declare single-writer boundary, benchmark real multi-workspace load, add complete application/database instances, and move tenant boundaries through tested export/import rather than sharing a live file |
| SES setup is intimidating | CloudFormation path, visual DNS checks, `doctor`, explicit remediation |
| Playbooks overpromise | Evidence levels, visible assumptions, original copy, workspace experiments |
| Patio11 association implies endorsement or copies protected material | Public-source attribution only; no naming/endorsement; legal/content review |
| Agents send unsafe email | Deterministic runtime, scoped service accounts, approval gates, recipient estimates, kill switch |
| Duplicate sends after ambiguous provider response | Ambiguous status, no blind retry, operator reconciliation, crash tests |
| Workflow editor becomes a JS application | Structured step stack, server rendering, progressive enhancements only |
| Contact-based incumbents compete on features | Focus on portability, SES ownership, agent operation, playbook strategy, and open source |
| Open/click analytics create misleading optimisation | Prioritize goal events and holdouts; label engagement metrics as directional |
| Self-hosted abuse damages brand | Clear acceptable-use policy for hosted service, no central control over community installs, deliverability education |
| Community playbooks contain poor or unsafe advice | Official/community distinction, manifest linting, evidence review, security/consent checks |
| One binary accumulates background-task complexity | Strict module boundaries, durable job abstraction, supervisor, metrics, bounded concurrency |

---

## 41. Decisions made by this PRD

1. Product name is **Mail for Agents**.
2. Public category descriptor is **agent-native lifecycle email automation**.
3. AWS SES is the default and only v1 delivery provider.
4. Community v1 uses Rust, Axum, HTMX, SQLite, and SeaORM.
5. The UI uses a structured workflow editor rather than a React-style freeform canvas.
6. Workflows are deterministic, versioned documents.
7. Agents propose; humans approve risky operations by default.
8. The full useful core is open source.
9. Hosted pricing avoids charging primarily by stored contacts.
10. The v1 catalogue contains PB-01 through PB-25.
11. Public expert sources may be attributed, but no endorsement is implied.
12. The visual system is original while drawing on the restrained developer-console qualities visible in xAI’s public console.
13. All v1 repositories use SQLite exclusively for persistence; one public application instance serves many tenant-scoped workspaces from a shared database, while the private cloud controller uses its own database and both remain single-writer per database.
14. Multi-workspace support is part of the useful public product. Hosted billing creates workspace access in a running instance rather than provisioning one container or database per customer.

---

## 42. Open questions requiring founder decisions

1. Is AGPL-3.0 acceptable, or is a different licence required for investor/enterprise goals?
2. Will the hosted product support only BYO SES at launch, or also a managed shared sending account?
3. Is the first ideal customer technical B2B SaaS, consultants, or agencies? The PRD recommends technical B2B SaaS first.
4. Should self-hosted multi-user collaboration be fully included from day one? This PRD says yes.
5. Which model provider is first-class for agent drafting, if any? The architecture remains provider-neutral.
6. Is workflow YAML intended as a hand-authored public contract in v1, or primarily an export format until schemas stabilize?
7. Which twelve playbooks receive full launch-quality copy first?
8. Will opt-in aggregate benchmark data be collected for community validation?
9. What operational thresholds pause marketing sends in hosted mode?
10. Is an ActiveCampaign importer required for public launch or the first post-launch milestone?
11. Will private agency playbooks be a cloud-only commercial feature or included self-hosted?

---

# Appendix A — Representative database tables

This is a domain sketch, not final migration SQL.

```text
workspaces(id, name, slug, timezone, created_at, updated_at)
users(id, email, password_hash, created_at, updated_at)
memberships(id, workspace_id, user_id, role, created_at)
service_accounts(id, workspace_id, name, scopes_json, revoked_at)
api_keys(id, service_account_id, prefix, secret_hash, last_used_at, expires_at)

contacts(id, workspace_id, external_id, primary_email, attributes_json, state, created_at, updated_at)
contact_identities(id, workspace_id, contact_id, namespace, external_id, email, verified_at)
field_definitions(id, workspace_id, name, type, indexed, created_at)
contact_field_values(id, workspace_id, contact_id, field_id, text_value, number_value, bool_value, datetime_value)
topics(id, workspace_id, key, name, message_class, created_at)
consent_records(id, workspace_id, contact_id, topic_id, status, source, evidence_json, occurred_at)
suppressions(id, workspace_id, contact_id, email, scope, reason, source, created_at, revoked_at)

segments(id, workspace_id, name, current_version_id, created_at)
segment_versions(id, segment_id, version, rule_json, hash, created_at)
segment_memberships(id, segment_version_id, contact_id, calculated_at)

events(id, workspace_id, contact_id, name, occurred_at, received_at, properties_json, source, idempotency_key)
event_schemas(id, workspace_id, name, schema_json, description, first_seen_at, last_seen_at)
event_aggregates(id, workspace_id, contact_id, event_name, count, first_at, last_at)

email_templates(id, workspace_id, name, current_version_id, created_at)
email_template_versions(id, template_id, version, subject, preheader, mjml, html, text, variables_json, hash, created_at)
messages(id, workspace_id, contact_id, template_version_id, workflow_step_run_id, class, topic_id, status, provider_message_id, dedupe_key, created_at)
message_attempts(id, message_id, attempt, status, provider_error, started_at, finished_at)
message_events(id, message_id, kind, provider_event_id, occurred_at, payload_json)

workflows(id, workspace_id, name, status, current_draft_version_id, current_published_version_id)
workflow_versions(id, workflow_id, version, source_yaml, compiled_json, hash, status, created_by, created_at)
workflow_enrollments(id, workspace_id, workflow_version_id, contact_id, status, enrolled_at, completed_at)
workflow_step_runs(id, enrollment_id, step_id, sequence, status, input_json, output_json, started_at, finished_at)
workflow_waits(id, enrollment_id, step_id, event_name, due_at, status)
workflow_goals(id, enrollment_id, goal_key, completed_at, source_event_id)

playbooks(id, key, title, official, created_at)
playbook_versions(id, playbook_id, version, manifest_yaml, evidence_json, hash, created_at)
playbook_installations(id, workspace_id, playbook_version_id, workflow_id, mapping_json, status, installed_at)

broadcasts(id, workspace_id, name, segment_version_id, template_version_id, status, scheduled_at, approved_at)
broadcast_recipients(id, broadcast_id, contact_id, status, inclusion_json, exclusion_reason)

ses_connections(id, workspace_id, region, credential_mode, role_arn, external_id, status, encrypted_config)
sending_identities(id, ses_connection_id, identity, type, status, dkim_status, mail_from_status, last_checked_at)

jobs(id, workspace_id, kind, payload_json, run_at, priority, status, attempts, max_attempts, locked_by, locked_until, dedupe_key, last_error_code, last_error_message)
approvals(id, workspace_id, actor_id, target_type, target_id, proposal_json, status, requested_at, decided_at)
audit_logs(id, workspace_id, actor_type, actor_id, action, target_type, target_id, safe_diff_json, request_id, occurred_at)
```

# Appendix B — Example public event SDK

```ts
import { MailForAgents } from "@mailforagents/sdk";

const mail = new MailForAgents({
  apiKey: process.env.MAILFA_API_KEY!,
  baseUrl: "https://mail.example.com",
});

await mail.events.track({
  idempotencyKey: `report:${report.id}:generated`,
  name: "report.generated",
  occurredAt: new Date().toISOString(),
  contact: {
    externalId: user.id,
    email: user.email,
  },
  properties: {
    reportType: "market_scan",
  },
});
```

# Appendix C — Example Rust event SDK

```rust
use mailforagents_sdk::{Client, ContactRef, Event};

let client = Client::new(
    std::env::var("MAILFA_API_KEY")?,
    "https://mail.example.com",
)?;

client
    .track(Event::new("report.generated")
        .idempotency_key(format!("report:{}:generated", report.id))
        .contact(ContactRef::new(user.id.to_string()).email(user.email.clone()))
        .property("report_type", "market_scan"))
    .await?;
```

# Appendix D — Postiz strategy checklist

Before public launch, verify:

- [ ] Full useful product is available self-hosted.
- [ ] Hosted product mirrors core capability.
- [ ] One-command deployment works on a clean server.
- [ ] README shows the agent workflow and SES connection.
- [ ] Public API is not an afterthought.
- [ ] MCP and CLI are documented.
- [ ] First-party n8n node exists or is imminent.
- [ ] Public playbooks are useful outside the hosted product.
- [ ] Migration pages target high-intent searches.
- [ ] Community issues and discussions receive timely responses.
- [ ] Revenue and usage claims are labelled accurately.
- [ ] The product has a clear convenience-based cloud upgrade.

# Appendix E — References

1. **Postiz GitHub repository**, public repository statistics, README, licence, and download/view claims. https://github.com/gitroomhq/postiz-app  
2. **Postiz homepage**, agentic positioning, 30+ networks, public API and automation integrations. https://postiz.com/  
3. Nevo David, **“I did it! My open-source company now makes $14.2k monthly as a single developer,”** Indie Hackers, 25 November 2025. https://www.indiehackers.com/post/i-did-it-my-open-source-company-now-makes-14-2k-monthly-as-a-single-developer-f2fec088a4  
4. James Fleischmann / Nevo David, **“Growing an open-source product to $1.3M ARR in two years,”** Indie Hackers, 4 June 2026. Figures are founder-reported. https://www.indiehackers.com/post/tech/growing-an-open-source-product-to-1-3m-arr-in-two-years-hbMiXIoZsueV9D3L58DP  
5. Patrick McKenzie, **“The Email Worth $200 A Recipient (And Other Fun Lifecycle Email Stories),”** Kalzumeus Training. https://training.kalzumeus.com/newsletters/archive/lifecycle_emails_3  
6. Patrick McKenzie, **“Hacking Lifecycle Emails for Software Companies,”** Kalzumeus Training. https://training.kalzumeus.com/lifecycle-emails  
7. Patrick McKenzie, **“Systematizing Sales With Software And Processes,”** Kalzumeus Training. https://training.kalzumeus.com/newsletters/archive/sales_automation  
8. Patrick McKenzie, **“Productizing A Freelancing / Consulting Business,”** Kalzumeus Training. https://training.kalzumeus.com/newsletters/archive/services_vs_products  
9. **Axum crate documentation**, current 0.8.x documentation. https://docs.rs/axum/latest/axum/  
10. **SeaORM 2.0 database connection documentation**, including SQLite. https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/  
11. **HTMX documentation** and stable 2.x materials. https://htmx.org/docs/  
12. **Amazon SES event publishing and configuration sets.** https://docs.aws.amazon.com/ses/latest/dg/monitor-using-event-publishing.html  
13. **Amazon SES account-level suppression list.** https://docs.aws.amazon.com/ses/latest/dg/sending-email-suppression-list.html  
14. IETF, **RFC 8058: Signaling One-Click Functionality for List Email Headers.** https://datatracker.ietf.org/doc/html/rfc8058  
15. **Amazon SES subscription management.** https://docs.aws.amazon.com/ses/latest/dg/sending-email-subscription-management.html  
16. **xAI Console home**, public description of API keys, teams, models, billing, and usage. https://console.x.ai/home  
17. **xAI Console Usage Explorer documentation**, public usage filters, groupings, and chart examples. https://docs.x.ai/console/usage  
18. **AWS SDK for Rust, SES v2 crate documentation.** https://docs.rs/aws-sdk-sesv2  

---

## Final product statement

> **Mail for Agents is an open-source, agent-native lifecycle email automation platform. It connects to AWS SES, installs evidence-informed business playbooks, executes deterministic workflows, and lets humans and agents operate the customer lifecycle through one inspectable system.**
