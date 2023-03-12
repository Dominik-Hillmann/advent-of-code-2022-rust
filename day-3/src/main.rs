use std::fs;

fn main() {
    println!("{}!", read_lines("sample-input.txt"));
    let text = read_lines("sample-input.txt");
    let lines: Vec<&str> = text.split("\n").collect();

    for l in lines {
        let line_length : usize = l.len();
        /*const half_length: usize = line_length / 2;
        let line_chars: Vec<char> = l.chars().collect();


        let first_half: [char; line_length / 2] = line_chars[0..(line_length / 2)];
        let second_half: [char; line_length / 2]  = line_chars[(line_length / 2)..line_length];*/

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
    if length / 2 != 0 {
        return Err("The length of the input string must be divisible by 2.");
    }

    let mut first_half: Vec<char> = Vec::new();
    let mut second_half: Vec<char> = Vec::new();
    

    Ok((vec!['a'], vec!['b']))
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::get_halves;

    #[test]
    fn test_get_halves_happy_path() {
        let (first_half, second_half) = get_halves("abcd");
        assert_eq!(first_half, vec!['a', 'b']);
        assert_eq!(second_half, vec!['c', 'd']);
    }
}
