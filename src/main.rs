extern crate pretty_locate;

use pretty_locate::*;

// TODO
// - Make it a complete wrapper. Add support for all locate options, multiple patterns, etc.
// - Optimize for speed: into_map(), get_paths()?
// - Organize modules better?
// - Do proper error-handling with Result
// - Handle wildcard in formatting


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

