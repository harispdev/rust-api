# Rust API Learning Project 🦀

A beginner-friendly Rust API project with Docker Compose setup to help you learn Rust web development.

## What You'll Learn

This project demonstrates:
- **Basic Rust syntax** and ownership concepts
- **HTTP server** using the Axum framework
- **JSON serialization/deserialization** with Serde
- **Async programming** with Tokio
- **Docker containerization** for Rust applications
- **REST API patterns** and error handling

## Quick Start

### Prerequisites
- Docker and Docker Compose installed
- Basic understanding of HTTP APIs (helpful but not required)

### Running the API

1. **Start the application:**
   ```bash
   docker-compose up --build
   ```

2. **Test the API:**
   ```bash
   # Health check
   curl http://localhost:3000/health

   # Get all users
   curl http://localhost:3000/users

   # Create a user
   curl -X POST http://localhost:3000/users \
     -H "Content-Type: application/json" \
     -d '{"name": "John Doe", "email": "john@example.com"}'

   # Get specific user
   curl http://localhost:3000/users/1
   ```

3. **Stop the application:**
   ```bash
   docker-compose down
   ```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Welcome message |
| GET | `/health` | Health check |
| GET | `/users` | Get all users |
| POST | `/users` | Create a new user |
| GET | `/users/:id` | Get user by ID |

## Learning Path

### 1. Understanding the Project Structure

```
rust-api/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   └── main.rs         # Main application code
├── Dockerfile          # Container configuration
├── docker-compose.yml  # Multi-container setup
└── README.md           # This file
```

### 2. Key Rust Concepts in This Project

#### **Ownership and Borrowing**
```rust
// Ownership: 'user' owns the data
let user = User { id: 1, name: "John".to_string(), email: "john@example.com".to_string() };

// Borrowing: '&user' borrows the data without taking ownership
fn print_user(user: &User) {
    println!("User: {}", user.name);
}
```

#### **Pattern Matching**
```rust
match storage.get(&id) {
    Some(user) => Ok(Json(ApiResponse { /* ... */ })),
    None => Err(StatusCode::NOT_FOUND),
}
```

#### **Error Handling**
```rust
// Result type for operations that can fail
async fn create_user(/* ... */) -> Result<Json<ApiResponse<User>>, StatusCode> {
    // Success case
    Ok(Json(ApiResponse { /* ... */ }))
}
```

#### **Async/Await**
```rust
#[tokio::main]
async fn main() {
    // Async main function
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### 3. Next Steps for Learning

1. **Modify the API:**
   - Add more fields to the User struct
   - Implement user update and delete endpoints
   - Add input validation

2. **Add a Database:**
   - Uncomment the PostgreSQL service in docker-compose.yml
   - Add database dependencies (sqlx, diesel, etc.)
   - Implement persistent storage

3. **Learn More Rust:**
   - [The Rust Book](https://doc.rust-lang.org/book/) - Official comprehensive guide
   - [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
   - [Axum Documentation](https://docs.rs/axum/) - Web framework docs

## Development Tips

### Running Without Docker
If you want to run the Rust code directly:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Run the application
cargo run

# Run in release mode (optimized)
cargo run --release
```

### Adding Dependencies
To add new dependencies, edit `Cargo.toml`:

```toml
[dependencies]
your-new-dependency = "1.0"
```

Then rebuild:
```bash
docker-compose up --build
```

### Debugging
- Add `println!` or `dbg!` macros for debugging
- Use `tracing` for structured logging (already included)
- Check logs with: `docker-compose logs rust-api`

## Common Rust Patterns You'll See

1. **Option and Result types** for handling null values and errors
2. **Struct definitions** with derive macros for automatic trait implementations
3. **Lifetime annotations** (though minimal in this example)
4. **Closures** and **iterators** for functional programming
5. **Module system** for organizing code

## Troubleshooting

- **Port already in use:** Change the port in docker-compose.yml
- **Build fails:** Make sure Docker is running and you have enough disk space
- **API not responding:** Check if the container is running with `docker-compose ps`

Happy learning! 🦀✨