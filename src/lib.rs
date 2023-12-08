use clap::Parser;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait HasFile {
  fn file(&self) -> String;
}

impl HasFile for Args {
  fn file(&self) -> String { self.file.clone() }
}

#[derive(Parser, Debug)]
struct Args {
  #[arg(short, long)]
  file: String,
}

pub fn input() -> Box<dyn Iterator<Item = String>> { return input_with_type::<Args>(None).0; }

pub fn input_with_type<T: Parser + HasFile>(
  default: Option<T>,
) -> (Box<dyn Iterator<Item = String>>, Box<T>) {
  let args = default.unwrap_or_else(|| T::parse());
  if args.file().is_empty() {
    return (
      Box::new(std::io::stdin().lock().lines().map(|s| s.unwrap())),
      Box::new(args),
    );
  }
  let file = File::open(args.file()).unwrap();
  let reader = BufReader::new(file);
  return (Box::new(reader.lines().map(|s| s.unwrap())), Box::new(args));
}
