# 📚 rexprs Examples

This document contains comprehensive examples for using rexprs in various scenarios.

## 🏃‍♂️ Quick Start Examples

### Basic Server

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

### TypeScript Example

```typescript
import { Rexprs, Request, Response } from 'rexprs';

const app = new Rexprs();

app.get('/', (req: Request, res: Response) => {
  res.json({ message: 'Hello from rexprs with TypeScript!' });
});

app.listen(3000, () => {
  console.log('Server running on http://localhost:3000');
});
```

## 🛣️ Routing Examples

### Basic Routes

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// GET route
app.get('/users', (req, res) => {
  res.json({ users: [] });
});

// POST route
app.post('/users', (req, res) => {
  res.status(201).json({ message: 'User created' });
});

// PUT route
app.put('/users/:id', (req, res) => {
  res.json({ message: `User ${req.params.id} updated` });
});

// DELETE route
app.delete('/users/:id', (req, res) => {
  res.status(204).send();
});

app.listen(3000);
```

### Route Parameters

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// Single parameter
app.get('/users/:id', (req, res) => {
  const userId = req.params.id;
  res.json({ user: `User ${userId}` });
});

// Multiple parameters
app.get('/users/:userId/posts/:postId', (req, res) => {
  const { userId, postId } = req.params;
  res.json({ 
    message: `Post ${postId} by user ${userId}` 
  });
});

// Optional parameters with query strings
app.get('/search', (req, res) => {
  const { q, limit = 10, offset = 0 } = req.query;
  res.json({ 
    query: q, 
    limit: parseInt(limit), 
    offset: parseInt(offset) 
  });
});

app.listen(3000);
```

### Wildcard Routes

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// Catch all routes
app.all('/api/*', (req, res) => {
  res.json({ 
    message: 'API endpoint', 
    path: req.path 
  });
});

// Specific method for all paths
app.get('*', (req, res) => {
  res.status(404).json({ error: 'Page not found' });
});

app.listen(3000);
```

## 🔧 Middleware Examples

### Global Middleware

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// Logging middleware
app.use((req, res, next) => {
  console.log(`${new Date().toISOString()} - ${req.method} ${req.path}`);
  next();
});

// CORS middleware
app.use((req, res, next) => {
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
  next();
});

// JSON body parser middleware
app.use((req, res, next) => {
  if (req.headers['content-type'] === 'application/json') {
    // Parse JSON body (implementation depends on your API)
    req.body = JSON.parse(req.rawBody);
  }
  next();
});

app.get('/', (req, res) => {
  res.json({ message: 'Hello with middleware!' });
});

app.listen(3000);
```

### Route-Specific Middleware

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// Authentication middleware
const authenticate = (req, res, next) => {
  const token = req.headers.authorization;
  
  if (!token) {
    return res.status(401).json({ error: 'No token provided' });
  }
  
  // Verify token logic here
  if (token !== 'Bearer valid-token') {
    return res.status(403).json({ error: 'Invalid token' });
  }
  
  req.user = { id: 1, name: 'John Doe' };
  next();
};

// Rate limiting middleware
const rateLimit = (req, res, next) => {
  // Simple rate limiting logic
  const ip = req.headers['x-forwarded-for'] || req.connection.remoteAddress;
  
  // Check rate limit for IP
  // This is a simplified example
  console.log(`Request from ${ip}`);
  next();
};

// Protected route with multiple middleware
app.get('/protected', authenticate, rateLimit, (req, res) => {
  res.json({ 
    message: 'Protected data', 
    user: req.user 
  });
});

// Public route
app.get('/public', (req, res) => {
  res.json({ message: 'Public data' });
});

