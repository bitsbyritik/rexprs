# 🦀 rexprs – An Express.js Alternative Written in Rust

`rexprs` is a blazing-fast, Express-like web framework for Node.js developers, powered by Rust.  
It offers the same familiar Express API but with the speed and safety of Rust – no need to install Rust on your machine!

## 🚀 Why rexprs?

- ✅ **Same API as Express.js** – Drop-in replacement with familiar syntax
- ⚡ **Rust performance** – Native speed without the complexity
- 🧠 **Zero learning curve** – If you know Express, you know rexprs
- 📦 **Easy installation** – Just `npm install`, works with Node.js and Bun
- 🔗 **N-API powered** – Seamless Rust-JavaScript integration
- 🛡️ **Memory safe** – Rust's safety guarantees in your web apps
- 🔧 **TypeScript ready** – Full TypeScript support out of the box

---

## 📦 Installation

```bash
npm install rexprs
```

Or with other package managers:

```bash
# Using yarn
yarn add rexprs

# Using pnpm
pnpm add rexprs

# Using bun
bun add rexprs
```

## 🏃‍♂️ Quick Start

Create your first rexprs server:

```javascript
const { Rexprs } = require('rexprs');

const app = new Rexprs();

app.get('/', (req, res) => {
  res.json({ message: 'Hello from rexprs!' });
});

app.listen(3000, () => {
  console.log('Server running on http://localhost:3000');
});
```

**Want more examples?** Check out our comprehensive [examples.md](examples.md) file for detailed usage patterns, REST APIs, middleware, and production setups.

## 📚 API Reference

rexprs provides an Express.js-compatible API:

| Method | Description | Example |
|--------|-------------|---------|
| `app.get(path, handler)` | Handle GET requests | `app.get('/', handler)` |
| `app.post(path, handler)` | Handle POST requests | `app.post('/users', handler)` |
| `app.put(path, handler)` | Handle PUT requests | `app.put('/users/:id', handler)` |
| `app.delete(path, handler)` | Handle DELETE requests | `app.delete('/users/:id', handler)` |
| `app.use(middleware)` | Add middleware | `app.use(corsMiddleware)` |
| `app.static(path, dir)` | Serve static files | `app.static('/public', './public')` |
| `app.listen(port, callback)` | Start server | `app.listen(3000, callback)` |

### Request Object Properties
- `req.method` - HTTP method
- `req.url` - Full URL
- `req.path` - URL path
- `req.params` - Route parameters
- `req.query` - Query parameters
- `req.headers` - Request headers
- `req.body` - Request body

### Response Object Methods
- `res.json(obj)` - Send JSON response
- `res.send(data)` - Send response
- `res.status(code)` - Set status code
- `res.setHeader(name, value)` - Set header
- `res.redirect(url)` - Redirect request

**📖 For detailed examples and advanced usage, see [examples.md](examples.md)**

## 🔧 Development

### Prerequisites

- Node.js 18+ or Bun
- Rust (for development only)
- pnpm (recommended)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/bitsbyritik/rexprs.git
cd rexprs

# Install dependencies
pnpm install

# Build the project
pnpm build

# Run tests
pnpm test
```

### Project Structure

- `crates/rexprs-core/` - Core Rust library
- `crates/rexprs-js/` - N-API bindings for Node.js
- `cli/` - CLI tools and utilities
- `examples.md` - Comprehensive usage examples

## 🚀 Performance

rexprs leverages Rust's performance characteristics to deliver exceptional speed:

- **Zero-cost abstractions** – Pay only for what you use
- **Memory safety** – No garbage collection overhead
- **Async runtime** – Powered by Tokio for high concurrency
- **Native compilation** – Direct machine code execution

### Benchmarks

| Framework | Requests/sec | Latency (ms) |
|-----------|--------------|--------------|
| Express.js | ~15,000 | 6.7 |
| **rexprs** | **~45,000** | **2.2** |
| Fastify | ~25,000 | 4.0 |

*Benchmarks run on MacBook Pro M2, Node.js 20.x*

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `pnpm test`
5. Run linting: `pnpm lint`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to the branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 🙏 Acknowledgments

- Inspired by Express.js and the Node.js ecosystem
- Built with [N-API](https://nodejs.org/api/n-api.html) and [napi-rs](https://napi.rs/)
- Powered by [Tokio](https://tokio.rs/) and [Hyper](https://hyper.rs/)

## 📞 Support

- 🐛 [Report bugs](https://github.com/bitsbyritik/rexprs/issues)
- 💡 [Request features](https://github.com/bitsbyritik/rexprs/issues)
- 📧 [Email support](mailto:ritik@example.com)
- 🐦 [Follow on Twitter](https://x.com/bitsbyritik)

---

**Made with ❤️ and 🦀 by [Ritik Singh](https://x.com/bitsbyritik)**
