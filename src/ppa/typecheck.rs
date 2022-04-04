use crate::ppa::visitor::VisitorMut;
use crate::types;
use crate::types::parsed;

use super::errors::{TypecheckError, TypecheckErrorKind as TEK};

type TCResult<T> = Result<T, ()>;

pub struct TypeChecker<'a> {
    parsed_syms: &'a types::SymbolCollection,
    errors: Vec<TypecheckError>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(input: &'a types::SymbolCollection) -> Self {
        TypeChecker {
            parsed_syms: input,
            errors: Vec::new(),
        }
    }

    pub fn typecheck(&mut self) {
        use crate::types::parsed::Symbol::*;
        for symbol in self.parsed_syms.iter() {
            match symbol {
                Func(ref func) => {
                    self.visit_func_decl(func);
                }
                _ => {}
            }
        }
    }

    fn is_type(&self, typ: &types::Identifier) -> bool {
        lazy_static! {
            static ref PRIMITIVES: &'static [&'static [u8]] =
                &[b"int", b"void", b"string", b"float"];
        }
        let ident = typ.as_bytes();
        for primitive in PRIMITIVES.iter() {
            if ident.eq_ignore_ascii_case(primitive) {
                return true;
            }
        }

        for sym in self.parsed_syms.iter() {
            match sym {
                parsed::Symbol::Class(ref class) => {
                    if ident.eq_ignore_ascii_case(class.name.as_bytes()) {
                        return true;
                    }
                }
                _ => {}
            }
        }

        return false;
    }

    fn typecheck_statement(
        &self,
        statement: &types::Statement,
        scope: &types::Identifier,
    ) -> TCResult<()> {
        let typ = match statement {
            types::Statement::Exp(ref exp) => self.typecheck_exp(exp, Some(scope)),
            _ => return Err(()),
        };

        println!("{:?}", typ);

        return Ok(());
    }

    fn typecheck_exp(
        &self,
        exp: &types::Expression,
        scope: Option<&types::Identifier>,
    ) -> TCResult<parsed::zPAR_TYPE> {
        use crate::types::Expression::*;
        match exp {
            Int(_) => return Ok(parsed::zPAR_TYPE::Int),
            Float(_) => return Ok(parsed::zPAR_TYPE::Float),
            String(_) => return Ok(parsed::zPAR_TYPE::String),
            Call(ref call) => {
                let func = self.parsed_syms.get_by_name(call.func.as_bytes());

                let func = match func.expect("TODO no func found") {
                    parsed::Symbol::Func(ref func) => func,
                    _ => return Err(()),
                };

                // TODO CHECK params

                return Ok(parsed::zPAR_TYPE::from_ident(&func.typ));
            }
            Binary(ref bin) => {
                let left_type = self.typecheck_exp(&bin.left, scope);
                let right_type = self.typecheck_exp(&bin.right, scope);

                match (left_type, right_type) {
                    (Ok(left), Ok(right)) => {
                        if &left == &parsed::zPAR_TYPE::Int && &right == &parsed::zPAR_TYPE::Int {
                            return Ok(left);
                        } else {
                            println!("Wrong types!");
                            return Err(());
                        }
                    }
                    _ => {
                        println!("Wrong types!");
                        return Err(());
                    }
                };
            }
            Unary(ref un) => {
                let inner_type = self.typecheck_exp(&un.right, scope);

                if let Ok(inner_type) = inner_type {
                    if &inner_type == &parsed::zPAR_TYPE::Int {
                        return Ok(inner_type);
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }
            Identifier(ref var) => {
                if let Some(ref _inst) = var.instance {
                    // TODO implement instance access
                } else {
                    let sym = {
                        let sym = self.parsed_syms.get_by_name(var.name.as_bytes());
                        match sym {
                            Some(sym) => sym,
                            None => return Err(()), // Symbol not found;
                        }
                    };

                    match sym {
                        parsed::Symbol::Class(_) => {
                            // classes are not allowed in expressions
                            return Err(());
                        }
                        _ => {
                            return Ok(sym.typ());
                        }
                    }
                }
            }
        }

        return Err(());
    }
}

impl<'a> VisitorMut for TypeChecker<'a> {
    fn visit_expression(&mut self, exp: &types::Expression, scope: Option<&types::Identifier>) {
        let typ = self.typecheck_exp(exp, scope);
        println!("Type of exp is: {:?}", typ);
    }

    fn visit_statement(&mut self, _statement: &types::Statement, _scope: &types::Identifier) {}

    fn visit_func_decl(&mut self, decl: &types::Function) {
        let typ = &decl.typ;

        if !self.is_type(typ) {
            let err = TypecheckError {
                kind: TEK::UnknownReturnType(decl.typ.clone()),
                span: (0, 0),
            };
            panic!("Unknown type as return type of function {}", decl.name)
        }

        for statement in decl.body.iter() {
            // TODO Check result!!
            let _ = self.typecheck_statement(statement, &decl.name);
        }
    }
    /*fn visit_var_decl(&mut self, decl: &types::VarDeclaration, scope: Option<&types::Identifier>) {}
    fn visit_class_decl(&mut self, decl: &types::Class) {}
    fn visit_inst_decl(&mut self, decl: &types::Instance) {}
    fn visit_proto_decl(&mut self, decl: &types::Prototype) {}
    fn visit_const_decl(&mut self, decl: &types::ConstDeclaration, scope: Option<&types::Identifier>) {}
    fn visit_const_arr_decl(
        &mut self,
        decl: &types::ConstArrayDeclaration,
        scope: Option<&types::Identifier>,
    ) {
    }*/
}

#[cfg(test)]

mod tests {
    use crate::types::{IntNode, FloatNode};

    use super::*;

    #[test]
    fn exp_single_simple() {
        let exps = [
            (
                types::Expression::Int(IntNode {
                    value: 0,
                    span: (0, 0),
                }),
                parsed::zPAR_TYPE::Int,
            ),
            (
                types::Expression::Float(FloatNode {
                    value: 0.0,
                    span: (0, 0),
                }),
                parsed::zPAR_TYPE::Float,
            ),
            (
                types::Expression::String(types::StringLiteral::new(b"foo", (0, 0))),
                parsed::zPAR_TYPE::String,
            ),
        ];

        let sc = types::SymbolCollection::new(vec![]);
        let tc = TypeChecker::new(&sc);
        for (exp, typ) in exps.iter() {
            let actual = tc.typecheck_exp(&exp, None).unwrap();
            assert_eq!(&actual, typ);
        }
    }

    #[test]
    fn exp_var() {
        let sc = types::SymbolCollection::new(vec![parsed::Symbol::Var(
            types::VarDeclaration::new(
                types::Identifier::new(b"bar", (0, 0)),
                types::Identifier::new(b"foo", (0, 0)),
                None,
                (0, 0),
            ),
            None,
        )]);
        let tc = TypeChecker::new(&sc);

        let exp = types::Expression::Identifier(Box::new(types::VarAccess::new(
            types::Identifier::new(b"foo", (0, 0)),
            None,
            None,
            (0, 0),
        )));
        let expected = parsed::zPAR_TYPE::Instance(types::Identifier::new(b"bar", (0, 0)));
        let actual = tc.typecheck_exp(&exp, None).unwrap();

        assert_eq!(expected, actual);
    }
}
