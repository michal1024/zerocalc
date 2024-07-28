use super::*;

fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut tokenizer = Tokenizer::new(input);
    std::iter::from_fn(move || {
        match tokenizer.next_token() {
            Token { kind: TokenKind::Eof, ..} => None,
            token => Some(token)
        }
    })
}

#[test]
fn test_tokenizer_loop() {
    let input = "123+0x4f";

    let mut tokenizer = tokenize(&input);

    let token = tokenizer.next().unwrap();

    assert_eq!(TokenKind::Literal(LiteralKind::Int(Base::Dec)), token.kind);
    assert_eq!("123", token.value_from(input));

    let token = tokenizer.next().unwrap();

    assert_eq!(TokenKind::Add, token.kind);
    assert_eq!("+", token.value_from(input));

    let token = tokenizer.next().unwrap();

    assert_eq!(TokenKind::Literal(LiteralKind::Int(Base::Hex)), token.kind);
    assert_eq!("0x4f", token.value_from(input));

    assert_eq!(None, tokenizer.next());
}

#[test]
fn test_ints() {
    let input = "1 23_33 +";
    let token = tokenize(input).next().unwrap();

    assert_eq!(TokenKind::Literal(LiteralKind::Int(Base::Dec)), token.kind);
    assert_eq!("1 23_33 ", token.value_from(input));
}

#[test]
fn test_float() {
    let input = "13.0";
    let token = tokenize(input).next().unwrap();

    assert_eq!(TokenKind::Literal(LiteralKind::Float), token.kind);
    assert_eq!(input, token.value_from(input));
}

#[test]
fn test_fraction_float() {
    let input = ".1";
    let token = tokenize(input).next().unwrap();

    assert_eq!(TokenKind::Literal(LiteralKind::Float), token.kind);
    assert_eq!(input, token.value_from(input));
}

#[test]
fn test_exp_float() {
    let input = ".1e -5";
    let token = tokenize(input).next().unwrap();

    assert_eq!(TokenKind::Literal(LiteralKind::Float), token.kind);
    assert_eq!(input, token.value_from(input));
}

#[test]
fn test_ident() {
    let input = "a0_b";
    let token = tokenize(input).next().unwrap();
    assert_eq!(TokenKind::Ident, token.kind);
    assert_eq!(input, token.value_from(input));
}

#[test]
fn test_string() {
    let input = "\"ab\\\"c\"";
    let token = tokenize(input).next().unwrap();
    assert_eq!(TokenKind::Literal(LiteralKind::String), token.kind);
    assert_eq!(input, token.value_from(input));
}

#[test]
fn test_eating_whitespaces() {
    let input = " 1 + 2 ";
    let tokens: Vec<Token> = tokenize(input).collect();
    let expected = vec![
        Token::new(TokenKind::Literal(LiteralKind::Int(Base::Dec)), 0, 3),
        Token::new(TokenKind::Add, 3, 1),
        Token::new(TokenKind::Literal(LiteralKind::Int(Base::Dec)), 4, 3),
    ];
    assert_eq!(expected, tokens);
}