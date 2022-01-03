enum Checking {
    Identifier, SpaceAfterIdentifier, Operator(char), ParenOpen, ParenClose, None
}

use Checking::*;

pub enum Status { Ok, Unexpected(usize, char), ExpectedAtEnd(String), Msg(String) }

macro_rules! decrement_or_unexpected {
    ($paren_depth: ident, ($i: ident, $ch: ident)) => {{
        if $paren_depth == 0 {
            return Status::Unexpected($i, $ch)
        } else {
            $paren_depth -= 1
        }
    }}
}

pub fn validar(expr: &str) -> Status {
    if expr.is_empty() || expr.chars().all(|x| x == ' ') {
        return Status::Msg("expresion vacia".to_string())
    }

    let mut last = Checking::None;
    let mut paren_depth: usize = 0;

    for (i, ch) in expr.chars().enumerate() {
        match last {
            None | ParenOpen => {
                match ch {
                    x if x.is_alphabetic() => last = Identifier,
                    '!' => last = Operator(ch),
                    '(' => {
                        paren_depth += 1;
                        last = ParenOpen
                    },
                    ' ' => (),
                    _ => return Status::Unexpected(i, ch)
                }
            },
            Identifier => {
                match ch {
                    x if x.is_alphabetic() => (),
                    '&' | '|' | '<' | '=' => last = Checking::Operator(ch),
                    ')' => {
                        decrement_or_unexpected!(paren_depth, (i, ch));
                        last = Checking::ParenClose
                    },
                    ' ' => last = SpaceAfterIdentifier,
                    _ => return Status::Unexpected(i, ch)
                }
            },
            Operator(last_ch) => {
                match last_ch {
                    '!' | '&' | '|' | '>' => match ch {
                        x if x.is_alphabetic() => last = Identifier,
                        '!' => last = Operator(ch),
                        '(' => {
                            paren_depth += 1;
                            last = ParenOpen
                        },
                        ' ' => (),
                        _ => return Status::Unexpected(i, ch)
                    },
                    '<' => match ch {
                        '=' => last = Operator(ch),
                        _ => return Status::Unexpected(i, ch)
                    },
                    '=' => match ch {
                        '>' => last = Operator(ch),
                        _ => return Status::Unexpected(i, ch)
                    },
                    _ => return Status::Unexpected(i, ch)
                }
            },
            SpaceAfterIdentifier | ParenClose => {
                match ch {
                    '&' | '|' | '<' | '=' => last = Operator(ch),
                    ')' => {
                        decrement_or_unexpected!(paren_depth, (i, ch));
                        last = ParenClose
                    },
                    ' ' => (),
                    _ => return Status::Unexpected(i, ch)
                }
            }
        }
    }

    if paren_depth != 0 {
        return Status::ExpectedAtEnd("')'".to_string())
    }

    match last {
        ParenOpen => unreachable!("Este caso debería haberse detectado antes"),
        Operator(_) => Status::ExpectedAtEnd("expresion".to_string()),
        _ => Status::Ok
    }
}