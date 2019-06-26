#[macro_use]
extern crate clap;
extern crate console;

use clap::{App, Arg};
use console::style;
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
        .index(1)
        .takes_value(true)
        .possible_values(&Languages::variants())
        .case_insensitive(true)
        .required(true),
    )
    .arg(Arg::with_name("projectName").index(2).required(true))
    .get_matches();

  println!(
    "    {} is boostrapping your app",
    style("Supreme").white().on_red().italic()
  );

  let mode = value_t!(matches, "mode", Languages).unwrap_or_else(|e| e.exit());
  let project_name = matches.value_of("projectName").unwrap();

  match mode {
    Languages::Reason => reason::handle(project_name),
  }
}
