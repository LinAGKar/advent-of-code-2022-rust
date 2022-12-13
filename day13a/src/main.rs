use std::cmp::Ordering;
use std::io::Read;
use std::iter::{Iterator, Peekable};
use std::str::Chars;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Token {
    Lbrace,
    Rbrace,
    Number(u8),
}

struct Tokenizer<'a> {
    src: Peekable<Chars<'a>>,
    buf: String,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            src: src.chars().peekable(),
            buf: String::new(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.src.peek() == Some(&',') {
            self.src.next();
        }
        match self.src.next() {
            Some('[') => Some(Token::Lbrace),
            Some(']') => Some(Token::Rbrace),
            Some(x) => {
                assert!(x.is_ascii_digit());
                self.buf.clear();
                self.buf.push(x);
                while let Some(&x @ '0'..='9') = self.src.peek() {
                    self.buf.push(x);
                    self.src.next();
                }
                Some(Token::Number(self.buf.parse().unwrap()))

            }
            None => None,
        }
    }
}

fn casting_compare(iter: &mut Tokenizer, b: u8) -> Ordering {
    let mut depth = 1;
    let a;
    loop {
        match iter.next() {
            Some(Token::Lbrace) => depth += 1,
            Some(Token::Rbrace) => return Ordering::Less,
            Some(Token::Number(x)) => { a = x; break; }
            None => panic!(),
        };
    }
    let ord = a.cmp(&b);
    if ord.is_ne() {
        return ord;
    }
    for _ in 0..depth {
        if iter.next().unwrap() != Token::Rbrace {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

fn cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let mut iters = [Tokenizer::new(a), Tokenizer::new(b)];

    loop {
        let mut tokens = [None; 2];
        for (token, iter) in tokens.iter_mut().zip(&mut iters) {
            *token = iter.next();
        }

        if let Some(result) = match tokens {
            [None, None] => Some(Ordering::Equal),
            [None, Some(_)] | [Some(_), None] => panic!(),
            [Some(Token::Lbrace), Some(Token::Lbrace)] => None,
            [Some(Token::Rbrace), Some(Token::Rbrace)] => None,
            [Some(Token::Rbrace), Some(Token::Lbrace | Token::Number(_))] => Some(Ordering::Less),
            [Some(Token::Lbrace | Token::Number(_)), Some(Token::Rbrace)] => Some(Ordering::Greater),
            [Some(Token::Number(a)), Some(Token::Number(b))] => Some(a.cmp(&b)).filter(|x| x.is_ne()),
            [Some(Token::Lbrace), Some(Token::Number(b))] => {
                Some(casting_compare(&mut iters[0], b)).filter(|x| x.is_ne())
            }
            [Some(Token::Number(a)), Some(Token::Lbrace)] => {
                Some(casting_compare(&mut iters[1], a)).filter(|x| x.is_ne()).map(|x| x.reverse())
            }
        } {
            return result;
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let mut count = 0;
    let mut sum = 0;
    while let (Some(a), Some(b)) = (lines.next(), lines.next()) {
        count += 1;
        if cmp(a, b) != Ordering::Greater {
            sum += count;
        }
        lines.next();
    }
    println!("{}", sum);
}
