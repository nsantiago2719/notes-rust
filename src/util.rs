
use std::io;

pub fn get_input_value(d: &str) -> String {
  let mut value = String::new();
  println!("{} ", d);
  io::stdin().read_line(&mut value).expect("Failed to get value");
  trim_newline(&value)
}

pub fn trim_newline(s: &String) -> String {
  s.trim_end_matches(&['\r', '\n']).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
}