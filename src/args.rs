use docopt::Docopt;

const USAGE: &'static str = "
Pretty locate.

Usage:
  pretty-locate <pattern> [--color] 
  pretty-locate (-u | --update)
  pretty-locate (-h | --help)

Options:
  -h --help      Show this screen.
  -u --update    Re-create the database.
  --color        Color appearances of <pattern> in the output.

";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_pattern: String,
    pub flag_color: bool,
    pub flag_update: bool,
}


pub fn get_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit())
}
