use itertools::*;
use walker::*;

use symbols::zSymb_Table::zSymbol_Table;
use symbols::zCPar_Symbol::{zCONTENT, zTYPE};



#[derive(Clone)]
pub enum BinaryOperator {Plus, Minus, Multiply, Divide, Mod, LSL, LSR, GT, LT, GE, LE, Eq, NotEq, And, BitAnd, Or, BitOr }

impl BinaryOperator {
    pub fn sign(&self) -> &str {
        match *self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Mod => "%",
            BinaryOperator::LSL => "<<",
            BinaryOperator::LSR => ">>",
            BinaryOperator::GT => ">",
            BinaryOperator::LT => "<",
            BinaryOperator::GE => ">=",
            BinaryOperator::LE => "<=",
            BinaryOperator::Eq => "==",
            BinaryOperator::NotEq => "!=",
            BinaryOperator::And => "&&",
            BinaryOperator::BitAnd => "&",
            BinaryOperator::Or => "||",
            BinaryOperator::BitOr => "|"
        }
    }

    pub fn from(v: Vec<u8>) -> BinaryOperator {
       match v.as_slice() {
            br"+" => BinaryOperator::Plus,
            br"-" => BinaryOperator::Minus,
            br"*" => BinaryOperator::Multiply,
            br"/" => BinaryOperator::Divide,
            br"%" => BinaryOperator::Mod,
            br"<<" => BinaryOperator::LSL,
            br">>" => BinaryOperator::LSR,
            br">" => BinaryOperator::GT,
            br"<" => BinaryOperator::LT,
            br">=" => BinaryOperator::GE,
            br"<=" => BinaryOperator::LE ,
            br"==" => BinaryOperator::Eq,
            br"!=" => BinaryOperator::NotEq,
            br"&&" => BinaryOperator::And,
            br"&" => BinaryOperator::BitAnd,
            br"||" => BinaryOperator::Or,
            br"|" => BinaryOperator::BitOr,
            _ => BinaryOperator::Plus
        }
    }

    pub fn get_order(&self) -> usize {
        match *self {
            BinaryOperator::LSL => 4,
            BinaryOperator::LSR => 4,
            BinaryOperator::BitAnd => 4,
            BinaryOperator::BitOr => 4,
            BinaryOperator::Multiply => 3,
            BinaryOperator::Divide => 3,
            BinaryOperator::Mod => 3,
            BinaryOperator::Plus => 2,
            BinaryOperator::Minus => 2,
            BinaryOperator::GT => 1,
            BinaryOperator::LT => 1,
            BinaryOperator::GE => 1,
            BinaryOperator::LE => 1,
            BinaryOperator::Eq => 1,
            BinaryOperator::NotEq => 1,
            BinaryOperator::And => 0,
            BinaryOperator::Or => 0
        }
    }

    pub fn apply(&self, left: i32, right: i32) -> i32 {
        match *self {
            BinaryOperator::LSL => left << right,
            BinaryOperator::LSR => left >> right,
            BinaryOperator::BitAnd => left & right,
            BinaryOperator::BitOr => left | right,
            BinaryOperator::Multiply => left * right,
            BinaryOperator::Divide => left / right,
            BinaryOperator::Mod => left % right,
            BinaryOperator::Plus => left + right,
            BinaryOperator::Minus => left - right,
            BinaryOperator::GT => (left > right) as i32,
            BinaryOperator::LT => (left < right) as i32,
            BinaryOperator::GE => (left >= right) as i32,
            BinaryOperator::LE => (left <= right) as i32,
            BinaryOperator::Eq => (left == right) as i32,
            BinaryOperator::NotEq => (left != right) as i32,
            BinaryOperator::And => (left != 0 && right != 0) as i32,
            BinaryOperator::Or => (left != 0 || right != 0) as i32
        }
    }

    pub fn needs_parentheses(&self, child: &Expression) -> bool {
        match *child {
            Expression::Float(_) => false,
            Expression::Value(_) => false,
            Expression::Variable(_) => false,
            Expression::Call(_) => false,
            Expression::Unary(_) => false,
            Expression::String(_) => false,
            Expression::Binary(ref bin) => self.get_order() > bin.as_ref().op.get_order()
        }
    }


}

#[derive(Clone)]
pub enum UnaryOperator {Plus, Minus, Flip, Negate}

impl UnaryOperator {
    pub fn apply(&self, val: i32) -> i32 {
        match *self {
            UnaryOperator::Plus => val,
            UnaryOperator::Minus => -val,
            UnaryOperator::Flip => !val,
            UnaryOperator::Negate => if val == 0 { 1 } else { 0 }
        }
    }
}

#[derive(Clone)]
pub struct ExpressionList {
    pub expressions: Vec<Expression>
}

