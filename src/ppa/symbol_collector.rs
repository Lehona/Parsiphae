use crate::file::db::FileId;
use crate::ppa::visitor::*;
use crate::types::parsed::{zPAR_TYPE, Symbol, SymbolKind};
use crate::types::{self, External, Identifier, Instance, VarDeclaration};

#[derive(Debug, Default)]
pub struct SymbolCollector {
    pub next_id: usize,
    pub file_id: FileId,
    pub syms: Vec<Symbol>,
}

impl SymbolCollector {
    pub fn new() -> Self {
        let syms = vec![Symbol {
            file_id: 0,
            id: 0,
            kind: SymbolKind::Inst(Instance {
                name: Identifier::new(b"\xFFinstance_help", (0, 0)),
                class: Identifier::new(b"xFFinstance_help", (0, 0)),
                body: vec![],
                span: (0, 0),
            }),
        }];

        SymbolCollector {
            next_id: 1,
            file_id: 0,
            syms,
        }
    }

    fn type_to_ident(typ: &zPAR_TYPE) -> Identifier {
        match typ {
            zPAR_TYPE::Void => Identifier::new(b"VOID", (0, 0)),
            zPAR_TYPE::Int => Identifier::new(b"INT", (0, 0)),
            zPAR_TYPE::Float => Identifier::new(b"FLOAT", (0, 0)),
            zPAR_TYPE::String => Identifier::new(b"STRING", (0, 0)),
            zPAR_TYPE::Func => Identifier::new(b"FUNC", (0, 0)),
            zPAR_TYPE::Instance(ident) => ident.clone(),
        }
    }

    pub(crate) fn add_externals(&mut self, gothic2_externals: &[External]) {
        for external in gothic2_externals {
            let symbol = Symbol {
                id: self.next_id,
                file_id: 0,
                kind: SymbolKind::External(external.clone()),
            };

            self.syms.push(symbol);
            self.next_id += 1;

            for (idx, param) in external.parameters.iter().enumerate() {
                let param_name = format!("par{idx}").into_bytes();
                let param_symbol = Symbol {
                    id: self.next_id,
                    file_id: 0,
                    kind: SymbolKind::Var(
                        VarDeclaration {
                            typ: Self::type_to_ident(param),
                            name: Identifier::new(&param_name, (0, 0)),
                            array_size: None,
                            span: (0, 0),
                        },
                        Some(Identifier::new(&external.name.0, (0, 0))),
                    ),
                };
                self.syms.push(param_symbol);
                self.next_id += 1;
            }
        }
    }
}

impl VisitorMut for SymbolCollector {
    fn visit_var_decl(&mut self, decl: &types::VarDeclaration, scope: Option<&types::Identifier>) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            id: self.next_id,
            kind: (decl.clone(), scope.cloned()).into(),
        });
        self.next_id += 1;
    }

    fn visit_func_decl(&mut self, decl: &types::Function) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            id: self.next_id,
            kind: decl.clone().into(),
        });
        self.next_id += 1;
    }

    fn visit_class_decl(&mut self, decl: &types::Class) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            id: self.next_id,
            kind: decl.clone().into(),
        });
        self.next_id += 1;
    }

    fn visit_inst_decl(&mut self, decl: &types::Instance) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            id: self.next_id,
            kind: decl.clone().into(),
        });
        self.next_id += 1;
    }

    fn visit_proto_decl(&mut self, decl: &types::Prototype) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            id: self.next_id,
            kind: decl.clone().into(),
        });
        self.next_id += 1;
    }

    fn visit_const_decl(
        &mut self,
        decl: &types::ConstDeclaration,
        scope: Option<&types::Identifier>,
    ) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            id: self.next_id,
            kind: (decl.clone(), scope.cloned()).into(),
        });
        self.next_id += 1;
    }

    fn visit_const_arr_decl(
        &mut self,
        decl: &types::ConstArrayDeclaration,
        scope: Option<&types::Identifier>,
    ) {
        self.syms.push(Symbol {
            file_id: self.file_id,
            id: self.next_id,
            kind: (decl.clone(), scope.cloned()).into(),
        });
        self.next_id += 1;
    }
}
