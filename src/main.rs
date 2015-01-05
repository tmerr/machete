#![feature(old_orphan_check)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use docopt::Docopt;

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
