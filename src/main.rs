#![feature(old_orphan_check)]
#![feature(associated_types)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use backend::LanguageBackend;

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

    let path = args.arg_path;
}
