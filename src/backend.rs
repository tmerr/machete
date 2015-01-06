use graph::Graph;
use std::path::posix::Path;

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
        let mut g = Graph::new();

        let a = g.add_node("a".to_string());
        let b = g.add_node("b".to_string());
        let c = g.add_node("c".to_string());

        g.add_edge(a, b, ());

        g
    }
}

