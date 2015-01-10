use graph::Graph;
use std::path::posix::Path;
use std::io::File;
use self::TokenClass::{Whitespace, Newline, Comment, BlockBegin, BlockEnd, IdentifierOrKeyword};
use regex::Regex;
use lexer::Lexer;
use lexer::Result::{Matched, Unmatched};
use backend::LanguageBackend;
use backend::GraphInfo;


pub struct Csharp;

impl LanguageBackend for Csharp {
    fn get_extensions(&self) -> Vec<String> {
        vec!["cs".to_string()]
    }

    fn build_graphs(&self, filenames: &[Path]) -> Vec<GraphInfo> {
        // Here's a graph where classes constitute the nodes.
        // Edges are formed when such classes reference instances of one another.
        // We're going to assume the C# file has valid syntax and pull out the
        // exact information we need.

        let lexer = build_csharp_lexer();
    
        for filename in filenames.iter() {
            let mut file = File::open(filename);
            //let mut text = "".to_string();
            let text = match File::open(filename) {
                Err(_) => {
                    println!("Failed to open file");
                    continue;
                },
                Ok(mut file) => {
                    file.read_to_string().unwrap()
                },
            };

            for tok in lexer.lex(&text[]) {
                match (tok.0, &tok.1[]) {
                    (Matched(Whitespace), _) => {},
                    (Matched(Newline), _) => {},
                    (Matched(Comment), _) => {},
                    (Matched(BlockBegin), _) => {},
                    (Matched(BlockEnd), _) => {},
                    (Matched(IdentifierOrKeyword), "class") => {},
                    (Matched(IdentifierOrKeyword), _) => {},
                    (Unmatched, _) => {},
                }
            }
        }

        //for tok in lexer.lex()


        let mut g = Graph::new();

        let a = g.add_node("a".to_string());
        let b = g.add_node("b".to_string());
        let c = g.add_node("c".to_string());

        g.add_edge(a, b, ());
        let mut vec = Vec::new();
        vec.push(GraphInfo { name: "reference graph".to_string(), graph: g} );
        vec
    }
}

#[derive(Clone)]
enum TokenClass {
    Whitespace,
    Newline,
    Comment,
    BlockBegin,
    BlockEnd,
    IdentifierOrKeyword,
}

fn build_csharp_lexer() -> Lexer<TokenClass> {
    let mut lexer = Lexer::new();

    lexer.define_token(Whitespace, regex!(r"^\p{Zs}|\x{0009}|\x{000B}\x{000C}"));
    lexer.define_token(Newline, regex!(r"^\x{000D}\x{000A}|\x{000D}|\x{000A}|\x{2028}|\x{2029}"));
    lexer.define_token(Comment, regex!(r"^(/\*[^\*/]\*/)|(//[^\x{000D}\x{000A}\x{2028}\x{2029}]*)"));
    lexer.define_token(BlockBegin, regex!(r"^\{"));
    lexer.define_token(BlockEnd, regex!(r"^\}"));
    lexer.define_token(IdentifierOrKeyword, regex!(r"^(_|\p{Lu}|\p{Ll}|\p{Lt}|\p{Lm}|\p{Lo}|\p{Nl})(\p{Lu}|\p{Ll}|\p{Lt}|\p{Lm}|\p{Lo}|\p{Nl}\p{Nd}|\p{Pc}|\p{Mn}|\p{Mc}|\p{Cf})*"));

    lexer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csharp_lexer() {
        let lexer = build_csharp_lexer();
        lexer.lex("class Fizz\n{\nBuzz buzz;\n}");
    }
}