app.listen(3000);
```

## 🗄️ Complete REST API Example

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// In-memory data store
let users = [
  { id: 1, name: 'John Doe', email: 'john@example.com', createdAt: new Date() },
  { id: 2, name: 'Jane Smith', email: 'jane@example.com', createdAt: new Date() }
];

let nextId = 3;

// Middleware
app.use((req, res, next) => {
  console.log(`${req.method} ${req.path}`);
  next();
});

// GET /api/users - Get all users
app.get('/api/users', (req, res) => {
  const { limit = 10, offset = 0 } = req.query;
  const paginatedUsers = users.slice(
    parseInt(offset), 
    parseInt(offset) + parseInt(limit)
  );
  
  res.json({
    users: paginatedUsers,
    total: users.length,
    limit: parseInt(limit),
    offset: parseInt(offset)
  });
});

// GET /api/users/:id - Get user by ID
app.get('/api/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const user = users.find(u => u.id === userId);
  
  if (!user) {
    return res.status(404).json({ 
      error: 'User not found',
      code: 'USER_NOT_FOUND'
    });
  }
  
  res.json({ user });
});

// POST /api/users - Create new user
app.post('/api/users', (req, res) => {
  const { name, email } = req.body;
  
  // Validation
  if (!name || !email) {
    return res.status(400).json({
      error: 'Name and email are required',
      code: 'VALIDATION_ERROR'
    });
  }
  
  // Check if email already exists
  if (users.some(u => u.email === email)) {
    return res.status(409).json({
      error: 'Email already exists',
      code: 'EMAIL_EXISTS'
    });
  }
  
  const newUser = {
    id: nextId++,
    name,
    email,
    createdAt: new Date()
  };
  
  users.push(newUser);
  res.status(201).json({ user: newUser });
});

// PUT /api/users/:id - Update user
app.put('/api/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const userIndex = users.findIndex(u => u.id === userId);
  
  if (userIndex === -1) {
    return res.status(404).json({ 
      error: 'User not found',
      code: 'USER_NOT_FOUND'
    });
  }
  
  const { name, email } = req.body;
  
  // Check if email is being changed and already exists
  if (email && email !== users[userIndex].email) {
    if (users.some(u => u.email === email && u.id !== userId)) {
      return res.status(409).json({
        error: 'Email already exists',
        code: 'EMAIL_EXISTS'
      });
    }
  }
  
  // Update user
  users[userIndex] = {
    ...users[userIndex],
    ...(name && { name }),
    ...(email && { email }),
    updatedAt: new Date()
  };
  
  res.json({ user: users[userIndex] });
});

// PATCH /api/users/:id - Partial update user
app.patch('/api/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const userIndex = users.findIndex(u => u.id === userId);
  
  if (userIndex === -1) {
    return res.status(404).json({ 
      error: 'User not found',
      code: 'USER_NOT_FOUND'
    });
  }
  
  // Only update provided fields
  const updates = {};
  if (req.body.name !== undefined) updates.name = req.body.name;
  if (req.body.email !== undefined) updates.email = req.body.email;
  
  users[userIndex] = {
    ...users[userIndex],
    ...updates,
    updatedAt: new Date()
  };
  
  res.json({ user: users[userIndex] });
});

// DELETE /api/users/:id - Delete user
app.delete('/api/users/:id', (req, res) => {
  const userId = parseInt(req.params.id);
  const userIndex = users.findIndex(u => u.id === userId);
  
  if (userIndex === -1) {
    return res.status(404).json({ 
      error: 'User not found',
      code: 'USER_NOT_FOUND'
    });
  }
  
  const deletedUser = users.splice(userIndex, 1)[0];
  res.json({ 
    message: 'User deleted successfully',
    user: deletedUser
  });
});

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ 
    status: 'healthy',
    timestamp: new Date(),
    uptime: process.uptime()
  });
});

// 404 handler
app.all('*', (req, res) => {
  res.status(404).json({
    error: 'Endpoint not found',
    code: 'NOT_FOUND',
    path: req.path
  });
});

app.listen(3000, () => {
  console.log('REST API server running on http://localhost:3000');
  console.log('Available endpoints:');
  console.log('  GET    /api/users');
  console.log('  GET    /api/users/:id');
  console.log('  POST   /api/users');
  console.log('  PUT    /api/users/:id');
  console.log('  PATCH  /api/users/:id');
  console.log('  DELETE /api/users/:id');
  console.log('  GET    /health');
});
```

## 📁 Static File Serving

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// Serve static files from 'public' directory
app.static('/public', './public');

// Serve assets with different mount path
app.static('/assets', './dist/assets');

// Serve with options
app.static('/images', './uploads/images', {
  maxAge: '1d',
  etag: true
});

// API routes
app.get('/api/info', (req, res) => {
  res.json({ message: 'API with static files' });
});

app.listen(3000, () => {
  console.log('Server with static files running on http://localhost:3000');
  console.log('Static files available at:');
  console.log('  /public/* -> ./public/*');
  console.log('  /assets/* -> ./dist/assets/*');
  console.log('  /images/* -> ./uploads/images/*');
});
```

## 🔧 Advanced Examples

### Custom Error Handling

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// Custom error class
class ApiError extends Error {
  constructor(message, statusCode = 500, code = 'INTERNAL_ERROR') {
    super(message);
    this.statusCode = statusCode;
    this.code = code;
  }
}

// Error handling middleware
app.use((err, req, res, next) => {
  console.error('Error:', err);
  
  if (err instanceof ApiError) {
    return res.status(err.statusCode).json({
      error: err.message,
      code: err.code
    });
  }
  
  // Generic error
  res.status(500).json({
    error: 'Internal server error',
    code: 'INTERNAL_ERROR'
  });
});

// Route that might throw an error
app.get('/api/users/:id', (req, res, next) => {
  try {
    const userId = parseInt(req.params.id);
    
    if (isNaN(userId)) {
      throw new ApiError('Invalid user ID', 400, 'INVALID_ID');
    }
    
    if (userId > 1000) {
      throw new ApiError('User not found', 404, 'USER_NOT_FOUND');
    }
    
    res.json({ user: { id: userId, name: 'John Doe' } });
  } catch (error) {
    next(error);
  }
});

app.listen(3000);
```

