# ✅ Rexpres Production Roadmap

A checklist to track progress toward a production-ready release of the `rexpres` web framework.

---

## ✅ Phase 1: Core HTTP Server
- [x] Basic HTTP server with `hyper`
- [ ] Add router system (`GET`, `POST`, `PUT`, `DELETE`)
- [ ] Route matching with dynamic params (`/user/:id`)
- [ ] Route handler type definition with params
- [ ] Static file serving
- [ ] Request body parsing (JSON, form data)

---

## ✅ Phase 2: Developer Ergonomics
- [ ] Middleware support
- [ ] Request/Response helpers (`res.send`, `res.json`, `res.status`)
- [ ] Chained API: `app.use().get().post()`
- [ ] Error handling middleware
- [ ] Logging middleware
- [ ] Custom response headers

---

## ✅ Phase 3: JS Bindings with NAPI-RS
- [x] napi-rs bindings to expose Rust server to Node.js
- [ ] `rexprs.start()` and route API exposed to JS
- [x] TypeScript declaration file (`.d.ts`) generation
- [ ] Build outputs to `dist/` with `index.js` and `rexprs.node`

---

## ✅ Phase 4: Advanced Features
- [ ] Support for async/await route handlers
- [ ] File upload handling (`multipart/form-data`)
- [ ] Query param + path param merging
- [ ] 404 and default error handlers
- [ ] Request validation helpers
- [ ] CORS support

---

## ✅ Phase 5: Utilities and DX
- [ ] `create-rexpres-app` CLI
- [ ] Project scaffolding with templates
- [ ] `rexprs.config.ts` support
- [ ] Dev mode with hot-reloading
- [ ] ESM + CJS module support

---

## ✅ Phase 6: Performance & Stability
- [ ] Benchmark against Express and Fastify
- [ ] Memory profiling
- [ ] Graceful shutdown
- [ ] Rate limiting middleware
- [ ] Unit tests and integration tests

---

## ✅ Phase 7: Database Support
- [ ] `.env` file support
- [ ] DB config abstraction layer
- [ ] ORM integration (support Prisma, Diesel, etc.)
- [ ] Example Postgres integration with connection pooling

---

## ✅ Phase 8: Authentication & Security
- [ ] JWT-based auth middleware
- [ ] Cookie + session support
- [ ] CSRF protection middleware
- [ ] Helmet-like security headers

---

## ✅ Phase 9: Documentation & Community
- [ ] Official documentation site
- [ ] Starter blog example in `rexpres`
- [ ] Tutorials and guides
- [ ] GitHub discussions or Discord for community

---

## ✅ Phase 10: Release
- [ ] Publish to npm with prebuilt binaries
- [ ] Setup CI/CD for multi-platform builds
- [x] Open source license (MIT)

---

> Stay fast. Stay express-like. Powered by Rust 🚀
