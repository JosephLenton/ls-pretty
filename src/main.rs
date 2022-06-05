//!
//! ls-pretty
//!
//! A prettier version of ls.
//!

#![deny(unused_extern_crates)]
#![warn(unused_import_braces)]

mod args;
mod dir_entry_kept_state;
mod palette;
mod print;
mod utils;

use ::std::cmp::Ordering;
use ::std::fs::read_dir;
use ::std::fs::DirEntry;
use ::std::io::Result as IOResult;
use ::std::path::Path;

use crate::args::Args;
use crate::print::print_dirs_files;
use crate::print::PrintDirsFilesOptions;

use crate::dir_entry_kept_state::dir_entry_kept_state;
use crate::dir_entry_kept_state::DirEntryKeptState;

fn main() -> IOResult<()> {
    let args = Args::new_from_args();
    let path = Path::new(&args.path);
    let mut file_names: Vec<DirEntry> = vec![];
    let mut directory_names: Vec<DirEntry> = vec![];

    // Build ls of files.
    for file in read_dir(path)? {
        let file = file?;

        match dir_entry_kept_state(&args, &file) {
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
        dirs_width: directory_name_output_width(&args, &directory_names),
        file_names,
        directory_names,
    })?;

    Ok(())
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
