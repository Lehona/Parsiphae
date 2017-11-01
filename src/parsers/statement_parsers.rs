use ast::*;
use parsers::*;
use parsers::declaration::*;

pub fn assignment() -> Parser<'static, u8, Assignment> {
    let parser = variable() + boundary(one_of(b"+-*/").opt() - sym(b'='))
        + (float().map(|f|Expression::Float(f))
            | expression_parser()
    );

    parser.map(assign_op_switch)

}

pub fn single_if() -> Parser<'static, u8, IfBranch> {
    (seq_s(b"IF") * expression_parser() - sym_s(b'{') + call(statement_list) - sym_s(b'}'))
        .map(|(exp, states)|IfBranch {cond: exp, body: states})
}

pub fn if_parser() -> Parser<'static, u8, IfStatement> {
    let ifs = (single_if() + (seq_s(b"ELSE") * single_if()).repeat(0..))
        .map(|(f, mut v)|{v.insert(0, f); v});

    let else_if = seq_s(b"ELSE") * sym_s(b'{') * call(statement_list) - sym_s(b'}');

    let parser = ifs + else_if.opt();



    parser.map(|(branches, else_branch)|make_if(branches, else_branch ))
}

pub fn return_parser() -> Parser<'static, u8, Expression> {
    seq_s(b"RETURN") * expression_parser()
}


pub fn var_decl_statement() -> Parser<'static, u8, Vec<VariableDeclaration>> {
    let parser = var_declaration() + (sym_s(b',') * var_declaration()).repeat(..);

    parser.map(|(mut vec, doublevec)| { vec.append(&mut flatten(doublevec)); vec })
}
pub fn statement_parser() -> Parser<'static, u8, Statement> {
    let poss = if_parser().map(|if_s|Statement::If(Box::new(if_s))) - one_of_s(b";").opt()
                | var_decl_statement().map(|var|Statement::VarDeclaration(var)) - one_of_s(b";")
                | const_declaration().map(|con|Statement::ConstDeclaration(con)) - one_of_s(b";")
                | return_parser().map(|ret|Statement::ReturnStatement(ret)) - one_of_s(b";")
                | assignment().map(|ass|Statement::Ass(ass)) - one_of_s(b";")
                | expression_parser().map(|exp|Statement::Exp(exp)) - one_of_s(b";");


    poss
}

pub fn statement_list() -> Parser<'static, u8, StatementList> {
    boundary(statement_parser()).repeat(1..).map(|vec| StatementList {statements: vec})
    | empty().map(|_v|StatementList{statements: Vec::new()})
}