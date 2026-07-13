# Public Product Application

This is the complete self-hostable product and the same application image used
by Mail for Agents Cloud. Read `../../prd.md` before changing product behavior.

- Design every data path around an explicit workspace boundary.
- Use SQLite with foreign keys, WAL mode, a busy timeout, and persistent storage.
- Never add private billing credentials, customer records, production topology,
  managed SES operations, or cloud support tooling here.
- Public code must never depend on the private cloud repository.
- Do not present scaffolded or future behavior as enabled in the UI or API.
- Preserve public contracts and existing passing behavior.
- Run formatting, locked checks, and locked tests before publishing a change.
