# rust-nestorwheelock

Fast Rust/Axum public frontend for nestorwheelock.com that reads from Django's PostgreSQL database.

## Architecture

```
Browser → Nginx:80 → Rust:3002 → PostgreSQL
                  → Django:8000 (admin, API, AI)
```

- **Rust/Axum** serves public pages (homepage, posts, search, contact)
- **Django** handles admin, dashboard, API, and AI endpoints
- Both read from the same PostgreSQL database

## Performance

| Endpoint | Response Time |
|----------|---------------|
| Homepage | ~13ms |
| Post Detail | ~5ms |
| Search | ~8ms |
| Contact | ~1ms |

## Tech Stack

- **Framework**: Axum 0.7
- **Database**: SQLx (PostgreSQL)
- **Templates**: Askama (Jinja2-like)
- **Styling**: Bootstrap 5

## Endpoints

| Route | Description |
|-------|-------------|
| `GET /` | Homepage with posts feed |
| `GET /posts/:id/` | Post detail with media |
| `GET /search/?q=` | Search results |
| `GET /contact/` | Contact form |
| `POST /contact/` | Submit contact form |
| `GET /browse/` | Browse posts |
| `GET /:slug/` | Static pages |

## Deployment

1. Copy environment file:
   ```bash
   cp .env.example .env
   # Edit .env with your database password
   ```

2. Build and run:
   ```bash
   docker-compose up -d --build
   ```

3. Access at `http://localhost:8088`

## Configuration

Environment variables in `.env`:

| Variable | Description |
|----------|-------------|
| `DB_PASSWORD` | PostgreSQL password |
| `RUST_LOG` | Log level (info, debug, trace) |

## Development

```bash
# Run locally (requires PostgreSQL)
cargo run

# Build release
cargo build --release

# Run tests
cargo test
```

## License

Copyright (c) 2025 Nestor Wheelock. All Rights Reserved.
