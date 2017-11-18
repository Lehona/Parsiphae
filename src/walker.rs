use  ast::*;
use itertools::Itertools;

fn walk_unary(un: &UnaryOp) -> String {
    let val1 = walk_exp(&un.a);

    match un.op {
        UnaryOperator::Plus => format!("+{}", val1),
        UnaryOperator::Minus => format!("-{}", val1),
        UnaryOperator::Negate => format!("!{}", val1),
        UnaryOperator::Flip => format!("~{}", val1)
    }
}


fn walk_binary(bin: &BinaryOp) -> String  {
    let exp1 = walk_exp(&bin.a);
    let exp2 = walk_exp(&bin.b);

    let left_string = if bin.op.needs_parentheses(&bin.a) {
        format!("({})", exp1)
    } else {
        format!("{}", exp1)
    };

    let right_string = if bin.op.needs_parentheses(&bin.b) {
        format!("({})", exp2)
    } else {
        format!("{}", exp2)
    };

    format!("{} {} {}", left_string, bin.op.sign(), right_string)
}


fn walk_call(call: &Call) -> String {
    let res = format!("{}({})", call.func, walk_exp_list(&call.params));
    res

}

pub fn walk_exp(exp: &Expression) -> String {
    match *exp {
        Expression::Float(f) => f.to_string(),
        Expression::Binary(ref b_op) => walk_binary(&*b_op),
        Expression::Unary(ref u_op) => walk_unary(&*u_op),
        Expression::Value(i) => i.to_string(),
        Expression::Variable(ref v) => walk_var_access(v),
        Expression::Call(ref c_exp) => walk_call(c_exp),
        Expression::String(ref s) => format!("\"{}\"", s)

    }
}

pub fn walk_array_index(index: &ArrayIndex) -> String {
    match *index {
        ArrayIndex::Identifier(ref s) => s.clone(),
        ArrayIndex::Number(ref i) => i.to_string()
    }
}

pub fn walk_var_access(var: &VarAccess) -> String {
    if var.index.is_some() && var.instance.is_some() {
        format!("{}.{}[{}]", var.instance.as_ref().unwrap(), var.name, walk_array_index(var.index.as_ref().unwrap()))
    } else if var.index.is_some() {
        format!("{}[{}]", var.name, walk_array_index(var.index.as_ref().unwrap()))
    } else if var.instance.is_some() {
        format!("{}.{}", var.instance.as_ref().unwrap(), var.name)
    } else {
        format!("{}", var.name)
    }
}

pub fn walk_exp_list(exps: &ExpressionList) -> String {
    let mut res = String::new();
    if exps.expressions.len() == 0 { return res; }

    res.push_str(&walk_exp(&exps.expressions[0]));

    for i in 1..exps.expressions.len() {
        res.push_str(&format!(", {}", &walk_exp(&exps.expressions[i])));
    }
    res
}

pub fn walk_const_array_decl(con: &ConstantArrayDeclaration) -> String {
    let initializer = match con.values {
        ConstArrayInitializerList::Numbers(ref vec) => vec.iter().map(|item|walk_exp(item)).join(", "),
        ConstArrayInitializerList::Strings(ref vec) => format!("\"{}\"", vec.iter().join("\", \""))
    };

    format!("const {} {}[{}] = {{{}}}", con.typ, con.name, walk_array_index(&con.array_size), initializer)
}

pub fn walk_const_value(val: &ConstantValue) -> String {
    match *val {
        ConstantValue::Float(ref f) => f.to_string(),
        ConstantValue::Exp(ref exp) => walk_exp(exp)
    }
}

pub fn walk_const_decl(con: &ConstantDeclaration) -> String {
    format!("const {} {} = {}", con.typ, con.name, walk_const_value(&con.value))
}

pub fn walk_var_decl(var: &VariableDeclaration) -> String {
    if var.array_size.is_some() {
        format!("var {} {}[{}]", var.typ, var.name, walk_array_index(var.array_size.as_ref().unwrap()))
    } else {
        format!("var {} {}", var.typ, var.name)
    }
}

pub fn walk_var_decl_list(vec: &Vec<VariableDeclaration>) -> String {
    if vec.len() == 0 { return "".to_string(); }
    vec.iter().map(|decl|walk_var_decl(decl)).join(";\n")
}

fn walk_param_list(params: &Vec<VariableDeclaration>) -> String {
   params.iter()
       .map(|v| format!("var int {}", v.name))
       .collect::<Vec<String>>()
       .join(", ")
}

pub fn walk_if_branch(branch: &IfBranch) -> String {
    format!("if ({}) {{\n{}}}", walk_exp(&branch.cond), walk_statement_list(&branch.body))
}

pub fn walk_if(if_s: &IfStatement) -> String {
    let b = if_s.branches.iter().map(|branch| walk_if_branch(branch)).join(" else ");
    let e = if_s.else_branch.as_ref()
        .map_or(
            "".to_string(),
            |state|format!(" else {{\n{}}}", walk_statement_list(state))
        );

    b + &e
}

fn walk_return_statement(exp: &Expression) -> String {
    format!("return {}", walk_exp(exp))
}

fn walk_assignment_operator(op: &AssignmentOperator) -> String {
    let res = match *op {
        AssignmentOperator::Eq => "=",
        AssignmentOperator::PlusEq => "+=",
        AssignmentOperator::MinusEq => "-=",
        AssignmentOperator::MultiplyEq => "*=",
        AssignmentOperator::DivideEq => "/="
    };

    res.to_string()
}
fn walk_assignment(ass: &Assignment) -> String {
    format!("{} {} {}", walk_var_access(&ass.var), walk_assignment_operator(&ass.op), walk_exp(&ass.exp))
}

fn walk_statement(statement: &Statement) -> String {
    let res = match *statement {
        Statement::Exp(ref exp) => walk_exp(exp),
        Statement::If(ref if_s) => walk_if(if_s),
        Statement::Ass(ref ass) => walk_assignment((ass)),
        Statement::VarDeclaration(ref var) => walk_var_decl_list(var),
        Statement::ConstDeclaration(ref con) => walk_const_decl(con),
        Statement::ConstArrayDeclaration(ref con_arr) => walk_const_array_decl(con_arr),
        Statement::ReturnStatement(ref exp) => walk_return_statement(exp)
    };

    format!("{};", res)
}

fn walk_statement_list(statements: &StatementList) -> String {
    statements.statements
        .iter()
        .map(|s|format!("{}\n", walk_statement(s)))
        .collect::<String>()
}

pub fn walk_func_decl(func: &Function) -> String {
    format!("func {} {}({}) {{\n{}}}",
            func.typ,
            func.name,
            walk_param_list(&func.params),
            walk_statement_list(&func.body.as_ref().unwrap()))
}

pub fn walk_class_decl(class: &Class) -> String {
    let member_strings = class.members.iter()
        .map(|var_decl| format!("{};\n", walk_var_decl(var_decl))).collect::<String>();

    format!("class {} {{\n{}}}", class.name, member_strings )
}

pub fn walk_inst_decl(inst: &Instance) -> String {
    format!("instance {} ({}) {{\n{}}}",
            inst.name,
            inst.class,
            inst.body.as_ref().map_or("".to_string(), |s|walk_statement_list(s)))
}

pub fn walk_proto_decl(proto: &Prototype) -> String {
    format!("prototype {} ({}) {{\n{}}}",
            proto.name,
            proto.class,
            proto.body.as_ref().map_or("".to_string(), |s|walk_statement_list(s)))
}






















