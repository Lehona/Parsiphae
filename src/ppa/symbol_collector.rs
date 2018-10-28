use ppa::visitor::*;
use std::collections::HashMap;
use types;
use types::PrintableByteVec;

#[derive(Debug)]
pub struct ClassCollector {
    class_defs: HashMap<PrintableByteVec, /*&'a*/ types::Class>,
}

impl ClassCollector {
    pub fn new() -> Self {
        ClassCollector {
            class_defs: HashMap::new(),
        }
    }
}

impl VisitorMut for ClassCollector {
    fn visit_class_decl(&mut self, decl: &types::Class) {
        self.class_defs.insert(
            PrintableByteVec(decl.name.as_bytes().to_vec()),
            decl.clone(),
        );
    }
}
