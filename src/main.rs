#![feature(plugin)]

extern crate serialize;
extern crate "rustc-serialize" as rustc_serialize;

extern crate docopt;
#[no_link] #[plugin] extern crate docopt_macros;

extern crate regex;
#[no_link] #[plugin] extern crate regex_macros;

use docopt::Docopt;
use backend::LanguageBackend;
use backend::GraphType;
use graph::Graph;

mod graph;
mod backend;
mod files;

docopt!(Args derive Show, "Usage: machete <path>");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    run(args.arg_path);
}

fn run(path: String) {
    let backends = [backend::Csharp];

    let mut exts = vec![];
    for backend in backends.iter() {
        exts.push_all(backend.get_extensions().as_slice());
    }
    
    let groups = files::gather_files(path, exts.as_slice());

    for backend in backends.iter() {
        let mut fnames = vec![];
        for ext in backend.get_extensions().iter() {
            if let Some(results) = groups.get(ext) {
                fnames.push_all(results.as_slice());
            }
        }

        let g = backend.build_graph(fnames.as_slice(), GraphType::Reference);
        print_ascii_graph(&g);
    }
}

fn print_ascii_graph(g: &Graph<String, ()>) {
    println!("Nodes:");
    g.each_node(|_, node| {
        println!("\t{}", node.data);
        true
    });
    println!("Edges:");
    g.each_edge(|_, edge| {
        println!("\t({}, {})",
                 g.node_data(edge.source()),
                 g.node_data(edge.target()));
        true
    });
}
