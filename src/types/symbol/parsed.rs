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
    External(types::External),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: usize,
    pub file_id: FileId,
    pub kind: SymbolKind,
}

impl SymbolKind {
    pub fn typ(&self) -> zPAR_TYPE {
        use self::SymbolKind::*;

        match self {
            Var(decl, _) => zPAR_TYPE::from_ident(&decl.typ),
            Func(func) => zPAR_TYPE::from_ident(&func.typ),
            Class(class) => zPAR_TYPE::from_ident(&class.name),
            Inst(inst) => {
                // TODO implement recursive lookup of instance type (in case of prototype)
                zPAR_TYPE::from_ident(&inst.class)
            }
            Proto(proto) => zPAR_TYPE::from_ident(&proto.class),
            Const(constant, _) => zPAR_TYPE::from_ident(&constant.typ),
            ConstArray(constant, _) => zPAR_TYPE::from_ident(&constant.typ),
            External(external) => external.return_type.clone(),
        }
    }
    pub fn name_without_scope(&self) -> Vec<u8> {
        use self::SymbolKind::*;

        match self {
            Var(decl, _scope) => decl.name.to_vec(),
            Func(func) => func.name.to_vec(),
            Class(class) => class.name.to_vec(),
            Inst(inst) => inst.name.to_vec(),
            Proto(proto) => proto.name.to_vec(),
            Const(decl, _scope) => decl.name.to_vec(),
            ConstArray(decl, _scope) => decl.name.to_vec(),
            External(external) => external.name.0.clone(),
        }
    }

    pub fn name(&self) -> Vec<u8> {
        use self::SymbolKind::*;

        let full_name = match self {
            Var(decl, scope) => {
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
            Func(func) => func.name.to_vec(),
            Class(class) => class.name.to_vec(),
            Inst(inst) => inst.name.to_vec(),
            Proto(proto) => proto.name.to_vec(),
            Const(decl, scope) => {
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
            ConstArray(decl, scope) => {
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
            External(external) => external.name.0.clone(),
        };

        full_name
    }

    /// Retrieve the span of a symbol
    /// For bigger symbols, this will return the span of the name instead.
    pub fn span(&self) -> (usize, usize) {
        use self::SymbolKind::*;
        match self {
            Var(decl, _) => decl.span,
            Func(func) => func.name.span,
            Class(class) => class.name.span,
            Inst(inst) => inst.span,
            Proto(proto) => proto.name.span,
            Const(constant, _) => constant.span,
            ConstArray(constant, _) => constant.span,
            External(_) => (0, 0),
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

impl std::fmt::Display for zPAR_TYPE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use zPAR_TYPE::*;
        write!(
            f,
            "{}",
            match self {
                Void => "void".into(),
                Int => "int".into(),
                Float => "float".into(),
                String => "string".into(),
                Func => "func".into(),
                Instance(ident) => ident.to_string(),
            }
        )
    }
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

    pub fn compatible(&self, rhs: &zPAR_TYPE) -> bool {
        match (self, rhs) {
            (zPAR_TYPE::Instance(_), zPAR_TYPE::Instance(_)) => true,
            (zPAR_TYPE::Float, zPAR_TYPE::Int) => true,
            // instances can be assigned to int (will convert to symbol ID)
            (zPAR_TYPE::Int, zPAR_TYPE::Instance(_)) => true,
            (a, b) if a == b => true,
            _ => false,
        }
    }
}
