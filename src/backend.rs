use graph::Graph;
use std::path::posix::Path;

pub struct GraphInfo {
    pub name: String,
    pub graph: Graph<String, ()>,
}

pub trait LanguageBackend {
    fn get_extensions(&self) -> Vec<String>;
    fn build_graphs(&self, filepaths: &[Path]) -> Vec<GraphInfo>;
}
