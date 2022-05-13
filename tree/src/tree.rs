use std::path;

pub fn list(target: &path::PathBuf, level: isize) {
    let files = target.read_dir().expect("Directory does not exist");

    for entry in files {
        let path = entry.unwrap().path();
        for _ in 0..level {
            print!("|  ");
        }
        let filename = path.file_name().unwrap().to_string_lossy();

        if path.is_dir() {
            println!("|-- {}/", filename);
            list(&path, level + 1);
        } else {
            println!("|-- {}", filename);
        }
    }
}
