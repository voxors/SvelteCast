# Svelte + Rust Template

This template provides a minimal setup for a Svelte frontend with a Rust backend powered by Axum.

## Project Structure

```
SvelteCast/
├─ frontend/       # Svelte + Vite
├─ backend/        # Rust backend (Axum)
```

## Getting Started

```bash
# In the frontend directory
cd frontend
npm install
npm run dev

# In the backend directory
cd backend
cargo run
```

The frontend will be served at `http://localhost:5173` and the backend API at `http://localhost:8000`. The frontend proxies API requests to the backend automatically via Vite config.
```
