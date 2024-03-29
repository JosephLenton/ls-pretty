use ::itertools::EitherOrBoth;
use ::itertools::Itertools;
use ::std::ffi::OsStr;
use ::std::fs::DirEntry;
use ::std::io::stdout;
use ::std::io::Result as IOResult;
use ::std::io::Write;

use crate::palette::Palette;
use crate::palette::DIRECTORIES_PALETTE;
use crate::palette::FILES_PALETTE;
use crate::utils::is_hidden_os_file;

///
/// print_dirs_files takes lots of arguments. So to make it a little easier to
/// read, this struct is created as a way to hold them.
///
pub struct PrintDirsFilesOptions {
    ///
    /// What to print at the start of each line.
    ///
    pub indent: &'static str,

    ///
    /// The width for the directory column.
    ///
    pub dirs_width: usize,

    pub directory_names: Vec<DirEntry>,
    pub file_names: Vec<DirEntry>,
}

const RESET_COLOUR: &str = "\x1b[0m";

///
/// Prints the list of directories, and list of files, given.
/// They are printed in two columns.
///
/// # Arguments
///
///  * `options` The options detailing what to print, and how.
///
pub fn print_dirs_files(options: PrintDirsFilesOptions) -> IOResult<()> {
    let stdout = stdout();
    let mut out = stdout.lock();

    for pair in options
        .directory_names
        .into_iter()
        .zip_longest(options.file_names)
    {
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
) -> IOResult<()> {
    match pair {
        EitherOrBoth::Both(directory, file) => {
            print_directory(out, directory, dirs_width)?;
            print_file(out, file)?;
        }
        EitherOrBoth::Left(directory) => {
            print_directory(out, directory, 0)?;
        }
        EitherOrBoth::Right(file) => {
            print_padding(out, dirs_width)?;
            print_file(out, file)?;
        }
    }

    Ok(())
}

fn print_directory(out: &mut dyn Write, entry: DirEntry, min_width: usize) -> IOResult<()> {
    print_entry_with_padding(out, entry, DIRECTORIES_PALETTE, min_width)?;

    Ok(())
}

fn print_file(out: &mut dyn Write, entry: DirEntry) -> IOResult<()> {
    print_entry_with_padding(out, entry, FILES_PALETTE, 0)?;

    Ok(())
}

fn print_entry_with_padding(
    out: &mut dyn Write,
    entry: DirEntry,
    palette: Palette,
    width: usize,
) -> IOResult<()> {
    let file_name = &entry.file_name();
    let file_name_str = &file_name.to_str().unwrap();

    let colour = calculate_colour(palette, &entry, &file_name);
    write!(out, "{}{:width$}", colour, file_name_str, width = width)?;

    Ok(())
}

fn calculate_colour(palette: Palette, entry: &DirEntry, file_name: &OsStr) -> &'static str {
    let is_sym_link = entry
        .file_type()
        .map(|file_type| file_type.is_symlink())
        .ok()
        .unwrap_or(false);
    let is_hidden = is_hidden_os_file(file_name);

    match (is_sym_link, is_hidden) {
        (true, true) => &palette.hidden_symlink_colour,
        (true, false) => &palette.standard_symlink_colour,
        (false, true) => &palette.hidden_colour,
        (false, false) => &palette.standard_colour,
    }
}

fn print_padding(out: &mut dyn Write, width: usize) -> IOResult<()> {
    write!(out, "{:width$}", "", width = width)?;

    Ok(())
}
