mod cli;
pub mod element;
pub mod find;
pub mod keyboard;
pub mod utils;
pub mod window;

fn main() {
  cli::match_commands();
}
