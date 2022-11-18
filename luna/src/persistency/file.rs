use std::fs;
use std::path;

pub fn does_file_exists(path: &str) -> bool {
    let path = path::Path::new(path);
    path.exists() && path.is_file()
}

pub fn create_file_if_not_exists(path_str: &str) -> fs::File {
    if !does_file_exists(path_str) {
        return create_file(path_str);
    } else {
        return open_file(path_str, false);
    }
}

pub fn create_trucatable_file_if_not_exists(path_str: &str, truncate: bool) -> fs::File {
    if !does_file_exists(path_str) {
        return create_file(path_str);
    } else {
        return open_file(path_str, truncate);
    }
}

fn create_file(path_str: &str) -> fs::File {
    let path = path::Path::new(path_str);
    let created_file = match fs::File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    return created_file;
}

fn open_file(path_str: &str, truncate: bool) -> fs::File {
    let path = path::Path::new(path_str);
    if let Ok(file) = fs::OpenOptions::new().read(true).write(true).truncate(truncate).open(path) {
        return file;
    } else {
        panic!("couldn't open {}", path.display());
    }
}
