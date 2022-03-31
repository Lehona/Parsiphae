use crate::types::{
    Class, ConstArrayDeclaration, ConstDeclaration, Function, Instance, Prototype, VarDeclaration,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Var(Vec<VarDeclaration>),
    Func(Function),
    Class(Class),
    Inst(Vec<Instance>),
    Proto(Prototype),
    Const(ConstDeclaration),
    ConstArray(ConstArrayDeclaration),
}
