use ast::*;
use symbols::zCPar_Symbol::*;
use parsers::util::flatten;

pub fn collect_function(func: &Function) -> Vec<zSymbol> {
    let func_sym = zSymbol::from_func(func);

    let mut param_symbols = collect_var_declaration_list(&func.params, Some(&func.name), false);

    let mut body_symbols = collect_statement_list(&func.body, &func.name);

    param_symbols.insert(0, func_sym);
    param_symbols.append(&mut body_symbols);
    param_symbols
}

pub fn collect_statement_list(statements: &StatementList, scope: &str) -> Vec<zSymbol> {
    flatten(statements.statements.iter().map(|s|collect_statement(s, scope)).collect::<Vec<_>>())
}

pub fn collect_statement(statement: &Statement, scope: &str) -> Vec<zSymbol> {
    match *statement {
        Statement::Exp(_) => Vec::new(),
        Statement::Ass(_) => Vec::new(),
        Statement::If(ref i) => collect_if_statement(i, scope),
        Statement::VarDeclaration(ref vec) => collect_var_declaration_list(vec, Some(scope), false),
        Statement::ConstDeclaration(ref con) => vec!(zSymbol::from_const_decl(con, Some(scope.to_string()))),
        Statement::ConstArrayDeclaration(_) => Vec::new(),
        Statement::ReturnStatement(_) => Vec::new()
    }
}

pub fn collect_var_declaration_list(var: &Vec<VariableDeclaration>, parent: Option<&str>, classvar: bool) -> Vec<zSymbol> {
    var.iter().map(|var|zSymbol::from_var_decl(&var, parent.map(|s|s.to_string()), classvar))
        .collect::<Vec<zSymbol>>()
}


pub fn collect_if_statement(i: &IfStatement, scope: &str) -> Vec<zSymbol> {
    let mut branch_symbols = flatten(i.branches.iter().map(|b|collect_if_branch(b, scope)).collect::<Vec<_>>());

    let mut else_symbols = i.else_branch.as_ref().map_or(Vec::new(), |states|collect_statement_list(states, scope));

    branch_symbols.append(&mut else_symbols);
    branch_symbols

}

pub fn collect_if_branch(branch: &IfBranch, parent: &str) -> Vec<zSymbol> {
    collect_statement_list(&branch.body, parent)
}

pub fn collect_class_declaration(class: &Class) -> Vec<zSymbol> {
    let class_symbol = zSymbol::from_class(class);

    let mut members = collect_var_declaration_list(&class.members, Some(&class.name), true);

    members.insert(0, class_symbol);
    members
}

pub fn collect_instance_declaration(inst: &Instance) -> Vec<zSymbol> {
    let inst_symbol = zSymbol::from_inst(inst);

    let mut inner_symbols = inst.body.as_ref().map_or(Vec::new(), |slist|collect_statement_list(slist, &inst.name));

    inner_symbols.insert(0, inst_symbol);
    inner_symbols
}

pub fn collect_prototype_declaration(proto: &Prototype) -> Vec<zSymbol> {
    let proto_symbol = zSymbol::from_proto(proto);

    let mut inner_symbols = proto.body.as_ref().map_or(Vec::new(), |slist|collect_statement_list(slist, &proto.name));

    inner_symbols.insert(0, proto_symbol);
    inner_symbols
}


pub fn collect_declaration(symb: &Symbol) -> Vec<zSymbol> {
   match *symb {
       Symbol::Func(ref f) => collect_function(f),
       Symbol::Var(ref vec) => collect_var_declaration_list(vec, None, false),
       Symbol::Class(ref c) => collect_class_declaration(c),
       Symbol::Inst(ref inst) => collect_instance_declaration(inst),
       Symbol::Proto(ref proto) => collect_prototype_declaration(proto),
       Symbol::Const(ref con) => vec!(zSymbol::from_const_decl(con, None)),
       Symbol::ConstArray(ref con_array) => vec!(zSymbol::from_const_array_decl(con_array))
    }
}