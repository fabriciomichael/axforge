use crate::generator::project;

pub fn execute(name: String) {
    if let Err(error) = project::create(&name) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
