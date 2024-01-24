macro_rules! to_token {
    ($num:literal) => {
        Token::Number($num)
    };
    (+) => {
        Token::Plus
    };
    (-) => {
        Token::Minus
    };
    (*) => {
        Token::Star
    };
    (/) => {
        Token::Slash
    };
    (() => {
        Token::LParen
    };
    ()) => {
        Token::RParen
    };
}

macro_rules! to_tokens {
    ($f:ident, $($token:tt)+) => {
        {
            let mut tokens = vec![];
            $(
                tokens.push(to_token!($token));
            )+
            $f(tokens, $($token)*);
            println!("=====");
        }
    };
}
