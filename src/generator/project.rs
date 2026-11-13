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

    Ok(())
}
