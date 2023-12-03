mod tests;

pub type State<'a> = &'a str;
pub type ParseResult<'a> = Result<(Token, State<'a>), State<'a>>;

pub trait Parser<'a> {
    fn parse(&self, state: State<'a>) -> ParseResult<'a>;
}

impl<'a, F> Parser<'a> for F
where
    F: Fn(State<'a>) -> ParseResult,
{
    fn parse(&self, state: State<'a>) -> ParseResult<'a> {
        self(state)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    String(String),
    Sequence(Vec<Token>),
    Repeat(Vec<Token>),
    Alternatively(Box<Token>),
    Number(i64),
    Predicate(char),
}

pub fn string<'a, 'b>(string: &'b str) -> impl Parser<'a> + 'b {
    move |state: State<'a>| {
        if let Some(stripped) = state.strip_prefix(string) {
            ParseResult::Ok((Token::String(string.to_string()), stripped))
        } else {
            ParseResult::Err(state)
        }
    }
}

pub fn sequence<'a: 'b, 'b, 'c: 'a + 'b>(
    parsers: Vec<Box<dyn Parser<'a>>>,
) -> impl Parser<'a> + 'b {
    move |mut state: State<'a>| {
        let save = state;
        let mut result = Vec::new();
        for parser in &parsers {
            match parser.parse(state) {
                ParseResult::Ok((token, new_state)) => {
                    result.push(token);
                    state = new_state;
                }
                ParseResult::Err(_) => return ParseResult::Err(save),
            }
        }

        ParseResult::Ok((Token::Sequence(result), state))
    }
}

pub fn repeat<'a, 'b, F: Parser<'a> + 'b>(
    parser: F,
    at_least: usize,
) -> impl Fn(State<'a>) -> ParseResult<'a> + 'b {
    move |mut state: State| {
        let save = state;
        let mut result = Vec::new();
        let mut count = 0;
        while let ParseResult::Ok((token, new_state)) = parser.parse(state) {
            result.push(token);
            state = new_state;
            count += 1;
        }

        if at_least <= count {
            ParseResult::Ok((Token::Repeat(result), state))
        } else {
            ParseResult::Err(save)
        }
    }
}

pub fn alternatively<'a, 'b, F>(parsers: Vec<F>) -> impl Parser<'a> + 'b
where
    F: Fn(State<'a>) -> ParseResult<'a> + 'b,
{
    move |state: State<'a>| {
        let save = state;
        for parser in &parsers {
            if let ParseResult::Ok(ret) = parser(state) {
                return ParseResult::Ok(ret);
            }
        }

        ParseResult::Err(save)
    }
}

pub fn number<'a>() -> impl Parser<'a> {
    move |mut state: State<'a>| {
        let save = state;
        let mut num_str = String::new();

        if let ParseResult::Ok((_, new_state)) = string("-").parse(state) {
            num_str.push('-');
            state = new_state;
        }

        for c in state.chars() {
            if c.is_ascii_digit() {
                num_str.push(c);
            } else {
                break;
            }
        }

        match num_str.parse::<i64>() {
            Ok(num) => ParseResult::Ok((Token::Number(num), &state[num_str.len()..])),
            Err(_) => ParseResult::Err(save),
        }
    }
}

pub fn char_predicate<'a, 'b, P: Fn(char) -> bool + 'b>(predicate: P) -> impl Parser<'a> + 'b {
    move |state: State<'a>| {
        if let Some(c) = state.chars().next() {
            if predicate(c) {
                return ParseResult::Ok((Token::Predicate(c), &state[1..]));
            }
        }
        ParseResult::Err(state)
    }
}
