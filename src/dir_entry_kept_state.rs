use ::std::fs::metadata;
use ::std::fs::DirEntry;

use crate::args::Args;
use crate::utils::is_hidden_os_file;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DirEntryKeptState {
    File,
    Dir,
}

pub fn dir_entry_kept_state<'a>(args: &'a Args, file: &DirEntry) -> Option<DirEntryKeptState> {
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
