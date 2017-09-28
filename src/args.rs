use docopt::Docopt;

const USAGE: &'static str = "
Pretty locate.

Usage:
  pretty-locate <pattern> [--color] [--wildcard]
  pretty-locate (-h | --help)

Options:
  -h --help      Show this screen.
  --color        Color appearances of <pattern> in the output.
  --wildcard     Append '*' to matching directories.

";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_pattern: String,
    pub flag_color: bool,
    pub flag_wildcard: bool,
}


pub fn get_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}
