use ::std::ffi::OsStr;

pub fn is_hidden_os_file(file_name: &OsStr) -> bool {
    file_name.to_str().map(is_hidden_file).unwrap_or(false)
}

pub fn is_hidden_file(file_name: &str) -> bool {
    file_name.chars().next() == Some('.')
}
