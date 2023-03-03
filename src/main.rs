use std::{
    fs::{self, read_dir},
    io,
    path::PathBuf,
};

use serde::Serialize;

use std::error::Error;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
    title: String,
}

static TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <h1>{title}</h1>
</body>
</html>
"#;

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

fn main() -> Result<(), Box<dyn Error>> {
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

    let mut tt = TinyTemplate::new();
    tt.add_template("hello", TEMPLATE)?;
    let context = Context {
        title: "Daniel".to_string(),
    };
    let rendered = tt.render("hello", &context)?;
    fs::write("out/title.html", rendered)?;

    Ok(())
}
