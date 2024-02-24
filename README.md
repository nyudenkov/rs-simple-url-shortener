# Simple URL shortener

This is a URL shortening service built with Rust, using Actix-web as the web framework and SQLx for database operations. The service provides an API to create shortened URLs and to redirect to the original URL using the shortened code.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust: You can download Rust from the official website [here](https://www.rust-lang.org/tools/install).
- SQLite: You can download SQLite from the official website [here](https://www.sqlite.org/download.html).

### Installing

1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Run `cargo build` to compile the project.

### Configuration

The project requires the following environment variables:

- `DATABASE_URL`: The connection string to your SQLite database.
- `AUTH_TOKEN`: The secret token for bearer authentication.

You can set these environment variables in a `.env` file in the root directory of the project.

### Database Setup

Before running the application, you need to create a SQLite database. Here's how you can do it:

1. Install SQLite if you haven't already. You can download SQLite from the official website [here](https://www.sqlite.org/download.html).
2. Open your terminal and navigate to the directory where you want to create the database file.
3. Run the following command to create a new SQLite database:

```bash
sqlite3 mydatabase.db
```

Replace `mydatabase.db` with the name you want to give to your database file. This will create a new SQLite database in the current directory.

Remember to update the `DATABASE_URL` environment variable in your `.env` file with the path to your database file.

### Running Migrations

This project uses SQLx for database operations, which includes a powerful command-line utility for managing database schemas. Before running the application, you need to apply the database migrations. Here's how you can do it:

1. Install the SQLx command-line utility if you haven't already. You can do this by running the following command:

```bash
cargo install sqlx-cli --no-default-features --features sqlite
```

2. Run the following command to apply the migrations:

```bash
sqlx migrate run
```

This will apply all pending migrations to your database, preparing it for use with the application.


Remember to replace `mydatabase.db` with the actual name of your database file.

### Running the Application

1. Run `cargo run` to start the application.
2. The application will start running at `http://127.0.0.1:8080`.

## API Endpoints

- `POST /secure/shorten`: Creates a new shortened URL. Requires a bearer token for authentication. Accepts a JSON body with `original_url` and `short_code` fields.
- `GET /{short_code}`: Redirects to the original URL associated with the given short code.

## Built With

- [Rust](https://www.rust-lang.org/) - The programming language used.
- [Actix-web](https://actix.rs/) - The web framework used.
- [SQLx](https://github.com/launchbadge/sqlx) - The database library used.
