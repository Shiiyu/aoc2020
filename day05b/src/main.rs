pub fn main() {
  let mut input: Vec<u16> = include_bytes!("../input.txt")
    .chunks(11)
    .map(|b| b[..10].iter().fold(0, |id, b| (id << 1) | (!b & 4) as u16 >> 2))
    .collect();

  input.sort_unstable();

  println!("Seat ID: {}", input.windows(2).find(|s| s[0] == s[1] - 2).unwrap()[0] + 1);
}
