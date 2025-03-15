use std::path::Path;

pub fn search_file_in_path_envar(filename: &str) -> Option<String> {

    let path = std::env::var("PATH");
    match path {
        Ok(path) => {
            let paths= path.split(':');
            for path in paths {
                let path = path.to_owned() + "/" + filename;
                if Path::exists(Path::new(&path)) {
                    return Some(path);
                }
            }
            None
        },
        Err(_) => None
    }
}