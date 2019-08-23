use structopt::StructOpt;
use std::path;

#[derive(Debug, StructOpt)]
#[structopt(name = "rtree", about = "tree command with color enabled by default")]
pub struct Opt {
    #[structopt(parse(from_os_str), default_value = "./")]
    pub path: path::PathBuf,
}
