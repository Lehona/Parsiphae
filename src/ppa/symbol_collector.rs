use crate::file::db::FileId;
use crate::ppa::visitor::*;
use crate::types;
use crate::types::parsed::Symbol;

#[derive(Debug, Default)]
pub struct SymbolCollector {
    pub file_id: FileId,
    pub syms: Vec<Symbol>,
}

impl SymbolCollector {
    pub fn new() -> Self {
        SymbolCollector {
            ..Default::default()
        }
    }
}

impl VisitorMut for SymbolCollector {
    fn visit_var_decl(&mut self, decl: &types::VarDeclaration, scope: Option<&types::Identifier>) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            kind: (decl.clone(), scope.cloned()).into()
        });
    }

    fn visit_func_decl(&mut self, decl: &types::Function) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            kind: decl.clone().into()
        });
    }

    fn visit_class_decl(&mut self, decl: &types::Class) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            kind: decl.clone().into()
        });
    }

    fn visit_inst_decl(&mut self, decl: &types::Instance) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            kind: decl.clone().into()
        });
    }

    fn visit_proto_decl(&mut self, decl: &types::Prototype) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            kind: decl.clone().into()
        });
    }

    fn visit_const_decl(
        &mut self,
        decl: &types::ConstDeclaration,
        scope: Option<&types::Identifier>,
    ) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            kind: (decl.clone(), scope.cloned()).into()
        });
    }

    fn visit_const_arr_decl(
        &mut self,
        decl: &types::ConstArrayDeclaration,
        scope: Option<&types::Identifier>,
    ) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            kind: (decl.clone(), scope.cloned()).into()
        });
    }
}
