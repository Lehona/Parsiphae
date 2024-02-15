use glob::glob;
use std::io::Read;
use std::path;
use std::path::{Path, PathBuf};

use crate::errors::SrcError;

pub struct SrcParser;

impl SrcParser {
    /// Parses an SRC file and resolves all entries to concrete file paths
    /// (i.e. resolving globs)
    ///
    /// # Parameters
    /// `path`: Path to the .src file
    ///
    /// # Return
    /// List of files included via .src
    pub fn parse_src<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, SrcError> {
        let path = path.as_ref();
        let mut file = ::std::fs::File::open(&path)
            .map_err(|_| SrcError::new(format!("Unable to open file '{}'.", path.display())))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|_| SrcError::new(format!("Unable to read file '{}'.", path.display())))?;

        let paths = Self::collect_paths(contents, &path)?;

        Ok(paths)
    }

    fn collect_paths<P: AsRef<Path>>(content: String, path: P) -> Result<Vec<PathBuf>, SrcError> {
        let path = path.as_ref();
        let parent = match path
            .canonicalize()
            .map(|can| can.parent().map(|p| p.to_path_buf()))
            .map_err(|_| {
                SrcError::new(format!("Unable to canonicalize path '{}'.", path.display()))
            })? {
            Some(parent) => parent,
            None => {
                return Err(SrcError::new(format!(
                    "Unable to get parent of path '{}'.",
                    path.display()
                )))
            }
        };

        let dir = parent.to_string_lossy();

        let mut result_vector = Vec::new();

        let lines = content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.starts_with("//"))
            .filter(|line| !line.is_empty())
            .map(|line| format!("{}\\{}", dir, line))
            .collect::<Vec<String>>();

        for line in lines {
            let line_normalized = line.replace("\\", &path::MAIN_SEPARATOR.to_string());

            let entries = glob(&line_normalized)
                .map_err(|_| SrcError::new(format!("Invalid line in Src: {line_normalized}")))?
                .map(|entry| {
                    entry.map_err(|e| {
                        SrcError::new(format!("Error reading file: {}", e.path().display()))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            for entry in entries {
                let extension = match entry
                    .extension()
                    .map(|s| s.as_encoded_bytes().to_ascii_lowercase())
                {
                    None => {
                        log::warn!("Path '{}' has no extension, assuming '.d'.", path.display());
                        "d"
                    }
                    Some(ext) if ext == b"src" => "src",
                    Some(ext) if ext == b"d" => "d",
                    Some(_) => {
                        return Err(SrcError::new(format!(
                            "Path '{}' has invalid extension.",
                            path.display()
                        ))
                        .into())
                    }
                };

                match extension {
                    "src" => result_vector.extend(Self::parse_src(path)?),
                    "d" => result_vector.push(path.to_path_buf()),
                    _ => unreachable!(),
                }
            }
        }
        Ok(result_vector)
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::SrcError;

    use super::SrcParser;
    #[test]
    fn empty_src() -> Result<(), SrcError> {
        let input = String::new();
        let output = SrcParser::collect_paths(input, String::from("."))?;

        assert!(output.is_empty());
        Ok(())
    }

    // TODO: Write more tests :^) Need to point into repo because glob will attempt to resolve paths.
}
