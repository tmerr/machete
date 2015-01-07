use graph::Graph;
use std::path::posix::Path;
use self::TokenClass::{Whitespace, Newline, Comment, IdentifierOrKeyword, Unknown};
use regex::Regex;


pub trait LanguageBackend {
    fn get_extensions(&self) -> Vec<String>;
    fn get_graph_types(&self) -> Vec<GraphType>;
    fn build_graph(&self, filenames: &[Path], graphtype: GraphType) -> Graph<String, ()>;
}


pub struct Csharp;

pub enum GraphType {
    Reference,
    Inheritance,
}

impl LanguageBackend for Csharp {
    fn get_extensions(&self) -> Vec<String> {
        vec!["cs".to_string()]
    }

    fn get_graph_types(&self) -> Vec<GraphType> {
        vec![GraphType::Reference]
    }

    fn build_graph(&self, filenames: &[Path], graphtype: GraphType) -> Graph<String, ()> {
        // Here's a graph where classes constitute the nodes.
        // Edges are formed when such classes reference instances of one another.
        // We're going to assume the C# file has valid syntax and pull out the
        // exact information we need.

        let mut g = Graph::new();

        let a = g.add_node("a".to_string());
        let b = g.add_node("b".to_string());
        let c = g.add_node("c".to_string());

        g.add_edge(a, b, ());

        g
    }
}

#[derive(Clone)]
enum TokenClass {
    Whitespace,
    Newline,
    Comment,
    IdentifierOrKeyword,
    Unknown,
}

/*
let tokens = [
    token {
        class: Whitespace,
        regex: regex!(r"^\p{Zs}|\x{0009}|\x{000B}\x{000C}"),
    },
    token {
        class: Newline,
        regex: regex!(r"^\x{000D}\x{000A}\|x{000D}|\x{000A}|\x{2028}|\x{2029}"),
    },
    token {
        class: Comment,
        */
        //regex: regex!(r"^(/\*[^\*/]\*/)|(//[^\x{000D}\x{000A}\x{2028}\x{2029})"),
        /*
    },
    token {
        class: BlockBegin,
        regex: regex!(r"^{"),
    },
    token {
        class: BlockEnd,
        regex: regex!(r"^}"),
    },
    token {
        class: IdentifierOrKeyword,
        regex: regex!(r"^(_|\p{Lu}|\p{Ll}|\p{Lt}|\p{Lm}|\p{Lo}|\p{Nl})" +
                      r"(\p{Lu}|\p{Ll}|\p{Lt}|\p{Lm}|\p{Lo}|\p{Nl}\p{Nd}|\p{Pc}|\p{Mn}|\p{Mc}|\p{Cf})*"),
    }
]
*/

struct Token {
    class: TokenClass,
    regex: Regex,
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer { tokens: vec![] }
    }

    fn define_token(&mut self, class: TokenClass, regex: Regex) {
        self.tokens.push(Token { class: class, regex: regex });
    }

    fn lex(&self, text: &str) -> TokenIterator {
        TokenIterator::new(self.tokens.as_slice(), text)
    }
}


struct TokenIterator<'a> {
    tokens: &'a [Token],
    text: String,
    idx: uint,
}

impl<'a> TokenIterator<'a> {
    fn new(tokens: &'a [Token], text: &str) -> TokenIterator<'a> {
        TokenIterator { tokens: tokens, text: String::from_str(text), idx: 0 }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = TokenClass;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.idx == self.text.len() {
            None
        } else {
            let textleft = self.text.slice_from(self.idx);
            for token in self.tokens.iter() {
                if let Some((_, end)) = token.regex.find(textleft) {
                    self.idx += end;
                    return Some(token.class.clone());
                }
            }
            self.idx += 1;
            Some(Unknown)
        }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (0, Some(self.text.len() - self.idx))
    }
}
