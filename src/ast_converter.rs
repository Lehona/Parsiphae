use ast::*;


pub fn exp_bin_op_switch(input: (Expression, Option<(BinaryOperator, Expression)>)) -> Result<Expression, &'static str> {
    let (a, opt) = input;

    if opt.is_none() {
        return Ok(a);
    }

    let (op, b) = opt.unwrap();

    Ok(Expression::Binary(Box::new(BinaryOp {op, a, b})))
}

pub fn exp_un_op_switch(input: (u8, Expression)) -> Expression {
    let (op, exp) = input;

    let oper = match op {
        b'+' => UnaryOperator::Plus,
        b'-' => UnaryOperator::Minus,
        b'!' => UnaryOperator::Negate,
        b'~' => UnaryOperator::Flip,
        _ => panic!("Invalid unary Operator")
    };

    let unary = UnaryOp {op: oper, a: exp};

    Expression::Unary(Box::new(unary))
}

pub fn assign_op_switch(input: ((VarAccess, Option<u8>), Expression)) -> Assignment {
    let ((var, op), exp) = input;

    let oper = if op.is_none() {
            AssignmentOperator::Eq
    } else { match op.unwrap() {
        b'+' => AssignmentOperator::PlusEq,
        b'-' => AssignmentOperator::MinusEq,
        b'*' => AssignmentOperator::MultiplyEq,
        b'/' => AssignmentOperator::DivideEq,
        _ => AssignmentOperator::Eq
    }};

    Assignment {var: var, op: oper, exp}
}

pub fn make_if(branches: Vec<IfBranch>, else_branch: Option<StatementList>) -> IfStatement {

    IfStatement { branches, else_branch}
}

pub fn make_func(typ: String, name: String, params: Vec<VariableDeclaration>, body: StatementList) -> Function {

    Function { typ, name, params, body}
}

pub fn make_var_declaration(typ: String, vec: Vec<(String, Option<ArrayIndex>)>) -> Vec<VariableDeclaration> {
    let mut result = Vec::new();

    for (name, array_size) in vec {
        result.push(VariableDeclaration {typ: typ.clone(), name, array_size});
    }

    result
}

pub fn make_var_access(first_string: String, second_string: Option<String>, index: Option<ArrayIndex>) -> VarAccess {
    // In case there is a second string it's an object access (instance.member), so we swap the parameters around.
    if second_string.is_some() {
        VarAccess {name: second_string.unwrap(), instance: Some(first_string), index}
    } else {
        VarAccess{name: first_string, instance: None, index}
    }
}