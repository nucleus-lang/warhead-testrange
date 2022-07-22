// Imports, e.g. Actix-Web things and the file server middleware
use actix_web::{App, HttpServer}; 
use actix_files::Files;

#[actix_web::main] // Make the main function usable by actix_web
async fn main() -> std::io::Result<()> {
    // Create a new server
    HttpServer::new(|| {
        App::new()
            .service(
                Files::new("/", "www/dist") // Create new file server with folder ./www/dist for route /
                .prefer_utf8(true) // Use UTF-8 encoding
                .index_file("index.html") // Set index.html to be displayed if no other file is being requested for the directory
            )
    })
    // Bind it to an IP address and port
    .bind(("127.0.0.1", 8080))?
    // Start the server
    .run()
    .await
}