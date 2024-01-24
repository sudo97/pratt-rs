macro_rules! to_token {
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
    ($num:literal) => {
        Token::Number($num)
    };
    (lp) => {
        Token::LParen
    };
    (rp) => {
        Token::RParen
    };
}

macro_rules! to_tokens {
    ($t:tt) => {to_token!($t)};
    ($t:tt $($token:tt)+) => {
        {
            vec![to_token!($t), $(to_token!($token)),+]
        }
    };
}

macro_rules! to_test {
    ($f:ident, $e:expr, $($token:tt)+) => {
        {
            let tokens = to_tokens!($($token)+);
            $f(tokens, $e);
            println!("=====");
        }
    };
}
