# Rust Auth API

A modular authentication API built with **Rust**, using the following stack:

- [Axum](https://github.com/tokio-rs/axum) – Fast web framework
- [SeaORM](https://www.sea-ql.org/SeaORM/) – Async ORM
- [PostgreSQL](https://www.postgresql.org/) – Database
- [JWT](https://github.com/Keats/jsonwebtoken) – Token-based authentication
- [Argon2](https://docs.rs/argon2/) – Password hashing
- [Utoipa + Swagger UI](https://docs.rs/utoipa/) – Auto-generated API docs

## 🚀 Getting Started

### Prerequisites

- Rust (stable)
- PostgreSQL

### Setup

```bash
git clone https://github.com/yourusername/rust-auth-api.git
cd rust-auth-api

# Create .env file and configure your DB connection
cp .env.example .env

# Run database migrations
cargo run -p migration -- up

# Start the server
cargo run -p rust-auth-api
```

### API Documentation

Once running, visit: [http://localhost:3000/docs](http://localhost:3000/docs)

## 🧱 Folder Structure

- `api/` - Main application code
- `migration/` - SeaORM migration crate
- `src/auth` - Authentication logic (handlers, service, JWT)
- `src/db` - Entity definitions
- `src/routes` - Route definitions

## 🛡️ Security

- Passwords are hashed using Argon2
- JWTs are signed securely and have expiration

---

Made with 🦀 Rust by Abiodun
