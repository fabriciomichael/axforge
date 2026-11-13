use crate::generator::project;

pub fn execute(name: String) {
    project::create(&name);
}
