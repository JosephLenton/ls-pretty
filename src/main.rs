
//!
//! ls-pretty
//!
//! A prettier version of ls.
//!

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

use ::std::io::Result;
use ::std::path::Path;
use ::std::fs::read_dir;
use ::std::fs::FileType;
use ::std::fs::DirEntry;
use ::std::cmp::Ordering;

use crate::args::Args;
use crate::print::is_hidden_file;
use crate::print::print_dirs_files;
use crate::print::PrintDirsFilesOptions;

mod args;
mod print;

fn main() {
    match run( & Args::new_from_args() ) {
        Ok(_) => {},
        Err(err) => { panic!("{}", err) },
    }
}

///
/// This is the real main. It runs, and returns any errors.
///
fn run<'a>(
    args : &'a Args,
) -> Result<()> {
    let path = Path::new( & args.path );
    let mut file_names : Vec<DirEntry> = vec![];
    let mut directory_names : Vec<DirEntry> = vec![];

    // Build ls of files.
    for file in read_dir( path )? {
        let file = file?;
        let file_type = & file.file_type().unwrap();
        let file_name = & file.file_name();
        let file_name_str = & file_name.to_str().unwrap();

        if !is_file_kept(args, file_type, file_name_str) {
            continue;
        }

        if file_type.is_file() {
            file_names.push(file);
        } else if file_type.is_dir() {
            directory_names.push(file);
        }
    }

    file_names.sort_by(file_name_sort);
    directory_names.sort_by(file_name_sort);

    print_dirs_files( PrintDirsFilesOptions {
        indent : "    ",
        dirs_width : directory_name_output_width(args, &directory_names),
        file_names,
        directory_names,
    })?;

    Ok(())
}

fn file_name_sort(
    a: &DirEntry,
    b: &DirEntry,
) -> Ordering {
    let a_name = & a.file_name().to_ascii_lowercase();
    let b_name = & b.file_name().to_ascii_lowercase();

    a_name.cmp(b_name)
}

fn is_file_kept(
    args: &Args,
    file_type: &FileType,
    file_name: &str,
) -> bool {
    let is_file_type_kept = file_type.is_file() || file_type.is_dir();
    let is_file_name_kept = args.all || !is_hidden_file(file_name);

    is_file_type_kept && is_file_name_kept
}

fn directory_name_output_width(
    args: &Args,
    directory_names: &Vec<DirEntry>,
) -> usize {
    let longest_dir_len = directory_names.iter().fold(0, |longest_len, entry| {
        let name = & entry.file_name();
        longest_len.max(name.len())
    }) + 1;
    longest_dir_len.max(args.dirs_width)
}
