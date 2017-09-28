#![feature(inclusive_range_syntax)]
extern crate docopt;
extern crate grep;
extern crate regex;
extern crate rustc_serialize;

mod locate;
mod update;
mod args;

use locate::Locations;
use args::get_args;
use update::list_all_files;


fn main() {
    println!("{}", list_all_files());
    // let args = get_args();
    // let pattern = &args.arg_pattern;
    // let locations = Locations::new(pattern, "full_database.txt");
    // println!("{}", locations.into_string());
}
