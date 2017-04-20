extern crate rustc_serialize;
extern crate docopt;

pub use args::get_args;
pub use locate::get_paths;
pub use format::format;
pub use process::collapse;


mod args;
mod locate;
mod format;
mod process;



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
