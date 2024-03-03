use crate::file::db::FileId;
use codespan_reporting::diagnostic::Label as LabelReporting;
use derive_builder::Builder;

#[derive(Builder, Clone)]
#[builder(pattern = "owned", setter(into))]
pub struct Diagnostic {
    pub message: String,
    pub code: String,
    #[builder(setter(each(name = "label")))]
    pub labels: Vec<Label>,
}

#[derive(Clone)]
pub struct Label {
    pub file_id: FileId,
    pub span: (usize, usize),
    pub message: String,
    pub primary: bool,
}

impl Label {
    pub fn to_report(self) -> LabelReporting<FileId> {
        if self.primary {
            LabelReporting::primary(self.file_id, self.span.0..self.span.1)
        } else {
            LabelReporting::secondary(self.file_id, self.span.0..self.span.1)
        }
        .with_message(self.message)
    }
}
