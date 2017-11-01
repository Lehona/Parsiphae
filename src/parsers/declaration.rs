use parsers::*;
use parsers::util::ExpectParser;
use parsers::statement_parsers::statement_list;


pub fn instance() -> Parser<'static, u8, Instance> {
    let parser = seq_s(b"INSTANCE")
        * identifier()
        - sym_s(b'(')
        + identifier()
        - sym_s(b')')
        + (sym_s(b'{')
        * statement_list()
        - sym_s(b'}')).opt();

    parser.map(|((name, class), body)|Instance {name, class, body})
}

pub fn prototype() -> Parser<'static, u8, Prototype> {
    let parser = seq_s(b"PROTOTYPE")
        * identifier()
        - sym_s(b'(')
        + identifier()
        - sym_s(b')')
        + (sym_s(b'{')
        * statement_list()
        - sym_s(b'}')).opt();

    parser.map(|((name, class), body)|Prototype {name, class, body})
}

pub fn class() -> Parser<'static, u8, Class> {
    let parser = seq_s(b"CLASS") * identifier() - sym_s(b'{') + (var_declaration() - sym_s(b';')).repeat(..) - sym_s(b'}');

    let parser_mapper =
    |(name, members): (String, Vec<Vec<VariableDeclaration>>)| -> Class {
        Class {name, members: flatten(members)}
    };

    parser.map(parser_mapper)
}

pub fn func() -> Parser<'static, u8, Function> {
    let parser = seq_s(b"FUNC")
        * (identifier()
            - whitespace()
            + identifier() // name
            - sym_s(b'(')
            + var_declaration_list() // params
            - sym_s(b')')
            - sym_s(b'{')
            + statement_list() // body
            - sym_s(b'}'))
        .expect("failed to parse fn");

    parser.map(|(((t, n), p), b)| make_func(t, n, p, b))
}

pub fn number_array_initializer() -> Parser<'static, u8, ConstArrayInitializerList> {
    (expression_parser() + (sym_s(b',') * expression_parser()).repeat(..))
        .map(|(first, mut vec)| { vec.insert(0, first); vec})
        .map(|vec|ConstArrayInitializerList::Numbers(vec))
}

pub fn string_array_initializer() -> Parser<'static, u8, ConstArrayInitializerList> {
    (string_parser() + (sym_s(b',') * string_parser()).repeat(..))
        .map(|(first, mut vec)| { vec.insert(0, first); vec})
        .map(|vec|ConstArrayInitializerList::Strings(vec))
}

pub fn const_string_array_declaration() -> Parser<'static, u8, ConstantArrayDeclaration> {
    let parser = seq_s(b"CONST")
        * seq_s(b"STRING").map(|_|"STRING".to_string())
        + boundary(identifier())
        - sym(b'[')
        + array_index()
        - sym(b']')
        - sym_s(b'=')
        - sym_s(b'{')
        + string_array_initializer()
        - sym_s(b'}');

    parser.map(|(((typ, name), array_size), values)|ConstantArrayDeclaration {typ, name, array_size, values })
}

pub fn const_int_array_declaration() -> Parser<'static, u8, ConstantArrayDeclaration> {
    let parser = seq_s(b"CONST")
        * seq_s(b"INT").map(|_|"INT".to_string())
        + boundary(identifier())
        - sym(b'[')
        + array_index()
        - sym(b']')
        - sym_s(b'=')
        - sym_s(b'{')
        + number_array_initializer()
        - sym_s(b'}');

    parser.map(|(((typ, name), array_size), values)| ConstantArrayDeclaration { typ, name, array_size, values })
}

pub fn const_array_declaration() -> Parser<'static, u8, ConstantArrayDeclaration> {
    const_int_array_declaration() | const_string_array_declaration()
}

pub fn const_declaration() -> Parser<'static, u8, ConstantDeclaration> {
    let parser = seq_s(b"CONST")
        * identifier()
        + boundary(identifier())
        - sym_s(b'=')
        + (float().map(|f|ConstantValue::Float(f))
            |expression_parser().map(|exp|ConstantValue::Exp(exp)));

    parser.map(|((typ, name), value)|ConstantDeclaration {typ, name, value})
}

pub fn name_index_pair() -> Parser<'static, u8, (String, Option<ArrayIndex>)> {
    boundary(!seq(b"VAR") * identifier()) + (sym_s(b'[') * array_index() - sym_s(b']')).opt()
}

pub fn var_declaration() -> Parser<'static, u8, Vec<VariableDeclaration>> {
    let parser = seq_s(b"VAR")
        * identifier()
        + (name_index_pair() + (sym_s(b',') * name_index_pair()).repeat(..)).map(push_identity);

    parser.map(|(typ, vec)|make_var_declaration(typ, vec))
}

pub fn var_declaration_list() -> Parser<'static, u8, Vec<VariableDeclaration>> {
    list(var_declaration(), sym_s(b','))
        .map(|v|flatten(v))
    | empty().map(|_|Vec::new())
}

pub fn declaration() -> Parser<'static, u8, Symbol> {
    let decl = func().name("FUNC").map(|f|Symbol::Func(f))
    | var_declaration().map(|v|Symbol::Var(v))
    | const_declaration().map(|c|Symbol::Const(c))
    | const_array_declaration().map(|ca|Symbol::ConstArray(ca))
    | instance().map(|i|Symbol::Inst(i))
    | prototype().map(|p|Symbol::Proto(p))
    | class().name("CLASS").map(|c|Symbol::Class(c));

    decl - sym_s(b';')
}

pub fn declaration_list() -> Parser<'static, u8, Vec<Symbol>> {
    whitespace() * declaration().repeat(1..)
}