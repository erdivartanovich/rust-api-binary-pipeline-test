// run with providing environment variable
// 1. QUERY_STRING="dir=%2Fvar%2Ftmp" ./list_dir
// 2. Or Using Pipe
// echo "dir=%2Fvar%2Ftmp" | { read -r QUERY_STRING; QUERY_STRING=$QUERY_STRING ./list_dir; }

use std::env;
use std::fs;
use urlencoding::decode;

fn main() {
    // Read QUERY_STRING from environment variable
    let query_string = match env::var("QUERY_STRING") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: QUERY_STRING not found");
            return;
        }
    };

    // Parse the query string
    let pairs: Vec<&str> = query_string.split('&').collect();
    let mut dir = String::new();

    for pair in pairs {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0];
            let value = decode(parts[1]).unwrap_or_else(|_| parts[1].to_string().into());

            if key == "dir" {
                dir = value.to_string();
            }
        }
    }

    // Validate and process the directory
    if dir.is_empty() {
        eprintln!("400: param \"dir\" not provided in the query string");
        return;
    }

    // List the contents of the specified directory
    match fs::read_dir(&dir) {
        Ok(entries) => {
            let file_names: Vec<String> = entries
                .filter_map(|entry| entry.ok().map(|e| e.file_name()))
                .map(|name| name.to_string_lossy().to_string())
                .collect();

            // Print the list of file names to standard output
            println!("results:\n");
            for file_name in file_names {
                println!("{}", file_name);
            }
        }
        Err(e) => {
            eprintln!("Error listing directory {}: {}", dir, e);
        }
    }
}
