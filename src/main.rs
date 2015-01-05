#![feature(old_orphan_check)]
#![feature(associated_types)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use backend::LanguageBackend;
use files::FileGroup;
use backend::GraphType;

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
    for group in groups.iter() {
        println!("{}", group.ext);
        println!("{}", group.filenames);
    }

    for backend in backends.iter() {
        let mut fnames = vec![];
        for group in groups.iter() {
            if backend.get_extensions().contains(&group.ext) {
                fnames.push_all(group.filenames.as_slice());
            }
        }
        let graph = backend.build_graph(fnames, GraphType::Reference);
    }
}
