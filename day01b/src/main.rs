fn main() {
  let input = include_str!("../input.txt").lines().map(|num| num.parse().unwrap()).collect::<Vec<u32>>();

  for (i, first) in input.iter().enumerate() {
    for (j, second) in input.iter().enumerate().skip(i + 1) {
      for third in input.iter().skip(j + 1) {
        if first + second + third == 2020 {
          println!("Expense Report: {}", first * second * third);
        }
      }
    }
  }
}
