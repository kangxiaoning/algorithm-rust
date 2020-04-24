mod sorting;
mod tree;
mod util;

use std::path::Path;

fn main() {
    sorting::selection::run();
    sorting::insertion::run();
    tree::bst::run();

    let path = Path::new("./src/main.rs");
    let mut words = Vec::new();
    match util::read_file(path, &mut words) {
        Ok(()) => println!("{:?}", words),
        Err(e) => println!("{:?}", e),
    }
}
