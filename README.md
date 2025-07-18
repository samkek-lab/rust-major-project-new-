# Customer Support System (Rust, Beginner Level)

This is a beginner-friendly backend for a Customer Support Ticketing System written in Rust using Actix-web.

## Features (MVP)
- User management (in-memory, simple demo)
- Ticket management (create/list, in-memory)
- REST API endpoints

## How to Run
1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Open a terminal in this directory.
3. Run:
   ```sh
   cargo run
   ```
4. Visit [http://127.0.0.1:8080/](http://127.0.0.1:8080/) in your browser.

## API Endpoints
- `GET /` - Welcome message
- `GET /users` - List all users
- `GET /tickets` - List all tickets
- `POST /tickets` - Create a new ticket (JSON body: `{ "title": "...", "description": "..." }`)

## Next Steps
- Add more endpoints (update, delete, assign tickets, etc.)
- Persist data with a database
- Add authentication and roles
- Implement real-time features

---
This project is structured for easy learning and extension. If you want to add features, just ask!
