use std::path;
use std::fs;
use colored::*;

#[derive(Debug)]
struct Dir {
    path: path::PathBuf,
    metadata: fs::Metadata,
    entries: Entries,
}

#[derive(Debug)]
struct File {
    path: path::PathBuf,
    metadata: fs::Metadata,
}

#[derive(Debug)]
enum Entry {
    Dir(Dir),
    File(File),
}

type Entries = Vec<Entry>;

type LayerIsLast = Vec<bool>;

fn generate_entry_tree(path: &path::Path) -> std::io::Result<Entries> {
    let mut entries: Entries = vec![];
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().is_dir() {
                entries.push(Entry::Dir(Dir {
                    path: entry.path(),
                    metadata: entry.metadata()?,
                    entries: generate_entry_tree(&entry.path())?
                }))
            } else { 
                entries.push(Entry::File(File {
                    metadata: entry.metadata()?,
                    path: entry.path(),
                }))
            }
        }
    }
    Ok(entries)
}

fn print_tree(entries: &[Entry], layer_is_last: LayerIsLast) {
    let last_entry = &entries.last().unwrap();
    let entries = &entries[..entries.len() - 1];
    for entry in entries {
        print_tree_line(entry, &layer_is_last);
        match entry {
            Entry::Dir(dir) => {
                if dir.entries.len() > 0 {
                    let mut sub_layer = layer_is_last.clone();
                    sub_layer.push(false);
                    print_tree(&dir.entries[..], sub_layer);
                }
            },
            Entry::File(_) => {},
        }
    }

    print_tree_line_last(last_entry, &layer_is_last);
    match last_entry {
        Entry::Dir(dir) => {
            if dir.entries.len() > 0 {
                let mut sub_layer = layer_is_last.clone();
                sub_layer.push(true);
                print_tree(&dir.entries[..], sub_layer);
            }
        },
        Entry::File(_) => {},
    }
}

fn print_tree_line(entry: &Entry, layer_is_last: &LayerIsLast) {
    for layer in layer_is_last {
        match layer {
            true => print!("     "),
            false => print!("│    "),
        }
    }
    match entry {
        Entry::Dir(dir) => println!("├─── {}",dir.path.file_name().unwrap().to_str().unwrap().green()),
        Entry::File(file) => println!("├─── {}",  file.path.file_name().unwrap().to_str().unwrap().blue()),
    }
}
fn print_tree_line_last(entry: &Entry, layer_is_last: &LayerIsLast) {
    for layer in layer_is_last {
        match layer {
            true => print!("     "),
            false => print!("│    "),
        }
    }
    match entry {
        Entry::Dir(dir) => println!("└─── {}",  dir.path.file_name().unwrap().to_str().unwrap().green()),
        Entry::File(file) => println!("└─── {}", file.path.file_name().unwrap().to_str().unwrap().blue()),
    }
}

fn main() {
    let tree = generate_entry_tree(path::Path::new("./")).expect("bala");
    print_tree(&tree[..], vec![]);
}
