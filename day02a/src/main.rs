use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  static ref REGEX: Regex = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)").unwrap();
}

fn main() {
  println!(
    "Num Valid Password: {}",
    include_str!("../input.txt").lines().fold(0, |total, pass| {
      let groups = REGEX.captures(pass).unwrap();
      let min = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
      let max = groups.get(2).unwrap().as_str().parse::<usize>().unwrap();
      let character = groups.get(3).unwrap().as_str();
      let password = groups.get(4).unwrap().as_str().split("").filter(|&c| c == character).count();

      total + (min <= password && password <= max) as u16
    })
  )
}
