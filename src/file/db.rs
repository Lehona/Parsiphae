use crate::lexer::Token;
use std::path::PathBuf;
pub type FileId = usize;

pub struct FileDb {
    files: Vec<File>,
}

impl Default for FileDb {
    fn default() -> Self {
        Self::new()
    }
}

impl FileDb {
    pub fn new() -> Self {
        FileDb { files: Vec::new() }
    }
    pub fn add(&mut self, file: File) -> FileId {
        self.files.push(file);

        self.files.len() - 1
    }

    pub fn get(&self, file_id: FileId) -> &File {
        &self.files[file_id]
    }

    pub fn iter(&self) -> impl Iterator<Item = (FileId, &File)> {
        self.files.iter().enumerate()
    }
}

impl<'a> codespan_reporting::files::Files<'a> for FileDb {
    type FileId = FileId;
    type Name = String;
    type Source = String;

    fn name(&self, id: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        if id > self.files.len() {
            return Err(codespan_reporting::files::Error::FileMissing);
        }

        Ok(self.files[id].path.to_string_lossy().to_string())
    }

    fn source(&self, id: Self::FileId) -> Result<Self::Source, codespan_reporting::files::Error> {
        if id > self.files.len() {
            return Err(codespan_reporting::files::Error::FileMissing);
        }

        Ok(String::from_utf8_lossy(&self.files[id].content).to_string())
    }

    fn line_index(
        &self,
        id: Self::FileId,
        byte_index: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        Ok(self.files[id]
            .line_starts
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1))
    }

    fn line_range(
        &self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, codespan_reporting::files::Error> {
        let line_start = self.files[id].line_start(line_index)?;
        let next_line_start = self.files[id].line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }
}

pub struct File {
    pub path: PathBuf,
    content: Vec<u8>,
    source: String,
    pub tokens: Vec<Token>,
    line_starts: Vec<usize>,
}

impl File {
    pub fn new(path: PathBuf, content: Vec<u8>, tokens: Vec<Token>) -> Self {
        let source = String::from_utf8_lossy(&content).to_string();
        let line_starts = codespan_reporting::files::line_starts(&source).collect();

        File {
            path,
            content,
            source,
            tokens,
            line_starts,
        }
    }

    fn line_start(&self, line_index: usize) -> Result<usize, codespan_reporting::files::Error> {
        use std::cmp::Ordering;

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(self.line_starts.get(line_index).copied().unwrap()),
            Ordering::Equal => Ok(self.source.len()),
            Ordering::Greater => Err(codespan_reporting::files::Error::LineTooLarge {
                given: line_index,
                max: self.line_starts.len() - 1,
            }),
        }
    }
}
