#![feature(old_orphan_check)]
#![feature(associated_types)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use backend::LanguageBackend;
use files::GroupedFiles;
use backend::GraphType;
use graph::Graph;

mod graph;
mod backend;
mod files;


static USAGE: &'static str = "Usage: machete <path>";

#[derive(RustcDecodable, Show)]
struct Args {
    arg_path: String,
}

fn main() {
    let args : Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

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
            match groups.get(ext) {
                Some(results) => fnames.push_all(results.as_slice()),
                None => {},
            };
        }
        let g = backend.build_graph(fnames.as_slice(), GraphType::Reference);
        print_ascii_graph(&g);
    }
}

fn print_ascii_graph(g: &Graph<String, ()>) {
    println!("print graph here");
}
