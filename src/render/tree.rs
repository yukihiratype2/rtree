use crate::types;
use colored::*;

pub fn print_tree(entries: &[types::Entry], layer_is_last: types::LayerIsLast) {
    let last_entry = &entries.last().unwrap();
    let entries = &entries[..entries.len() - 1];
    for entry in entries {
        print_tree_line(entry, &layer_is_last);
        match entry {
            types::Entry::Dir(dir) => {
                if dir.entries.len() > 0 {
                    let mut sub_layer = layer_is_last.clone();
                    sub_layer.push(false);
                    print_tree(&dir.entries[..], sub_layer);
                }
            },
            types::Entry::File(_) => {},
        }
    }

    print_tree_line_last(last_entry, &layer_is_last);
    match last_entry {
        types::Entry::Dir(dir) => {
            if dir.entries.len() > 0 {
                let mut sub_layer = layer_is_last.clone();
                sub_layer.push(true);
                print_tree(&dir.entries[..], sub_layer);
            }
        },
        types::Entry::File(_) => {},
    }
}

fn print_tree_line(entry: &types::Entry, layer_is_last: &types::LayerIsLast) {
    for layer in layer_is_last {
        match layer {
            true => print!("     "),
            false => print!("│    "),
        }
    }
    match entry {
        types::Entry::Dir(dir) => println!("├─── {}",dir.path.file_name().unwrap().to_str().unwrap().green()),
        types::Entry::File(file) => println!("├─── {}",  file.path.file_name().unwrap().to_str().unwrap().blue()),
    }
}
fn print_tree_line_last(entry: &types::Entry, layer_is_last: &types::LayerIsLast) {
    for layer in layer_is_last {
        match layer {
            true => print!("     "),
            false => print!("│    "),
        }
    }
    match entry {
        types::Entry::Dir(dir) => println!("└─── {}",  dir.path.file_name().unwrap().to_str().unwrap().green()),
        types::Entry::File(file) => println!("└─── {}", file.path.file_name().unwrap().to_str().unwrap().blue()),
    }
}