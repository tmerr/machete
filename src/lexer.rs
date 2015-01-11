use regex::Regex;

pub use self::Token::{Matched, Unmatched};

pub struct Lexer<T> {
    tokens: Vec<(T, Regex)>,
}

impl<T> Lexer<T> {
    pub fn new() -> Lexer<T> {
        Lexer { tokens: vec![] }
    }

    pub fn define_token(&mut self, class: T, regex: Regex) {
        self.tokens.push((class, regex));
    }

    pub fn lex(&self, text: &str) -> TokenIterator<T> {
        TokenIterator::new(&self.tokens[], text)
    }
}

#[derive(PartialEq, Clone, Show)]
pub enum Token<T> {
    Matched(T),
    Unmatched,
}

pub struct TokenIterator<'a, 'b, T: 'a> {
    tokens: &'a [(T, Regex)],
    text: String,
    idx: usize,
}

impl<'a, 'b, T> TokenIterator<'a, 'b, T> {
    fn new(tokens: &'a [(T, Regex)], text: &str) -> TokenIterator<'a, 'b, T> {
        TokenIterator { tokens: tokens, text: String::from_str(text), idx: 0 }
    }
}

impl<'a, 'b, T: Clone> Iterator for TokenIterator<'a, 'b, T> {
    type Item = (Token<T>, &'b str);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.idx == self.text.len() {
            None
        } else {
            let textleft = &self.text[self.idx..];
            for &(ref class, ref regex) in self.tokens.iter() {
                if let Some((begin, end)) = regex.find(textleft) {
                    self.idx += end;
                    return Some((Matched(class.clone()), &textleft[begin..end]));
                }
            }

            let ch = textleft.slice_chars(0, 1);
            self.idx += ch.len();
            Some((Unmatched, ch))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.text.len() - self.idx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn simple_lexer() {
        let mut lexer = Lexer::new();
        lexer.define_token("space", regex!(r"^ +"));
        lexer.define_token("letters", regex!(r"^([a-zA-Z])+"));
        lexer.define_token("numbers", regex!(r"^[0-9]+"));

        let vec: Vec<(Token<&str>, &str)> = lexer.lex("apple bat 42  cat").collect();
        let tokentypes: Vec<&str> = vec.iter().map(|v| match v.0 { Matched(x) => x, _ => panic!("nope") } ).collect();
        let texts: Vec<&str> = vec.iter().map(|v| v.1.clone()).collect();

        assert_eq!(&tokentypes[], ["letters", "space", "letters", "space", "numbers", "space", "letters"]);
        assert_eq!(&texts[], ["apple", " ", "bat", " ", "42", "  ", "cat"]);
    }
}

