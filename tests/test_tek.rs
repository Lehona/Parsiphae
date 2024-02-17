use parsiphae::{
    lexer::Lexer,
    parser::parser::Parser,
    ppa::{
        errors::TypecheckErrorVariant, symbol_collector::SymbolCollector, typecheck::TypeChecker,
        visitor,
    },
    types::{SymbolCollection, AST},
};
use strum::IntoEnumIterator;

#[test]
fn test_tek() {
    for variant in TypecheckErrorVariant::iter() {
        let code = match variant {
            TypecheckErrorVariant::InternalFailure => continue,
            TypecheckErrorVariant::UnknownIdentifier => "func void baz() { foo.bar; };",
            TypecheckErrorVariant::UnknownReturnType => "func bar foo() {};",
            TypecheckErrorVariant::UnknownParameterType => "func void foo(var bar baz) {};",
            TypecheckErrorVariant::UnknownFunctionCall => "func void foo() { bar(); };",
            TypecheckErrorVariant::UnknownVariableType => "func void foo() { var bar fox; };",
            TypecheckErrorVariant::FunctionCallWrongType => {
                "var int bar; func void foo() { bar(); };"
            }
            TypecheckErrorVariant::UnknownIdentifierInExpression => "func void foo() { 3 + a; };",
            TypecheckErrorVariant::IdentifierIsClassInExpression => {
                "class fox { var int bar; }; func void foo() { fox + 3; };"
            }
            TypecheckErrorVariant::FunctionCallParameterWrongType => {
                "func void foo(var int bar) { foo(\"hi\"); };"
            }
            TypecheckErrorVariant::FunctionCallWrongAmountOfParameters => {
                "func void foo(var int a, var int b) { foo(1, 2, 3); };"
            }
            TypecheckErrorVariant::BinaryExpressionNotInt => "func void foo() { 3 + 3.5; };",
            TypecheckErrorVariant::UnaryExpressionNotInt => "func void foo() { !3.5; };",
            TypecheckErrorVariant::AssignmentWrongTypes => {
                "func void foo() { var int a; a = 3.5; };"
            }
            TypecheckErrorVariant::WrongTypeInArrayInitialization => {
                "const int arr[3] = {1, \"hello\", 3};"
            }
            TypecheckErrorVariant::CanOnlyAssignToString => {
                "func void foo() { var string s; s += \"bar\"; };"
            }
            TypecheckErrorVariant::CanOnlyAssignToFloat => {
                "func void foo() { var float f; f += 3.5; };"
            }
            TypecheckErrorVariant::CanOnlyAssignToInstance => {
                "class fox {}; func void foo() { var fox bar; bar += bar; };"
            }
            TypecheckErrorVariant::ConditionNotInt => {
                "func void foo() { var string s; if (s) {}; };"
            }
            TypecheckErrorVariant::ReturnExpressionDoesNotMatchReturnType => {
                "func int foo() { var string s; return s; };"
            }
            TypecheckErrorVariant::ReturnWithoutExpression => "func int foo() { return; };",
            TypecheckErrorVariant::ReturnExpressionInVoidFunction => {
                "func void foo() { return 3; };"
            }
            TypecheckErrorVariant::UnknownIdentifierInArraySize => "var int arr[FOO];",
            TypecheckErrorVariant::NonConstantArraySize => "var int foo; var int arr[foo];",
            TypecheckErrorVariant::ArraySizeIsNotInteger => {
                "const string s = \"hi\"; var int arr[s];"
            }
            TypecheckErrorVariant::InstanceHasUnknownParent => "instance foo(bar);",
            TypecheckErrorVariant::InstanceParentNotClassOrProto => {
                "var int bar; instance foo(bar);"
            }
            TypecheckErrorVariant::IdentifierIsNotType => "var int foo; var foo bar;",
            TypecheckErrorVariant::IdentifierIsNotInstance => "func void foo() { foo.bar; };",
            TypecheckErrorVariant::TypeIsPrimitive => "func void foo() { var int bar; bar.fox; };",
            TypecheckErrorVariant::UnknownMember => {
                "class fox { var int bar; }; func void foo() { var fox baz; baz.boz; };"
            }
        };

        let mut visitor = SymbolCollector::new();
        println!("{variant:?}");

        let tokens = Lexer::lex(code.as_bytes()).expect("Unable to tokenize");
        let mut parser = Parser::new(&tokens);
        let result = parser
            .start()
            .map(|declarations| AST { declarations })
            .map_err(|e| e.with_token_start(parser.progress() + 1));

        let ast = result.unwrap();
        visitor::visit_ast(&ast, &mut visitor);

        let symbols = SymbolCollection::with_symbols(visitor.syms);
        let mut typechk = TypeChecker::new(&symbols);
        let _ = typechk.typecheck();
        let actual_variant: TypecheckErrorVariant = (&typechk.errors[0].kind).into();

        assert_eq!(variant, actual_variant);
    }
}
