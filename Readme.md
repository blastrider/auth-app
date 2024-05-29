# Auth-App

## Description

Auth-App is a fast authentication application developed in Rust using the Actix-web framework. It allows for simple authentication via a REST API and logs all valid connections and errors to a specified log file.

## Prerequisites

- Rust (version 1.56+)
- Cargo
- `cargo-watch` (for development)

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/your-username/auth-app.git
   cd auth-app
   ```

2. Install dependencies:

   ```sh
   cargo build
   ```

3. Install `cargo-watch` (if not already installed):
   ```sh
   cargo install cargo-watch
   ```

## Configuration

1. Create a `.env` file at the root of the project and add the following variables:

   ```env
   HOST=127.0.0.1
   PORT=8080
   LOG_FILE_PATH=logs/log.txt
   ```

2. Create the `logs` directory at the root of the project:
   ```sh
   mkdir logs
   ```

## Running the Application

1. To run the application with `cargo watch` and monitor changes:

   ```sh
   cargo watch -x 'run'
   ```

2. To run the application without `cargo watch`:
   ```sh
   cargo run
   ```

## Usage

### Authentication Endpoint

- **URL**: `/login`
- **Method**: POST
- **Headers**:
  - `Content-Type: application/json`
- **Body**:
  ```json
  {
    "username": "admin",
    "password": "password"
  }
  ```

### Example `curl` Request

```sh
curl -X POST http://127.0.0.1:8080/login -H "Content-Type: application/json" -d '{"username":"admin","password":"password"}'
```

### Logs

- Valid connections and errors will be logged to the file specified by `LOG_FILE_PATH`, which defaults to `logs/log.txt`.

## Project Structure

```
auth-app/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── handlers.rs
│   ├── models.rs
│   ├── routes.rs
│   └── utils.rs
├── logs/
│   └── log.txt
└── .env
```

## Contributing

1. Fork the repository.
2. Create a branch for your feature (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push your branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

## License

Distributed under the MIT License. See `LICENSE` for more information.
