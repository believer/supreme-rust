#[macro_use]
extern crate clap;

use clap::{App, Arg};
mod reason;

arg_enum! {
  #[derive(Debug)]
  enum Languages {
    Reason
  }
}

fn main() {
  let matches = App::new("Supreme")
    .version("1.0")
    .about("Bootstrap an app")
    .author(crate_authors!())
    .arg(
      Arg::with_name("mode")
        .short("m")
        .long("mode")
        .takes_value(true)
        .possible_values(&Languages::variants())
        .case_insensitive(true)
        .required(true),
    )
    .get_matches();

  let m = value_t!(matches, "mode", Languages).unwrap_or_else(|e| e.exit());

  match m {
    Languages::Reason => reason::handle(),
  }
}
