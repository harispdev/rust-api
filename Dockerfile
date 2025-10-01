FROM rust:1.75

# Set environment variables
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=3000

# Set working directory
WORKDIR /app

# Copy the entire project
COPY . .

# Expose the port
EXPOSE 3000

# Run the application
CMD ["cargo", "run"]