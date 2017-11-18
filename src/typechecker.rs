pub fn typecheck_const_declaration(con: &ConstantDeclaration) -> Result<(), String> {

}

pub fn typecheck_declaration(symb: &zSymbol) -> Result<(), String> {
    match *symb {
        Symbol::Func(ref f) => typecheck_function(f),
        Symbol::Var(ref vec) => Ok(()),
        Symbol::Class(ref c) => Ok(()),
        Symbol::Inst(ref inst) => typecheck_instance_declaration(inst),
        Symbol::Proto(ref proto) => typecheck_prototype_declaration(proto),
        Symbol::Const(ref con) => typecheck_const_declaration(con),
        Symbol::ConstArray(ref con_array) => typecheck_const_array_declaration(con_array)
    }
}