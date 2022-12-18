// use std::env;
use std::fs;
use std::string::String;

fn main() {
  let file_path = String::from("./real-input.txt");

  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let bags = contents.split("\n\n");

  let mut calory_counts: Vec<u32> = Vec::new();
  for bag in bags {
    let calories_in_bag: Vec<u32> = bag.lines()
      .map(|calories_str| calories_str.parse::<u32>().unwrap())
      .collect();
    
    let sum: u32 = calories_in_bag.iter().sum();
    calory_counts.push(sum);
  }

  let mut largest: u32 = 0;
  for count in calory_counts {
    largest = largest.max(count);
  }

  println!("{}", largest);
}
