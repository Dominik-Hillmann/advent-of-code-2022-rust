use std::fs;
use std::string::String;

#[derive(Debug, PartialEq)]
enum Draw {
    Rock,
    Paper,
    Scissors,
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

            _ => Err(s),
        }
    }

    fn points_worth(&self) -> u16 {
        match *self {
            Draw::Rock => 1,
            Draw::Paper => 2,
            Draw::Scissors => 3
        }
    }

    fn compare_to(&self, other: Draw) -> GameOutcome {
        match *self {
            Draw::Rock => match other {
                Draw::Rock => GameOutcome::Tie,
                Draw::Paper => GameOutcome::Lose,
                Draw::Scissors => GameOutcome::Win,
            },
            Draw::Paper => match other {
                Draw::Rock => GameOutcome::Win,
                Draw::Paper => GameOutcome::Tie,
                Draw::Scissors => GameOutcome::Lose,
            },
            Draw::Scissors => match other {
                Draw::Rock => GameOutcome::Lose,
                Draw::Paper => GameOutcome::Win,
                Draw::Scissors => GameOutcome::Tie,
            },
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum GameOutcome {
    Win,
    Tie,
    Lose,
}

impl GameOutcome {
    fn points_worth(&self) -> u16 {
        match *self {
            GameOutcome::Win => 6,
            GameOutcome::Tie => 3,
            GameOutcome::Lose => 0
        }
    }
}

fn main() {
    let lines = read_lines("./puzzle-input.txt");
    
    let mut points: u16 = 0;
    for line in lines.split("\n") {
        let (their_draw, my_draw) = line_to_draws(line);

        let points_round = match my_draw {
            Ok(my_draw) => match their_draw {
                Ok(their_draw) => {
                    let tmp = my_draw.points_worth() + my_draw.compare_to(their_draw).points_worth();
                    // print!("{} vs {} -> {} points", my_draw., their_draw, tmp);
                    tmp
                },
                Err(s) => panic!("{}", s)
            },
            Err(s) => panic!("{}", s)
        };
        
        print!("+ {}\n", points_round);
        points += points_round;
    }

    print!("{}", points);
}

fn read_lines(filename: &str) -> String {
    let file_path = String::from(filename);

    let mut contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents = contents.replace("\r\n", "\n");

    contents
}

fn line_to_draws(line: &str) -> (Result<Draw, &str>, Result<Draw, &str>) {
    let letters: Vec<&str> = line.split(" ").collect();
    let [my_draw_letter, their_draw_letter] = <[&str; 2]>::try_from(letters).ok().unwrap();
    
    (Draw::from(my_draw_letter), Draw::from(their_draw_letter))
}

#[cfg(test)]
mod tests {
    use crate::{Draw, GameOutcome, line_to_draws};
    use std::io;

    #[test]
    fn dislikes_illegal_letters() {
        assert_eq!(Draw::from("A").is_ok(), true);
        assert_eq!(Draw::from("Q").is_ok(), false);
    }

    #[test]
    fn map_letter_to_draw() {
        let draw = Draw::from("A");

        // assert_eq!(draw.is_ok(), true);
        assert_eq!(draw.unwrap(), Draw::Rock);
    }

    #[test]
    fn test_points_worth() {
        assert_eq!(Draw::Rock.points_worth(), 1);
    }

    #[test]
    fn test_draw_comparison() {
        let outcome = Draw::Rock.compare_to(Draw::Scissors);

        assert_eq!(outcome, GameOutcome::Win);
    }

    #[test]
    fn test_line_to_draws() {
        let (draw_one, draw_two) = line_to_draws("A Y");
        
        assert_eq!(draw_one.is_ok(), true);
        assert_eq!(draw_one.unwrap(), Draw::Rock);
        assert_eq!(draw_two.is_ok(), true);
        assert_eq!(draw_two.unwrap(), Draw::Paper);
    }
}
