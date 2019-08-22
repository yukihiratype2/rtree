use crate::types;
use std::fs;
use std::path;

pub fn generate_entry_tree(path: &path::Path) -> std::io::Result<types::Entries> {
    let mut entries: types::Entries = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                entries.push(types::Entry::Dir(types::Dir {
                    path: entry.path(),
                    metadata: entry.metadata()?,
                    entries: generate_entry_tree(&entry.path())?
                }))
            } else { 
                entries.push(types::Entry::File(types::File {
                    metadata: entry.metadata()?,
                    path: entry.path(),
                }))
            }
        }
    }
    Ok(entries)
}