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

pub fn get_executables_available_in_path() -> Vec<String> {
    let path = std::env::var("PATH");
    if let Err(_) = path {
        return Vec::new();
    }
    let path = path.unwrap();
    let paths= path.split(':');
    let mut files = Vec::new();

    for path in paths {
        let path = Path::new(&path);
        if !Path::is_dir(path) {
            continue
        }

        let read_dir = path.read_dir();
        if read_dir.is_err() {
            return Vec::new();
        }

        let read_dir = read_dir.unwrap();


        for file in read_dir {
            if file.is_err() {
                continue;
            }

            let file_name = file.unwrap().file_name().into_string();
            if let Ok(file_name) = file_name {
                files.push(file_name);
            }
        }
    }

    files
} 