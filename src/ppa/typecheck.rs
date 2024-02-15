use crate::types::parsed::{self, zPAR_TYPE};
use crate::types::{self, AssignmentOperator, IfStatement, Statement, VarAccess};

use super::errors::{TypecheckError, TypecheckErrorKind as TEK};

// Functions return Err(()) when they cannot reasonably continue typechecking, e.g. 3 + "foo" embedded within an expression
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

    pub fn typecheck(&mut self) -> TCResult<()> {
        use crate::types::parsed::Symbol::*;
        for symbol in self.parsed_syms.iter() {
            match symbol {
                Func(ref func) => {
                    self.typecheck_func(func);
                }
                Class(ref class) => {
                    let _ = self.typecheck_class(class);
                }
                Inst(ref inst) => {
                    self.typecheck_instance(inst)?;
                }
                Proto(ref proto) => {
                    self.typecheck_prototype(proto)?;
                }
                Var(decl, None) => {
                    // Only check unscoped Variables, all other variables will be typechecked as part of other stuff anyway
                    self.typecheck_var_decl(decl, None)?;
                }
                Var(_decl, Some(_scope)) => {}
                _ => todo!(),
            }
        }

        if !self.errors.is_empty() {
            Err(())
        } else {
            Ok(())
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

    fn typecheck_func(&mut self, decl: &types::Function) {
        if !self.is_type(&decl.typ) {
            let err = TypecheckError {
                kind: TEK::UnknownReturnType(decl.typ.clone()),
                span: decl.typ.span,
            };
            self.errors.push(err);
        }

        for param in &decl.params {
            if !self.is_type(&param.typ) {
                let err = TypecheckError {
                    kind: TEK::UnknownParameterType(param.typ.clone()),
                    span: param.typ.span,
                };
                self.errors.push(err);
            }
        }

        for statement in decl.body.iter() {
            // TODO Check result!!
            let _ = self.typecheck_statement(statement, Some(&decl.name));
        }
    }

    fn typecheck_statement(
        &mut self,
        statement: &types::Statement,
        scope: Option<&types::Identifier>,
    ) -> TCResult<()> {
        match statement {
            Statement::Exp(ref exp) => {
                self.typecheck_expression(exp, scope)?;
            }
            Statement::Ass(ref ass) => {
                self.typecheck_assignment(ass, scope)?;
            }
            Statement::If(ref if_clause) => {
                self.typecheck_if_clause(if_clause, scope)?;
            }
            Statement::ReturnStatement(ref ret) => {
                self.typecheck_return(ret, scope)?;
            }
            Statement::VarDeclarations(ref decls) => {
                for decl in decls {
                    self.typecheck_var_decl(decl, scope)?;
                }
            }
            _ => unimplemented!(), // TODO
        }
        Ok(())
    }

    fn typecheck_var_access(
        &mut self,
        var: &VarAccess,
        scope: Option<&types::Identifier>,
    ) -> TCResult<zPAR_TYPE> {
        // TODO: Check array access / index etc.
        if let Some(ref _inst) = var.instance {
            // TODO implement instance access
            unimplemented!()
        } else {
            let sym = {
                let sym = self.parsed_syms.lookup_symbol(&var.name, scope);
                match sym {
                    Some(sym) => sym,
                    None => {
                        self.errors.push(TypecheckError {
                            kind: TEK::UnknownIdentifierInExpression(var.name.clone()),
                            span: var.span,
                        });
                        return Err(()); // TODO Symbol not found, abort typechecking or maybe assume zPAR_TYPE::Int?
                    }
                }
            };

            match sym {
                parsed::Symbol::Class(_) => {
                    self.errors.push(TypecheckError {
                        kind: TEK::IdentifierIsClassInExpression(var.name.clone()),
                        span: var.span,
                    });
                    return Err(());
                }
                _ => {
                    return Ok(sym.typ());
                }
            }
        }
    }

    fn typecheck_expression(
        &mut self,
        exp: &types::Expression,
        scope: Option<&types::Identifier>,
    ) -> TCResult<zPAR_TYPE> {
        use crate::types::Expression::*;
        match exp {
            Int(_) => return Ok(zPAR_TYPE::Int),
            Float(_) => return Ok(zPAR_TYPE::Float),
            String(_) => return Ok(zPAR_TYPE::String),
            Call(ref call) => {
                let target = self.parsed_syms.lookup_symbol(&call.func, None);
                let target = match target {
                    None => {
                        self.errors.push(TypecheckError {
                            kind: TEK::UnknownFunctionCall(call.func.clone()),
                            span: call.span,
                        }); // TODO: maybe span should be identifier only?
                        return Err(());
                    }
                    Some(symb) => match symb {
                        parsed::Symbol::Func(ref func) => func,
                        _ => {
                            self.errors.push(TypecheckError {
                                kind: TEK::FunctionCallWrongType(call.func.clone(), ()),
                                span: call.span,
                            });
                            return Err(());
                        }
                    },
                };

                // TODO account for incorrect number of parameters
                for (i, param) in call.params.iter().enumerate() {
                    let expected = zPAR_TYPE::from_ident(&target.params[i].typ);
                    let actual = self.typecheck_expression(&param, scope)?;

                    if expected != actual {
                        self.errors.push(TypecheckError {
                            kind: TEK::FunctionCallParameterWrongType(expected, actual),
                            span: param.get_span(),
                        });
                    }
                }

                return Ok(zPAR_TYPE::from_ident(&target.typ));
            }
            Binary(ref bin) => {
                let left = self.typecheck_expression(&bin.left, scope)?;
                let right = self.typecheck_expression(&bin.right, scope)?;

                if left == zPAR_TYPE::Int && right == zPAR_TYPE::Int {
                    return Ok(zPAR_TYPE::Int);
                } else {
                    if left != zPAR_TYPE::Int {
                        self.errors.push(TypecheckError {
                            kind: TEK::BinaryExpressionNotInt,
                            span: bin.left.get_span(),
                        });
                    }
                    if right != zPAR_TYPE::Int {
                        self.errors.push(TypecheckError {
                            kind: TEK::BinaryExpressionNotInt,
                            span: bin.right.get_span(),
                        });
                    }

                    // TODO: Think about return OK(Int) anyway, because we can assume that
                    // the user meant to type this as int anyway.
                    return Err(());
                }
            }
            Unary(ref un) => {
                let inner_type = self.typecheck_expression(&un.right, scope)?;
                if inner_type == zPAR_TYPE::Int {
                    return Ok(inner_type);
                } else {
                    self.errors.push(TypecheckError {
                        kind: TEK::UnaryExpressionNotInt,
                        span: un.right.get_span(),
                    });
                    return Err(());
                }
            }
            Identifier(ref var) => return self.typecheck_var_access(var, scope),
        }
    }

    fn typecheck_assignment(
        &mut self,
        ass: &types::Assignment,
        scope: Option<&types::Identifier>,
    ) -> TCResult<zPAR_TYPE> {
        let left_type = self.typecheck_var_access(&ass.var, scope)?;
        let right_type = self.typecheck_expression(&ass.exp, scope)?;

        if left_type != right_type {
            self.errors.push(TypecheckError {
                kind: TEK::AssignmentWrongTypes(
                    left_type,
                    ass.var.span,
                    right_type,
                    ass.exp.get_span(),
                ),
                span: (ass.span),
            });
            return Err(());
        }

        if left_type == zPAR_TYPE::String && ass.op != AssignmentOperator::Eq {
            self.errors.push(TypecheckError {
                kind: TEK::CanOnlyAssignToString,
                span: ass.span,
            });
            return Err(());
        }
        if left_type == zPAR_TYPE::Float && ass.op != AssignmentOperator::Eq {
            self.errors.push(TypecheckError {
                kind: TEK::CanOnlyAssignToFloat,
                span: ass.span,
            });
            return Err(());
        }
        if let zPAR_TYPE::Instance(_) = left_type {
            // TODO refactor this once if-let chains become available
            if ass.op != AssignmentOperator::Eq {
                self.errors.push(TypecheckError {
                    kind: TEK::CanOnlyAssignToInstance,
                    span: ass.span,
                });
                return Err(());
            }
        }

        Ok(left_type)
    }

    fn typecheck_if_clause(
        &mut self,
        if_clause: &IfStatement,
        scope: Option<&types::Identifier>,
    ) -> TCResult<()> {
        for branch in &if_clause.branches {
            let cond_type = match self.typecheck_expression(&branch.cond, scope) {
                Ok(typ) => typ,
                Err(_) => continue,
            };

            if cond_type != zPAR_TYPE::Int {
                self.errors.push(TypecheckError {
                    kind: TEK::ConditionNotInt(cond_type),
                    span: branch.cond.get_span(),
                });
            }

            for statement in &branch.body {
                let _ = self.typecheck_statement(&statement, scope);
            }
        }

        if let Some(else_branch) = &if_clause.else_branch {
            for statement in else_branch {
                let _ = self.typecheck_statement(&statement, scope);
            }
        }

        Ok(())
    }

    fn typecheck_return(
        &mut self,
        ret: &types::ReturnStatement,
        scope: Option<&types::Identifier>,
    ) -> TCResult<()> {
        match scope {
            Some(func_name) => {
                match self.parsed_syms.lookup_symbol(&func_name, None) {
                    Some(symb) => {
                        if let parsed::Symbol::Func(func) = symb {
                            let return_type = zPAR_TYPE::from_ident(&func.typ);
                            match &ret.exp {
                                Some(exp) if return_type != zPAR_TYPE::Void => {
                                    let return_exp_type = self.typecheck_expression(exp, scope)?;
                                    if return_exp_type != return_type {
                                        self.errors.push(TypecheckError {
                                            kind: TEK::ReturnExpressionDoesNotMatchReturnType(
                                                func.typ.clone(),
                                                return_exp_type,
                                            ),
                                            span: exp.get_span(),
                                        });
                                    }
                                }
                                Some(_exp) => {
                                    self.errors.push(TypecheckError {
                                        kind: TEK::ReturnExpressionInVoidFunction(func.typ.clone()),
                                        span: ret.span,
                                    });
                                }
                                None if return_type == zPAR_TYPE::Void => return Ok(()),
                                None => {
                                    self.errors.push(TypecheckError {
                                        kind: TEK::ReturnWithoutExpression(func.typ.clone()),
                                        span: ret.span,
                                    });
                                }
                            }
                        } else {
                            self.errors.push(TypecheckError {
                                kind: TEK::InternalFailure(
                                    "Scope of return statement is not a function".to_string(),
                                ),
                                span: ret.span,
                            });
                        }
                    }
                    None => {
                        self.errors.push(TypecheckError {
                            kind: TEK::InternalFailure(format!(
                                "Unable to find containing function {}",
                                String::from_utf8_lossy(func_name.as_bytes())
                            )),
                            span: func_name.span,
                        });
                        return Err(());
                    }
                };
                Ok(())
            }
            None => {
                self.errors.push(TypecheckError {
                    kind: TEK::InternalFailure(
                        "Return Statement outside of function scope".to_string(),
                    ),
                    span: ret.span,
                });
                Err(())
            }
        }
    }

    fn typecheck_class(&mut self, class: &types::Class) -> TCResult<()> {
        for decl in &class.members {
            let _ = self.typecheck_var_decl(decl, Some(&class.name));
        }

        Ok(())
    }

    fn typecheck_var_decl(
        &mut self,
        decl: &types::VarDeclaration,
        scope: Option<&types::Identifier>,
    ) -> TCResult<zPAR_TYPE> {
        // TODO clean this up
        // TODO check whether is_type(decl.typ)
        match &decl.array_size {
            Some(types::ArraySizeDeclaration::Identifier(constant)) => {
                match self.parsed_syms.lookup_symbol(&constant, scope) {
                    Some(symb) => match symb {
                        parsed::Symbol::Const(const_decl, _) => {
                            let const_type = zPAR_TYPE::from_ident(&const_decl.typ);
                            if const_type != zPAR_TYPE::Int {
                                self.errors.push(TypecheckError {
                                    kind: TEK::ArraySizeIsNotInteger(const_type, const_decl.span),
                                    span: constant.span,
                                });
                                return Err(());
                            }
                            Ok(zPAR_TYPE::from_ident(&decl.typ))
                        }
                        _ => {
                            self.errors.push(TypecheckError {
                                kind: TEK::NonConstantArraySize,
                                span: constant.span,
                            }); // TODO Add symbol kind to error msg
                            return Err(());
                        }
                    },
                    None => {
                        self.errors.push(TypecheckError {
                            kind: TEK::UnknownIdentifierInArraySize(constant.clone()),
                            span: constant.span,
                        });
                        return Err(());
                    }
                }
            }
            Some(types::ArraySizeDeclaration::Size(i)) if *i > 256 => {
                // TODO add warning about array size here
                Ok(zPAR_TYPE::from_ident(&decl.typ))
            }
            _ => Ok(zPAR_TYPE::from_ident(&decl.typ)),
        }
    }

    fn typecheck_prototype(&mut self, proto: &types::Prototype) -> TCResult<zPAR_TYPE> {
        let parent = match self.parsed_syms.lookup_symbol(&proto.class, None) {
            Some(symb) => symb,
            None => {
                self.errors.push(TypecheckError {
                    kind: TEK::InstanceHasUnknownParent(proto.class.clone()),
                    span: (proto.span.0, proto.class.span.1),
                }); // TODO really make sure these spans make any sense
                return Err(()); // Trying to typecheck with a "wrong" scope will lead to tons of followup errors, better to just stop
            }
        };

        let scope = match parent {
            parsed::Symbol::Class(class) => &class.name,
            parsed::Symbol::Proto(proto) => &proto.class, // TODO an invalid prototype (e.g. invalid proto's parent) might produce a lot of errors
            _ => {
                self.errors.push(TypecheckError {
                    kind: TEK::InstanceParentNotClassOrProto(proto.class.clone(), ()),
                    span: (proto.span.0, proto.class.span.1),
                }); // TODO really make sure these spans make any sense
                return Err(()); // Trying to typecheck with a "wrong" scope will lead to tons of followup errors, better to just stop
            }
        };

        for statement in &proto.body {
            self.typecheck_statement(statement, Some(scope))?;
        }
        Ok(zPAR_TYPE::from_ident(&proto.class))
    }
    fn typecheck_instance(&mut self, inst: &types::Instance) -> TCResult<zPAR_TYPE> {
        let parent = match self.parsed_syms.lookup_symbol(&inst.class, None) {
            Some(symb) => symb,
            None => {
                self.errors.push(TypecheckError {
                    kind: TEK::InstanceHasUnknownParent(inst.class.clone()),
                    span: (inst.span.0, inst.class.span.1),
                }); // TODO really make sure these spans make any sense
                return Err(()); // Trying to typecheck with a "wrong" scope will lead to tons of followup errors, better to just stop
            }
        };

        let scope = match parent {
            parsed::Symbol::Class(class) => &class.name,
            parsed::Symbol::Proto(proto) => &proto.class, // TODO an invalid prototype (e.g. invalid proto's parent) might produce a lot of errors
            _ => {
                self.errors.push(TypecheckError {
                    kind: TEK::InstanceParentNotClassOrProto(inst.class.clone(), ()),
                    span: (inst.span.0, inst.class.span.1),
                }); // TODO really make sure these spans make any sense
                return Err(()); // Trying to typecheck with a "wrong" scope will lead to tons of followup errors, better to just stop
            }
        };

        for statement in &inst.body {
            self.typecheck_statement(statement, Some(scope))?;
        }
        Ok(zPAR_TYPE::from_ident(&inst.class))
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use crate::{
        lexer::Lexer,
        ppa::symbol_collector::SymbolCollector,
        types::{FloatNode, Identifier, IntNode, SymbolCollection, AST},
    };

    use super::*;

    fn setup_typecheck_errors(input: &[u8]) -> Vec<TypecheckError> {
        let tokens = Lexer::lex(input).expect("Unable to tokenize");
        let mut parser = crate::parser::parser::Parser::new(&tokens);
        let declarations = parser.start().expect("Unable to parse code");
        let mut visitor = SymbolCollector::new();
        crate::ppa::visitor::visit_ast(&AST { declarations }, &mut visitor);
        let symbols = SymbolCollection::new(visitor.syms);
        let mut typechecker = TypeChecker::new(&symbols);
        typechecker
            .typecheck()
            .expect_err("Typechecking succeeded unexpectly");

        typechecker.errors
    }

    #[test]
    fn exp_single_simple() {
        let exps = [
            (
                types::Expression::Int(IntNode {
                    value: 0,
                    span: (0, 0),
                }),
                zPAR_TYPE::Int,
            ),
            (
                types::Expression::Float(FloatNode {
                    value: 0.0,
                    span: (0, 0),
                }),
                zPAR_TYPE::Float,
            ),
            (
                types::Expression::String(types::StringLiteral::new(b"foo", (0, 0))),
                zPAR_TYPE::String,
            ),
        ];

        let sc = types::SymbolCollection::new(vec![]);
        let mut tc = TypeChecker::new(&sc);
        for (exp, typ) in exps.iter() {
            let actual = tc.typecheck_expression(&exp, None).unwrap();
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
        let mut tc = TypeChecker::new(&sc);

        let exp = types::Expression::Identifier(Box::new(types::VarAccess::new(
            types::Identifier::new(b"foo", (0, 0)),
            None,
            None,
            (0, 0),
        )));
        let expected = zPAR_TYPE::Instance(types::Identifier::new(b"bar", (0, 0)));
        let actual = tc.typecheck_expression(&exp, None).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn unknown_identifier() {
        let expected = vec![TypecheckError {
            kind: TEK::UnknownIdentifierInExpression(Identifier::new(b"baz", (17, 20))),
            span: (17, 20),
        }];
        let actual = setup_typecheck_errors(b"func int foo() { baz; };");

        assert_eq!(expected, actual);
    }

    #[test]
    fn unknown_return_type() {
        let expected = vec![TypecheckError {
            kind: TEK::UnknownReturnType(Identifier::new(b"baz", (5, 8))),
            span: (5, 8),
        }];
        let actual = setup_typecheck_errors(b"func baz foo() {};");

        assert_eq!(expected, actual);
    }
    #[test]
    fn mixing_float_and_int() {
        let expected = vec![TypecheckError {
            kind: TEK::BinaryExpressionNotInt,
            span: (21, 24),
        }];
        let actual = setup_typecheck_errors(b"func int foo() { 3 + 3.5; };");

        assert_eq!(expected, actual);
    }

    #[test]
    fn missing_return_expression() {
        let expected = vec![TypecheckError {
            kind: TEK::ReturnWithoutExpression(Identifier::new(b"int", (5, 8))),
            span: (17, 24),
        }];
        let actual = setup_typecheck_errors(b"func int foo() { return; };");

        assert_eq!(expected, actual);
    }

    #[test]
    fn return_expression_in_void_function() {
        let expected = vec![TypecheckError {
            kind: TEK::ReturnExpressionInVoidFunction(Identifier::new(b"void", (5, 9))),
            span: (18, 27),
        }];
        let actual = setup_typecheck_errors(b"func void foo() { return 3; };");

        assert_eq!(expected, actual);
    }

    #[test]
    fn wrong_return_expression_type() {
        let expected = vec![TypecheckError {
            kind: TEK::ReturnExpressionDoesNotMatchReturnType(
                Identifier::new(b"int", (5, 8)),
                zPAR_TYPE::String,
            ),
            span: (24, 31),
        }];
        let actual = setup_typecheck_errors(b"func int foo() { return \"hello\"; };");

        assert_eq!(expected, actual);
    }
    #[test]
    fn mixing_int_and_string() {
        let expected = vec![TypecheckError {
            kind: TEK::BinaryExpressionNotInt,
            span: (35, 36),
        }];
        let actual = setup_typecheck_errors(b"func int foo() { var string s; 3 + s; };");
        assert_eq!(expected, actual);
    }

    #[test]
    fn mixing_string_and_int() {
        let expected = vec![TypecheckError {
            kind: TEK::BinaryExpressionNotInt,
            span: (31, 32),
        }];
        let actual = setup_typecheck_errors(b"func int foo() { var string s; s + 3; };");
        assert_eq!(expected, actual);
    }
}
