use std::path::posix::Path;
use std::io::File;
use regex::Regex;
use std::collections::{HashSet, HashMap};

use graph::Graph;
use self::TokenClass::{Whitespace, Newline, Comment, StringLiteral, BlockBegin, BlockEnd, IdentifierOrKeyword};
use lexer::Lexer;
use lexer::TokenIterator;
use lexer::Token::{Matched, Unmatched};
use lexer::Token;
use backend::LanguageBackend;
use backend::GraphInfo;


macro_rules! unwrap_or_return {
    ($i:expr, $j:path) => (
        {
            match $i {
                Some(t) => t,
                None => return $j,
            }
        }
    );
}


pub struct Csharp;

impl LanguageBackend for Csharp {
    fn get_extensions(&self) -> Vec<String> {
        vec!["cs".to_string()]
    }

    /// Build a graph where classes constitute the nodes, and edges are
    /// formed by references between them.
    fn build_graphs(&self, paths: &[Path]) -> Vec<GraphInfo> {
        // We're going to assume the C# file has valid syntax and pull out the
        // exact information we need.
        
        let mut g = Graph::new();
        
        let map = build_map(paths);

        let mut nodes = HashMap::new();
        for classname in map.keys() {
            nodes.insert(classname, g.add_node(classname.to_string()));
        }

        for nameA in map.keys() {
            for (nameB, set) in map.iter() {
                if set.contains(nameA) {
                    g.add_edge(*nodes.get(nameB).unwrap(), *nodes.get(nameA).unwrap(), ());
                }
            }
        }

        let mut vec = Vec::new();
        vec.push(GraphInfo { name: "reference_graph".to_string(), graph: g});

        vec
    }
}

/// Build a map from each class to a set of all identifiers/keywords within it.
fn build_map(paths: &[Path]) -> HashMap<String, HashSet<String>> {
    let lexer = build_csharp_lexer();
    let mut map = HashMap::new();

    for path in paths.iter() {
        let text = match File::open(path) {
            Err(_) => {
                println!("Failed to open file");
                continue;
            },
            Ok(mut file) => {
                file.read_to_string().unwrap()
            },
        };

        let mut tokens = lexer.lex(&text[]);
        loop {
            if let Some(tok) = tokens.next() {
                match (tok.0, tok.1) {
                    (Matched(IdentifierOrKeyword), txt) => {
                        if txt == "class" || txt == "struct" {
                            if let Some((classname, wordset)) = class_x(&mut tokens) {
                                map.insert(classname, wordset);
                            }
                        }
                    },
                    _ => {},
                }
            } else {
                break;
            }
        }
    }

    map
}

/// parse class _ { ... }, starting at _, ending at }, returning the name of the class
/// and the set of identifier/keyword tokens within "...".
fn class_x(tokens: &mut TokenIterator<TokenClass>) -> Option<(String, HashSet<String>)> {
    let mut set = HashSet::new();

    let classname = {
        let tok = unwrap_or_return!(next_meaningful(tokens), None);
        match (tok.0, tok.1) {
            (Matched(IdentifierOrKeyword), x) => x.to_string(),
            _ => return None,
        }
    };

    let tok2 = unwrap_or_return!(next_meaningful(tokens), None);
    match tok2.0 {
        Matched(BlockBegin) => {},
        _ => return None,
    }

    let mut block_depth = 1;
    
    while block_depth >= 0 {
        let tok = unwrap_or_return!(next_meaningful(tokens), None);
        match (tok.0, tok.1) {
            (Matched(BlockBegin), x) => { block_depth += 1; },
            (Matched(BlockEnd), x) => { block_depth -= 1; },
            (Matched(IdentifierOrKeyword), x) => { set.insert(x.to_string()); },
            _ => {},
        }
    }

    Some((classname, set))
}

/// Get the next token that is not a whitespace, newline, or comment.
fn next_meaningful(tokens: &mut TokenIterator<TokenClass>) -> Option<(Token<TokenClass>, String)> {
    loop {
        let tok = unwrap_or_return!(tokens.next(), None);
        match (tok.0, tok.1) {
            (Matched(Whitespace), x) => {},
            (Matched(Newline), x) => {},
            (Matched(Comment), x) => {},
            (a, b) => return Some((a, b.to_string())),
        }
    }
}

#[derive(PartialEq, Clone, Show)]
enum TokenClass {
    Whitespace,
    Newline,
    Comment,
    StringLiteral,
    BlockBegin,
    BlockEnd,
    IdentifierOrKeyword,
}

fn build_csharp_lexer() -> Lexer<TokenClass> {
    let mut lexer = Lexer::new();

    lexer.define_token(Whitespace, regex!(r"^(\p{Zs}|\x{0009}|\x{000B}\x{000C})"));
    lexer.define_token(Newline, regex!(r"^((\r\n)|\r|\n|\x{2028}|\x{2029})"));
    lexer.define_token(Comment, regex!(r"^((/\*[^\*/]\*/)|(//[^\x{000D}\x{000A}\x{2028}\x{2029}]*))"));
    lexer.define_token(StringLiteral, regex!(r##"^"([^"\r\n\x{2028}\x{2029}\\]|\\'|\\"|\\\\|\\0|\\a|\\b|\\f|\\n|\\r|\\t|\\v|(\\x[0-9A-Fa-f]{1,4})|(\\u[0-9A-Fa-f]{4})|(\\U[0-9A-Fa-f]{8}))*""##));
    lexer.define_token(BlockBegin, regex!(r"^\{"));
    lexer.define_token(BlockEnd, regex!(r"^\}"));
    lexer.define_token(IdentifierOrKeyword, regex!(r"^((_|\p{L}|\p{Nl})(\p{L}|\p{Nl}\p{Nd}|\p{Pc}|\p{Mn}|\p{Mc}|\p{Cf})*)"));

    lexer
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::TokenClass::{Whitespace, Newline, Comment, BlockBegin, BlockEnd, IdentifierOrKeyword};
    use super::TokenClass;
    use lexer::Token::{Matched, Unmatched};
    use lexer::Token;

    #[test]
    fn test_csharp_lexer() {
        let lexer = super::build_csharp_lexer();
        let result: Vec<(Token<TokenClass>, &str)> = lexer.lex("class Fizz\n{\nBuzz buzz;\n}").collect();
        let kind: Vec<Token<TokenClass>> = result.iter().map(|r| r.0.clone()).collect();
        let expected = [Matched(IdentifierOrKeyword), Matched(Whitespace), Matched(IdentifierOrKeyword),
                        Matched(Newline), Matched(BlockBegin), Matched(Newline), Matched(IdentifierOrKeyword),
                        Matched(Whitespace), Matched(IdentifierOrKeyword), Unmatched, Matched(Newline), Matched(BlockEnd)];

        assert_eq!(&kind[], &expected[]);
    }

    #[test]
    fn test_class_x() {
        let lexer = super::build_csharp_lexer();
        let mut iter = lexer.lex("class Fizz\n{\nBuzz buzz}\n}");
        iter.next(); // consume "class"
        match super::class_x(&mut iter) {
            Some((classname, set)) => {
                assert_eq!(classname, "Fizz");
                assert!(set.contains("Buzz"));
                assert!(set.contains("buzz"));
            },
            None => {
                panic!("Test failed.");
            },
        }
    }
}
