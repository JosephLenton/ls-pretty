
//!
//! ls-pretty
//!
//! A prettier version of ls.
//!

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]

use std::io::Result;
use std::path::Path;
use std::fs::read_dir;

#[macro_use]
extern crate structopt_derive;
extern crate structopt;

use args::Args;
use print::print_dirs_files;
use print::PrintDirsFilesOptions;

mod args;
mod print;

///
/// We pre-allocate a vector to hold all files and directories found.
/// This is how big it will be.
///
const INITIAL_CAPACITY : usize = 750;

fn main() {
    match run( & Args::new_from_args() ) {
        Ok(_) => {},
        Err(err) => { panic!(err) },
    }
}

///
/// This is the real main. It runs, and returns any errors.
///
fn run<'a>(
    args : &'a Args,
) -> Result<()> {
    let path = Path::new( & args.path );
    let mut files = String::with_capacity( INITIAL_CAPACITY );
    let mut dirs = String::with_capacity( INITIAL_CAPACITY );
    let mut longest_dir = 0;

    // Build ls of files.
    for file in read_dir( path )? {
        let file = file?;
        let file_type = & file.file_type().unwrap();
        let file_name = & file.file_name();
        let file_name_str = & file_name.to_str().unwrap();

        if file_type.is_file() {
            files.push_str( & file_name_str );
            files.push( '\n' );
        } else if file_type.is_dir() {
            dirs.push_str( & file_name_str );
            dirs.push( '\n' );

            let len = file_name_str.len();
            if len > longest_dir {
                longest_dir = len
            }
        }
    }

    print_dirs_files( PrintDirsFilesOptions {
        args : & args,

        indent : "    ",

        dirs_width : longest_dir + 1,
        dirs  : dirs,
        files : files,
    })?;

    // format files into string
    // output

    Ok(())
}

