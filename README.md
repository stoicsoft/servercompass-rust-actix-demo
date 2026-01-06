# Server Compass Rust Actix Demo

A minimal Actix Web server that surfaces public environment variables with a `Not set` fallback and keeps private values on the backend.

## Features
- Home page shows public env vars (`APP_NAME`, `API_URL`, `ENVIRONMENT`, `VERSION`) with `Not set` fallback.
- JSON endpoint `/api/env` returns the same public values.
- Private envs (`DATABASE_URL`, `API_SECRET_KEY`) stay server-side and are never exposed to clients.
- Defaults are loaded at startup so the app works out of the box.

## Quick start
```bash
cd servercompass-rust-actix-demo
cp .env.example .env  # optional; dotenv is loaded automatically
# or export vars manually if you prefer
# export APP_NAME="ServerCompass Rust Actix"
# export API_URL="https://api.servercompass.app"
# export ENVIRONMENT="production"
# export VERSION="1.0.0"
# export DATABASE_URL="postgresql://user:password@localhost:5432/servercompass"
# export API_SECRET_KEY="your-secret-key-here"

cargo run
# open http://localhost:8080 (or PORT if set)
```

## Endpoints
- `GET /` – HTML page listing public env vars with fallback styling
- `GET /api/env` – JSON `{ envs: [{ key, value }] }`
- `GET /health` – Simple health check

## Environment variables
Public (shown):
- `APP_NAME`
- `API_URL`
- `ENVIRONMENT`
- `VERSION`

Private (server-only):
- `DATABASE_URL`
- `API_SECRET_KEY`

Unset public values render as `Not set` in both HTML and JSON responses; otherwise defaults from `.env`/built-ins are used.
