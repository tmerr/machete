use test::Bencher;
use lexer::Lexer;

fn generate_text() -> String {
    let mut txt = "".to_string();
    for i in range(0, 500) {
        txt.push_str("abcd ");
    }
    return txt;
}

#[bench]
fn lexer(b: &mut Bencher) {
    let txt = generate_text();

    let mut lexer = Lexer::new();
    lexer.define_token("word", regex!(r"^\w+"));
    lexer.define_token("space", regex!(r"^ +"));

    b.iter(|| {
        let mut i: i32 = 0;
        for tok in lexer.lex(&txt[]) {
            match (tok.0, tok.1) {
                _ => i += 1,
            }
        }
    })
}