#[derive(Clone)]
pub enum Expression {
    Value(i32),
    Float(f32),
    Variable(VarAccess),
    Binary(Box<BinaryOp>),
    Unary(Box<UnaryOp>),
    Call(Box<Call>),
    String(String)
}

impl Expression {
    pub fn is_float(&self) -> bool {
        match *self {
            Expression::Float(_) => true,
            _ => false
        }
    }

    pub fn is_constant_int(&self) -> bool {
        match *self {
            Expression::Value(_) => true,
            Expression::Float(_) => false,
            Expression::Variable(_) => true,
            Expression::Binary(_) => true,
            Expression::Unary(_) => true,
            Expression::Call(_) => false,
            Expression::String(_) => false
        }
    }
    pub fn is_constant(&self) -> bool {
        match *self {
            Expression::Value(_) => true,
            Expression::Float(_) => true,
            Expression::Variable(ref var) => var.is_constant(),
            Expression::Binary(ref bin) => bin.is_constant(),
            Expression::Unary(ref un) => un.is_constant(),
            Expression::Call(_) => false,
            Expression::String(_) => true
        }
    }

    pub fn fold(&self, table: &zSymbol_Table) -> ConstantFoldedValue {
        match *self {
            Expression::Value(i) => ConstantFoldedValue::Int(i),
            Expression::Binary(ref bin) => bin.fold(table),
            Expression::Unary(ref un) => un.fold(table),
            Expression::String(ref s) => ConstantFoldedValue::String(s.to_string()),
            Expression::Variable(ref var) => var.fold(table),
            _ => panic!("Trying to fold non-const value")
        }
    }
}

#[derive(Clone)]
pub struct VarAccess {
    pub name: String,
    pub instance: Option<String>,
    pub index: Option<ArrayIndex>
}

impl VarAccess {
    pub fn fold(&self, table: &zSymbol_Table) -> ConstantFoldedValue {
        println!("va: {}", &self.name);
        let symb_id = table.symbID_by_name(&self.name);
        let symb = table.symb_by_ID(symb_id);

        match symb.typ {
            zTYPE::Func => return ConstantFoldedValue::Int(symb_id as i32),
            _ => ()
        }

        table.symb_by_name(&self.name).fold_const_value(table)
    }


