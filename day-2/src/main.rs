use std::fs;
use std::string::String;

enum Draw {
  Rock,
  Paper,
    Scissors
}

impl Draw {
    fn from(s: &str) -> Result<Draw, &str> {
        match s {
            "A" => Ok(Draw::Rock),
            "B" => Ok(Draw::Paper),
            "C" => Ok(Draw::Scissors),

            "X" => Ok(Draw::Rock),
            "Y" => Ok(Draw::Paper),
            "Z" => Ok(Draw::Scissors),

            _ => Err(s)
        }
    }
}

fn main() {
    let lines = read_lines("./sample-input.txt");

    for line in lines.split("\n") {
        let letters: Vec<&str> = line.split(" ").collect();
        let [my_draw, their_draw] = <[&str; 2]>::try_from(letters).ok().unwrap();
        print!("{}{} ", my_draw, their_draw);
        /*
        if let [my_draw, their_draw] = letters[0..=1] {
            print!("{} {} - ", my_draw, their_draw);
        } else {
            panic!("Line does not follow two letter pattern.");
        }*/
        // let my_draw = letters[0];
        // let their_draw = letters[1];

        // print!("{} {}", my_draw, their_draw);
    }
}

fn read_lines(filename: &str) -> String {
    let file_path = String::from(filename);

    let mut contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents = contents.replace("\r\n", "\n");

    contents
}

#[cfg(test)]
mod tests {
    use std::io;
    use crate::Draw;
    use io::Result;

    #[test]
    fn panics_on_illegal_letter() {
        assert_eq!(Draw::from("A").is_ok(), true);
        assert_eq!(Draw::from("Q").is_ok(), false);
    }

    #[test]
    fn map_letter_to_draw() -> io::Result<()> {
        match Draw::from("A") {
            Ok(Draw::Rock) => Ok(()),
            _ => Err(())
        }
    }
}
