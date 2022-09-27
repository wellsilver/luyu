// portable http server uses lua backend
use std::fs;
struct Settings {
  // object with all settings used by toml_set()
}
/*
excerpt from std::fs DirEntry doc.rs
use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())
}*/
fn all_files() -> std::io::Result<()> {
  let mut i = vec![String::new()];
  let mut i_point = 1;
  for entry in fs::read_dir(".")? { // WHY DO IAHVED TO DO THIS RESULT THING NO EW NO BYE
    let dir = entry?;
    i[i_point] = format!("{:?}",dir.path()); // why do I find myself using format so much with strings
    i_point+=1;
  }
  Ok(())
}
fn main() {
  if format!("{:?}",all_files()) == "Ok(())" {
    //OH MY GOD IT WORKED
    // WHY IS THIS EVEN THE WAY TO DO IT
    // WHY DO I HAVE TO DO SO MANY WORKAROUNDS
    // RUST WHY
  }
}
