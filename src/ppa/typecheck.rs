use crate::types::parsed::{self, zPAR_TYPE};
use crate::types::{
    self, AssignmentOperator, ConstArrayDeclaration, ConstDeclaration, Identifier, IfStatement,
    Statement, VarAccess,
};

use super::errors::{TypecheckError, TypecheckErrorKind as TEK};

// Functions return Err(()) when they cannot reasonably continue typechecking, e.g. 3 + "foo" embedded within an expression
type TCResult<T> = Result<T, ()>;

pub enum IsType {
    UnknownIdentifier,
    NotType,
    IsType,
}

pub struct TypeChecker<'a> {
    parsed_syms: &'a types::SymbolCollection,
    errors: Vec<TypecheckError>,
    warnings: Vec<String>, // TODO: Change type once we have warnings to emit ;D
}

impl<'a> TypeChecker<'a> {
    pub fn new(input: &'a types::SymbolCollection) -> Self {
        TypeChecker {
            parsed_syms: input,
            errors: Vec::new(),
            warnings: Vec::new(),
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
                    self.typecheck_var_decl(decl, None)?;
                }
                Const(decl, None) => {
                    self.typecheck_const_decl(decl, None)?;
                }
                ConstArray(decl, None) => {
                    self.typecheck_const_array_decl(decl, None)?;
                }
                Var(_, Some(_)) | Const(_, Some(_)) | ConstArray(_, Some(_)) => {
                    // Intentionally left blank, only typecheck symbols at the top level
                    // Everything else will be typechecked by other symbols
                }
            }
        }

        if !self.errors.is_empty() {
            Err(())
        } else {
            Ok(())
        }
    }

    fn is_type(&self, typ: &Identifier) -> IsType {
        lazy_static! {
            static ref PRIMITIVES: &'static [&'static [u8]] =
                &[b"int", b"void", b"string", b"float"];
        }
        let ident = typ.as_bytes();
        for primitive in PRIMITIVES.iter() {
            if ident.eq_ignore_ascii_case(primitive) {
                return IsType::IsType;
            }
        }

        for sym in self.parsed_syms.iter() {
            match sym {
                parsed::Symbol::Class(ref class) => {
                    if ident.eq_ignore_ascii_case(class.name.as_bytes()) {
                        return IsType::IsType;
                    }
                }
                _ => {}
            }
        }

        // We assume that types are never in a scope. I don't actually know whether this is allowed in Daedalus?
        if self.parsed_syms.lookup_symbol(typ, None).is_some() {
            IsType::NotType
        } else {
            IsType::UnknownIdentifier
        }
    }

    fn typecheck_func(&mut self, decl: &types::Function) {
        match self.is_type(&decl.typ) {
            IsType::UnknownIdentifier => self.errors.push(TypecheckError {
                kind: TEK::UnknownReturnType(decl.typ.clone()),
                span: decl.typ.span,
            }),
            IsType::NotType => self
                .errors
                .push(TypecheckError::not_a_type(decl.typ.clone())),
            IsType::IsType => {}
        }

        for param in &decl.params {
            match self.is_type(&param.typ) {
                IsType::UnknownIdentifier => self.errors.push(TypecheckError {
                    kind: TEK::UnknownParameterType(param.typ.clone()),
                    span: param.typ.span,
                }),
                IsType::NotType => self
                    .errors
                    .push(TypecheckError::not_a_type(decl.typ.clone())),
                IsType::IsType => {}
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
        scope: Option<&Identifier>,
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
            Statement::ConstDeclaration(decl) => {
                self.typecheck_const_decl(decl, scope)?;
            }
            Statement::ConstArrayDeclaration(decl) => {
                self.typecheck_const_array_decl(decl, scope)?;
            }
        }
        Ok(())
    }

    fn typecheck_var_access(
        &mut self,
        var: &VarAccess,
        scope: Option<&Identifier>,
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
        scope: Option<&Identifier>,
    ) -> TCResult<zPAR_TYPE> {
        use crate::types::Expression::*;
        match exp {
            Int(_) => return Ok(zPAR_TYPE::Int),
            Float(_) => return Ok(zPAR_TYPE::Float),
            String(_) => return Ok(zPAR_TYPE::String),
            Call(ref call) => {
                let target = match self.parsed_syms.lookup_symbol(&call.func, None) {
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

                if target.params.len() != call.params.len() {
                    self.errors.push(TypecheckError {
                        kind: TEK::FunctionCallWrongAmountOfParameters(
                            target.params.len(),
                            call.params.len(),
                        ),
                        span: call.span,
                    })
                }

                for (call_param, target_param) in call.params.iter().zip(target.params.iter()) {
                    let expected = zPAR_TYPE::from_ident(&target_param.typ);
                    let actual = self.typecheck_expression(&call_param, scope)?;

                    if expected != actual {
                        self.errors.push(TypecheckError {
                            kind: TEK::FunctionCallParameterWrongType(expected, actual),
                            span: call_param.get_span(),
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
        scope: Option<&Identifier>,
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
        scope: Option<&Identifier>,
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
        scope: Option<&Identifier>,
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
            // Even if  one decl fails the typecheck we can continue checking others.
            let _ = self.typecheck_var_decl(decl, Some(&class.name));
        }

        Ok(())
    }

    fn typecheck_var_decl(
        &mut self,
        decl: &types::VarDeclaration,
        scope: Option<&Identifier>,
    ) -> TCResult<zPAR_TYPE> {
        // Dadealus doesn't really have the concept of array-types,
        // hence we return e.g. `int` even if the actual type of `var int foo[30]`
        // should be foo[]. Maybe this could be improved in the future?
        match self.is_type(&decl.typ) {
            IsType::UnknownIdentifier => self.errors.push(TypecheckError {
                kind: TEK::UnknownVariableType(decl.typ.clone()),
                span: decl.typ.span,
            }),
            IsType::NotType => self
                .errors
                .push(TypecheckError::not_a_type(decl.typ.clone())),
            IsType::IsType => {}
        }

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

    fn typecheck_const_decl(
        &mut self,
        decl: &ConstDeclaration,
        scope: Option<&Identifier>,
    ) -> TCResult<zPAR_TYPE> {
        match self.is_type(&decl.typ) {
            IsType::NotType => {
                self.errors
                    .push(TypecheckError::not_a_type(decl.typ.clone()));
                return Err(());
            }
            IsType::UnknownIdentifier => {
                self.errors.push(TypecheckError {
                    kind: TEK::UnknownVariableType(decl.typ.clone()),
                    span: decl.typ.span,
                });
                return Err(());
            }
            IsType::IsType => {}
        }

        let expression_type = self.typecheck_expression(&decl.initializer, scope)?;
        let decl_type = zPAR_TYPE::from_ident(&decl.typ);
        if expression_type != decl_type {
            self.errors.push(TypecheckError {
                kind: TEK::AssignmentWrongTypes(
                    decl_type.clone(),
                    decl.typ.span,
                    expression_type,
                    decl.initializer.get_span(),
                ),
                span: decl.span,
            })
        }

        Ok(decl_type)
    }

    fn typecheck_const_array_decl(
        &mut self,
        decl: &ConstArrayDeclaration,
        scope: Option<&Identifier>,
    ) -> TCResult<zPAR_TYPE> {
        match self.is_type(&decl.typ) {
            IsType::NotType => {
                self.errors
                    .push(TypecheckError::not_a_type(decl.typ.clone()));
                return Err(());
            }
            IsType::UnknownIdentifier => {
                self.errors.push(TypecheckError {
                    kind: TEK::UnknownVariableType(decl.typ.clone()),
                    span: decl.typ.span,
                });
                return Err(());
            }
            IsType::IsType => {}
        }

        match &decl.array_size {
            types::ArraySizeDeclaration::Identifier(constant) => {
                match self.parsed_syms.lookup_symbol(&constant, scope) {
                    Some(symb) => match symb {
                        parsed::Symbol::Const(const_decl, _) => {
                            let const_type = zPAR_TYPE::from_ident(&const_decl.typ);
                            if const_type != zPAR_TYPE::Int {
                                self.errors.push(TypecheckError {
                                    kind: TEK::ArraySizeIsNotInteger(const_type, const_decl.span),
                                    span: constant.span,
                                });
                            }
                        }
                        _ => {
                            self.errors.push(TypecheckError {
                                kind: TEK::NonConstantArraySize,
                                span: constant.span,
                            }); // TODO Add symbol kind to error msg
                        }
                    },
                    None => {
                        self.errors.push(TypecheckError {
                            kind: TEK::UnknownIdentifierInArraySize(constant.clone()),
                            span: constant.span,
                        });
                    }
                }
            }
            types::ArraySizeDeclaration::Size(i) if *i > 256 => {
                // TODO add warning about array size here
            }
            _ => {}
        }

        let decl_type = zPAR_TYPE::from_ident(&decl.typ);
        for init_expression in &decl.initializer.expressions {
            // continue type checking other expressions
            let init_type = match self.typecheck_expression(&init_expression, scope) {
                Ok(t) => t,
                Err(_) => continue,
            };

            if init_type != decl_type {
                self.errors.push(TypecheckError {
                    kind: TEK::WrongTypeInArrayInitialization(
                        decl_type.clone(),
                        decl.typ.span,
                        init_type,
                        init_expression.get_span(),
                    ),
                    span: decl.span,
                })
            }
        }

        // TODO: Make sure that initializer size and declared array size match.
        //       This requires evaluation of expressions if possible, and resolving
        //       of constants. We need constant resolving without endlessly looping.

        Ok(decl_type)
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

    #[test]
    fn wrong_const_decl() {
        let expected = vec![TypecheckError {
            kind: TEK::AssignmentWrongTypes(zPAR_TYPE::Int, (6, 9), zPAR_TYPE::String, (16, 19)),
            span: (0, 20),
        }];
        let actual = setup_typecheck_errors(b"const int foo = \"3\";");
        assert_eq!(expected, actual);
    }

    #[test]
    fn const_decl_unknown_type() {
        let expected = vec![TypecheckError {
            kind: TEK::UnknownVariableType(Identifier::new(b"baz", (6, 9))),
            span: (6, 9),
        }];
        let actual = setup_typecheck_errors(b"const baz foo = \"3\";");
        assert_eq!(expected, actual);
    }

    #[test]
    fn wrong_const_array_decl() {
        let expected = vec![TypecheckError {
            kind: TEK::WrongTypeInArrayInitialization(
                zPAR_TYPE::Int,
                (6, 9),
                zPAR_TYPE::String,
                (23, 26),
            ),
            span: (0, 32),
        }];
        let actual = setup_typecheck_errors(b"const int foo[3] = {1, \"2\", 3 };");
        assert_eq!(expected, actual);
    }
}
