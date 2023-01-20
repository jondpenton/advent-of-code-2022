use std::fs;

fn main() {
  println!("Hello, world!");

  println!("{:#?}", get_max_elf_calories("assets/part0.txt"))
}

fn get_max_elf_calories(path: &str) -> u64 {
  let contents =
    fs::read_to_string(path).expect("Should have been able to read the file");
  let elves = contents.trim().split("\n\n").map(|elf| {
    elf.split("\n").map(|calories| {
      calories
        .parse::<u64>()
        .expect(format!("{:#?} should have been an int.", calories).as_str())
    })
  });
  let max_elf_calories = elves.fold(0, |total, elf| {
    match elf.fold(0, |calorie, total| calorie + total) {
      calories if calories > total => calories,
      _ => total,
    }
  });

  max_elf_calories
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part0() {
    assert_eq!(get_max_elf_calories("assets/part0.txt"), 24000)
  }

  #[test]
  fn part1() {
    assert_eq!(get_max_elf_calories("assets/part1.txt"), 75501)
  }
}
