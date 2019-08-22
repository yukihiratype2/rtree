use std::path;
use std::fs;
pub struct Dir {
    pub path: path::PathBuf,
    pub metadata: fs::Metadata,
    pub entries: Entries,
}
pub struct File {
    pub path: path::PathBuf,
    pub metadata: fs::Metadata,
}
pub enum Entry {
    Dir(Dir),
    File(File),
}
pub type Entries = Vec<Entry>;

pub type LayerIsLast = Vec<bool>;