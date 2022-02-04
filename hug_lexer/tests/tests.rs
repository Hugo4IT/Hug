use hug_lexer::{run_test, tokenizer::{TokenKind, KeywordKind, LiteralKind, Base}};

const COMMENTS_PROGRAM: &str = r###"
// Hello
// This is a test
/* Of multiple diffirent comment types */
//
/**/
/*/ */
/ /
"###;

const COMMENTS_EXPECTED_RESULT: &[(TokenKind, usize)] = &[
    (TokenKind::Whitespace, 1),
    (TokenKind::LineComment, 9),
    (TokenKind::LineComment, 18),
    (TokenKind::BlockComment, 42),
    (TokenKind::LineComment, 3),
    (TokenKind::BlockComment, 5),
    (TokenKind::BlockComment, 7),
    (TokenKind::Divide, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Divide, 1),
    (TokenKind::Whitespace, 1),
];

#[test]
fn comments() {
    run_test(COMMENTS_PROGRAM, COMMENTS_EXPECTED_RESULT);
}

const VARIABLES_PROGRAM: &str = r###"
var some_Weird_hellishName: MyType[] = [MyType(50), MyType(13)];
var is_snake_case_epic = true;
var andCamelCaseTooRight = true;
var this_is_a_string = "wowie";
var this_is_a_formatted_string = f"This is wowie: {wowie}";
var this_is_an_escaped_string = "Hehehe, \"";
"###;

const VARIABLES_EXPECTED_RESULT: &[(TokenKind, usize)] = &[
    (TokenKind::Whitespace, 1), // \n

    // var some_Weird_hellishName: MyType[] = [MyType(50), MyType(13)];

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

    // var is_snake_case_epic = true;

    (TokenKind::Keyword(KeywordKind::Var), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier, 18),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::Boolean), 4),
    (TokenKind::SemiColon, 1),
    (TokenKind::Whitespace, 1),

    // var andCamelCaseTooRight = true;

    (TokenKind::Keyword(KeywordKind::Var), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier, 20),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::Boolean), 4),
    (TokenKind::SemiColon, 1),
    (TokenKind::Whitespace, 1),

    // var this_is_a_string = "wowie";

    (TokenKind::Keyword(KeywordKind::Var), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier, 16),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::String), 7),
    (TokenKind::SemiColon, 1),
    (TokenKind::Whitespace, 1),

    // var this_is_a_formatted_string = f"This is wowie: {wowie}";

    (TokenKind::Keyword(KeywordKind::Var), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier, 26),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::FormatString), 25),
    (TokenKind::SemiColon, 1),
    (TokenKind::Whitespace, 1),

    // var this_is_an_escaped_string = "Hehehe, \"";

    (TokenKind::Keyword(KeywordKind::Var), 3),
    (TokenKind::Whitespace, 1),
    (TokenKind::Identifier, 25),
    (TokenKind::Whitespace, 1),
    (TokenKind::Assign, 1),
    (TokenKind::Whitespace, 1),
    (TokenKind::Literal(LiteralKind::String), 12),
    (TokenKind::SemiColon, 1),
    (TokenKind::Whitespace, 1),

    (TokenKind::Whitespace, 1), // \n
];

#[test]
fn variables() {
    run_test(VARIABLES_PROGRAM, VARIABLES_EXPECTED_RESULT);
}