# Rust REST API with Session Authentication

A modern, production-ready Rust REST API built with Axum, SeaORM, PostgreSQL, and Redis, featuring session-based authentication with HTTP-only cookies.

## 🚀 Features

- **Modern Stack**: Axum 0.7, SeaORM 0.12, PostgreSQL 16, Redis 7
- **Session Authentication**: HTTP-only cookies with Redis-backed sessions
- **Type Safety**: Full compile-time type checking with SeaORM
- **Async/Await**: Built for high-performance async operations
- **Containerized**: Complete Docker setup with multi-stage builds
- **Security**: Argon2 password hashing, session management, input validation
- **Database**: PostgreSQL with automated schema initialization
- **API Documentation**: Ready for OpenAPI/Swagger integration
- **Health Checks**: Built-in health monitoring for all services
- **Logging**: Structured logging with tracing
- **Clean Architecture**: Modular design with separation of concerns

## 🏗️ Architecture

```
src/
├── main.rs                 # Application entry point
├── common/                 # Shared utilities and infrastructure
│   ├── config.rs          # Configuration management
│   ├── database/          # Database connection and setup
│   ├── errors/            # Custom error types and handling
│   ├── repositories/      # Base repository traits
│   ├── session/           # Session management with Redis
│   └── state/             # Application state management
├── modules/               # Feature modules (business logic)
│   ├── auth/              # Authentication module
│   │   ├── entity.rs      # Auth DTOs and user info
│   │   ├── controller.rs  # Login/logout/register handlers
│   │   ├── service.rs     # Auth business logic
│   │   ├── repository.rs  # Auth data access
│   │   ├── middleware.rs  # Authentication middleware
│   │   └── route.rs       # Auth route definitions
│   └── user/              # User management module
│       ├── entity.rs      # User models and DTOs
│       ├── controller.rs  # User CRUD handlers
│       ├── service.rs     # User business logic
│       ├── repository.rs  # User data access
│       └── route.rs       # User route definitions
└── routes/                # Main router configuration
    └── mod.rs             # Route aggregation
```

## 🛠️ Tech Stack

- **Web Framework**: Axum 0.7
- **ORM**: SeaORM 0.12
- **Database**: PostgreSQL 16
- **Session Store**: Redis 7
- **Session Management**: tower-sessions with Redis backend
- **Async Runtime**: Tokio
- **Validation**: Validator
- **Password Hashing**: Argon2
- **Serialization**: Serde
- **Logging**: Tracing
- **Containerization**: Docker & Docker Compose

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+
- Docker & Docker Compose

### 1. Clone and Setup

```bash
git clone <your-repo>
cd rust-api

# Create environment file
cp env.example .env

# Edit .env with your settings (optional)
nano .env

# Start all services
docker compose up --build -d
```

### 2. Using Docker (Recommended)

```bash
# Start all services (PostgreSQL + Redis + API)
docker compose up --build -d

# Check service status
docker compose ps

# View logs
docker compose logs -f
```

### 3. Using Local Development

```bash
# Start database services only
docker compose up -d postgres redis

# Run the API locally
cargo run
```

## 📋 Available Commands

```bash
# Development
cargo build         # Build the application
cargo run           # Run locally
cargo test          # Run tests
cargo check         # Check compilation

# Docker
docker compose up --build -d     # Start all services
docker compose down              # Stop all services
docker compose down -v           # Stop and remove volumes
docker compose logs -f           # View logs
docker compose ps                # Check service status

# Database
# Database schema is auto-initialized by PostgreSQL init scripts
```

## 🗄️ Database

### Schema

The `users` table includes:
- `id` (UUID, Primary Key)
- `account_id` (UUID, Required)
- `branch_id` (UUID, Optional)
- `name` (VARCHAR, Optional)
- `email` (VARCHAR, Unique, Required)
- `password_hash` (VARCHAR, Required)
- `role` (VARCHAR, Default: 'CUSTOMER')
- `status` (VARCHAR, Default: 'Active')
- `created_at` (TIMESTAMPTZ)
- `updated_at` (TIMESTAMPTZ)
- `deleted_at` (TIMESTAMPTZ, Soft Delete)

### User Roles

- `ROOT` - System administrator
- `GENERAL_MANAGER` - General manager
- `MANAGER` - Manager
- `CUSTOMER` - Customer (default)
- `WAITER` - Waiter
- `COOK` - Cook
- `BARMAN` - Barman
- `CASH_REGISTER` - Cash register operator

## 🔌 API Endpoints

### Public Endpoints
- `GET /health` - Health status
- `POST /auth/register` - User registration
- `POST /auth/login` - User login
- `DELETE /auth/logout` - User logout

### Protected Endpoints (Require Authentication)
- `GET /users` - List all users
- `GET /users/{id}` - Get user by ID
- `POST /users` - Create new user
- `PUT /users/{id}` - Update user
- `DELETE /users/{id}` - Delete user (soft delete)