### Request Validation

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// Validation middleware
const validateUser = (req, res, next) => {
  const { name, email, age } = req.body;
  const errors = [];
  
  if (!name || typeof name !== 'string' || name.trim().length < 2) {
    errors.push('Name must be at least 2 characters long');
  }
  
  if (!email || !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    errors.push('Valid email is required');
  }
  
  if (age !== undefined && (!Number.isInteger(age) || age < 0 || age > 150)) {
    errors.push('Age must be a valid integer between 0 and 150');
  }
  
  if (errors.length > 0) {
    return res.status(400).json({
      error: 'Validation failed',
      details: errors
    });
  }
  
  next();
};

// Use validation middleware
app.post('/api/users', validateUser, (req, res) => {
  const { name, email, age } = req.body;
  
  // Create user logic here
  const user = {
    id: Math.floor(Math.random() * 1000),
    name: name.trim(),
    email: email.toLowerCase(),
    age
  };
  
  res.status(201).json({ user });
});

app.listen(3000);
```

### Database Integration Example (Conceptual)

```javascript
const { Rexprs } = require('rexprs');
// const { Pool } = require('pg'); // PostgreSQL example

const app = new Rexprs();

// Database connection (conceptual)
// const db = new Pool({
//   connectionString: process.env.DATABASE_URL
// });

// Database middleware
const withDb = (req, res, next) => {
  // req.db = db;
  next();
};

app.use(withDb);

// Get users from database
app.get('/api/users', async (req, res) => {
  try {
    // const result = await req.db.query('SELECT * FROM users');
    // res.json({ users: result.rows });
    
    // Mock response
    res.json({ users: [] });
  } catch (error) {
    console.error('Database error:', error);
    res.status(500).json({ error: 'Database error' });
  }
});

// Create user in database
app.post('/api/users', async (req, res) => {
  try {
    const { name, email } = req.body;
    
    // const result = await req.db.query(
    //   'INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *',
    //   [name, email]
    // );
    
    // Mock response
    const user = { id: 1, name, email };
    res.status(201).json({ user });
  } catch (error) {
    console.error('Database error:', error);
    res.status(500).json({ error: 'Database error' });
  }
});

app.listen(3000);
```

## 🧪 Testing Examples

### Basic Testing Setup

```javascript
// test/server.test.js
const { Rexprs } = require('rexprs');

// Test helper to create app
function createTestApp() {
  const app = new Rexprs();
  
  app.get('/test', (req, res) => {
    res.json({ message: 'test response' });
  });
  
  app.post('/echo', (req, res) => {
    res.json({ received: req.body });
  });
  
  return app;
}

// Example test (using your preferred testing framework)
describe('rexprs server', () => {
  let app;
  let server;
  
  beforeEach(() => {
    app = createTestApp();
    server = app.listen(0); // Use random port
  });
  
  afterEach(() => {
    server.close();
  });
  
  it('should respond to GET requests', async () => {
    const response = await fetch(`http://localhost:${server.port}/test`);
    const data = await response.json();
    
    expect(data.message).toBe('test response');
  });
  
  it('should handle POST requests', async () => {
    const response = await fetch(`http://localhost:${server.port}/echo`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ test: 'data' })
    });
    
    const data = await response.json();
    expect(data.received.test).toBe('data');
  });
});
```

## 🚀 Production Examples

### Production Server Setup

```javascript
const { Rexprs } = require('rexprs');
const app = new Rexprs();

// Production middleware
app.use((req, res, next) => {
  // Security headers
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('X-Frame-Options', 'DENY');
  res.setHeader('X-XSS-Protection', '1; mode=block');
  next();
});

// Request logging
app.use((req, res, next) => {
  const start = Date.now();
  
  res.on('finish', () => {
    const duration = Date.now() - start;
    console.log(`${req.method} ${req.path} - ${res.statusCode} - ${duration}ms`);
  });
  
  next();
});

// Health check
app.get('/health', (req, res) => {
  res.json({ 
    status: 'ok',
    timestamp: new Date().toISOString(),
    version: process.env.APP_VERSION || '1.0.0'
  });
});

// API routes
app.get('/api/status', (req, res) => {
  res.json({ 
    server: 'rexprs',
    environment: process.env.NODE_ENV || 'development',
    uptime: process.uptime()
  });
});

// Error handling
app.use((err, req, res, next) => {
  console.error('Unhandled error:', err);
  res.status(500).json({ 
    error: 'Internal server error',
    requestId: req.id || 'unknown'
  });
});

// 404 handler
app.all('*', (req, res) => {
  res.status(404).json({ 
    error: 'Not found',
    path: req.path 
  });
});

const port = process.env.PORT || 3000;
app.listen(port, '0.0.0.0', () => {
  console.log(`Production server running on port ${port}`);
});
```

---

**Need more examples?** Check out the [GitHub repository](https://github.com/bitsbyritik/rexprs) for additional use cases and community contributions.