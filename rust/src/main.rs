// Imports, e.g. Actix-Web things and the file server middleware
use actix_web::{App, HttpServer}; 
use actix_files::Files;

mod constant;
use constant::{BINDADDR, BINDPORT, WWWROOT};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a new server
    HttpServer::new(move || {
        App::new()
            .service(
                Files::new("/", WWWROOT)
                .prefer_utf8(true)
                .index_file("index.html")
            )
    })
    // Bind it to an IP address and port
    .bind((BINDADDR, BINDPORT))?
    // Start the server
    .run()
    .await
}