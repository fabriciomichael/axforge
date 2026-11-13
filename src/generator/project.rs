use std::fs;
use std::io;
use std::path::Path;

pub fn create(name: &str) -> io::Result<()> {
    let root = Path::new(name);

    if root.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Project directory '{}' already exists.", name),
        ));
    }

    create_directories(root)?;
    create_files(root, name)?;

    println!("Creating project '{}'...", name);
    println!("✓ Project directory");
    println!("✓ Source structure");
    println!("✓ Templates structure");
    println!("✓ Static structure");
    println!("✓ Migrations directory");
    println!("✓ Cargo.toml");
    println!("✓ main.rs");
    println!("✓ app.rs");
    println!("✓ routes.rs");
    println!("✓ controllers/mod.rs");
    println!("✓ controllers/home_controller.rs");

    Ok(())
}

fn create_directories(root: &Path) -> io::Result<()> {
    let directories = [
        "src",
        "src/controllers",
        "src/models",
        "src/services",
        "src/errors",
        "templates",
        "templates/layouts",
        "templates/pages",
        "templates/components",
        "static",
        "static/css",
        "static/js",
        "static/images",
        "migrations",
    ];

    for directory in directories {
        fs::create_dir_all(root.join(directory))?;
    }

    Ok(())
}

fn create_files(root: &Path, name: &str) -> io::Result<()> {
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = {{ version = "1", features = ["full"] }}
tera = "1"
serde = {{ version = "1", features = ["derive"] }}
"#,
        name
    );

    fs::write(root.join("Cargo.toml"), cargo_toml)?;

    let main_rs = r#"mod app;
mod routes;
mod controllers;

#[tokio::main]
async fn main() {
    let app = app::create_app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
"#;

    fs::write(root.join("src/main.rs"), main_rs)?;

    let app_rs = r#"use axum::Router;

pub fn create_app() -> Router {
    crate::routes::create_routes()
}
"#;

    fs::write(root.join("src/app.rs"), app_rs)?;

    let routes_rs = r#"use axum::{routing::get, Router};

use crate::controllers::home_controller;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(home_controller::index))
}
"#;

    fs::write(root.join("src/routes.rs"), routes_rs)?;

    let controllers_mod = r#"pub mod home_controller;
"#;

    fs::write(root.join("src/controllers/mod.rs"), controllers_mod)?;

    let home_controller = r#"pub async fn index() -> &'static str {
    "Hello from Axforge!"
}
"#;

    fs::write(
        root.join("src/controllers/home_controller.rs"),
        home_controller,
    )?;

    Ok(())
}
