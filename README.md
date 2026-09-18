# web_server

A multithreaded HTTP server written in Rust, used to serve a personal portfolio site behind nginx on EC2.

The server started from the final-project chapter of [_The Rust Programming Language_](https://doc.rust-lang.org/book/) ("The Book") — a single-threaded handler hardened with a thread pool — and was extended into something I'd actually want to deploy: a request router with a middleware chain, binary-safe responses, MIME-aware static file serving with path-traversal protection, env-driven config, and a real container/IaC/CI pipeline.

This is primarily a learning project — building the server, deploying it with IaC, and operating it on real infrastructure end-to-end. No frameworks (axum, actix, hyper, tokio); the standard library only.

## Goals

- **Own the stack.** Replace a Vercel-hosted Next.js site with hand-written HTML/CSS/JS served by a custom Rust server.
- **Operate it like production.** Docker, IaC, reverse proxy, TLS, logging.
- **Keep the server small.** Clean module boundaries, no premature abstractions.

## Architecture

```
                 ┌─────────────┐
   client ──HTTPS┤    nginx    │──HTTP──► web_server (Docker, 127.0.0.1:8080)
                 │  (host EC2) │             │
                 └─────────────┘             ▼
                                       ./static/* (HTML/CSS/JS)
```

- nginx terminates TLS (Let's Encrypt), handles gzip/caching, proxies to the Rust server.
- web_server runs in a Docker container, listens on a loopback-bound port published by Docker.
- All HTML/CSS/JS lives under `./static/` and is baked into the image.

## Project layout

```
src/
├── main.rs              # entry point: builds router, starts listener
├── lib.rs
├── error.rs             # JSON error helper
├── config.rs            # env-var parsing (BIND_ADDR, PORT, THREAD_POOL_SIZE)
├── server/
│   ├── connection.rs    # per-connection handler
│   └── thread_pool.rs
├── http/
│   ├── request.rs       # Request parsing (Method, path, headers, body)
│   ├── response.rs      # Response with Vec<u8> body, Connection: close
│   ├── status.rs
│   ├── method.rs        # Method enum
│   └── mime.rs          # extension → MIME type
├── router/
│   ├── mod.rs           # Router: builder pattern with .wrap()/.get()/.post()
│   └── route.rs         # Route, Pattern (Exact / Prefix wildcard), Handler
├── middleware/
│   ├── mod.rs           # Middleware trait + Next chain (around pattern)
│   └── logger.rs        # method/path/elapsed
└── handlers/
    ├── static_files.rs  # serves files under ./static/ with path-traversal guard
    ├── not_found.rs
    └── api/             # placeholder for future API handlers

static/                   # site content (baked into Docker image)
deploy/
├── terraform/            # EC2, SG, EIP, ECR, Route53, GitHub OIDC roles
└── ansible/              # nginx, Docker, certbot, container deploy
```

## Tech stack

| Layer | Tool |
|------|------|
| Web server | Rust (std-only), custom thread pool |
| Static content | HTML / CSS / vanilla JS |
| Container | Docker (multi-stage build, distroless-style runtime) |
| Reverse proxy / TLS | nginx + Let's Encrypt (certbot) |
| Infrastructure | Terraform (EC2, Security Group, EIP, Route53) |
| Server provisioning | Ansible (Docker install, nginx config, certbot, deploy) |
| Host | AWS EC2 |

## Running locally

```sh
cargo run
# Listening on 127.0.0.1:8080
```

Configurable via environment variables (or `.env`):

| Variable | Default | Purpose |
|---|---|---|
| `BIND_ADDR` | `127.0.0.1` | Bind address. Set to `0.0.0.0` inside Docker. |
| `PORT` | `8080` | Listen port |
| `THREAD_POOL_SIZE` | `num_cpus::get()` | Worker thread count |

Smoke test:

```sh
curl -i http://127.0.0.1:8080/
curl -i http://127.0.0.1:8080/about
curl -i http://127.0.0.1:8080/static/style.css
curl -i http://127.0.0.1:8080/nonexistent   # → 404
```

## Deployment

**Current (2026-09):** the site is served as static files from S3 + CloudFront at
<https://hyeontae.gwinam.com/>. The Rust server is used for local development only.

- Infrastructure: stack `prod/homepage` in [HyeonTee/aws](https://github.com/HyeonTee/aws) (OpenTofu; S3, CloudFront, ACM, Route53)
- Publish: [`deploy/static/deploy.sh`](deploy/static/deploy.sh) — syncs `static/` to the bucket and invalidates the CDN

**Previous (retired):** the container ran on EC2 behind nginx. That setup is kept for reference:

- [`deploy/terraform`](deploy/terraform) (EC2, ECR, Route53, GitHub OIDC roles)
- [`deploy/ansible`](deploy/ansible) (nginx, Docker, Let's Encrypt, systemd-managed container)
- GitHub Actions in [`.github/workflows`](.github/workflows) — see [`.github/SETUP.md`](.github/SETUP.md)
