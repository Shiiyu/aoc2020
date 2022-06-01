fn main() {
  println!(
    "Highest Seat ID: {}",
    include_bytes!("../input.txt")
      .chunks(11)
      .map(|b| b[..10].iter().fold(0, |id, b| (id << 1) | (!b & 4) as u16 >> 2))
      .max()
      .unwrap()
  );
}
