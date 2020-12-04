mod checksum;

fn main() {
    println!(
        "{}",
        checksum::hash_file(std::path::Path::new("Cargo.toml")).expect("asdf")
    );
}