### Example Requests

#### Register User
```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@example.com",
    "password": "securepassword123",
    "account_id": "550e8400-e29b-41d4-a716-446655440000",
    "role": "CUSTOMER"
  }'
```

#### Login User
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "securepassword123"
  }'
```

#### Access Protected Route
```bash
# Use the session cookie returned from login
curl -X GET http://localhost:3000/users \
  -H "Cookie: connect.sid=your-session-id"
```

#### Health Check
```bash
curl http://localhost:3000/health
```

## 🔧 Configuration

Environment variables (see `.env.example`):

```bash
# Server
HOST=0.0.0.0
PORT=3000

# Database
DATABASE_HOST=postgres
DATABASE_PORT=5432
DATABASE_NAME=rust_api
DATABASE_USER=postgres
DATABASE_PASSWORD=your-secure-password-change-in-production

# Session Configuration
SESSION_SECRET=your-super-secret-session-key-change-in-production
REDIS_URL=redis://redis:6379
SESSION_COOKIE_NAME=connect.sid
SESSION_COOKIE_DOMAIN=.table-tap.app
SESSION_COOKIE_SECURE=false
SESSION_COOKIE_SAME_SITE=lax
SESSION_MAX_AGE_SECONDS=86400

# Logging
RUST_LOG=info
```

## 🐳 Docker Services

### PostgreSQL
- **Image**: postgres:16-alpine
- **Port**: 5432
- **Database**: rust_api
- **User**: postgres
- **Health Check**: Built-in

### Redis
- **Image**: redis:7-alpine
- **Port**: 6379
- **Purpose**: Session storage
- **Health Check**: Built-in

### API
- **Port**: 3000
- **Dependencies**: PostgreSQL, Redis
- **Health Check**: Built-in

## 🔒 Security Features

- **Session Authentication**: HTTP-only cookies with Redis backend
- **Password Hashing**: Argon2 with random salts
- **Input Validation**: Comprehensive validation with custom error messages
- **Type Safety**: Compile-time SQL injection prevention
- **Error Handling**: Secure error responses without sensitive data exposure
- **Container Security**: Non-root user in containers
- **CSRF Protection**: SameSite cookie attribute
- **XSS Protection**: HttpOnly cookie attribute

## 🚀 Production Deployment

### Environment Setup
1. Set strong `SESSION_SECRET`
2. Configure production database URL
3. Set production Redis URL
4. Configure secure cookie settings
5. Set appropriate log levels
6. Configure connection pool settings

### Docker Production
```bash
# Build production images
docker compose build

# Deploy
docker compose up -d
```

## 🧪 Testing

```bash
# Run tests
cargo test

# Run with output
cargo test -- --nocapture

# Test with Docker
docker compose up --build -d
# Then test endpoints with curl
```

## 📊 Monitoring

- **Health Check**: `GET /health`
- **Service Health**: All services have health checks
- **Logging**: Structured JSON logs
- **Metrics**: Ready for Prometheus integration
- **Tracing**: Request tracing support

## 🔄 Development Workflow

1. **Start Services**: `docker compose up --build -d`
2. **Make Changes**: Edit code
3. **Test**: `cargo test`
4. **Run**: `cargo run` (or auto-reload with `cargo watch`)
5. **Deploy**: `docker compose build && docker compose up -d`

## 📚 Why This Architecture?

### Session vs JWT Authentication

**Session-based authentication** was chosen because:
- ✅ **Security**: HTTP-only cookies prevent XSS attacks
- ✅ **Server Control**: Can invalidate sessions server-side
- ✅ **Scalability**: Redis-backed sessions scale horizontally
- ✅ **Simplicity**: Easier to implement and debug
- ✅ **Compatibility**: Matches existing Node.js implementation

### SeaORM vs SQLx vs Diesel

**SeaORM** was chosen for 2025 because:
- ✅ **Async-first**: Built for modern async Rust
- ✅ **Type Safety**: Compile-time query verification
- ✅ **Clean API**: Intuitive, developer-friendly
- ✅ **Active Development**: Regular updates and improvements
- ✅ **Performance**: Excellent performance characteristics
- ✅ **Migration System**: Built-in migration management

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 🆘 Troubleshooting

### Common Issues

**Database Connection Failed**
```bash
# Check if PostgreSQL is running
docker compose ps

# Check logs
docker compose logs postgres
```

**Redis Connection Failed**
```bash
# Check if Redis is running
docker compose ps

# Check logs
docker compose logs redis
```

**Session Issues**
```bash
# Check Redis connectivity
docker compose exec redis redis-cli ping

# Clear Redis data
docker compose down -v
docker compose up --build -d
```

**Build Errors**
```bash
# Clean and rebuild
cargo clean
cargo build
```

## 📞 Support

- Create an issue for bugs
- Start a discussion for questions
- Check the documentation

---

**Built with ❤️ using Rust, Axum, SeaORM, and Redis**