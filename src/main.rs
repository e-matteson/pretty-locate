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
use update::create_database;

const DATABASE_PATH: &str = "database/pretty-locate-database.txt";

fn main() {
    let args = get_args();
    println!("{:?}", args);

    if args.flag_update {
        println!("update!");
        // create_database(DATABASE_PATH);
        return;
    }

    let pattern = &args.arg_pattern;
    let locations = Locations::new(pattern, DATABASE_PATH);
    println!("{}", locations.into_string());
}
