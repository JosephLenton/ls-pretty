
use std::io::Result;
use std::cmp::max;

use args::Args;

/// 
/// print_dirs_files takes lots of arguments. So to make it a little easier to
/// read, this struct is created as a way to hold them.
///
pub struct PrintDirsFilesOptions<'a> {

    /// 
    /// The arguments which were passed to the main application.
    ///
    pub args : &'a Args,

    /// 
    /// What to print at the start of each line.
    ///
    pub indent : &'static str,

    /// 
    /// The width for the directory column.
    ///
    /// Note this isn't advised, or minimum, or anything like that.
    /// This is the actual width it will use.
    /// 
    pub dirs_width : usize,

    /// 
    /// An end of line seperated list of directories.
    ///
    pub dirs  : String,

    /// 
    /// An end of line seperated list of files.
    ///
    pub files : String,

}

/// 
/// Prints the list of directories, and list of files, given.
/// They are printed in two columns.
///
/// # Arguments
///
///  * `options` The options detailing what to print, and how.
///
pub fn print_dirs_files<'a>(
    options : PrintDirsFilesOptions<'a>,
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

