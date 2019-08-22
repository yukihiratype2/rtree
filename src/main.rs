use std::path;
pub mod types;
mod render;
mod parser;

fn main() {
    let tree = parser::default::generate_entry_tree(path::Path::new("./")).expect("bala");
    render::tree::print_tree(&tree[..], vec![]);
}
