use crate::types;
use crate::types::*;

#[allow(unused_variables)]
pub trait VisitorMut {
    fn visit_expression(&mut self, exp: &Expression, scope: Option<&types::Identifier>) {}
    fn visit_statement(&mut self, statement: &Statement, scope: &types::Identifier) {}
    fn visit_var_decl(&mut self, decl: &VarDeclaration, scope: Option<&types::Identifier>) {}
    fn visit_func_decl(&mut self, decl: &Function) {}
    fn visit_class_decl(&mut self, decl: &Class) {}
    fn visit_inst_decl(&mut self, decl: &Instance) {}
    fn visit_proto_decl(&mut self, decl: &Prototype) {}
    fn visit_const_decl(&mut self, decl: &ConstDeclaration, scope: Option<&types::Identifier>) {}
    fn visit_const_arr_decl(
        &mut self,
        decl: &ConstArrayDeclaration,
        scope: Option<&types::Identifier>,
    ) {
    }
}

#[allow(unused_variables)]
pub trait Visitor {
    fn visit_expression(&mut self, _exp: &Expression, scope: Option<&types::Identifier>) {}
    fn visit_statement(&mut self, _statement: &Statement, scope: &types::Identifier) {}
    fn visit_var_decl(&mut self, decl: &VarDeclaration, scope: Option<&types::Identifier>) {}
    fn visit_func_decl(&mut self, decl: &Function) {}
    fn visit_class_decl(&mut self, decl: &Class) {}
    fn visit_inst_decl(&mut self, decl: &Instance) {}
    fn visit_proto_decl(&mut self, decl: &Prototype) {}
    fn visit_const_decl(&mut self, decl: &ConstDeclaration, scope: Option<&types::Identifier>) {}
    fn visit_const_arr_decl(
        &mut self,
        decl: &ConstArrayDeclaration,
        scope: Option<&types::Identifier>,
    ) {
    }
}
/*
impl<'a, T> VisitorMut for &'a mut T
where
    T: Visitor,
{
    fn visit_expression(&mut self, exp: &Expression, scope: Option<&types::Identifier>) {
        let slf: &T = &mut self;
        Visitor::visit_expression(slf, exp, scope);
    }
    fn visit_statement(&mut self, statement: &Statement, scope: &types::Identifier) {
        let slf: &T = &mut self;
        Visitor::visit_statement(slf, statement, scope);
    }
    fn visit_var_decl(&mut self, decl: &VarDeclaration, scope: Option<&types::Identifier>) {
        let slf: &T = &mut self;
        Visitor::visit_var_decl(slf, decl, scope);
    }
    fn visit_func_decl(&mut self, decl: &Function) {
        let slf: &T = &mut self;
        Visitor::visit_func_decl(slf, decl);
    }
    fn visit_class_decl(&mut self, decl: &Class) {
        let slf: &T = &mut self;
        Visitor::visit_class_decl(slf, decl);
    }
    fn visit_inst_decl(&mut self, decl: &Instance) {
        let slf: &T = &mut self;
        Visitor::visit_inst_decl(slf, decl);
    }
    fn visit_proto_decl(&mut self, decl: &Prototype) {
        let slf: &T = &mut self;
        Visitor::visit_proto_decl(slf, decl);
    }
    fn visit_const_decl(&mut self, decl: &ConstDeclaration, scope: Option<&types::Identifier>) {
        let slf: &T = &mut self;
        Visitor::visit_const_decl(slf, decl, scope);
    }
    fn visit_const_arr_decl(&mut self, decl: &ConstArrayDeclaration, scope: Option<&types::Identifier>) {
        let slf: &T = &mut self;
        Visitor::visit_const_arr_decl(slf, decl, scope);
    }
}
*/

struct VisitorEngine<'a, V: VisitorMut + 'a> {
    visitor: &'a mut V,
}

