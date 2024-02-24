use glob::glob;
use std::io::Read;

use std::path::{Path, PathBuf};

use crate::errors::{PipelineFailure, SrcError};

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
    pub fn parse_src<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, PipelineFailure> {
        let path = path.as_ref();
        let mut file = ::std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let paths = Self::collect_paths(contents, path)?;

        Ok(paths)
    }

    fn collect_paths<P: AsRef<Path>>(
        content: String,
        path: P,
    ) -> Result<Vec<PathBuf>, PipelineFailure> {
        let path = path.as_ref();
        let parent = match path
            .canonicalize()
            .map(|can| can.parent().map(|p| p.to_path_buf()))?
        {
            Some(parent) => parent,
            None => {
                return Err(SrcError::new(format!(
                    "Unable to get parent of path '{}'.",
                    path.display()
                ))
                .into())
            }
        };

        // This prefix prevents glob() from working
        let dir = parent
            .display()
            .to_string()
            .trim_start_matches("\\\\?\\")
            .to_string();

        let mut result_vector = Vec::new();

        let lines = content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.starts_with("//"))
            .filter(|line| !line.is_empty())
            .map(|line| format!("{}\\{}", dir, line))
            .collect::<Vec<String>>();

        for line in lines {
            let line_normalized = line.replace('\\', std::path::MAIN_SEPARATOR_STR);

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
                        log::warn!(
                            "Path '{}' has no extension, assuming '.d'.",
                            entry.display()
                        );
                        "d"
                    }
                    Some(ext) if ext == b"src" => "src",
                    Some(ext) if ext == b"d" => "d",
                    Some(_) => {
                        return Err(SrcError::new(format!(
                            "Path '{}' has invalid extension.",
                            entry.display()
                        ))
                        .into())
                    }
                };

                let new_paths = match extension {
                    "src" => Self::parse_src(entry)?,
                    "d" => vec![entry.to_path_buf()],
                    _ => unreachable!(),
                };

                for new_path in new_paths {
                    if !result_vector.contains(&new_path) {
                        result_vector.push(new_path);
                    }
                }
            }
        }
        Ok(result_vector)
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::PipelineFailure;

    use super::SrcParser;
    #[test]
    fn empty_src() -> Result<(), PipelineFailure> {
        let input = String::new();
        let output = SrcParser::collect_paths(input, String::from("."))?;

        assert!(output.is_empty());
        Ok(())
    }

    // TODO: Write more tests :^) Need to point into repo because glob will attempt to resolve paths.
}
