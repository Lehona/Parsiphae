use crate::types;

#[derive(Debug, Clone, PartialEq, From)]
pub enum Symbol {
    Var(types::VarDeclaration, Option<types::Identifier>),
    Func(types::Function),
    Class(types::Class),
    Inst(types::Instance),
    Proto(types::Prototype),
    Const(types::ConstDeclaration, Option<types::Identifier>),
    ConstArray(types::ConstArrayDeclaration, Option<types::Identifier>),
}

impl Symbol {
    pub fn typ(&self) -> zPAR_TYPE {
        use self::Symbol::*;

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
        use self::Symbol::*;

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

        return full_name;
    }
}

// TODO implement void type?
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum zPAR_TYPE {
    Void,
    Int,
    Float,
    String,
    Instance(types::Identifier),
}

impl zPAR_TYPE {
    pub fn from_ident(ident: &types::Identifier) -> Self {
        let ident_b = ident.as_bytes();

        if ident_b.eq_ignore_ascii_case(b"int") {
            return zPAR_TYPE::Int;
        } else if ident_b.eq_ignore_ascii_case(b"float") {
            return zPAR_TYPE::Float;
        } else if ident_b.eq_ignore_ascii_case(b"string") {
            return zPAR_TYPE::String;
        } else if ident_b.eq_ignore_ascii_case(b"void") {
            return zPAR_TYPE::Void;
        } else {
            return zPAR_TYPE::Instance(ident.clone());
        }
    }
}
