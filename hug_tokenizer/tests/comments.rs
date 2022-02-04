use hug_tokenizer::{TokenKind, run_test};

const PROGRAM: &str = r###"
// Hello
// This is a test
/* Of multiple diffirent comment types */
//
/**/
/*/ */
/ /
"###;

const EXPECTED_RESULT: &[(TokenKind, usize)] = &[
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
    run_test(PROGRAM, EXPECTED_RESULT);
}