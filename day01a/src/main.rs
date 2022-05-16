fn main() {
  let input = include_str!("../input.txt").lines().map(|num| num.parse().unwrap()).collect::<Vec<u32>>();

  for i in input.iter() {
    for j in input.iter() {
      if i == j {
        continue;
      } else if i + j == 2020 {
        return println!("Expense Report: {}", i * j);
      }
    }
  }
}
