use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Define a simple User struct
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: usize,
    name: String,
    role: String, // e.g., "agent" or "admin"
}

// Define a simple Ticket struct
#[derive(Serialize, Deserialize, Clone)]
struct Ticket {
    id: usize,
    title: String,
    description: String,
    status: String, // e.g., "Open", "Closed"
    assigned_to: Option<usize>, // User ID
}

// App state holds users and tickets in memory
struct AppState {
    users: Mutex<Vec<User>>,
    tickets: Mutex<Vec<Ticket>>,
}

// Handlers for basic endpoints
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Customer Support System!")
}

// List all users
async fn list_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

// List all tickets
async fn list_tickets(data: web::Data<AppState>) -> impl Responder {
    let tickets = data.tickets.lock().unwrap();
    HttpResponse::Ok().json(&*tickets)
}

// Add a new ticket
#[derive(Deserialize)]
struct NewTicket {
    title: String,
    description: String,
}

async fn add_ticket(
    data: web::Data<AppState>,
    ticket: web::Json<NewTicket>,
) -> impl Responder {
    let mut tickets = data.tickets.lock().unwrap();
    let id = tickets.len() + 1;
    let new_ticket = Ticket {
        id,
        title: ticket.title.clone(),
        description: ticket.description.clone(),
        status: "Open".to_string(),
        assigned_to: None,
    };
    tickets.push(new_ticket);
    HttpResponse::Ok().body("Ticket created!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Mutex::new(vec![User {
            id: 1,
            name: "Alice".to_string(),
            role: "agent".to_string(),
        }]),
        tickets: Mutex::new(vec![]),
    });

    println!("Server running at http://127.0.0.1:8080/");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(hello))
            .route("/users", web::get().to(list_users))
            .route("/tickets", web::get().to(list_tickets))
            .route("/tickets", web::post().to(add_ticket))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
