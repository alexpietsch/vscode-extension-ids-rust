mod args;

use std::path::Path;
use std::io::Write;
use args::ExtensionsArgs;
use clap::Parser;

fn handle_path(file_path: &str) -> Option<&Path> {
    let path = Path::new(file_path);
    if path.exists() {
        return Some(path);
    } else {
        return None;
    }
}

fn main() -> std::result::Result<(), &'static str> {
    let args = ExtensionsArgs::parse();

    let path = handle_path(&args.path).unwrap_or_else(|| {
        println!("Error reading file");
        std::process::exit(1);
    });

    let file = std::fs::read_to_string(path).expect("Something went wrong reading the file");
    
    if file.to_string().len() > 0 {
        let file_data = file.parse::<serde_json::Value>().expect("Invalid JSON");

        let mut extensions: Vec<String> = Vec::new();

        for e in file_data.as_array().unwrap() {
            let ex_id: String = e["identifier"]["id"].as_str().unwrap().to_string();
            if args.with_prefix {
                extensions.push(format!("@id:{}", ex_id));
            } else {
                extensions.push(ex_id);
            }
        }

        match args.output_path {
            Some(path) => {
                let mut file = std::fs::File::create(format!("{}/extensions.txt", path)).expect("Unable to output create file");
                for e in extensions {
                    file.write_all(format!("{}\n", e).as_bytes()).expect("Unable to write data");
                }
                println!("File saved to {}/extensions.txt", path);
            },
            None => {
                for e in extensions {
                    println!("{}", e);
                }
            }
        }
    } else {
        println!("File is empty");
    }

    Ok(())
}
