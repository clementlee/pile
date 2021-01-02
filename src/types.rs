use std::path::Path;

struct StorageLocation {
    mountpoint: String,
}
/// A file is he
#[derive(Debug)]
struct File {
    path: Box<Path>,
    hash: String,
    size: u64,
}
