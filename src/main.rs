use structopt::StructOpt;

pub mod types;
mod render;
mod parser;
mod cmd;

fn main() {
    let opt = cmd::Opt::from_args();
    let tree = parser::default::generate_entry_tree(&opt.path).expect("failed to read dir");
    render::tree::print_tree(&tree[..], vec![]);
}
