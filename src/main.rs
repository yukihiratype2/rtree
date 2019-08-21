use std::path;
use std::fs;

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


fn main() {
    let tree = generate_entry_tree(path::Path::new("./")).expect("bala");
    print_tree(&tree, 1);
}

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

fn print_tree(entries: &Entries, layer: usize) {
    for (i, entry) in entries.iter().enumerate() {
        if i == entries.len() - 1 {
            print!("{:│>1$} {}", "└", layer);
        } else {
            print!("{:│>1$} {}", "├", layer);
        }
        match entry {
            Entry::Dir(v) => {
                println!("dir {}", v.path.display());
                print_tree(&v.entries, layer + 1);
            },
            Entry::File(v) => {
                println!("file {:?}", v.path.display())
            }
        }
    }
}
