extern crate pretty_locate;

use pretty_locate::*;


fn main() {
    let args = get_args();
    let pattern = &args.arg_pattern;

    let paths = get_paths(pattern);
    if paths.is_empty() {
        return;
    }

    let lines = collapse(paths, pattern, args.flag_wildcard);
    let output = format(&lines, pattern, args.flag_color);
    println!("{}", output);
}

