# Elias Stepanik - Portfolio

A personal developer portfolio showcasing projects and skills, built with Rust (Leptos + Axum) and PostgreSQL.

## Features

- 🦀 Full-stack Rust with Leptos (WASM) frontend and Axum backend
- 🎨 Dark/Light mode toggle with persistence
- 📱 Fully responsive design
- 🚀 Project showcase with language filtering
- 📊 PostgreSQL database with SQLx
- 🐳 Docker Compose for easy deployment
- ⚡ WebAssembly for near-native frontend performance

## Prerequisites

- Rust (stable)
- `wasm32-unknown-unknown` target
- Trunk (for building and serving frontend)
- Docker & Docker Compose (for PostgreSQL)
- SQLx CLI (for database migrations)

## Setup

1. Install Rust target for WebAssembly:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. Install required tools:
   ```bash
   cargo install trunk
   cargo install sqlx-cli --no-default-features --features postgres
   ```

3. Start PostgreSQL:
   ```bash
   docker-compose up -d postgres
   ```

4. Set up database:
   ```bash
   cd backend
   sqlx database create
   sqlx migrate run
   ```

## Development

Run the full stack:
```bash
just dev
```

Or run components separately:
- Backend: `just backend-dev`
- Frontend: `just run`
- Database: `docker-compose up -d postgres`

The application will be available at:
- Frontend: `http://localhost:8080` (or 8081)
- Backend API: `http://localhost:3000`

## Building for Production

Build frontend:
```bash
just build-web
```

Build backend:
```bash
cd backend && cargo build --release
```

Build with Docker:
```bash
docker-compose build
```

## Project Structure

```
.
├── backend/                  # Axum API server
│   ├── src/
│   │   ├── models/          # Database models
│   │   ├── routes/          # API endpoints
│   │   └── main.rs          # Server entry point
│   └── migrations/          # SQL migrations
├── frontend/                # Leptos WASM app
│   ├── src/
│   │   ├── components/      # Reusable components
│   │   ├── pages/          # Page components
│   │   └── app.rs          # Root component
│   └── public/             # Static assets
└── docker-compose.yml      # Service orchestration
```

## API Endpoints

- `GET /api/projects` - List all projects (with optional language filter)
- `GET /api/projects/:id` - Get single project
- `GET /health` - Health check

## Technologies

- [Leptos](https://leptos.dev/) - Reactive web framework
- [Axum](https://github.com/tokio-rs/axum) - Web application framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [PostgreSQL](https://www.postgresql.org/) - Database
- [WebAssembly](https://webassembly.org/) - Frontend compilation target
- [Trunk](https://trunkrs.dev/) - WASM bundler

## License

© 2025 Elias Stepanik. All rights reserved.