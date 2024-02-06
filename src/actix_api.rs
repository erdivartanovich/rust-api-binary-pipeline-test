use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use std::process::{Command, Stdio};

async fn list_dir(req: HttpRequest) -> impl Responder {
    // Extract data from the HTTP request
    let query_string = req.query_string();

    // Call list_dir binary with the extracted data
    let output = Command::new("./list_dir")
        .env("QUERY_STRING", &query_string)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute list_dir binary");

    // Display the output from list_dir binary
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check for errors
    if !stderr.is_empty() {
        eprintln!("Error from list_dir binary: {}", stderr);
        return format!("Error from list_dir binary: {}", stderr);
    }

    // Display the standard output from list_dir binary
    stdout.to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/ls").to(list_dir))
    })
    .bind("0.0.0.0:8080")?  // Bind to all network interfaces
    .run()
    .await
}
