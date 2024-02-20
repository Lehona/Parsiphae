use crate::{file::db::FileId, types};

#[derive(Debug, Clone, PartialEq, From)]
pub enum SymbolKind {
    Var(types::VarDeclaration, Option<types::Identifier>),
    Func(types::Function),
    Class(types::Class),
    Inst(types::Instance),
    Proto(types::Prototype),
    Const(types::ConstDeclaration, Option<types::Identifier>),
    ConstArray(types::ConstArrayDeclaration, Option<types::Identifier>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub file_id: FileId,
    pub kind: SymbolKind,
}

impl SymbolKind {
    pub fn typ(&self) -> zPAR_TYPE {
        use self::SymbolKind::*;

        match *self {
            Var(ref decl, _) => zPAR_TYPE::from_ident(&decl.typ),
            Func(ref func) => zPAR_TYPE::from_ident(&func.typ),
            Class(ref class) => zPAR_TYPE::from_ident(&class.name),
            Inst(ref inst) => {
                // TODO implement recursive lookup of instance type (in case of prototype)
                zPAR_TYPE::from_ident(&inst.class)
            }
            Proto(ref proto) => zPAR_TYPE::from_ident(&proto.class),
            Const(ref constant, _) => zPAR_TYPE::from_ident(&constant.typ),
            ConstArray(ref constant, _) => zPAR_TYPE::from_ident(&constant.typ),
        }
    }

    pub fn name(&self) -> Vec<u8> {
        use self::SymbolKind::*;

        let full_name = match *self {
            Var(ref decl, ref scope) => {
                let name = &decl.name;
                if let Some(scope) = scope {
                    let mut bytes = scope.as_bytes().to_vec();
                    bytes.push(b'.');
                    bytes.extend_from_slice(name.as_bytes());
                    bytes
                } else {
                    name.name.to_vec()
                }
            }
            Func(ref func) => func.name.to_vec(),
            Class(ref class) => class.name.to_vec(),
            Inst(ref inst) => inst.name.to_vec(),
            Proto(ref proto) => proto.name.to_vec(),
            Const(ref decl, ref scope) => {
                let name = &decl.name;
                if let Some(scope) = scope {
                    let mut bytes = scope.as_bytes().to_vec();
                    bytes.push(b'.');
                    bytes.extend_from_slice(name.as_bytes());
                    bytes
                } else {
                    name.name.to_vec()
                }
            }
            ConstArray(ref decl, ref scope) => {
                let name = &decl.name;
                if let Some(scope) = scope {
                    let mut bytes = scope.as_bytes().to_vec();
                    bytes.push(b'.');
                    bytes.extend_from_slice(name.as_bytes());
                    bytes
                } else {
                    name.name.to_vec()
                }
            }
        };

        full_name
    }

    /// Retrieve the span of a symbol
    /// For bigger symbols, this will return the span of the name instead.
    pub fn span(&self) -> (usize, usize) {
        use self::SymbolKind::*;
        match self {
            Var(ref decl, _) => decl.span,
            Func(ref func) => func.name.span,
            Class(ref class) => class.name.span,
            Inst(ref inst) => inst.span,
            Proto(ref proto) => proto.name.span,
            Const(ref constant, _) => constant.span,
            ConstArray(ref constant, _) => constant.span,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum zPAR_TYPE {
    Void,
    Int,
    Float,
    String,
    Func,
    Instance(types::Identifier),
}

impl zPAR_TYPE {
    pub fn from_ident(ident: &types::Identifier) -> Self {
        let ident_b = ident.as_bytes();

        if ident_b.eq_ignore_ascii_case(b"int") {
            zPAR_TYPE::Int
        } else if ident_b.eq_ignore_ascii_case(b"float") {
            zPAR_TYPE::Float
        } else if ident_b.eq_ignore_ascii_case(b"string") {
            zPAR_TYPE::String
        } else if ident_b.eq_ignore_ascii_case(b"void") {
            zPAR_TYPE::Void
        } else if ident_b.eq_ignore_ascii_case(b"func") {
            zPAR_TYPE::Func
        } else {
            zPAR_TYPE::Instance(ident.clone())
        }
    }

    pub fn compatible(&self, other: &zPAR_TYPE) -> bool {
        match (self, other) {
            (zPAR_TYPE::Instance(_), zPAR_TYPE::Instance(_)) => true,
            (a, b) if a == b => true,
            _ => false,
        }
    }
}
