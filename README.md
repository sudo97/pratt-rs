# Dead Simple Pratt Parser

This might be inaccurate implementation. It also might be buggy. But if you just like me struggle to understand spaghetti from these articles:
  - [this one is actually fine](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
  - [this one I was going to read if I don't make any progress with this implemtation](https://chidiwilliams.com/posts/on-recursive-descent-and-pratt-parsing)
  - [and this](https://abarker.github.io/typped/pratt_parsing_intro.html)
  - [this is quite good](https://gist.github.com/aisamanra/e52791fcea7b75905c68)
  - [now this is too much for me](https://github.com/jwurzer/bantam-rust/blob/master/src/bantam/parser.rs)
  - [as well as this(it's the basis of previous one)](https://github.com/munificent/bantam/tree/master/src/com/stuffwithstuff/bantam)
  - [and this, this is where it all started](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
  - [too much](http://crockford.com/javascript/tdop/tdop.html)
  - [a little spaghetti to me](https://blog.bruce-hill.com/packrat-parsing-from-scratch)
  - [finally this is what made me learn it](https://craftinginterpreters.com/compiling-expressions.html#single-pass-compilation)

well.. then you might find my attempt of making my own spaghetti helpful.

## Main idea

Main idea was to make the tiniest language possible. It has only these tokens:
```rust
enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
}
```

that's it. So it's not really a programming language, obviously it's Turing incomplete, but at least this is almost the amount of lines I can keep in my head at once, so if you're just like me, welcome.

## What I did differently from most of the implementations

I didn't create dynamic hash-maps, in Rust it is quite a nightmare to put closures inside hash-maps, and I guess it's only needed in some cases when ceratin user/consumer of my code might need to extend a number of operators. I don't need this, so instead of hash maps I just do good old `match` expression.

I also didn't produce any proper `Expression` tree. Even though I created `Expr` type alias for simplicity, it's just a vector of stack operations, so basically we compile our language into the bytecode in one run(cool huh?). I did this because this is what Crafting Interpreters does it, so I might as well use this draft as an inspiration for my future studies of that book.

## Bugs

I would be surprised if there aren't any, I made it in one sit. Please open an issue.

Also if you're a seasoned Rustacean think that some idioms or datastructures I used are not ideal, make an issue as well. This is very appreciated, because I was just going with compiler suggestion 45% of the time, other 45% I was using GPT-4, and the last 10% were just my vision on what would make this algorythm easily read.

## ~~TODO~~

~~I think I'll add brackets...~~

Added

## to_tokens macro

After I added support for LParen and RParen, I realized it's painful to test things like so:
```rust
let prog = vec![Token::Number(2), Token::Plus, Token::Number(2)];
```

Imagine writing more complex expressions. So I decided trying to use rust macro system, which allows to traverse the stream of tokens(so I thought). I wanted to be able to write
```rust
let prog = to_tokens!(2 + 2); // and it would expand to the former call
```

then I wrapped it into another thing called `to_test` which also accepted function to call. Initially I wanted it to look like this:
```rust
to_test(test_program, 2 + 2);
```
But the problem is that Rust doesn't allow to match for `(` or `)` in the macros, so instead of parentheses I had to use some other characters instead. I settled with this:

```rust
macro_rules! to_token {
    ...
    ("(") => {
        Token::LParen
    };
    (")") => {
        Token::RParen
    };
    ...
}
```

so now it looks like this:
```rust
    to_test!(test_program, 2 + 2 * 2, 2 + 2 * 2);
    to_test!(test_program, 2 * (2 + 2), 2 * "(" 2 + 2 ")");
```

Not ideal, but better than it used to be. As always suggestions are welcome, maybe I don't know something

UPD: thanks to [this reply](https://www.reddit.com/r/rust/comments/19eikn5/comment/kjdme3m/), I managed to make it work