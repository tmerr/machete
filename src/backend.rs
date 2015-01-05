use graph::Graph;

pub trait LanguageBackend {
    fn get_extensions() -> Vec<String>;
    fn build_graph(filenames: Vec<String>) -> Graph<String, ()>;
}


pub struct Csharp;

impl LanguageBackend for Csharp {
    fn get_extensions() -> Vec<String> {
        vec!["cs".to_string()]
    }

    fn build_graph(filenames: Vec<String>) -> Graph<String, ()> {
        let mut g = Graph::new();

        let a = g.add_node("a".to_string());
        let b = g.add_node("b".to_string());
        let c = g.add_node("c".to_string());

        g.add_edge(a, b, ());

        g
    }
}
