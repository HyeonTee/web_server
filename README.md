# web_server

A multithreaded HTTP server written in Rust from scratch, used to serve a personal portfolio site behind nginx on EC2.

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
deploy/                   # IaC (planned)
├── terraform/            # EC2, SG, EIP, Route53
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

## Progress

### ✅ Phase 1 — Server core (done)
- [x] Modular project layout (`http/`, `router/`, `middleware/`, `handlers/`, `server/`)
- [x] `Request` parsing with `Method` enum
- [x] `Response` with `Vec<u8>` body (binary-safe), auto `Connection: close`
- [x] `Router` with builder pattern + Exact/Prefix patterns + method matching
- [x] `Middleware` trait with `Next` chain (around pattern), `Logger` middleware
- [x] Static file handler with `canonicalize()`-based path-traversal guard
- [x] `Config::from_env` (`BIND_ADDR`, `PORT`, `THREAD_POOL_SIZE`)
- [x] Stability: 16 KB read buffer, no panics on bad client connections, `Connection: close`

### ✅ Phase 2 — Static site content (done)
- [x] `static/index.html` — landing page with typing greeting
- [x] `static/about.html` — about / resume page (Introduction, Skills, Bio, Contact)
- [x] `static/404.html` — branded not-found page
- [x] `static/css/style.css` — CSS-variable theme (light/dark), cards, timeline, mobile responsive
- [x] `static/js/app.js` — flash-free theme toggle, nav active state, typing animation, auto current-date
- [ ] Assets: favicon, og:image, resume.pdf

### ✅ Phase 3 — Dockerization (done)
- [x] Multi-stage `Dockerfile` (rust:1-bookworm → debian:bookworm-slim, non-root user)
- [x] `.dockerignore`
- [x] Local verification — image ~98 MB, container responds 76–123µs

### 🚧 Phase 4 — IaC (in progress)
- [x] **Terraform**: default VPC, t3.micro Ubuntu 24.04, SG (22/80/443), EIP, SSH key, ECR + lifecycle, IAM instance profile (ECR read-only), Route53 zone + apex/www A records
- [x] Named AWS profile support + `expected_account_id` fail-safe check
- [ ] State backend (start local, migrate to S3 + DynamoDB lock)
- [ ] **Ansible** playbooks:
  - [ ] Base hardening (unattended-upgrades, ufw)
  - [ ] Docker install
  - [ ] nginx install + reverse proxy config (`proxy_pass http://127.0.0.1:8080`)
  - [ ] certbot install + Let's Encrypt issuance + auto-renewal
  - [ ] App deploy role: ECR login, pull image, systemd-managed container

### ⏳ Phase 5 — Cutover
- [ ] Apply Terraform, run Ansible
- [ ] DNS switch from Vercel to EC2 EIP
- [ ] Verify HTTPS, security headers, response times
- [ ] Decommission Vercel deployment

### Possible follow-ups
- GitHub Actions: build & push image on `main`, trigger Ansible deploy
- Small JSON API endpoints (e.g. visit counter)
- Structured logging + access log shipping
- Health-check endpoint + nginx upstream healthcheck

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

## Non-goals

- Becoming a general-purpose web framework
- Async / tokio support (synchronous thread pool is intentional)
- HTTP/2 or HTTP/3
- Keep-alive connections (single request per connection, `Connection: close`)
