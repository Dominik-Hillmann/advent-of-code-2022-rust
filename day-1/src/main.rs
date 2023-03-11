use std::fs;
use std::string::String;

fn main() {
    let file_path = String::from("./real-input.txt");

    let mut contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents = contents.replace("\r\n", "\n");

    let bags: Vec<&str> = contents.split("\n\n").collect();

    let calory_counts: Vec<u32> = to_int_calories(bags);
    let largest = find_largest(calory_counts);

    println!("The largest bag of calories contains {} calories.", largest);
}

fn to_int_calories(bags: Vec<&str>) -> Vec<u32> {
    let mut calory_counts: Vec<u32> = Vec::new();

    for bag in bags {
        let calories_in_bag: Vec<u32> = bag
            .lines()
            .map(|calories_str| calories_str.parse::<u32>().unwrap())
            .collect();

        let sum: u32 = calories_in_bag.iter().sum();
        calory_counts.push(sum);
    }

    calory_counts
}

fn find_largest(calories: Vec<u32>) -> u32 {
    let mut largest: u32 = 0;
    for calory_value in calories {
        largest = largest.max(calory_value);
    }

    largest
}

#[cfg(test)]
mod tests {
    use crate::{find_largest, to_int_calories};

    #[test]
    fn test_to_int_calories() {
        let string_bags = vec!["1", "2", "3"];
        let int_bags = to_int_calories(string_bags);

        assert_eq!(int_bags, vec![1, 2, 3]);
    }

    #[test]
    fn test_find_largest() {
        let actual_largest = 3;
        let largest = find_largest(vec![1, 2, actual_largest]);

        assert_eq!(largest, actual_largest);
    }

    #[test]
    fn test_find_largest_on_empty() {
        let largest = find_largest(vec![]);

        assert_eq!(largest, 0);
    }
}
