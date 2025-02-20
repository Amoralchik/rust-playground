# Omni Project

This repository contains a simple Rust program that uses the `hyper` crate to make HTTP requests. It's designed to demonstrate how to interact with web servers using asynchronous I/O in Rust.

## How to Use

1. **Clone the Repository**:

   ```sh
   git clone https://github.com/amoralchik/omni.git
   cd omni
   ```

2. **Run the Program**:
   To run the program, execute the following command in your terminal:

   ```sh
   cargo run --bin omni <url>
   ```

   Replace `<url>` with the desired URL you want to request.

3. **Example Usage**:
   For example, to request the homepage of GitHub, you would run:
   ```sh
   cargo run --bin omni example.com
   ```

## Dependencies

- **clap**: A powerful command-line argument parser.
- **hyper**: A high-level HTTP client library for Rust.
- **tokio**: A runtime for asynchronous I/O operations in Rust.
