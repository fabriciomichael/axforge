pub fn validate(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Project name cannot be empty.".to_string());
    }

    Ok(())
}
