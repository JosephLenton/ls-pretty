use ::std::io::{self, Result, Write};
use ::std::fs::DirEntry;
use ::itertools::{
    Itertools,
    EitherOrBoth,
};

///
/// print_dirs_files takes lots of arguments. So to make it a little easier to
/// read, this struct is created as a way to hold them.
///
pub struct PrintDirsFilesOptions {
    ///
    /// What to print at the start of each line.
    ///
    pub indent : &'static str,

    ///
    /// The width for the directory column.
    ///
    pub dirs_width : usize,

    pub directory_names : Vec<DirEntry>,
    pub file_names : Vec<DirEntry>,
}

const HIDDEN_DIRECTORY_COLOUR : &str = "\x1b[38;2;140;85;24m";
const STANDARD_DIRECTORY_COLOUR : &str = "\x1b[38;2;230;115;10m";

const HIDDEN_FILE_COLOUR : &str = "\x1b[38;2;30;150;30m";
const STANDARD_FILE_COLOUR : &str = "\x1b[38;2;60;230;60m";

const RESET_COLOUR : &str = "\x1b[0m";

///
/// Prints the list of directories, and list of files, given.
/// They are printed in two columns.
///
/// # Arguments
///
///  * `options` The options detailing what to print, and how.
///
pub fn print_dirs_files(
    options : PrintDirsFilesOptions,
) -> Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for pair in options.directory_names.into_iter().zip_longest(options.file_names) {
        write!(out, "{}", options.indent)?;
        print_pair(&mut out, pair, options.dirs_width)?;
        writeln!(out, "")?;
    }

    write!(out, "{}", RESET_COLOUR)?;

    Ok(())
}

fn print_pair(
    out: &mut dyn Write,
    pair: EitherOrBoth<DirEntry, DirEntry>,
    dirs_width: usize,
) -> Result<()> {
    match pair {
        EitherOrBoth::Both(directory, file) => {
            print_directory(out, directory, dirs_width)?;
            print_file(out, file)?;
        },
        EitherOrBoth::Left(directory) => {
            print_directory(out, directory, 0)?;
        },
        EitherOrBoth::Right(file) => {
            print_padding(out, dirs_width)?;
            print_file(out, file)?;
        },
    }

    Ok(())
}

fn print_directory(
    out: &mut dyn Write,
    entry: DirEntry,
    min_width: usize,
) -> Result<()> {
    print_entry_with_padding(
        out,
        entry,
        HIDDEN_DIRECTORY_COLOUR,
        STANDARD_DIRECTORY_COLOUR,
        min_width,
    )?;

    Ok(())
}

fn print_file(
    out: &mut dyn Write,
    entry: DirEntry,
) -> Result<()> {
    print_entry_with_padding(
        out,
        entry,
        HIDDEN_FILE_COLOUR,
        STANDARD_FILE_COLOUR,
        0,
    )?;

    Ok(())
}

fn print_entry_with_padding(
    out: &mut dyn Write,
    entry: DirEntry,
    hidden_colour: &'static str,
    standard_colour: &'static str,
    width: usize,
) -> Result<()> {
    let file_name = & entry.file_name();
    let file_name_str = & file_name.to_str().unwrap();

    if is_hidden_file(file_name_str) {
        write!(out, "{}{:width$}", hidden_colour, file_name_str, width = width)?;
    } else {
        write!(out, "{}{:width$}", standard_colour, file_name_str, width = width)?;
    }

    Ok(())
}

fn print_padding(
    out: &mut dyn Write,
    width: usize,
) -> Result<()> {
    write!(out, "{:width$}", "", width = width)?;

    Ok(())
}

pub fn is_hidden_file(
    file_name: &str,
) -> bool {
    file_name.chars().next() == Some('.')
}