impl<'a, V: VisitorMut + 'a> VisitorMut for VisitorEngine<'a, V> {
    fn visit_expression(&mut self, exp: &Expression, scope: Option<&types::Identifier>) {
        self.visitor.visit_expression(exp, scope);
    }

    fn visit_statement(&mut self, statement: &Statement, scope: &types::Identifier) {
        self.visitor.visit_statement(statement, scope);

        match statement {
            &Statement::Exp(ref exp) => self.visit_expression(exp, Some(scope)),
            &Statement::Ass(ref ass) => self.visit_expression(&ass.exp, Some(scope)),
            &Statement::If(ref if_statement) => {
                for branch in &if_statement.branches {
                    self.visit_expression(&branch.cond, Some(scope));
                    for statement in &branch.body {
                        self.visit_statement(statement, scope);
                    }
                }

                if let Some(ref else_branch) = &if_statement.else_branch {
                    for statement in else_branch {
                        self.visit_statement(statement, scope);
                    }
                }
            }
            &Statement::VarDeclarations(ref var_decls) => {
                for decl in var_decls {
                    self.visit_var_decl(decl, Some(scope));
                }
            }
            &Statement::ConstDeclaration(ref const_decl) => {
                self.visit_const_decl(const_decl, Some(scope));
            }
            &Statement::ConstArrayDeclaration(ref const_arr_decl) => {
                self.visit_const_arr_decl(const_arr_decl, Some(scope));
            }
            &Statement::ReturnStatement(ref opt_exp) => {
                if let Some(ref exp) = opt_exp {
                    self.visit_expression(exp, Some(scope))
                }
            }
        }
    }

    fn visit_var_decl(&mut self, decl: &VarDeclaration, scope: Option<&types::Identifier>) {
        self.visitor.visit_var_decl(decl, scope);
    }

    fn visit_func_decl(&mut self, decl: &Function) {
        self.visitor.visit_func_decl(decl);

        for statement in &decl.body {
            self.visit_statement(statement, &decl.name);
        }
    }

    fn visit_class_decl(&mut self, decl: &Class) {
        self.visitor.visit_class_decl(decl);

        for var_decl in &decl.members {
            self.visit_var_decl(var_decl, Some(&decl.name));
        }
    }

    fn visit_inst_decl(&mut self, decl: &Instance) {
        self.visitor.visit_inst_decl(decl);
        for statement in &decl.body {
            self.visit_statement(statement, &decl.name);
        }
    }

    fn visit_proto_decl(&mut self, decl: &Prototype) {
        self.visitor.visit_proto_decl(decl);
    }

    fn visit_const_decl(&mut self, decl: &ConstDeclaration, scope: Option<&types::Identifier>) {
        self.visitor.visit_const_decl(decl, scope);
    }

    fn visit_const_arr_decl(
        &mut self,
        decl: &ConstArrayDeclaration,
        scope: Option<&types::Identifier>,
    ) {
        self.visitor.visit_const_arr_decl(decl, scope);
    }
}

//fn visit_ast_mut<V: VisitorMut>(ast: &types::AST, visitor: &mut V) {}
pub fn visit_ast<V: VisitorMut>(ast: &types::AST, visitor: &mut V) {
    let mut engine = VisitorEngine { visitor };
    for decl in &ast.declarations {
        use crate::types::Declaration::*;
        match decl {
            Var(ref vec) => {
                for var in vec {
                    engine.visit_var_decl(var, None);
                }
            }
            Func(ref func) => {
                engine.visit_func_decl(func);
            }
            Class(ref class) => {
                engine.visit_class_decl(class);
            }
            Inst(ref vec) => {
                for inst in vec {
                    engine.visit_inst_decl(inst);
                }
            }
            Proto(ref proto) => {
                engine.visit_proto_decl(proto);
            }
            Const(ref const_decl) => {
                engine.visit_const_decl(const_decl, None);
            }
            ConstArray(ref const_arr) => {
                engine.visit_const_arr_decl(const_arr, None);
            }
        }
    }
}
