use ast::*;
use symbols::zCPar_Symbol::*;
use parsers::util::flatten;

pub fn collect_function(func: &mut Function) -> Vec<zSymbol> {
    let func_sym = zSymbol::from_func(func);

    let mut param_symbols = collect_var_declaration_list(&mut func.params, Some(&func.name), false);

    let mut body_symbols = collect_statement_list(&mut func.body.as_mut().unwrap(), &func.name);

    param_symbols.insert(0, func_sym);
    param_symbols.append(&mut body_symbols);
    param_symbols
}

pub fn collect_statement_list(statements: &mut StatementList, scope: &str) -> Vec<zSymbol> {
    flatten(statements.statements.iter_mut().map(|s|collect_statement(s, scope)).collect::<Vec<_>>())
}

pub fn collect_statement(statement: &mut Statement, scope: &str) -> Vec<zSymbol> {
    match *statement {
        Statement::Exp(_) => Vec::new(),
        Statement::Ass(_) => Vec::new(),
        Statement::If(ref mut i) => collect_if_statement(i, scope),
        Statement::VarDeclaration(ref mut vec) => collect_var_declaration_list(vec, Some(scope), false),
        Statement::ConstDeclaration(ref mut con) => vec!(zSymbol::from_const_decl(con, Some(scope.to_string()))),
        Statement::ConstArrayDeclaration(_) => Vec::new(),
        Statement::ReturnStatement(_) => Vec::new()
    }
}

pub fn collect_var_declaration_list(var: &mut Vec<VariableDeclaration>, parent: Option<&str>, classvar: bool) -> Vec<zSymbol> {
    var.iter().map(|var|zSymbol::from_var_decl(&var, parent.map(|s|s.to_string()), classvar))
        .collect::<Vec<zSymbol>>()
}


pub fn collect_if_statement(i: &mut IfStatement, scope: &str) -> Vec<zSymbol> {
    let mut branch_symbols = flatten(i.branches.iter_mut().map(|b|collect_if_branch(b, scope)).collect::<Vec<_>>());

    let mut else_symbols = i.else_branch.as_mut().map_or(Vec::new(), |states|collect_statement_list(states, scope));

    branch_symbols.append(&mut else_symbols);
    branch_symbols

}

pub fn collect_if_branch(branch: &mut IfBranch, parent: &str) -> Vec<zSymbol> {
    collect_statement_list(&mut branch.body, parent)
}

pub fn collect_class_declaration(class: &mut Class) -> Vec<zSymbol> {
    let class_symbol = zSymbol::from_class(class);

    let mut members = collect_var_declaration_list(&mut class.members, Some(&class.name), true);

    members.insert(0, class_symbol);
    members
}

pub fn collect_instance_declaration(inst: &mut Instance) -> Vec<zSymbol> {
    let inst_symbol = zSymbol::from_inst(inst);
    let name = &inst.name;

    let mut inner_symbols = {
        inst.body.as_mut()
        .map_or(Vec::new(),
        |slist|collect_statement_list(slist, name))
    };


    inner_symbols.insert(0, inst_symbol);
    inner_symbols
}

pub fn collect_prototype_declaration(proto: &mut Prototype) -> Vec<zSymbol> {
    let proto_symbol = zSymbol::from_proto(proto);
    let name = &proto.name;

    let mut inner_symbols = {
        proto.body.as_mut()
            .map_or(Vec::new(),
                    |slist|collect_statement_list(slist, name))
    };

    let proto_symbol = zSymbol::from_proto(proto);

    inner_symbols.insert(0, proto_symbol);
    inner_symbols
}


pub fn collect_declaration(symb: &mut Symbol) -> Vec<zSymbol> {
   match *symb {
       Symbol::Func(ref mut f) => collect_function(f),
       Symbol::Var(ref mut vec) => collect_var_declaration_list(vec, None, false),
       Symbol::Class(ref mut c) => collect_class_declaration(c),
       Symbol::Inst(ref mut inst) => collect_instance_declaration(inst),
       Symbol::Proto(ref mut proto) => collect_prototype_declaration(proto),
       Symbol::Const(ref mut con) => vec!(zSymbol::from_const_decl(con, None)),
       Symbol::ConstArray(ref mut con_array) => vec!(zSymbol::from_const_array_decl(con_array))
    }
}