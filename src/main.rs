
use std::io::Result;
use std::path::Path;
use std::fs::read_dir;
use std::cmp::max;

use structopt::StructOpt;

#[macro_use]
extern crate structopt_derive;
extern crate structopt;

/// 
/// We pre-allocate a vector to hold all files and directories found.
/// This is how big it will be.
/// 
const INITIAL_CAPACITY : usize = 20;

/// 
/// The structure of the commands for the app.
/// 
/// A large amount of this is generated by StructOpt.
/// See that project for how to write large amounts of this.
/// 
/// The gist however is that we make a struct that will hold
/// all of our arguments. Commands are then parsed, and then
/// turned into this struct.
/// 
#[derive(StructOpt, Debug)]
#[structopt(name = "ls-pretty", about = "Like ls, but pretty.")]
pub struct Args {
    /// Enable logging, use multiple `v`s to increase verbosity
    #[structopt(short = "a", long = "all", help = "Set to show all hidden files and directories.")]
    pub all: bool,

    /// Enable logging, use multiple `v`s to increase verbosity
    #[structopt(default_value = "0", short = "d", long = "directory-width", help = "Minimum width of the directory column.")]
    pub dirs_width: usize,

    #[structopt(default_value = ".", help = "Set to show all hidden files and directories.")]
    pub path: String,
}

fn main() {
    match run( & Args::from_args() ) {
        Ok(_) => {},
        Err(err) => { panic!(err) },
    }
}

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

    print_dirs_files( PrintDirFilesOptions {
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

pub struct PrintDirFilesOptions<'a> {
    args : &'a Args,
    indent : &'static str,
    dirs_width : usize,
    dirs  : String,
    files : String,
}

pub fn print_dirs_files<'a>(
    options : PrintDirFilesOptions<'a>,
) -> Result<()> {
    let include_hidden = options.args.all;
    let dirs  = options.dirs;
    let files = options.files;
    let dirs_width = max( options.dirs_width, options.args.dirs_width );

    let mut files_chars = files.chars();
    let mut dirs_chars = dirs.chars();

    let mut i = 0;
    let mut is_print_started = false;

    println!( "" );
    while let Some(c) = dirs_chars.next() {
        if ! is_print_started {
            if c == '.' {
                if ! include_hidden {
                    while let Some(c) = dirs_chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }

                    continue;
                }

                print!( "{}\x1b[38;2;140;85;24m", options.indent );
            } else {
                print!( "{}\x1b[38;2;230;115;10m", options.indent );
            }

            is_print_started = true
        }

        if c == '\n' {
            print!( "\x1b[0m" );

            // Write out the padding after the character.
            for _ in 0 .. (dirs_width-i) {
                print!( " " );
            }

            is_print_started = false;
            while let Some(c) = files_chars.next() {
                if ! is_print_started {
                    if c == '.' {
                        if ! include_hidden {
                            while let Some(c) = files_chars.next() {
                                if c == '\n' {
                                    break;
                                }
                            }

                            continue;
                        }

                        print!( "\x1b[38;2;30;150;30m" );
                    } else {
                        print!( "\x1b[38;2;60;230;60m" );
                    }

                    is_print_started = true
                }

                if c == '\n' {
                    break;
                }

                print!( "{}", c );
            }

            print!( "\x1b[0m" );
            print!( "\n" );

            i = 0;
            is_print_started = false
        } else {
            print!( "{}", c );

            i = i + 1;
        }
    }

    if is_print_started {
        print!( "\x1b[0m" );
    }

    // Print any remaining files.
    while let Some(c) = files_chars.next() {
        if ! is_print_started {
            if c == '.' {
                if ! include_hidden {
                    while let Some(c) = files_chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }

                    continue;
                }
            }

            print!( "{}", options.indent );

            // Write out the padding after the character.
            for _ in 0 .. dirs_width {
                print!( " " );
            }

            if c == '.' {
                print!( "\x1b[38;2;30;150;30m" );
            } else {
                print!( "\x1b[38;2;60;230;60m" );
            }

            is_print_started = true;
        }

        if c == '\n' {
            print!( "\x1b[0m" );
            is_print_started = false;
        }

        print!( "{}", c );
    }

    if is_print_started {
        print!( "\x1b[0m" );
    }

    print!( "\n" );

    Ok(())
}

