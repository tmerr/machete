use std::borrow::IntoCow;
use graphviz as dot;
use graph::{Graph, NodeIndex, EdgeIndex};
use backend::GraphInfo;

pub type Nd = NodeIndex;
pub type Ed = EdgeIndex;

/// Render the graph to the file. Note the graph name must match the regex
/// [a-zA-Z_][a-zA-Z_0-9]* or this will panic.
pub fn render<W: Writer>(ginfo: &GraphInfo, output: &mut W) {
    dot::render(ginfo, output).unwrap()
}

impl<'a> dot::Labeller<'a, Nd, Ed> for GraphInfo {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new(&self.name[]).ok().expect("Graph name does not conform to required format")
    }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(&self.graph.node_data(*n)[]).unwrap()
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for GraphInfo {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        let mut result = vec![];
        self.graph.each_node(|i, _| {result.push(i); true});
        result.into_cow()
    }

    fn edges(&self) -> dot::Edges<'a, Ed> {
        let mut result = vec![];
        self.graph.each_edge(|i, _| {result.push(i); true});
        result.into_cow()
    }

    fn source(&self, e: &Ed) -> Nd {
        self.graph.edge(*e).source()
    }

    fn target(&self, e: &Ed) -> Nd {
        self.graph.edge(*e).target()
    }
}
