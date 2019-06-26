extern crate console;

use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

pub fn handle(project_name: &str) {
  let bar = ProgressBar::new(3);

  bar.set_style(
    ProgressStyle::default_bar()
      .template("\n   [{bar:40.red/white}]")
      .progress_chars("#>-"),
  );

  let output = Command::new("sh")
    .arg("-c")
    .arg(format!("bsb -init {} -theme react-hooks", project_name))
    .output()
    .expect("failed");

  bar.inc(1);

  Command::new("cd")
    .arg(project_name)
    .output()
    .expect("Could not cd in to app");

  bar.inc(1);

  Command::new("npm")
    .arg("install")
    .output()
    .expect("Could not install dependencies");

  bar.finish_and_clear();

  if output.status.success() {
    println!(
      "
    {} app created!

    - cd {}
    - npm install
    - npm start
    - npm run server (in another terminal window)
    ",
      style(project_name).blue(),
      project_name,
    );
  }
}
