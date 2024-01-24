macro_rules! to_tokens_impl {
    (($($parsed:tt)*) + $($rest:tt)*) => { to_tokens_impl!(($($parsed)* Token::Plus,) $($rest)*) };
    (($($parsed:tt)*) - $($rest:tt)*) => { to_tokens_impl!(($($parsed)* Token::Minus,) $($rest)*) };
    (($($parsed:tt)*) * $($rest:tt)*) => { to_tokens_impl!(($($parsed)* Token::Star,) $($rest)*) };
    (($($parsed:tt)*) / $($rest:tt)*) => { to_tokens_impl!(($($parsed)* Token::Slash,) $($rest)*) };
    (($($parsed:tt)*) (rparen) $($rest:tt)*) => { to_tokens_impl!(($($parsed)* Token::RParen,) $($rest)*) };
    (($($parsed:tt)*) ($($inner:tt)*) $($rest:tt)*) => { to_tokens_impl!(($($parsed)* Token::LParen,) $($inner)* (rparen) $($rest)*) };
    (($($parsed:tt)*) $num:literal $($rest:tt)*) => { to_tokens_impl!(($($parsed)* Token::Number($num),) $($rest)*) };
    (($($parsed:tt)*)) => { vec![$($parsed)*] };
}

macro_rules! to_tokens {
    ($($tokens:tt)+) => {
        to_tokens_impl!(() $($tokens)*)
    };
}

macro_rules! to_test_inner {
    ($f:ident, $tokens:expr, $($token:tt)+) => {
        {
            let result = ($($token)+);
            $f($tokens, result);
            println!("=====");
        }
    };
}

macro_rules! to_test {
    ($f:ident, $($token:tt)+) => {
        {
            let tokens = to_tokens!($($token)+);
            to_test_inner!($f, tokens, $($token)+);
        }
    };
}
