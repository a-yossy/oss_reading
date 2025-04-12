use std::path::PathBuf;

pub fn path_buf_example() {
  let mut path = PathBuf::new();

  path.push("example1");
  path.push("example2/example3");
  path.set_extension("rs");

  println!("{}", path.display());
}
