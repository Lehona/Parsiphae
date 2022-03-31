#[test]
fn test_tokenize_ikarus() {
    let input = include_bytes!("input/Ikarus.d");
    let output: Vec<_> = include_str!("output/Ikarus.tok").lines().collect();
    let tokens = parsiphae::lexer::lex(input).expect("Failed to tokenize");

    assert_eq!(output.len(), tokens.len());

    for i in 0..output.len() {
        assert_eq!(
            tokens[i].stringified(),
            output[i],
            "Token at line {} does not match!",
            i
        );
    }
}

#[allow(unused_variables)]
#[test]
fn test_parse_ikarus() {
    let input = include_bytes!("input/Ikarus.d");
    let output = include_str!("output/Ikarus.ast");

    let tokens = parsiphae::lexer::lex(input).expect("Failed to tokenize");
    let mut parser = parsiphae::parser::parser::Parser::new(tokens);
    let decls = parser.start().expect("Unable to parse");

    // TODO Figure out a way to produce a meaningful diff here to find the difference.
    // assert_eq!(&format!("{:#?}", decls), output);
}
