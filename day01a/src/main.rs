fn main() {
  let input = include_str!("../input.txt").lines().map(|num| num.parse().unwrap()).collect::<Vec<u32>>();

  for (i, first) in input.iter().enumerate() {
    for second in input.iter().skip(i + 1) {
      if first + second == 2020 {
        println!("Expense Report: {}", first * second);
      }
    }
  }
}
