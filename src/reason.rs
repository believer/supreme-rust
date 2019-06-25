use std::process::Command;

pub fn handle() {
  println!("Reaaason");

  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(&["/C", "echo hello"])
      .output()
      .expect("failed execute")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("failed")
  };

  if output.status.success() {
    let s = String::from_utf8_lossy(&output.stdout);
    println!("nice {}", s);
  }
}