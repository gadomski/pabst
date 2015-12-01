//! Simple executable.

extern crate docopt;
extern crate rustc_serialize;
extern crate pabst;
extern crate toml;

use std::fs::File;
use std::io::Read;
use std::process::exit;

use docopt::Docopt;
use pabst::{open_file_source, open_file_sink};

const USAGE: &'static str = "
Use pabst on point cloud data.

Usage:
    pabst convert <infile> <outfile> [--config=<config-file>]
    pabst --version
    pabst (-h | --help)

Options:
    -h --help                   Print this message.
    --version                   Print the version.
    --config=<config-file>      TOML configuration file.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_convert: bool,
    arg_infile: String,
    arg_outfile: String,
    flag_config: Option<String>,
}

const DEFAULT_CHUNK_SIZE: usize = 10000;

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| {
                             d.version(Some(env!("CARGO_PKG_VERSION").to_string())).decode()
                         })
                         .unwrap_or_else(|e| e.exit());

    if args.cmd_convert {
        let mut source_config = None;
        let mut sink_config = None;
        let mut limit = None;
        let mut chunk_size = DEFAULT_CHUNK_SIZE;

        if let Some(config_file) = args.flag_config {
            let mut file = File::open(config_file).unwrap_or_else(|e| {
                println!("ERROR: unable to open configuration file: {}", e);
                exit(1);
            });
            let ref mut config = String::new();
            file.read_to_string(config).unwrap_or_else(|e| {
                println!("ERROR: could not read file into string: {}", e);
                exit(1);
            });
            let mut parser = toml::Parser::new(config);
            if let Some(mut table) = parser.parse() {
                if let Some(v) = table.get("limit") {
                    limit = v.as_integer();
                    println!("Limit: {}", limit.unwrap());
                }
                if let Some(n) = table.get("chunk_size") {
                    if let Some(n) = n.as_integer() {
                        chunk_size = n as usize;
                    }
                }
                source_config = table.remove("source");
                sink_config = table.remove("sink");
            } else {
                println!("ERROR: unable to parse TOML configuration file: {:?}", parser.errors);
                exit(1);
            }
        }

        let mut source = open_file_source(args.arg_infile, source_config).unwrap();
        let mut sink = open_file_sink(args.arg_outfile, sink_config).unwrap();
        let mut nread = 0;
        'outer: loop {
            let points = source.source(chunk_size).unwrap();
            if let Some(points) = points {
                for ref point in points {
                    sink.sink(point).unwrap();
                    nread += 1;
                    if let Some(limit) = limit {
                        if nread >= limit {
                            break 'outer;
                        }
                    }
                }
            } else {
                break;
            }
        }
        sink.close_sink().unwrap();
    }
}
