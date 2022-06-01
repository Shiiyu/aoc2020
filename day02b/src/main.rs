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
      let pos_one = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
      let pos_two = groups.get(2).unwrap().as_str().parse::<usize>().unwrap();
      let character = groups.get(3).unwrap().as_str();
      let password = groups.get(4).unwrap().as_str();

      total
        + ((&password[(pos_one - 1)..pos_one] == character && &password[(pos_two - 1)..pos_two] != character)
          || (&password[(pos_one - 1)..pos_one] != character && &password[(pos_two - 1)..pos_two] == character))
          as u16
    })
  )
}
