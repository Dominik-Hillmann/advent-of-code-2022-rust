use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

enum MyDraw {
    X,
    Y,
    Z
}

impl FromStr for MyDraw {

    type Err = ();

    fn from_str(s: &str) -> Result<MyDraw, Self::Err> {
        match s {
            "X" => Ok(MyDraw::X),
            "Y" => Ok(MyDraw::Y),
            "Z" => Ok(MyDraw::Z),
            _ => Err(()),
        }
    }
}

enum TheirDraw {
    A,
    B,
    C   
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parser() {
        assert_eq!(MyDraw::from_str("X"), MyDraw::X);
    }
}
