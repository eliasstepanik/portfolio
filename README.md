# Open Freedom Project

A modern web application built with Rust and Leptos framework.

## Features

- >€ Built with Rust for safety and performance
- ¡ WebAssembly for near-native speed
- = Client-side routing with no page reloads
- =ñ Responsive design
- <¯ Fine-grained reactivity

## Prerequisites

- Rust (stable or nightly)
- `wasm32-unknown-unknown` target
- Trunk (for building and serving)

## Setup

1. Install Rust target for WebAssembly:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. Install Trunk:
   ```bash
   cargo install trunk
   ```

## Development

Run the development server:
```bash
just run
# or
cd frontend && trunk serve --open
```

The application will be available at `http://localhost:8080`

## Building for Production

Build optimized static files:
```bash
just build-web
# or
cd frontend && trunk build --release
```

The static files will be generated in `frontend/dist/` directory.

## Project Structure

- `frontend/` - Web application source code
  - `src/` - Rust source files
    - `app.rs` - Root application component
    - `components/` - Reusable UI components
    - `pages/` - Page components
  - `public/` - Static assets (CSS, images)
  - `index.html` - HTML entry point
  - `Trunk.toml` - Trunk configuration

## Technologies

- [Leptos](https://leptos.dev/) - Rust web framework
- [WebAssembly](https://webassembly.org/) - Compilation target
- [Trunk](https://trunkrs.dev/) - WASM web application bundler