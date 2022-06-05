//!
//! ls-pretty
//!
//! A prettier version of ls.
//!

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

use ::std::cmp::Ordering;
use ::std::fs::metadata;
use ::std::fs::read_dir;
use ::std::fs::DirEntry;
use ::std::io::Result;
use ::std::path::Path;

use crate::args::Args;
use crate::print::is_hidden_os_file;
use crate::print::print_dirs_files;
use crate::print::PrintDirsFilesOptions;

mod palette;
use self::palette::*;

mod args;
mod print;

fn main() {
    match run(&Args::new_from_args()) {
        Ok(_) => {}
        Err(err) => {
            panic!("{}", err)
        }
    }
}

///
/// This is the real main. It runs, and returns any errors.
///
fn run<'a>(args: &'a Args) -> Result<()> {
    let path = Path::new(&args.path);
    let mut file_names: Vec<DirEntry> = vec![];
    let mut directory_names: Vec<DirEntry> = vec![];

    // Build ls of files.
    for file in read_dir(path)? {
        let file = file?;

        match get_file_kept(args, &file) {
            Some(DirEntryKeptState::File) => {
                file_names.push(file);
            }
            Some(DirEntryKeptState::Dir) => {
                directory_names.push(file);
            }
            None => {}
        }
    }

    file_names.sort_by(file_name_sort);
    directory_names.sort_by(file_name_sort);

    print_dirs_files(PrintDirsFilesOptions {
        indent: "    ",
        dirs_width: directory_name_output_width(args, &directory_names),
        file_names,
        directory_names,
    })?;

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum DirEntryKeptState {
    File,
    Dir,
}

fn get_file_kept<'a>(args: &'a Args, file: &DirEntry) -> Option<DirEntryKeptState> {
    let file_name = &file.file_name();

    // Skip if we aren't included hidden files.
    if is_hidden_os_file(file_name) && !args.all {
        return None;
    }

    let file_type = &file.file_type().unwrap();
    if file_type.is_symlink() {
        let meta = metadata(file.path()).unwrap();

        if meta.is_dir() {
            Some(DirEntryKeptState::Dir)
        } else if meta.is_file() {
            Some(DirEntryKeptState::File)
        } else {
            None
        }
    } else if file_type.is_dir() {
        Some(DirEntryKeptState::Dir)
    } else if file_type.is_file() {
        Some(DirEntryKeptState::File)
    } else {
        // This will be something weird.
        None
    }
}

fn file_name_sort(a: &DirEntry, b: &DirEntry) -> Ordering {
    let a_name = &a.file_name().to_ascii_lowercase();
    let b_name = &b.file_name().to_ascii_lowercase();

    a_name.cmp(b_name)
}

fn directory_name_output_width(args: &Args, directory_names: &Vec<DirEntry>) -> usize {
    let longest_dir_len = directory_names.iter().fold(0, |longest_len, entry| {
        let name = &entry.file_name();
        longest_len.max(name.len())
    }) + 1;
    longest_dir_len.max(args.dirs_width)
}
