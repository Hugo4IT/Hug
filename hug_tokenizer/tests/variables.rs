use hug_tokenizer::{TokenKind, run_test, KeywordKind, LiteralKind, Base};

const PROGRAM: &str = r###"
var some_Weird_hellishName: MyType[] = [MyType(50), MyType(13)];
"###;

const EXPECTED_RESULT: &[(TokenKind, usize)] = &[
    (TokenKind::Whitespace, 1),
    (TokenKind::Keyword(KeywordKind::Var), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier, 22),
    (TokenKind::Colon, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier, 6),
    (TokenKind::OpenBracket, 1),
    (TokenKind::CloseBracket, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::OpenBracket, 1),
    (TokenKind::Identifier, 6),
    (TokenKind::OpenParenthesis, 1),
    (TokenKind::Literal(LiteralKind::Integer(Base::Decimal)), 2),
    (TokenKind::CloseParenthesis, 1),
    (TokenKind::Comma, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier, 6),
    (TokenKind::OpenParenthesis, 1),
    (TokenKind::Literal(LiteralKind::Integer(Base::Decimal)), 2),
    (TokenKind::CloseParenthesis, 1),
    (TokenKind::CloseBracket, 1),
    (TokenKind::SemiColon, 1),
    (TokenKind::Whitespace, 1),
];

#[test]
fn comments() {
    run_test(PROGRAM, EXPECTED_RESULT);
}