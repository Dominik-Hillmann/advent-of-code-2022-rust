use std::fs;

fn main() {
    let prio = match get_priority('c') {
        Ok(prio) => prio,
        Err(s) => panic!(s)
    };

    println!("{}!", read_lines("sample-input.txt"));
    let text = read_lines("sample-input.txt");
    let lines: Vec<&str> = text.split("\n").collect();

    for line in lines {
        let res = get_halves(line);
        let (compartment_1, compartment_2) = match res {
            Ok((c1, c2)) => (c1, c2),
            Err(s) => panic!("{}", s)
        };

        let common = find_common_item(compartment_1, compartment_2);

        match common {
            Ok(common) => print!("{} has common {}.\n", line, common),
            Err(s) => panic!("{}", s)
        }
    }
}

fn read_lines(filename: &str) -> String {
    let file_path = String::from(filename);

    let mut contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents = contents.replace("\r\n", "\n");

    contents
}

fn get_halves(s: &str) -> Result<(Vec<char>, Vec<char>), &str> {
    let letters: Vec<char> = s.chars().collect();
    let length: usize = letters.len();

    if length.rem_euclid(2) != 0 {
        return Err("The length of the input string must be divisible by 2.");
    }

    let half_length = length / 2;
    let mut first_half: Vec<char> = Vec::new();
    let mut second_half: Vec<char> = Vec::new();

    for i in 0..length {
        if i < half_length {
            first_half.push(letters[i]);
        } else {
            second_half.push(letters[i]);
        }
    }

    Ok((first_half, second_half))
}

fn find_common_item(compartment_1: Vec<char>, compartment_2: Vec<char>)
    -> Result<char, &'static str> {

    for item in compartment_1 {
        if compartment_2.contains(&item) {
            return Ok(item.clone());
        }
    }

    Err("Compartment 1 and compartment 2 do not share any items.")
}

fn get_priority(c: char) -> Result<u8, &'static str> {
    let lower_case = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
        'y', 'z'
    ];
    let upper_case = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z'
    ];

    return if !lower_case.contains(&c) && !upper_case.contains(&c) {
        Err(&format!("{} is not a valid character.", c))
    } else if lower_case.contains(&c) {
        let index: Option<usize> =  lower_case.iter().position(|&letter| letter == c);
        Ok(1 + index.try_into().unwrap())
    } else {
        Ok(26 + upper_case.iter().position(|&letter| letter == c).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::{get_halves, find_common_item};

    #[test]
    fn test_get_halves_happy_path() {
        let res = get_halves("abcdef");

        assert_eq!(res.is_ok(), true);

        let (first_half, second_half) = res.unwrap();

        assert_eq!(first_half, vec!['a', 'b', 'c']);
        assert_eq!(second_half, vec!['d', 'e', 'f']);
    }

    #[test]
    fn panics_on_uneven_length_input() {
        let res = get_halves("aaaaa");

        assert_eq!(res.is_ok(), false);
    }

    #[test]
    fn test_find_common_item() {
        let res = find_common_item(vec!['a', 'b'], vec!['a', 'c']);

        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 'a');
    }

    #[test]
    fn test_no_common_item() {
        let res = find_common_item(vec!['a', 'b'], vec!['c', 'd']);

        assert_eq!(res.is_ok(), false);
    }
}