    pub fn is_constant(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct UnaryOp {
    pub op: UnaryOperator,
    pub a: Expression
}

impl UnaryOp {
    pub fn is_constant(&self) -> bool {
        self.a.is_constant()
    }

    pub fn fold(&self, table: &zSymbol_Table) -> ConstantFoldedValue {
        let val_folded = self.a.fold(table);

        let val = match val_folded {
            ConstantFoldedValue::Int(i) => i,
            _ => panic!("trying to fold non-int value in a unary expression")
        };

        ConstantFoldedValue::Int(self.op.apply(val))
    }
}

#[derive(Clone)]
pub struct BinaryOp {
    pub op: BinaryOperator,
    pub a: Expression,
    pub b: Expression
}

impl BinaryOp {
    pub fn is_constant(&self) -> bool {
        self.a.is_constant() && self.b.is_constant()
    }

    pub fn fold(&self, table: &zSymbol_Table) -> ConstantFoldedValue {
        let left = self.a.fold(table);
        let right = self.b.fold(table);

        let left_value =  match left {
            ConstantFoldedValue::Int(i) => i,
            _ => panic!("trying to fold a non-int in a binary expression")
        };

        let right_value =  match right {
            ConstantFoldedValue::Int(i) => i,
            _ => panic!("trying to fold a non-int in a binary expression")
        };

        ConstantFoldedValue::Int(self.op.apply(left_value, right_value))
    }
}

#[derive(Clone)]
pub struct Call {
    pub func: String,
    pub params: ExpressionList
}

#[derive(Clone)]
pub enum ArrayIndex {
    Identifier(String),
    Number(i32)
}

impl ArrayIndex {
    pub fn make_number(&self, table: &zSymbol_Table) -> ArrayIndex {
        match *self {
            ArrayIndex::Number(i) => ArrayIndex::Number(i),
            ArrayIndex::Identifier(ref s) => {
                let symb = table.symb_by_name(s);
                match *symb.content.as_ref().expect("trying to resolve an identifier in make_number without content") {
                    zCONTENT::array(_) => panic!("make number resolved an identifier to an array!"),
                    zCONTENT::single(ref s) => {
                        match *s {
                            ConstantFoldedValue::Int(i) => ArrayIndex::Number(i),
                            _ => panic!("make_number resolved an identifier that was not an int!")
                        }
                    }
                }
            }
        }
    }

    pub fn as_number(&self) -> i32 {
        match *self {
            ArrayIndex::Number(i) => i,
            _ => panic!("trying to cast an array_index::identifier to a number")
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            ArrayIndex::Identifier(_) => true,
            ArrayIndex::Number(i) => i > 1
        }
    }
    pub fn single_value(&self) -> bool {
        match *self {
            ArrayIndex::Identifier(_) => false,
            ArrayIndex::Number(i) => i == 1
        }
    }
    pub fn to_string(&self) -> String {
        match *self {
            ArrayIndex::Identifier(ref s) => s.to_string(),
            ArrayIndex::Number(ref i) => i.to_string()
        }
    }
}




// --------------------------------------
// ----------- STATEMENTS ---------------
// --------------------------------------


pub struct StatementList {
    pub statements: Vec<Statement>
}

pub enum Statement {
    Exp(Expression),
    Ass(Assignment),
    If(Box<IfStatement>),
    VarDeclaration(Vec<VariableDeclaration>),
    ConstDeclaration(ConstantDeclaration),
    ConstArrayDeclaration(ConstantArrayDeclaration),
    ReturnStatement(Expression)
}

pub enum AssignmentOperator {
    PlusEq, MinusEq, MultiplyEq, DivideEq, Eq
}

pub struct Assignment {
    pub var: VarAccess,
    pub op: AssignmentOperator,
    pub exp: Expression
}

pub struct IfBranch {
    pub cond: Expression,
    pub body: StatementList
}

pub struct IfStatement {
    pub branches: Vec<IfBranch>,
    pub else_branch: Option<StatementList>
}

pub struct Instance {
    pub name: String,
    pub class: String,
    pub body: Option<StatementList>
}

pub struct Prototype {
    pub name: String,
    pub class: String,
    pub body: Option<StatementList>
}

pub struct Class {
    pub name: String,
    pub members: Vec<VariableDeclaration>
}

pub struct Function {
    pub typ: String,
    pub name: String,
    pub params: Vec<VariableDeclaration>,
    pub body: Option<StatementList>
}

#[derive(Clone)]
pub enum ConstArrayInitializerList {
    Strings(Vec<String>),
    Numbers(Vec<Expression>)
}

impl ConstArrayInitializerList {
    pub fn fold(&self, table: &zSymbol_Table) -> Vec<ConstantFoldedValue> {
        match *self {
            ConstArrayInitializerList::Strings(ref vec) =>
                        vec.iter().map(|s|ConstantFoldedValue::String(s.to_owned())).collect(),
            ConstArrayInitializerList::Numbers(ref vec) =>
                        vec.iter().map(|exp|exp.fold(table)).collect()
        }
    }
    pub fn to_string(&self) -> String {
        match *self {
            ConstArrayInitializerList::Strings(ref vec) => format!("{{ \"{}\" }}", vec.iter().join("\", \"")),
            ConstArrayInitializerList::Numbers(ref vec) => format!("{{ {} }}", vec.iter().map(walk_exp).join(", "))
        }
    }
}

pub struct ConstantArrayDeclaration {
    pub typ: String,
    pub name: String,
    pub array_size: ArrayIndex,
    pub values: ConstArrayInitializerList
}

pub struct ConstantDeclaration {
    pub typ: String,
    pub name: String,
    pub value: ConstantValue
}

pub enum ConstantFoldedValue {
    Float(f32),
    Int(i32),
    String(String)
}

impl ToString for ConstantFoldedValue {
    fn to_string(&self) -> String {
        match *self {
            ConstantFoldedValue::Float(f) => f.to_string(),
            ConstantFoldedValue::Int(i) => i.to_string(),
            ConstantFoldedValue::String(ref s) => s.to_string()
        }
    }
}



#[derive(Clone)]
pub enum ConstantValue {
    Float(f32),
    Exp(Expression)
}

impl ConstantValue {
    pub fn fold(&self, table: &zSymbol_Table) -> ConstantFoldedValue {
        match *self {
            ConstantValue::Float(f) => ConstantFoldedValue::Float(f),
            ConstantValue::Exp(ref exp) => exp.fold(table.clone())
        }
    }
}

pub struct VariableDeclaration {
    pub typ: String,
    pub name: String,
    pub array_size: Option<ArrayIndex>
}

impl VariableDeclaration {
    pub fn get_size(&self) -> i32 {
        match self.typ.as_ref() {
            "INT" => 4,
            "FLOAT" => 4,
            "STRING" => 20,
            _ => 0
        }
    }
}

pub enum Symbol {
    Var(Vec<VariableDeclaration>),
    Func(Function),
    Class(Class),
    Inst(Instance),
    Proto(Prototype),
    Const(ConstantDeclaration),
    ConstArray(ConstantArrayDeclaration)
}












