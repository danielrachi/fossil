use std::{
    fs::{self, read_dir},
    io,
    path::PathBuf,
};

/// Get the path of all the files in a given folder.
fn get_paths(dir: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let paths_with_dirs: Vec<PathBuf> = read_dir(dir)?.map(|entry| entry.unwrap().path()).collect();
    for path in paths_with_dirs {
        if path.is_file() {
            paths.push(path);
        } else if path.is_dir() {
            let dir_paths = get_paths(path)?;
            for path in dir_paths {
                paths.push(path);
            }
        }
    }
    Ok(paths)
}

fn main() {
    let paths = get_paths(PathBuf::from("content")).unwrap();
    let _: Vec<_> = paths
        .into_iter()
        .map(|path| {
            println!(
                "path: {:?} \n contents:{:?}",
                &path,
                fs::read_to_string(&path)
            )
        })
        .collect();
}
