#[cfg(test)]
use super::*;

#[test]
fn test_str1() {
    assert_eq!(
        string("test").parse("test 1"),
        ParseResult::Ok((Token::String(String::from("test")), " 1"))
    );
    assert_eq!(
        string("").parse("test 1"),
        ParseResult::Ok((Token::String(String::from("")), "test 1"))
    );
    assert_eq!(string("test 1").parse(""), ParseResult::Err(""));
    assert_eq!(
        string("test").parse("not test"),
        ParseResult::Err("not test")
    );
}

#[test]
fn test_seq1() {
    assert_eq!(
        sequence(vec!(
            Box::new(string("hello")),
            Box::new(string(" ")),
            Box::new(string("how"))
        ))
        .parse("hello how are you"),
        ParseResult::Ok((
            Token::Sequence(vec!(
                Token::String(String::from("hello")),
                Token::String(String::from(" ")),
                Token::String(String::from("how")),
            )),
            " are you"
        ))
    );
    assert_eq!(
        sequence(vec!(
            Box::new(string("not")),
            Box::new(string(" ")),
            Box::new(string("how"))
        ))
        .parse("hello how are you"),
        ParseResult::Err("hello how are you")
    );
}

#[test]
fn test_repeat1() {
    assert_eq!(
        repeat(string("hello "), 4)("hello hello hello hello how are you"),
        ParseResult::Ok((
            Token::Repeat(vec!(
                Token::String(String::from("hello ")),
                Token::String(String::from("hello ")),
                Token::String(String::from("hello ")),
                Token::String(String::from("hello ")),
            )),
            "how are you"
        ))
    );
    assert_eq!(
        repeat(string("hello "), 4).parse("hello hello hello how are you"),
        ParseResult::Err("hello hello hello how are you")
    );
}

#[test]
fn test_number1() {
    assert_eq!(
        number().parse("234hello"),
        ParseResult::Ok((Token::Number(234), "hello"))
    );
    assert_eq!(number().parse("j234hello"), ParseResult::Err("j234hello"));
}

#[test]
fn test_predicate() {
    assert_eq!(
        char_predicate(&|c: char| c.is_whitespace()).parse("  hello"),
        ParseResult::Ok((Token::Predicate(' '), " hello"))
    );
    assert_eq!(
        repeat(char_predicate(&|c: char| c.is_whitespace()), 1).parse("   hello"),
        ParseResult::Ok((
            Token::Repeat(vec![
                Token::Predicate(' '),
                Token::Predicate(' '),
                Token::Predicate(' ')
            ]),
            "hello"
        ))
    );
}

#[test]
fn test_mixed() {
    assert_eq!(
        sequence(vec![Box::new(string("Game ")), Box::new(number())]).parse("Game 34"),
        ParseResult::Ok((
            Token::Sequence(vec![
                Token::String(String::from("Game ")),
                Token::Number(34)
            ]),
            ""
        ))
    );
}
