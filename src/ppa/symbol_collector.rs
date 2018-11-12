use ppa::visitor::*;
use types;
use types::parsed;

#[derive(Debug, Default)]
pub struct SymbolCollector {
    pub syms: Vec<parsed::Symbol>,
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
        self.syms
            .push(parsed::Symbol::Var(decl.clone(), scope.cloned()));
    }

    fn visit_func_decl(&mut self, decl: &types::Function) {
        self.syms.push(parsed::Symbol::Func(decl.clone()));
    }

    fn visit_class_decl(&mut self, decl: &types::Class) {
        self.syms.push(parsed::Symbol::Class(decl.clone()));
    }

    fn visit_inst_decl(&mut self, decl: &types::Instance) {
        self.syms.push(parsed::Symbol::Inst(decl.clone()));
    }

    fn visit_proto_decl(&mut self, decl: &types::Prototype) {
        self.syms.push(parsed::Symbol::Proto(decl.clone()));
    }

    fn visit_const_decl(
        &mut self,
        decl: &types::ConstDeclaration,
        scope: Option<&types::Identifier>,
    ) {
        self.syms
            .push(parsed::Symbol::Const(decl.clone(), scope.cloned()));
    }

    fn visit_const_arr_decl(
        &mut self,
        decl: &types::ConstArrayDeclaration,
        scope: Option<&types::Identifier>,
    ) {
        self.syms
            .push(parsed::Symbol::ConstArray(decl.clone(), scope.cloned()));
    }
}
