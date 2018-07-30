use errors::*;
use glob::glob;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn parse_src<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let mut file = ::std::fs::File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let paths = collect_paths(contents, &path)?;

    Ok(paths)
}

fn collect_paths<P: AsRef<Path>>(content: String, path: P) -> Result<Vec<PathBuf>> {
    let path = path.as_ref();
    let parent = parent_dir(path)?;
    let dir = parent.to_string_lossy();

    let mut vec = Vec::new();

    let lines = content
        .lines()
        .filter_map(|line| fix_line(line))
        .map(|line| format!("{}\\{}", dir, line))
        .collect::<Vec<String>>();

    for line in lines {
        for entry in glob(&line).unwrap() {
            match entry {
                Ok(path) => {
                    let extension = {
                        let extension = match path.extension() {
                            None => {
                                println!(
                                    "invalid extension of path {:?}\nline: {}\ndir: {}",
                                    &path, &line, &dir
                                );
                                ::std::ffi::OsStr::new("abc")
                            }
                            Some(ext) => ext,
                        };
                        extension
                            .to_str()
                            .expect("found invalid file name")
                            .to_owned()
                            .to_uppercase()
                    };

                    match extension.as_ref() {
                        "D" => {
                            vec.push(path);
                        }
                        "SRC" => {
                            let inner_vec = parse_src(path)?;
                            vec.extend(inner_vec.into_iter());
                        }
                        other => {
                            println!("invalid extension {} in path {:?}", other, path);
                            return Err(::std::io::Error::new(
                                ::std::io::ErrorKind::InvalidData,
                                "Invalid extension",
                            ).into());
                        }
                    }
                }

                // if the path matched but was unreadable,
                // thereby preventing its contents from matching
                Err(e) => println!("{:?}", e),
            }
        }
    }

    Ok(vec)
}

fn parent_dir(path: &Path) -> Result<PathBuf> {
    let parent_path = if path.is_relative() {
        let mut absolute_path = ::std::env::current_dir()?;
        absolute_path.push(path);
        absolute_path.parent().map(|p| p.to_owned())
    } else {
        path.parent().map(|p| p.to_owned())
    };

    parent_path.ok_or(
        ::std::io::Error::new(
            ::std::io::ErrorKind::NotFound,
            "Unable to get parent directory",
        ).into(),
    )
}

fn fix_line(line: &str) -> Option<&str> {
    let fixed = line.split("//").next().unwrap().trim();
    if fixed.is_empty() {
        return None;
    }

    Some(fixed)
}
