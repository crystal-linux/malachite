mod package;
mod repo;

pub fn build(pkg: String) {
    package::build(pkg);
}

pub fn generate() { repo::generate(); }