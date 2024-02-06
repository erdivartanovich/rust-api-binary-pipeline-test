use actix_web::{web, App, HttpServer, Responder};
use urlencoding::decode;
use std::collections::HashMap;
use std::fs;

async fn list_dir(query: web::Query<HashMap<String, String>>) -> impl Responder {
    // Get the "dir" parameter from the query string and URL decode it
    let dir_param = query.get("dir").unwrap_or_else(|| {
        println!("400: param \"dir\" not implemented");
        std::process::exit(1);
    });

    let dir = decode(dir_param).unwrap_or_else(|_| {
        println!("Error: Failed to decode directory parameter");
        std::process::exit(1);
    });

    // Validate and process the directory
    if dir.is_empty() {
        println!("400: param \"dir\" not implemented");
        std::process::exit(1);
    }

    // List the contents of the specified directory
    match fs::read_dir(&*dir) {
        Ok(entries) => {
            let file_names: Vec<String> = entries
                .filter_map(|entry| entry.ok().map(|e| e.file_name()))
                .map(|name| name.to_string_lossy().to_string())
                .collect();

            // Return the list of file names as a response
            let list = file_names.join("\n");
            format!("results:\n\n{}", list)
        }
        Err(e) => {
            // Return an error response
            format!("Error listing directory {}: {}", dir, e)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/ls").to(list_dir))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
